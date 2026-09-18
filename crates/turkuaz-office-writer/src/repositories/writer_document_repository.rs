// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/repositories/writer_document_repository.rs
// # 📌 Amac: Writer belge storage kontratini domain servislerinden ayirir
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Repository trait ve testler icin bellek ici implementasyon saglar
// Bagimli Oldugu Katman: Repo

use std::collections::HashMap;

use turkuaz_office_core::DocumentId;

use crate::services::writer_types::WriterDocument;

pub trait WriterDocumentRepository {
    fn save(&mut self, document: WriterDocument);
    fn find(&self, id: &DocumentId) -> Option<WriterDocument>;
}

#[derive(Default)]
pub struct InMemoryWriterDocumentRepository {
    documents: HashMap<DocumentId, WriterDocument>,
}

impl InMemoryWriterDocumentRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl WriterDocumentRepository for InMemoryWriterDocumentRepository {
    fn save(&mut self, document: WriterDocument) {
        self.documents.insert(document.id.clone(), document);
    }

    fn find(&self, id: &DocumentId) -> Option<WriterDocument> {
        self.documents.get(id).cloned()
    }
}
