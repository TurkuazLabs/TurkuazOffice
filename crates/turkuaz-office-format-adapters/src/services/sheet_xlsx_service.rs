// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/services/sheet_xlsx_service.rs
// # 📌 Amac: XLSX ZIP/XML modelini canonical SheetDocument modeline map eder ve canonical workbook'u XLSX byte akimina cevirir
// # 📌 Modul - FileType: Service - Rust
// Version: 0.3.0
// Aciklama: Value-only SpreadsheetML import/export, worksheet/cell validation ve relationship target normalization business kurallarini uygular
// Bagimli Oldugu Katman: Service -> Model -> Tool -> Sheet

use std::collections::{BTreeMap, HashSet};

use turkuaz_office_core::config::constants::DEFAULT_DOCUMENT_TITLE;
use turkuaz_office_core::DocumentSchemaVersion;
use turkuaz_office_sheet::config::constants::{
    MAX_CELL_TEXT_LENGTH, MAX_WORKSHEET_NAME_LENGTH,
};
use turkuaz_office_sheet::{
    CellAddress, CellReferenceTool, CellValue, SheetDocument, SheetIdTool, Worksheet,
};

use crate::config::sheet_constants::{
    MAX_XLSX_WORKSHEETS, XLSX_CONTENT_TYPES_ENTRY, XLSX_ROOT_RELATIONSHIPS_ENTRY,
    XLSX_SHARED_STRINGS_ENTRY, XLSX_WORKBOOK_ENTRY, XLSX_WORKBOOK_RELATIONSHIPS_ENTRY,
    XLSX_WORKSHEET_PREFIX, XLSX_WORKSHEET_SUFFIX,
};
use crate::models::sheet_xlsx_model::{
    XlsxCellModel, XlsxCellValue, XlsxWorkbookModel, XlsxWorksheetModel,
};
use crate::tools::sheet_xlsx_archive_tool::{
    SheetXlsxArchiveError, SheetXlsxArchiveTool,
};
use crate::tools::sheet_xlsx_xml_tool::{SheetXlsxXmlError, SheetXlsxXmlTool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SheetXlsxError {
    InvalidPackage,
    ResourceLimit,
    InvalidWorksheetName,
    InvalidCellReference,
    CellTextTooLong,
    InvalidCellText,
    CellNumberNotFinite,
    UnsupportedFormula,
    UnsupportedCellType,
}

pub struct SheetXlsxService;

impl SheetXlsxService {
    pub fn import<I>(
        id_tool: &I,
        title: impl Into<String>,
        bytes: &[u8],
    ) -> Result<SheetDocument, SheetXlsxError>
    where
        I: SheetIdTool,
    {
        let entries = SheetXlsxArchiveTool::decode(bytes).map_err(Self::map_archive_error)?;
        for required in [
            XLSX_CONTENT_TYPES_ENTRY,
            XLSX_ROOT_RELATIONSHIPS_ENTRY,
            XLSX_WORKBOOK_ENTRY,
            XLSX_WORKBOOK_RELATIONSHIPS_ENTRY,
        ] {
            if !entries.contains_key(required) {
                return Err(SheetXlsxError::InvalidPackage);
            }
        }

        let root_relationships = entries
            .get(XLSX_ROOT_RELATIONSHIPS_ENTRY)
            .ok_or(SheetXlsxError::InvalidPackage)?;
        SheetXlsxXmlTool::validate_root_relationships(root_relationships)
            .map_err(Self::map_xml_error)?;

        let workbook_xml = entries
            .get(XLSX_WORKBOOK_ENTRY)
            .ok_or(SheetXlsxError::InvalidPackage)?;
        let relationship_xml = entries
            .get(XLSX_WORKBOOK_RELATIONSHIPS_ENTRY)
            .ok_or(SheetXlsxError::InvalidPackage)?;

        let descriptors =
            SheetXlsxXmlTool::parse_workbook(workbook_xml).map_err(Self::map_xml_error)?;
        if descriptors.is_empty() || descriptors.len() > MAX_XLSX_WORKSHEETS {
            return Err(SheetXlsxError::ResourceLimit);
        }

        let relationships = SheetXlsxXmlTool::parse_workbook_relationships(relationship_xml)
            .map_err(Self::map_xml_error)?;
        let shared_strings = entries
            .get(XLSX_SHARED_STRINGS_ENTRY)
            .map(|xml| SheetXlsxXmlTool::parse_shared_strings(xml))
            .transpose()
            .map_err(Self::map_xml_error)?
            .unwrap_or_default();

        let mut worksheets = Vec::with_capacity(descriptors.len());
        let mut worksheet_names = HashSet::new();
        for descriptor in descriptors {
            let normalized_name = descriptor.name.to_lowercase();
            if !worksheet_names.insert(normalized_name) {
                return Err(SheetXlsxError::InvalidWorksheetName);
            }
            Self::validate_worksheet_name(&descriptor.name)?;
            let target = relationships
                .get(&descriptor.relationship_id)
                .ok_or(SheetXlsxError::InvalidPackage)?;
            let entry_name = Self::worksheet_entry_name(target)?;
            let worksheet_xml = entries
                .get(&entry_name)
                .ok_or(SheetXlsxError::InvalidPackage)?;
            let model = SheetXlsxXmlTool::parse_worksheet(
                descriptor.name,
                worksheet_xml,
                &shared_strings,
            )
            .map_err(Self::map_xml_error)?;
            worksheets.push(Self::to_canonical_worksheet(id_tool, model)?);
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
            worksheets,
        })
    }

    pub fn export(document: &SheetDocument) -> Result<Vec<u8>, SheetXlsxError> {
        if document.worksheets.is_empty() || document.worksheets.len() > MAX_XLSX_WORKSHEETS {
            return Err(SheetXlsxError::ResourceLimit);
        }

        let mut worksheet_names = HashSet::new();
        let worksheets = document
            .worksheets
            .iter()
            .map(|worksheet| {
                let normalized_name = worksheet.name.to_lowercase();
                if !worksheet_names.insert(normalized_name) {
                    return Err(SheetXlsxError::InvalidWorksheetName);
                }
                Self::to_xlsx_worksheet(worksheet)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let workbook = XlsxWorkbookModel { worksheets };

        let mut entries = Vec::with_capacity(workbook.worksheets.len() + 4);
        entries.push((
            XLSX_CONTENT_TYPES_ENTRY.to_owned(),
            SheetXlsxXmlTool::content_types_xml(workbook.worksheets.len()).into_bytes(),
        ));
        entries.push((
            XLSX_ROOT_RELATIONSHIPS_ENTRY.to_owned(),
            SheetXlsxXmlTool::root_relationships_xml().into_bytes(),
        ));
        entries.push((
            XLSX_WORKBOOK_ENTRY.to_owned(),
            SheetXlsxXmlTool::workbook_xml(&workbook.worksheets).into_bytes(),
        ));
        entries.push((
            XLSX_WORKBOOK_RELATIONSHIPS_ENTRY.to_owned(),
            SheetXlsxXmlTool::workbook_relationships_xml(workbook.worksheets.len()).into_bytes(),
        ));

        for (index, worksheet) in workbook.worksheets.iter().enumerate() {
            entries.push((
                format!(
                    "{XLSX_WORKSHEET_PREFIX}{}{XLSX_WORKSHEET_SUFFIX}",
                    index + 1
                ),
                SheetXlsxXmlTool::worksheet_xml(worksheet).into_bytes(),
            ));
        }

        SheetXlsxArchiveTool::encode(&entries).map_err(Self::map_archive_error)
    }

    fn to_canonical_worksheet<I>(
        id_tool: &I,
        model: XlsxWorksheetModel,
    ) -> Result<Worksheet, SheetXlsxError>
    where
        I: SheetIdTool,
    {
        let mut cells = BTreeMap::new();
        for cell in model.cells {
            let address = CellReferenceTool::parse(&cell.reference)
                .map_err(|_| SheetXlsxError::InvalidCellReference)?;
            let value = match cell.value {
                XlsxCellValue::Text(value) => {
                    Self::validate_cell_text(&value)?;
                    CellValue::Text(value)
                }
                XlsxCellValue::Number(value) => {
                    if !value.is_finite() {
                        return Err(SheetXlsxError::CellNumberNotFinite);
                    }
                    CellValue::Number(value)
                }
                XlsxCellValue::Boolean(value) => CellValue::Boolean(value),
            };
            if cells.insert(address, value).is_some() {
                return Err(SheetXlsxError::InvalidPackage);
            }
        }

        Ok(Worksheet {
            id: id_tool.next_worksheet_id(),
            name: model.name,
            cells,
        })
    }

    fn to_xlsx_worksheet(worksheet: &Worksheet) -> Result<XlsxWorksheetModel, SheetXlsxError> {
        Self::validate_worksheet_name(&worksheet.name)?;
        let mut cells = Vec::with_capacity(worksheet.cells.len());

        for (address, value) in &worksheet.cells {
            let reference = CellReferenceTool::format(*address)
                .map_err(|_| SheetXlsxError::InvalidCellReference)?;
            let value = match value {
                CellValue::Text(value) => {
                    Self::validate_cell_text(value)?;
                    XlsxCellValue::Text(value.clone())
                }
                CellValue::Number(value) => {
                    if !value.is_finite() {
                        return Err(SheetXlsxError::CellNumberNotFinite);
                    }
                    XlsxCellValue::Number(*value)
                }
                CellValue::Boolean(value) => XlsxCellValue::Boolean(*value),
            };
            cells.push(XlsxCellModel { reference, value });
        }

        Ok(XlsxWorksheetModel {
            name: worksheet.name.clone(),
            cells,
        })
    }

    fn validate_worksheet_name(name: &str) -> Result<(), SheetXlsxError> {
        let trimmed = name.trim();
        if trimmed.is_empty()
            || trimmed.chars().count() > MAX_WORKSHEET_NAME_LENGTH
            || trimmed.starts_with('\'')
            || trimmed.ends_with('\'')
            || trimmed.chars().any(|character| {
                !Self::is_xml_1_0_character(character)
                    || matches!(character, '[' | ']' | ':' | '*' | '?' | '/' | '\\')
            })
        {
            return Err(SheetXlsxError::InvalidWorksheetName);
        }
        Ok(())
    }

    fn validate_cell_text(value: &str) -> Result<(), SheetXlsxError> {
        if value.chars().count() > MAX_CELL_TEXT_LENGTH {
            return Err(SheetXlsxError::CellTextTooLong);
        }
        if value.chars().any(|character| !Self::is_xml_1_0_character(character)) {
            return Err(SheetXlsxError::InvalidCellText);
        }
        Ok(())
    }

    fn is_xml_1_0_character(character: char) -> bool {
        matches!(character, '\u{0009}' | '\u{000A}' | '\u{000D}')
            || ('\u{0020}'..='\u{D7FF}').contains(&character)
            || ('\u{E000}'..='\u{FFFD}').contains(&character)
            || ('\u{10000}'..='\u{10FFFF}').contains(&character)
    }

    fn worksheet_entry_name(target: &str) -> Result<String, SheetXlsxError> {
        let trimmed = target.trim();
        if trimmed.is_empty()
            || trimmed.contains('\\')
            || trimmed.contains(':')
            || trimmed
                .trim_start_matches('/')
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err(SheetXlsxError::InvalidPackage);
        }

        let relative = trimmed.trim_start_matches('/');
        if relative.starts_with("xl/") {
            Ok(relative.to_owned())
        } else {
            Ok(format!("xl/{relative}"))
        }
    }

    fn map_archive_error(error: SheetXlsxArchiveError) -> SheetXlsxError {
        match error {
            SheetXlsxArchiveError::PackageTooLarge
            | SheetXlsxArchiveError::TooManyEntries
            | SheetXlsxArchiveError::EntryTooLarge => SheetXlsxError::ResourceLimit,
            SheetXlsxArchiveError::UnsafeEntryName
            | SheetXlsxArchiveError::DuplicateEntry
            | SheetXlsxArchiveError::UnsupportedCompression
            | SheetXlsxArchiveError::ReadFailed
            | SheetXlsxArchiveError::WriteFailed => SheetXlsxError::InvalidPackage,
        }
    }

    fn map_xml_error(error: SheetXlsxXmlError) -> SheetXlsxError {
        match error {
            SheetXlsxXmlError::DocumentTooLarge
            | SheetXlsxXmlError::TooDeep
            | SheetXlsxXmlError::TooManyNodes => SheetXlsxError::ResourceLimit,
            SheetXlsxXmlError::UnsupportedFormula => SheetXlsxError::UnsupportedFormula,
            SheetXlsxXmlError::UnsupportedCellType => SheetXlsxError::UnsupportedCellType,
            SheetXlsxXmlError::InvalidNumber
            | SheetXlsxXmlError::InvalidXml
            | SheetXlsxXmlError::InvalidUtf8
            | SheetXlsxXmlError::InvalidAttribute
            | SheetXlsxXmlError::MissingAttribute
            | SheetXlsxXmlError::SharedStringOutOfBounds
            | SheetXlsxXmlError::DocTypeUnsupported => SheetXlsxError::InvalidPackage,
        }
    }
}
