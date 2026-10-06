// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-web-bridge/src/views/writer_tko_bridge_view.rs
// # 📌 Amac: Writer TKO inspect sonucunu JavaScript icin sade read-only DTO olarak tanimlar
// # 📌 Modul - FileType: View - Rust
// Version: 0.4.0
// Aciklama: Canonical Writer documentin kimlik/schema/revision ve yapisal sayaclarini binary asset payload tasimadan acar
// Bagimli Oldugu Katman: View

use serde::Serialize;
use turkuaz_office_writer::WriterDocument;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct WebWriterTkoSummaryView {
    pub id: String,
    pub title: String,
    pub schema_version: u32,
    pub revision: u64,
    pub section_count: usize,
    pub asset_count: usize,
}

impl From<&WriterDocument> for WebWriterTkoSummaryView {
    fn from(document: &WriterDocument) -> Self {
        Self {
            id: document.id.as_str().to_owned(),
            title: document.title.clone(),
            schema_version: document.schema_version.value(),
            revision: document.revision,
            section_count: document.sections.len(),
            asset_count: document.assets.len(),
        }
    }
}
