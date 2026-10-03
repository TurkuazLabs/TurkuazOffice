// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/tools/id_tool.rs
// # 📌 Amac: Belge kimligi uretimini dis dunya detayindan ayiran Tool kontratini saglar
// # 📌 Modul - FileType: Core - Rust
// # Version: 0.2.0
// # Aciklama: Belge kimligi uretimini dis dunya detayindan ayiran Tool kontratini saglar
// Bagimli Oldugu Katman: Tool

use std::sync::atomic::{AtomicU64, Ordering};

pub trait IdTool {
    fn next_id(&self) -> String;
}

#[derive(Default)]
pub struct SequentialIdTool {
    counter: AtomicU64,
}

impl SequentialIdTool {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl IdTool for SequentialIdTool {
    fn next_id(&self) -> String {
        let value = self.counter.fetch_add(1, Ordering::Relaxed) + 1;
        format!("document-{value}")
    }
}
