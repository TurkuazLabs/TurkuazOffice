// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/services/sheet_service.rs
// # 📌 Amac: Sheet document yasam dongusu, cell validation ve sparse mutation business kurallarini yonetir
// # 📌 Modul - FileType: Service - Rust
// Version: 0.3.0
// Aciklama: Default worksheet create, A1 cell set/get/clear, revision ve value/grid limitlerini Repo/Tool uzerinden koordine eder
// Bagimli Oldugu Katman: Service -> Repo -> Tool

use std::collections::BTreeMap;

use turkuaz_office_core::config::constants::DEFAULT_DOCUMENT_TITLE;
use turkuaz_office_core::{DocumentId, DocumentSchemaVersion};

use crate::config::constants::{
    DEFAULT_WORKSHEET_NAME, MAX_CELL_TEXT_LENGTH, MAX_SHEET_COLUMNS, MAX_SHEET_ROWS,
};
use crate::repositories::sheet_document_repository::SheetDocumentRepository;
use crate::services::sheet_types::{
    Cell, CellAddress, CellValue, SheetDocument, Worksheet, WorksheetId,
};
use crate::tools::cell_reference_tool::CellReferenceTool;
use crate::tools::sheet_id_tool::SheetIdTool;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SheetError {
    DocumentNotFound,
    WorksheetNotFound,
    InvalidCellReference,
    CellOutOfBounds,
    CellTextTooLong,
    CellNumberNotFinite,
}

pub struct SheetService<R, I>
where
    R: SheetDocumentRepository,
    I: SheetIdTool,
{
    repository: R,
    id_tool: I,
}

impl<R, I> SheetService<R, I>
where
    R: SheetDocumentRepository,
    I: SheetIdTool,
{
    pub fn new(repository: R, id_tool: I) -> Self {
        Self {
            repository,
            id_tool,
        }
    }

    pub fn create_document(&mut self, title: impl Into<String>) -> SheetDocument {
        let raw_title = title.into();
        let normalized_title = raw_title.trim();
        let title = if normalized_title.is_empty() {
            DEFAULT_DOCUMENT_TITLE.to_owned()
        } else {
            normalized_title.to_owned()
        };

        let document = SheetDocument {
            id: self.id_tool.next_document_id(),
            title,
            schema_version: DocumentSchemaVersion::current(),
            revision: 0,
            worksheets: vec![Worksheet {
                id: self.id_tool.next_worksheet_id(),
                name: DEFAULT_WORKSHEET_NAME.to_owned(),
                cells: BTreeMap::new(),
            }],
        };
        self.repository.save(document.clone());
        document
    }

    pub fn get_document(&self, id: &DocumentId) -> Option<SheetDocument> {
        self.repository.find(id)
    }

    pub fn set_cell_by_a1(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        reference: &str,
        value: CellValue,
    ) -> Result<SheetDocument, SheetError> {
        let address =
            CellReferenceTool::parse(reference).map_err(|_| SheetError::InvalidCellReference)?;
        self.set_cell(document_id, worksheet_id, address, value)
    }

    pub fn set_cell(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        address: CellAddress,
        value: CellValue,
    ) -> Result<SheetDocument, SheetError> {
        Self::validate_address(address)?;
        Self::validate_value(&value)?;

        let mut document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        let worksheet = document
            .worksheets
            .iter_mut()
            .find(|worksheet| &worksheet.id == worksheet_id)
            .ok_or(SheetError::WorksheetNotFound)?;

        if worksheet.cells.get(&address) == Some(&value) {
            return Ok(document);
        }

        worksheet.cells.insert(address, value);
        document.revision = document.revision.saturating_add(1);
        self.repository.save(document.clone());
        Ok(document)
    }

    pub fn clear_cell_by_a1(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        reference: &str,
    ) -> Result<SheetDocument, SheetError> {
        let address =
            CellReferenceTool::parse(reference).map_err(|_| SheetError::InvalidCellReference)?;
        self.clear_cell(document_id, worksheet_id, address)
    }

    pub fn clear_cell(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        address: CellAddress,
    ) -> Result<SheetDocument, SheetError> {
        Self::validate_address(address)?;
        let mut document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        let worksheet = document
            .worksheets
            .iter_mut()
            .find(|worksheet| &worksheet.id == worksheet_id)
            .ok_or(SheetError::WorksheetNotFound)?;

        if worksheet.cells.remove(&address).is_some() {
            document.revision = document.revision.saturating_add(1);
            self.repository.save(document.clone());
        }
        Ok(document)
    }

    pub fn cell_by_a1(
        &self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        reference: &str,
    ) -> Result<Option<Cell>, SheetError> {
        let address =
            CellReferenceTool::parse(reference).map_err(|_| SheetError::InvalidCellReference)?;
        self.cell(document_id, worksheet_id, address)
    }

    pub fn cell(
        &self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        address: CellAddress,
    ) -> Result<Option<Cell>, SheetError> {
        Self::validate_address(address)?;
        let document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        let worksheet = document
            .worksheets
            .iter()
            .find(|worksheet| &worksheet.id == worksheet_id)
            .ok_or(SheetError::WorksheetNotFound)?;
        Ok(worksheet.cells.get(&address).cloned().map(|value| Cell {
            address,
            value,
        }))
    }

    fn validate_address(address: CellAddress) -> Result<(), SheetError> {
        if address.row >= MAX_SHEET_ROWS || address.column >= MAX_SHEET_COLUMNS {
            return Err(SheetError::CellOutOfBounds);
        }
        Ok(())
    }

    fn validate_value(value: &CellValue) -> Result<(), SheetError> {
        match value {
            CellValue::Text(text) if text.chars().count() > MAX_CELL_TEXT_LENGTH => {
                Err(SheetError::CellTextTooLong)
            }
            CellValue::Number(number) if !number.is_finite() => {
                Err(SheetError::CellNumberNotFinite)
            }
            CellValue::Text(_) | CellValue::Number(_) | CellValue::Boolean(_) => Ok(()),
        }
    }
}
