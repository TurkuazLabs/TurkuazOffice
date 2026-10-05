// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/sheet_dto.rs
// # 📌 Amac: Sheet domain View modellerini Tauri frontend icin stabil serializable DTO kontratina cevirir
// # 📌 Modul - FileType: View - Rust
// Version: 0.7.0
// Aciklama: Sheet belge, cell/format, table, conditional-format, range-summary ve non-mutating row-query DTO'larini camelCase IPC kontrati olarak sunar
// Bagimli Oldugu Katman: View

use serde::{Deserialize, Serialize};
use turkuaz_office_sheet::{
    CellFormat, CellFormatView, CellValueView, CellView, HorizontalAlignment,
    HorizontalAlignmentView, SheetConditionalFormatCondition, SheetConditionalFormatConditionView,
    SheetConditionalFormatMatchView, SheetConditionalFormatRuleView, SheetConditionalFormatStyle,
    SheetConditionalFormatStyleView, SheetDocumentView, SheetFilter, SheetFilterCondition,
    SheetRange, SheetRangeSummaryView, SheetRowQueryView, SheetSort, SheetSortDirection,
    SheetTableView, WorksheetView,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum SheetConditionalFormatConditionDto {
    NumberGreaterThan(f64),
    NumberLessThan(f64),
    NumberEquals(f64),
    TextContains(String),
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SheetConditionalFormatStyleDto {
    Warning,
    Success,
    Accent,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetConditionalFormatCreateRequestDto {
    pub document_id: String,
    pub worksheet_id: String,
    pub range: SheetRangeDto,
    pub condition: SheetConditionalFormatConditionDto,
    pub style: SheetConditionalFormatStyleDto,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetConditionalFormatRemoveRequestDto {
    pub document_id: String,
    pub rule_id: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetConditionalFormatMatchesRequestDto {
    pub document_id: String,
    pub worksheet_id: String,
    pub range: SheetRangeDto,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetConditionalFormatRuleDto {
    pub id: String,
    pub worksheet_id: String,
    pub start_row: u32,
    pub end_row: u32,
    pub start_column: u32,
    pub end_column: u32,
    pub condition: SheetConditionalFormatConditionDto,
    pub style: SheetConditionalFormatStyleDto,
    pub priority: u32,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetConditionalFormatMatchDto {
    pub row: u32,
    pub column: u32,
    pub style: SheetConditionalFormatStyleDto,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetTableCreateRequestDto {
    pub document_id: String,
    pub worksheet_id: String,
    pub range: SheetRangeDto,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetTableRemoveRequestDto {
    pub document_id: String,
    pub table_id: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetTableDto {
    pub id: String,
    pub worksheet_id: String,
    pub name: String,
    pub start_row: u32,
    pub end_row: u32,
    pub start_column: u32,
    pub end_column: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetRangeSummaryRequestDto {
    pub document_id: String,
    pub worksheet_id: String,
    pub range: SheetRangeDto,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetRangeSummaryDto {
    pub count: usize,
    pub numeric_count: usize,
    pub sum: f64,
    pub average: Option<f64>,
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
    pub conditional_formats: Vec<SheetConditionalFormatRuleDto>,
    pub tables: Vec<SheetTableDto>,
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

impl From<SheetConditionalFormatConditionDto> for SheetConditionalFormatCondition {
    fn from(condition: SheetConditionalFormatConditionDto) -> Self {
        match condition {
            SheetConditionalFormatConditionDto::NumberGreaterThan(value) => {
                Self::NumberGreaterThan(value)
            }
            SheetConditionalFormatConditionDto::NumberLessThan(value) => {
                Self::NumberLessThan(value)
            }
            SheetConditionalFormatConditionDto::NumberEquals(value) => Self::NumberEquals(value),
            SheetConditionalFormatConditionDto::TextContains(value) => Self::TextContains(value),
        }
    }
}

impl From<SheetConditionalFormatConditionView> for SheetConditionalFormatConditionDto {
    fn from(condition: SheetConditionalFormatConditionView) -> Self {
        match condition {
            SheetConditionalFormatConditionView::NumberGreaterThan(value) => {
                Self::NumberGreaterThan(value)
            }
            SheetConditionalFormatConditionView::NumberLessThan(value) => {
                Self::NumberLessThan(value)
            }
            SheetConditionalFormatConditionView::NumberEquals(value) => Self::NumberEquals(value),
            SheetConditionalFormatConditionView::TextContains(value) => Self::TextContains(value),
        }
    }
}

impl From<SheetConditionalFormatStyleDto> for SheetConditionalFormatStyle {
    fn from(style: SheetConditionalFormatStyleDto) -> Self {
        match style {
            SheetConditionalFormatStyleDto::Warning => Self::Warning,
            SheetConditionalFormatStyleDto::Success => Self::Success,
            SheetConditionalFormatStyleDto::Accent => Self::Accent,
        }
    }
}

impl From<SheetConditionalFormatStyleView> for SheetConditionalFormatStyleDto {
    fn from(style: SheetConditionalFormatStyleView) -> Self {
        match style {
            SheetConditionalFormatStyleView::Warning => Self::Warning,
            SheetConditionalFormatStyleView::Success => Self::Success,
            SheetConditionalFormatStyleView::Accent => Self::Accent,
        }
    }
}

impl From<SheetConditionalFormatRuleView> for SheetConditionalFormatRuleDto {
    fn from(rule: SheetConditionalFormatRuleView) -> Self {
        Self {
            id: rule.id,
            worksheet_id: rule.worksheet_id,
            start_row: rule.start_row,
            end_row: rule.end_row,
            start_column: rule.start_column,
            end_column: rule.end_column,
            condition: rule.condition.into(),
            style: rule.style.into(),
            priority: rule.priority,
        }
    }
}

impl From<SheetConditionalFormatMatchView> for SheetConditionalFormatMatchDto {
    fn from(item: SheetConditionalFormatMatchView) -> Self {
        Self {
            row: item.row,
            column: item.column,
            style: item.style.into(),
        }
    }
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

impl From<SheetTableView> for SheetTableDto {
    fn from(table: SheetTableView) -> Self {
        Self {
            id: table.id,
            worksheet_id: table.worksheet_id,
            name: table.name,
            start_row: table.start_row,
            end_row: table.end_row,
            start_column: table.start_column,
            end_column: table.end_column,
        }
    }
}

impl From<SheetRangeSummaryView> for SheetRangeSummaryDto {
    fn from(summary: SheetRangeSummaryView) -> Self {
        Self {
            count: summary.count,
            numeric_count: summary.numeric_count,
            sum: summary.sum,
            average: summary.average,
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
            conditional_formats: document
                .conditional_formats
                .into_iter()
                .map(SheetConditionalFormatRuleDto::from)
                .collect(),
            tables: document
                .tables
                .into_iter()
                .map(SheetTableDto::from)
                .collect(),
            chart_count: document.charts.len(),
        }
    }
}
