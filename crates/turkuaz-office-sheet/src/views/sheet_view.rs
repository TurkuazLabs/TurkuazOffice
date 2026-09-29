// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/views/sheet_view.rs
// # 📌 Amac: Canonical Sheet domain modelini UI/API icin read-only deterministic View modeline cevirir
// # 📌 Modul - FileType: View - Rust
// Version: 0.3.0
// Aciklama: Document/worksheet/cell/value alanlarini sparse BTreeMap sirasini koruyarak presentation katmanina tasir
// Bagimli Oldugu Katman: View -> Service

use crate::services::sheet_types::{Cell, CellValue, SheetDocument, Worksheet};

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
        }
    }
}
