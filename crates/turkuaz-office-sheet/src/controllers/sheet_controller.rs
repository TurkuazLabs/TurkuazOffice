// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/controllers/sheet_controller.rs
// # 📌 Amac: Sheet request girdilerini alip yalnizca SheetService cagirir
// # 📌 Modul - FileType: Controller - Rust
// Version: 0.3.0
// Aciklama: Create/get/set/formula/evaluate/clear/cell A1 request yuzeyini business logic tasimadan typed View modeline cevirir
// Bagimli Oldugu Katman: Controller -> Service

use turkuaz_office_core::DocumentId;

use crate::repositories::sheet_document_repository::SheetDocumentRepository;
use crate::services::sheet_service::{SheetError, SheetService};
use crate::services::sheet_types::{
    CellFormat, CellValue, ChartId, ChartType, SheetFilter, SheetRange, SheetSort, WorksheetId,
};
use crate::tools::sheet_id_tool::SheetIdTool;
use crate::views::sheet_view::{
    CellFormatView, CellView, ChartDataView, SheetDocumentView, SheetRowQueryView,
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
        expression: &str,
    ) -> Result<SheetDocumentView, SheetError> {
        self.service
            .set_formula_by_a1(
                &DocumentId::new(document_id),
                &WorksheetId::new(worksheet_id),
                reference,
                expression,
            )
            .map(SheetDocumentView::from)
    }

    pub fn set_cell_format_a1(
        &mut self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
        format: CellFormat,
    ) -> Result<SheetDocumentView, SheetError> {
        self.service
            .set_cell_format_by_a1(
                &DocumentId::new(document_id),
                &WorksheetId::new(worksheet_id),
                reference,
                format,
            )
            .map(SheetDocumentView::from)
    }

    pub fn cell_format_a1(
        &self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
    ) -> Result<CellFormatView, SheetError> {
        self.service
            .cell_format_by_a1(
                &DocumentId::new(document_id),
                &WorksheetId::new(worksheet_id),
                reference,
            )
            .map(CellFormatView::from)
    }

    pub fn query_rows(
        &self,
        document_id: &str,
        worksheet_id: &str,
        range: SheetRange,
        filter: Option<&SheetFilter>,
        sort: Option<SheetSort>,
    ) -> Result<SheetRowQueryView, SheetError> {
        self.service
            .query_rows(
                &DocumentId::new(document_id),
                &WorksheetId::new(worksheet_id),
                range,
                filter,
                sort,
            )
            .map(SheetRowQueryView::from)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_chart(
        &mut self,
        document_id: &str,
        worksheet_id: &str,
        chart_type: ChartType,
        title: &str,
        start_row: u32,
        end_row: u32,
        category_column: u32,
        value_column: u32,
    ) -> Result<SheetDocumentView, SheetError> {
        self.service
            .create_chart(
                &DocumentId::new(document_id),
                &WorksheetId::new(worksheet_id),
                chart_type,
                title,
                start_row,
                end_row,
                category_column,
                value_column,
            )
            .map(SheetDocumentView::from)
    }

    pub fn remove_chart(
        &mut self,
        document_id: &str,
        chart_id: &str,
    ) -> Result<SheetDocumentView, SheetError> {
        self.service
            .remove_chart(&DocumentId::new(document_id), &ChartId::new(chart_id))
            .map(SheetDocumentView::from)
    }

    pub fn chart_data(
        &self,
        document_id: &str,
        chart_id: &str,
    ) -> Result<ChartDataView, SheetError> {
        self.service
            .chart_data(&DocumentId::new(document_id), &ChartId::new(chart_id))
            .map(ChartDataView::from)
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

    pub fn evaluated_cell_a1(
        &self,
        document_id: &str,
        worksheet_id: &str,
        reference: &str,
    ) -> Result<Option<CellView>, SheetError> {
        self.service
            .evaluated_cell_by_a1(
                &DocumentId::new(document_id),
                &WorksheetId::new(worksheet_id),
                reference,
            )
            .map(|cell| cell.map(CellView::from))
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
