// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/sheet_dto.rs
// # 📌 Amac: Sheet domain View modellerini Tauri frontend icin stabil serializable DTO kontratina cevirir
// # 📌 Modul - FileType: View - Rust
// Version: 0.4.0
// Aciklama: Sheet belge, cell/format ve non-mutating row-query DTO'larini camelCase IPC kontrati olarak sunar
// Bagimli Oldugu Katman: View

use serde::{Deserialize, Serialize};
use turkuaz_office_sheet::{
    CellFormat, CellFormatView, CellValueView, CellView, HorizontalAlignment,
    HorizontalAlignmentView, SheetDocumentView, SheetFilter, SheetFilterCondition, SheetRange,
    SheetRowQueryView, SheetSort, SheetSortDirection, WorksheetView,
};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SheetHorizontalAlignmentDto {
    General,
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetCellFormatDto {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub horizontal_alignment: SheetHorizontalAlignmentDto,
    pub decimal_places: Option<u8>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum SheetFilterConditionDto {
    NonEmpty,
    TextContains(String),
    NumberGreaterThan(f64),
    NumberLessThan(f64),
    BooleanEquals(bool),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetFilterDto {
    pub column: u32,
    pub condition: SheetFilterConditionDto,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SheetSortDirectionDto {
    Ascending,
    Descending,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetSortDto {
    pub column: u32,
    pub direction: SheetSortDirectionDto,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetRangeDto {
    pub start_row: u32,
    pub end_row: u32,
    pub start_column: u32,
    pub end_column: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetRowQueryRequestDto {
    pub document_id: String,
    pub worksheet_id: String,
    pub range: SheetRangeDto,
    pub filter: Option<SheetFilterDto>,
    pub sort: Option<SheetSortDto>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetRowQueryResultDto {
    pub rows: Vec<u32>,
}

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

impl From<SheetFilterConditionDto> for SheetFilterCondition {
    fn from(condition: SheetFilterConditionDto) -> Self {
        match condition {
            SheetFilterConditionDto::NonEmpty => Self::NonEmpty,
            SheetFilterConditionDto::TextContains(value) => Self::TextContains(value),
            SheetFilterConditionDto::NumberGreaterThan(value) => Self::NumberGreaterThan(value),
            SheetFilterConditionDto::NumberLessThan(value) => Self::NumberLessThan(value),
            SheetFilterConditionDto::BooleanEquals(value) => Self::BooleanEquals(value),
        }
    }
}

impl From<SheetFilterDto> for SheetFilter {
    fn from(filter: SheetFilterDto) -> Self {
        Self {
            column: filter.column,
            condition: filter.condition.into(),
        }
    }
}

impl From<SheetSortDirectionDto> for SheetSortDirection {
    fn from(direction: SheetSortDirectionDto) -> Self {
        match direction {
            SheetSortDirectionDto::Ascending => Self::Ascending,
            SheetSortDirectionDto::Descending => Self::Descending,
        }
    }
}

impl From<SheetSortDto> for SheetSort {
    fn from(sort: SheetSortDto) -> Self {
        Self {
            column: sort.column,
            direction: sort.direction.into(),
        }
    }
}

impl From<SheetRangeDto> for SheetRange {
    fn from(range: SheetRangeDto) -> Self {
        Self {
            start_row: range.start_row,
            end_row: range.end_row,
            start_column: range.start_column,
            end_column: range.end_column,
        }
    }
}

impl From<SheetRowQueryView> for SheetRowQueryResultDto {
    fn from(result: SheetRowQueryView) -> Self {
        Self { rows: result.rows }
    }
}

impl From<HorizontalAlignmentView> for SheetHorizontalAlignmentDto {
    fn from(value: HorizontalAlignmentView) -> Self {
        match value {
            HorizontalAlignmentView::General => Self::General,
            HorizontalAlignmentView::Left => Self::Left,
            HorizontalAlignmentView::Center => Self::Center,
            HorizontalAlignmentView::Right => Self::Right,
        }
    }
}

impl From<SheetHorizontalAlignmentDto> for HorizontalAlignment {
    fn from(value: SheetHorizontalAlignmentDto) -> Self {
        match value {
            SheetHorizontalAlignmentDto::General => Self::General,
            SheetHorizontalAlignmentDto::Left => Self::Left,
            SheetHorizontalAlignmentDto::Center => Self::Center,
            SheetHorizontalAlignmentDto::Right => Self::Right,
        }
    }
}

impl From<CellFormatView> for SheetCellFormatDto {
    fn from(format: CellFormatView) -> Self {
        Self {
            bold: format.bold,
            italic: format.italic,
            underline: format.underline,
            horizontal_alignment: format.horizontal_alignment.into(),
            decimal_places: format.decimal_places,
        }
    }
}

impl From<SheetCellFormatDto> for CellFormat {
    fn from(format: SheetCellFormatDto) -> Self {
        Self {
            bold: format.bold,
            italic: format.italic,
            underline: format.underline,
            horizontal_alignment: format.horizontal_alignment.into(),
            decimal_places: format.decimal_places,
        }
    }
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
