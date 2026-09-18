// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/tools/writer_id_tool.rs
// # 📌 Amac: Writer document, node ve asset kimlik uretimini domain mantigindan ayirir
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Stabil kimlikler icin Tool kontrati ve deterministik test implementasyonu saglar
// Bagimli Oldugu Katman: Tool

use std::sync::atomic::{AtomicU64, Ordering};

use crate::config::constants::{ASSET_ID_PREFIX, DOCUMENT_ID_PREFIX, NODE_ID_PREFIX};
use crate::services::writer_types::NodeId;
use turkuaz_office_core::DocumentId;

pub trait WriterIdTool {
    fn next_document_id(&self) -> DocumentId;
    fn next_node_id(&self) -> NodeId;
    fn next_asset_id(&self) -> String;
}

#[derive(Default)]
pub struct SequentialWriterIdTool {
    document_counter: AtomicU64,
    node_counter: AtomicU64,
    asset_counter: AtomicU64,
}

impl SequentialWriterIdTool {
    pub fn new() -> Self {
        Self::default()
    }
}

impl WriterIdTool for SequentialWriterIdTool {
    fn next_document_id(&self) -> DocumentId {
        let value = self.document_counter.fetch_add(1, Ordering::Relaxed) + 1;
        DocumentId::new(format!("{DOCUMENT_ID_PREFIX}-{value}"))
    }

    fn next_node_id(&self) -> NodeId {
        let value = self.node_counter.fetch_add(1, Ordering::Relaxed) + 1;
        NodeId::new(format!("{NODE_ID_PREFIX}-{value}"))
    }

    fn next_asset_id(&self) -> String {
        let value = self.asset_counter.fetch_add(1, Ordering::Relaxed) + 1;
        format!("{ASSET_ID_PREFIX}-{value}")
    }
}
