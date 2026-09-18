// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/views/document_view.rs
// # 📌 Amac: Service sonucunu UI katmanina tasinacak sade View DTO formatina cevirir
// # 📌 Modul - FileType: Core - Rust
// # Version: 0.2.0
// # Aciklama: Belge kimligi, title, text, schema version ve revision alanlarini View DTO olarak tasir
// Bagimli Oldugu Katman: View

use crate::services::document_types::Document;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentView {
    pub id: String,
    pub title: String,
    pub text: String,
    pub schema_version: u32,
    pub revision: u64,
}

impl From<Document> for DocumentView {
    fn from(document: Document) -> Self {
        Self {
            id: document.id.as_str().to_owned(),
            title: document.title,
            text: document.text,
            schema_version: document.schema_version.value(),
            revision: document.revision,
        }
    }
}
