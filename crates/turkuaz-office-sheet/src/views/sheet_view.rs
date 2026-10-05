// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/views/sheet_view.rs
// # 📌 Amac: Canonical Sheet domain modelini UI/API icin read-only deterministic View modeline cevirir
// # 📌 Modul - FileType: View - Rust
// Version: 0.5.0
// Aciklama: Document/worksheet/cell/value ve range summary alanlarini presentation katmanina tasir
// Bagimli Oldugu Katman: View -> Service

use crate::services::sheet_types::{
    Cell, CellFormat, CellValue, ChartDataPoint, ChartType, HorizontalAlignment, SheetChart,
    SheetDocument, SheetRangeSummary, Worksheet,
};

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
    pub charts: Vec<SheetChartView>,
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
            charts: document
                .charts
                .into_values()
                .map(SheetChartView::from)
                .collect(),
        }
    }
}
