// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/tools/sheet_id_tool.rs
// # 📌 Amac: Sheet document ve worksheet kimlik uretimini domain business logic'inden ayirir
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.3.0
// Aciklama: Deterministik test edilebilir document/worksheet ID kontrati ve sequential implementasyon saglar
// Bagimli Oldugu Katman: Tool -> Config

use std::sync::atomic::{AtomicU64, Ordering};

use turkuaz_office_core::DocumentId;

use crate::config::constants::{SHEET_DOCUMENT_ID_PREFIX, WORKSHEET_ID_PREFIX};
use crate::services::sheet_types::WorksheetId;

pub trait SheetIdTool {
    fn next_document_id(&self) -> DocumentId;
    fn next_worksheet_id(&self) -> WorksheetId;
}

#[derive(Default)]
pub struct SequentialSheetIdTool {
    document_counter: AtomicU64,
    worksheet_counter: AtomicU64,
}

impl SequentialSheetIdTool {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SheetIdTool for SequentialSheetIdTool {
    fn next_document_id(&self) -> DocumentId {
        let value = self.document_counter.fetch_add(1, Ordering::Relaxed) + 1;
        DocumentId::new(format!("{SHEET_DOCUMENT_ID_PREFIX}-{value}"))
    }

    fn next_worksheet_id(&self) -> WorksheetId {
        let value = self.worksheet_counter.fetch_add(1, Ordering::Relaxed) + 1;
        WorksheetId::new(format!("{WORKSHEET_ID_PREFIX}-{value}"))
    }
}
