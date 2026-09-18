// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/repositories/document_repository.rs
// # 📌 Amac: Belge saklama kontratini ve test/ilk surum icin bellek ici implementasyonu saglar
// # 📌 Modul - FileType: Core - Rust
// # Version: 0.2.0
// # Aciklama: Belge saklama kontratini ve test/ilk surum icin bellek ici implementasyonu saglar
// Bagimli Oldugu Katman: Repo

use std::collections::HashMap;

use crate::services::document_types::{Document, DocumentId};

pub trait DocumentRepository {
    fn save(&mut self, document: Document);
    fn find(&self, id: &DocumentId) -> Option<Document>;
}

#[derive(Default)]
pub struct InMemoryDocumentRepository {
    documents: HashMap<DocumentId, Document>,
}

impl InMemoryDocumentRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl DocumentRepository for InMemoryDocumentRepository {
    fn save(&mut self, document: Document) {
        self.documents.insert(document.id.clone(), document);
    }

    fn find(&self, id: &DocumentId) -> Option<Document> {
        self.documents.get(id).cloned()
    }
}
