// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/services/sheet_types.rs
// # 📌 Amac: Format/UI bagimsiz Sheet document, worksheet, cell, format, query ve chart canonical modelini tanimlar
// # 📌 Modul - FileType: Service - Rust
// Version: 0.3.0
// Aciklama: Sparse cell storage, canonical format metadata, table-query ve basic chart tiplerini tasir
// Bagimli Oldugu Katman: Service

use std::collections::BTreeMap;

use turkuaz_office_core::{DocumentId, DocumentSchemaVersion};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CellAddress {
    pub row: u32,
    pub column: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormulaCell {
    pub expression: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    Formula(FormulaCell),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub address: CellAddress,
    pub value: CellValue,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HorizontalAlignment {
    #[default]
    General,
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CellFormat {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub horizontal_alignment: HorizontalAlignment,
    pub decimal_places: Option<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SheetFilterCondition {
    NonEmpty,
    TextContains(String),
    NumberGreaterThan(f64),
    NumberLessThan(f64),
    BooleanEquals(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SheetFilter {
    pub column: u32,
    pub condition: SheetFilterCondition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SheetSortDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SheetSort {
    pub column: u32,
    pub direction: SheetSortDirection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SheetRange {
    pub start_row: u32,
    pub end_row: u32,
    pub start_column: u32,
    pub end_column: u32,
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChartId(String);

impl ChartId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SheetChart {
    pub id: ChartId,
    pub worksheet_id: WorksheetId,
    pub chart_type: ChartType,
    pub title: String,
    pub start_row: u32,
    pub end_row: u32,
    pub category_column: u32,
    pub value_column: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartDataPoint {
    pub category: String,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorksheetId(String);

impl WorksheetId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Worksheet {
    pub id: WorksheetId,
    pub name: String,
    pub cells: BTreeMap<CellAddress, CellValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SheetDocument {
    pub id: DocumentId,
    pub title: String,
    pub schema_version: DocumentSchemaVersion,
    pub revision: u64,
    pub worksheets: Vec<Worksheet>,
    pub cell_formats: BTreeMap<WorksheetId, BTreeMap<CellAddress, CellFormat>>,
    pub charts: BTreeMap<ChartId, SheetChart>,
}
