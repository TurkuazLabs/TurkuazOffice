// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/services/sheet_types.rs
// # 📌 Amac: Format/UI bagimsiz Sheet document, worksheet, cell address ve value canonical modelini tanimlar
// # 📌 Modul - FileType: Service - Rust
// Version: 0.3.0
// Aciklama: Sparse BTreeMap cell storage ile text/number/boolean/formula hucre degerlerini ve evaluated value read-modelini tasir
// Bagimli Oldugu Katman: Service

use std::collections::BTreeMap;

use turkuaz_office_core::{DocumentId, DocumentSchemaVersion};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CellAddress {
    pub row: u32,
    pub column: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    Formula(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EvaluatedCellValue {
    Text(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub address: CellAddress,
    pub value: CellValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
}
