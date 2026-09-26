// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/models/sheet_xlsx_model.rs
// # 📌 Amac: XLSX SpreadsheetML workbook, worksheet ve cell degerlerini canonical Sheet modelinden ayri temsil eder
// # 📌 Modul - FileType: Model - Rust
// Version: 0.3.0
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
