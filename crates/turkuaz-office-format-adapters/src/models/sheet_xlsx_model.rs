// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/models/sheet_xlsx_model.rs
// # 📌 Amac: XLSX SpreadsheetML workbook, worksheet, table ve conditional-format verilerini canonical Sheet modelinden ayri temsil eder
// # 📌 Modul - FileType: Model - Rust
// Version: 0.9.0
// Aciklama: XLSX XML Tool ile Sheet Service mapping arasinda format-specific typed veri modeli saglar
// Bagimli Oldugu Katman: Model

#[derive(Clone, Debug, PartialEq)]
pub struct XlsxWorkbookModel {
    pub worksheets: Vec<XlsxWorksheetModel>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct XlsxWorksheetModel {
    pub name: String,
    pub cells: Vec<XlsxCellModel>,
    pub conditional_formats: Vec<XlsxConditionalFormatModel>,
    pub table_relationship_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct XlsxCellModel {
    pub reference: String,
    pub value: XlsxCellValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum XlsxCellValue {
    Text(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XlsxSheetDescriptor {
    pub name: String,
    pub relationship_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XlsxTableModel {
    pub name: String,
    pub range_reference: String,
    pub column_names: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct XlsxConditionalFormatModel {
    pub range_reference: String,
    pub condition: XlsxConditionalFormatCondition,
    pub differential_style_id: usize,
    pub priority: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum XlsxConditionalFormatCondition {
    NumberGreaterThan(f64),
    NumberLessThan(f64),
    NumberEquals(f64),
    TextContains(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XlsxDifferentialStyleModel {
    pub fill_rgb: Option<String>,
}
