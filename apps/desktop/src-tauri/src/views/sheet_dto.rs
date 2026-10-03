// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/sheet_dto.rs
// # 📌 Amac: Sheet domain View modellerini Tauri frontend icin stabil serializable DTO kontratina cevirir
// # 📌 Modul - FileType: View - Rust
// Version: 0.4.0
// Aciklama: Sheet belge, worksheet, raw cell ve evaluated cell degerlerini camelCase IPC read-model olarak sunar
// Bagimli Oldugu Katman: View

use serde::Serialize;
use turkuaz_office_sheet::{CellValueView, CellView, SheetDocumentView, WorksheetView};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetDocumentDto {
    pub id: String,
    pub title: String,
    pub revision: u64,
    pub worksheets: Vec<SheetWorksheetDto>,
    pub chart_count: usize,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetWorksheetDto {
    pub id: String,
    pub name: String,
    pub cell_count: usize,
    pub cells: Vec<SheetCellDto>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetCellDto {
    pub row: u32,
    pub column: u32,
    pub value: SheetCellValueDto,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum SheetCellValueDto {
    Text(String),
    Number(f64),
    Boolean(bool),
    Formula(String),
}

impl From<CellValueView> for SheetCellValueDto {
    fn from(value: CellValueView) -> Self {
        match value {
            CellValueView::Text(text) => Self::Text(text),
            CellValueView::Number(number) => Self::Number(number),
            CellValueView::Boolean(value) => Self::Boolean(value),
            CellValueView::Formula(expression) => Self::Formula(expression),
        }
    }
}

impl From<CellView> for SheetCellDto {
    fn from(cell: CellView) -> Self {
        Self {
            row: cell.row,
            column: cell.column,
            value: cell.value.into(),
        }
    }
}

impl From<WorksheetView> for SheetWorksheetDto {
    fn from(worksheet: WorksheetView) -> Self {
        Self {
            id: worksheet.id,
            name: worksheet.name,
            cell_count: worksheet.cell_count,
            cells: worksheet
                .cells
                .into_iter()
                .map(SheetCellDto::from)
                .collect(),
        }
    }
}

impl From<SheetDocumentView> for SheetDocumentDto {
    fn from(document: SheetDocumentView) -> Self {
        Self {
            id: document.id,
            title: document.title,
            revision: document.revision,
            worksheets: document
                .worksheets
                .into_iter()
                .map(SheetWorksheetDto::from)
                .collect(),
            chart_count: document.charts.len(),
        }
    }
}
