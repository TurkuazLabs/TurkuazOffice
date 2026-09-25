// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/services/sheet_csv_service.rs
// # 📌 Amac: CSV satir-hucre matrisini canonical SheetDocument/Worksheet modeline map eder ve geri export eder
// # 📌 Modul - FileType: Service - Rust
// Version: 0.3.0
// Aciklama: CSV importu type metadata olmadigi icin text-only yapar; exportta canonical text/number/boolean degerlerini CSV stringine cevirir
// Bagimli Oldugu Katman: Service -> Tool -> Sheet

use std::collections::BTreeMap;

use turkuaz_office_core::config::constants::DEFAULT_DOCUMENT_TITLE;
use turkuaz_office_core::DocumentSchemaVersion;
use turkuaz_office_sheet::config::constants::DEFAULT_WORKSHEET_NAME;
use turkuaz_office_sheet::{
    CellAddress, CellValue, SheetDocument, SheetIdTool, Worksheet,
};

use crate::tools::sheet_csv_tool::{SheetCsvTool, SheetCsvToolError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SheetCsvError {
    InvalidCsv,
    ResourceLimit,
}

pub struct SheetCsvService;

impl SheetCsvService {
    pub fn import<I>(
        id_tool: &I,
        title: impl Into<String>,
        bytes: &[u8],
    ) -> Result<SheetDocument, SheetCsvError>
    where
        I: SheetIdTool,
    {
        let rows = SheetCsvTool::parse(bytes).map_err(Self::map_tool_error)?;
        let mut cells = BTreeMap::new();

        for (row_index, row) in rows.into_iter().enumerate() {
            for (column_index, value) in row.into_iter().enumerate() {
                if value.is_empty() {
                    continue;
                }
                let row = u32::try_from(row_index).map_err(|_| SheetCsvError::ResourceLimit)?;
                let column =
                    u32::try_from(column_index).map_err(|_| SheetCsvError::ResourceLimit)?;
                cells.insert(
                    CellAddress { row, column },
                    CellValue::Text(value),
                );
            }
        }

        let raw_title = title.into();
        let normalized = raw_title.trim();
        let title = if normalized.is_empty() {
            DEFAULT_DOCUMENT_TITLE.to_owned()
        } else {
            normalized.to_owned()
        };

        Ok(SheetDocument {
            id: id_tool.next_document_id(),
            title,
            schema_version: DocumentSchemaVersion::current(),
            revision: 0,
            worksheets: vec![Worksheet {
                id: id_tool.next_worksheet_id(),
                name: DEFAULT_WORKSHEET_NAME.to_owned(),
                cells,
            }],
        })
    }

    pub fn export_worksheet(worksheet: &Worksheet) -> Result<Vec<u8>, SheetCsvError> {
        let Some(last_address) = worksheet.cells.keys().next_back().copied() else {
            return Ok(Vec::new());
        };

        let row_count =
            usize::try_from(last_address.row).map_err(|_| SheetCsvError::ResourceLimit)? + 1;
        let max_column = worksheet
            .cells
            .keys()
            .map(|address| address.column)
            .max()
            .unwrap_or(0);
        let column_count =
            usize::try_from(max_column).map_err(|_| SheetCsvError::ResourceLimit)? + 1;

        let mut rows = vec![vec![String::new(); column_count]; row_count];
        for (address, value) in &worksheet.cells {
            let row = usize::try_from(address.row).map_err(|_| SheetCsvError::ResourceLimit)?;
            let column =
                usize::try_from(address.column).map_err(|_| SheetCsvError::ResourceLimit)?;
            rows[row][column] = Self::value_text(value);
        }

        SheetCsvTool::encode(&rows).map_err(Self::map_tool_error)
    }

    fn value_text(value: &CellValue) -> String {
        match value {
            CellValue::Text(text) => text.clone(),
            CellValue::Number(number) => number.to_string(),
            CellValue::Boolean(value) => {
                if *value {
                    "TRUE".to_owned()
                } else {
                    "FALSE".to_owned()
                }
            }
        }
    }

    fn map_tool_error(error: SheetCsvToolError) -> SheetCsvError {
        match error {
            SheetCsvToolError::InvalidUtf8 | SheetCsvToolError::InvalidCsv => {
                SheetCsvError::InvalidCsv
            }
            SheetCsvToolError::TooLarge
            | SheetCsvToolError::TooManyRows
            | SheetCsvToolError::TooManyColumns => SheetCsvError::ResourceLimit,
        }
    }
}
