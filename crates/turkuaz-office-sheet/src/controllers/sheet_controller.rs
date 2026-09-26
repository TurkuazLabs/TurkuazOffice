// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/controllers/sheet_controller.rs
// # 📌 Amac: Sheet request girdilerini alip yalnizca SheetService cagirir
// # 📌 Modul - FileType: Controller - Rust
// Version: 0.3.0
// Aciklama: Create/get/set/clear/cell A1 request yuzeyini business logic tasimadan typed View modeline cevirir
// Bagimli Oldugu Katman: Controller -> Service

use turkuaz_office_core::DocumentId;

use crate::repositories::sheet_document_repository::SheetDocumentRepository;
use crate::services::sheet_service::{SheetError, SheetService};
use crate::services::sheet_types::{CellValue, WorksheetId};
use crate::tools::sheet_id_tool::SheetIdTool;
use crate::views::sheet_view::{
    CellView, EvaluatedCellValueView, SheetDocumentView,
};

pub struct SheetController<R, I>
where
    R: SheetDocumentRepository,
    I: SheetIdTool,
{
    service: SheetService<R, I>,
}

impl<R, I> SheetController<R, I>
where
    R: SheetDocumentRepository,
    I: SheetIdTool,
{
    pub fn new(service: SheetService<R, I>) -> Self {
        Self { service }
    }

    pub fn create(&mut self, title: &str) -> SheetDocumentView {
        self.service.create_document(title).into()
    }

    pub fn get(&self, document_id: &str) -> Option<SheetDocumentView> {
        self.service
            .get_document(&DocumentId::new(document_id))
            .map(SheetDocumentView::from)
    }

    pub fn set_cell_a1(
        &mut self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
        value: CellValue,
    ) -> Result<SheetDocumentView, SheetError> {
        self.service
            .set_cell_by_a1(
                &DocumentId::new(document_id),
                &WorksheetId::new(worksheet_id),
                reference,
                value,
            )
            .map(SheetDocumentView::from)
    }

    pub fn set_formula_a1(
        &mut self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
        source: &str,
    ) -> Result<SheetDocumentView, SheetError> {
        self.service
            .set_formula_by_a1(
                &DocumentId::new(document_id),
                &WorksheetId::new(worksheet_id),
                reference,
                source,
            )
            .map(SheetDocumentView::from)
    }

    pub fn evaluated_cell_a1(
        &self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
    ) -> Result<Option<EvaluatedCellValueView>, SheetError> {
        self.service
            .evaluated_cell_by_a1(
                &DocumentId::new(document_id),
                &WorksheetId::new(worksheet_id),
                reference,
            )
            .map(|value| value.map(EvaluatedCellValueView::from))
    }

    pub fn clear_cell_a1(
        &mut self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
    ) -> Result<SheetDocumentView, SheetError> {
        self.service
            .clear_cell_by_a1(
                &DocumentId::new(document_id),
                &WorksheetId::new(worksheet_id),
                reference,
            )
            .map(SheetDocumentView::from)
    }

    pub fn cell_a1(
        &self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
    ) -> Result<Option<CellView>, SheetError> {
        self.service
            .cell_by_a1(
                &DocumentId::new(document_id),
                &WorksheetId::new(worksheet_id),
                reference,
            )
            .map(|cell| cell.map(CellView::from))
    }
}
