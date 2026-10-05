// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/views/sheet_view.rs
// # 📌 Amac: Canonical Sheet domain modelini UI/API icin read-only deterministic View modeline cevirir
// # 📌 Modul - FileType: View - Rust
// Version: 0.7.0
// Aciklama: Document/worksheet/cell/value, table, conditional formatting ve range summary alanlarini presentation katmanina tasir
// Bagimli Oldugu Katman: View -> Service

use crate::services::sheet_types::{
    Cell, CellFormat, CellValue, ChartDataPoint, ChartType, HorizontalAlignment, SheetChart,
    SheetConditionalFormatCondition, SheetConditionalFormatMatch, SheetConditionalFormatRule,
    SheetConditionalFormatStyle, SheetDocument, SheetRangeSummary, SheetTable, Worksheet,
};

#[derive(Clone, Debug, PartialEq)]
pub enum SheetConditionalFormatConditionView {
    NumberGreaterThan(f64),
    NumberLessThan(f64),
    NumberEquals(f64),
    TextContains(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SheetConditionalFormatStyleView {
    Warning,
    Success,
    Accent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SheetConditionalFormatRuleView {
    pub id: String,
    pub worksheet_id: String,
    pub start_row: u32,
    pub end_row: u32,
    pub start_column: u32,
    pub end_column: u32,
    pub condition: SheetConditionalFormatConditionView,
    pub style: SheetConditionalFormatStyleView,
    pub priority: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SheetConditionalFormatMatchView {
    pub row: u32,
    pub column: u32,
    pub style: SheetConditionalFormatStyleView,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SheetTableView {
    pub id: String,
    pub worksheet_id: String,
    pub name: String,
    pub start_row: u32,
    pub end_row: u32,
    pub start_column: u32,
    pub end_column: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartTypeView {
    Bar,
    Line,
    Pie,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SheetChartView {
    pub id: String,
    pub worksheet_id: String,
    pub chart_type: ChartTypeView,
    pub title: String,
    pub start_row: u32,
    pub end_row: u32,
    pub category_column: u32,
    pub value_column: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartDataPointView {
    pub category: String,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartDataView {
    pub points: Vec<ChartDataPointView>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HorizontalAlignmentView {
    General,
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellFormatView {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub horizontal_alignment: HorizontalAlignmentView,
    pub decimal_places: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SheetRowQueryView {
    pub rows: Vec<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SheetRangeSummaryView {
    pub count: usize,
    pub numeric_count: usize,
    pub sum: f64,
    pub average: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellValueView {
    Text(String),
    Number(f64),
    Boolean(bool),
    Formula(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CellView {
    pub row: u32,
    pub column: u32,
    pub value: CellValueView,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorksheetView {
    pub id: String,
    pub name: String,
    pub cell_count: usize,
    pub cells: Vec<CellView>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SheetDocumentView {
    pub id: String,
    pub title: String,
    pub revision: u64,
    pub worksheets: Vec<WorksheetView>,
    pub conditional_formats: Vec<SheetConditionalFormatRuleView>,
    pub tables: Vec<SheetTableView>,
    pub charts: Vec<SheetChartView>,
}

impl From<SheetConditionalFormatCondition> for SheetConditionalFormatConditionView {
    fn from(condition: SheetConditionalFormatCondition) -> Self {
        match condition {
            SheetConditionalFormatCondition::NumberGreaterThan(value) => {
                Self::NumberGreaterThan(value)
            }
            SheetConditionalFormatCondition::NumberLessThan(value) => Self::NumberLessThan(value),
            SheetConditionalFormatCondition::NumberEquals(value) => Self::NumberEquals(value),
            SheetConditionalFormatCondition::TextContains(value) => Self::TextContains(value),
        }
    }
}

impl From<SheetConditionalFormatStyle> for SheetConditionalFormatStyleView {
    fn from(style: SheetConditionalFormatStyle) -> Self {
        match style {
            SheetConditionalFormatStyle::Warning => Self::Warning,
            SheetConditionalFormatStyle::Success => Self::Success,
            SheetConditionalFormatStyle::Accent => Self::Accent,
        }
    }
}

impl From<SheetConditionalFormatRule> for SheetConditionalFormatRuleView {
    fn from(rule: SheetConditionalFormatRule) -> Self {
        Self {
            id: rule.id.as_str().to_owned(),
            worksheet_id: rule.worksheet_id.as_str().to_owned(),
            start_row: rule.range.start_row,
            end_row: rule.range.end_row,
            start_column: rule.range.start_column,
            end_column: rule.range.end_column,
            condition: rule.condition.into(),
            style: rule.style.into(),
            priority: rule.priority,
        }
    }
}

impl From<SheetConditionalFormatMatch> for SheetConditionalFormatMatchView {
    fn from(item: SheetConditionalFormatMatch) -> Self {
        Self {
            row: item.address.row,
            column: item.address.column,
            style: item.style.into(),
        }
    }
}

impl From<SheetTable> for SheetTableView {
    fn from(table: SheetTable) -> Self {
        Self {
            id: table.id.as_str().to_owned(),
            worksheet_id: table.worksheet_id.as_str().to_owned(),
            name: table.name,
            start_row: table.range.start_row,
            end_row: table.range.end_row,
            start_column: table.range.start_column,
            end_column: table.range.end_column,
        }
    }
}

impl From<ChartType> for ChartTypeView {
    fn from(value: ChartType) -> Self {
        match value {
            ChartType::Bar => Self::Bar,
            ChartType::Line => Self::Line,
            ChartType::Pie => Self::Pie,
        }
    }
}

impl From<SheetChart> for SheetChartView {
    fn from(chart: SheetChart) -> Self {
        Self {
            id: chart.id.as_str().to_owned(),
            worksheet_id: chart.worksheet_id.as_str().to_owned(),
            chart_type: chart.chart_type.into(),
            title: chart.title,
            start_row: chart.start_row,
            end_row: chart.end_row,
            category_column: chart.category_column,
            value_column: chart.value_column,
        }
    }
}

impl From<ChartDataPoint> for ChartDataPointView {
    fn from(point: ChartDataPoint) -> Self {
        Self {
            category: point.category,
            value: point.value,
        }
    }
}

impl From<Vec<ChartDataPoint>> for ChartDataView {
    fn from(points: Vec<ChartDataPoint>) -> Self {
        Self {
            points: points.into_iter().map(ChartDataPointView::from).collect(),
        }
    }
}

impl From<HorizontalAlignment> for HorizontalAlignmentView {
    fn from(value: HorizontalAlignment) -> Self {
        match value {
            HorizontalAlignment::General => Self::General,
            HorizontalAlignment::Left => Self::Left,
            HorizontalAlignment::Center => Self::Center,
            HorizontalAlignment::Right => Self::Right,
        }
    }
}

impl From<CellFormat> for CellFormatView {
    fn from(value: CellFormat) -> Self {
        Self {
            bold: value.bold,
            italic: value.italic,
            underline: value.underline,
            horizontal_alignment: value.horizontal_alignment.into(),
            decimal_places: value.decimal_places,
        }
    }
}

impl From<Vec<u32>> for SheetRowQueryView {
    fn from(rows: Vec<u32>) -> Self {
        Self { rows }
    }
}

impl From<SheetRangeSummary> for SheetRangeSummaryView {
    fn from(summary: SheetRangeSummary) -> Self {
        Self {
            count: summary.count,
            numeric_count: summary.numeric_count,
            sum: summary.sum,
            average: summary.average,
        }
    }
}

impl From<CellValue> for CellValueView {
    fn from(value: CellValue) -> Self {
        match value {
            CellValue::Text(text) => Self::Text(text),
            CellValue::Number(number) => Self::Number(number),
            CellValue::Boolean(value) => Self::Boolean(value),
            CellValue::Formula(formula) => Self::Formula(formula.expression),
        }
    }
}

impl From<Cell> for CellView {
    fn from(cell: Cell) -> Self {
        Self {
            row: cell.address.row,
            column: cell.address.column,
            value: cell.value.into(),
        }
    }
}

impl From<Worksheet> for WorksheetView {
    fn from(worksheet: Worksheet) -> Self {
        let cells = worksheet
            .cells
            .into_iter()
            .map(|(address, value)| CellView {
                row: address.row,
                column: address.column,
                value: value.into(),
            })
            .collect::<Vec<_>>();
        Self {
            id: worksheet.id.as_str().to_owned(),
            name: worksheet.name,
            cell_count: cells.len(),
            cells,
        }
    }
}

impl From<SheetDocument> for SheetDocumentView {
    fn from(document: SheetDocument) -> Self {
        Self {
            id: document.id.as_str().to_owned(),
            title: document.title,
            revision: document.revision,
            worksheets: document
                .worksheets
                .into_iter()
                .map(WorksheetView::from)
                .collect(),
            conditional_formats: document
                .conditional_formats
                .into_values()
                .map(SheetConditionalFormatRuleView::from)
                .collect(),
            tables: document
                .tables
                .into_values()
                .map(SheetTableView::from)
                .collect(),
            charts: document
                .charts
                .into_values()
                .map(SheetChartView::from)
                .collect(),
        }
    }
}
