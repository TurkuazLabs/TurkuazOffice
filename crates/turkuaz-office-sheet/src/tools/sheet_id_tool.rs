// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/tools/sheet_id_tool.rs
// # 📌 Amac: Sheet document, worksheet, table ve chart kimlik uretimini domain business logic'inden ayirir
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.6.0
// Aciklama: Deterministik test edilebilir document/worksheet ID kontrati ve sequential implementasyon saglar
// Bagimli Oldugu Katman: Tool -> Config

use std::sync::atomic::{AtomicU64, Ordering};

use turkuaz_office_core::DocumentId;

use crate::config::constants::{
    CHART_ID_PREFIX, SHEET_DOCUMENT_ID_PREFIX, TABLE_ID_PREFIX, WORKSHEET_ID_PREFIX,
};
use crate::services::sheet_types::{ChartId, TableId, WorksheetId};

pub trait SheetIdTool {
    fn next_document_id(&self) -> DocumentId;
    fn next_worksheet_id(&self) -> WorksheetId;
    fn next_table_id(&self) -> TableId;
    fn next_chart_id(&self) -> ChartId;
}

#[derive(Default)]
pub struct SequentialSheetIdTool {
    document_counter: AtomicU64,
    worksheet_counter: AtomicU64,
    table_counter: AtomicU64,
    chart_counter: AtomicU64,
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

    fn next_table_id(&self) -> TableId {
        let value = self.table_counter.fetch_add(1, Ordering::Relaxed) + 1;
        TableId::new(format!("{TABLE_ID_PREFIX}-{value}"))
    }

    fn next_chart_id(&self) -> ChartId {
        let value = self.chart_counter.fetch_add(1, Ordering::Relaxed) + 1;
        ChartId::new(format!("{CHART_ID_PREFIX}-{value}"))
    }
}
