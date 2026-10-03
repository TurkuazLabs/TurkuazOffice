// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/services/sheet_desktop_service.rs
// # 📌 Amac: Desktop Sheet requestlerini canonical Sheet Controller akisina cevirir
// # 📌 Modul - FileType: Service - Rust
// Version: 0.4.0
// Aciklama: Create/get/cell/formula/clear islemlerini domain Controller uzerinden koordine eder
// Bagimli Oldugu Katman: Service -> Controller -> Service -> Repo -> Tool

use turkuaz_office_sheet::{
    CellValue, InMemorySheetDocumentRepository, SequentialSheetIdTool, SheetController,
    SheetDocumentView, SheetError, SheetService,
};

type DesktopSheetController =
    SheetController<InMemorySheetDocumentRepository, SequentialSheetIdTool>;

pub struct SheetDesktopService {
    controller: DesktopSheetController,
}

impl SheetDesktopService {
    pub fn new() -> Self {
        let service = SheetService::new(
            InMemorySheetDocumentRepository::new(),
            SequentialSheetIdTool::new(),
        );
        Self {
            controller: SheetController::new(service),
        }
    }

    pub fn create_document(&mut self) -> SheetDocumentView {
        self.controller.create("")
    }

    pub fn get_document(&self, document_id: &str) -> Result<SheetDocumentView, SheetError> {
        self.controller
            .get(document_id)
            .ok_or(SheetError::DocumentNotFound)
    }

    pub fn set_text(
        &mut self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
        value: &str,
    ) -> Result<SheetDocumentView, SheetError> {
        self.controller.set_cell_a1(
            document_id,
            worksheet_id,
            reference,
            CellValue::Text(value.to_owned()),
        )
    }

    pub fn set_number(
        &mut self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
        value: f64,
    ) -> Result<SheetDocumentView, SheetError> {
        self.controller.set_cell_a1(
            document_id,
            worksheet_id,
            reference,
            CellValue::Number(value),
        )
    }

    pub fn set_boolean(
        &mut self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
        value: bool,
    ) -> Result<SheetDocumentView, SheetError> {
        self.controller.set_cell_a1(
            document_id,
            worksheet_id,
            reference,
            CellValue::Boolean(value),
        )
    }

    pub fn set_formula(
        &mut self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
        expression: &str,
    ) -> Result<SheetDocumentView, SheetError> {
        self.controller
            .set_formula_a1(document_id, worksheet_id, reference, expression)
    }

    pub fn clear_cell(
        &mut self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
    ) -> Result<SheetDocumentView, SheetError> {
        self.controller
            .clear_cell_a1(document_id, worksheet_id, reference)
    }
}

impl Default for SheetDesktopService {
    fn default() -> Self {
        Self::new()
    }
}
