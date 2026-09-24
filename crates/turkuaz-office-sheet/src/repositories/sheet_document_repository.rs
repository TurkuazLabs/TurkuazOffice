// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/repositories/sheet_document_repository.rs
// # 📌 Amac: Sheet document storage kontratini Service katmanindan ayirir
// # 📌 Modul - FileType: Repo - Rust
// Version: 0.3.0
// Aciklama: Repository trait ve domain/regression testleri icin bellek ici sparse document implementasyonu saglar
// Bagimli Oldugu Katman: Repo -> Service

use std::collections::HashMap;

use turkuaz_office_core::DocumentId;

use crate::services::sheet_types::SheetDocument;

pub trait SheetDocumentRepository {
    fn save(&mut self, document: SheetDocument);
    fn find(&self, id: &DocumentId) -> Option<SheetDocument>;
}

#[derive(Default)]
pub struct InMemorySheetDocumentRepository {
    documents: HashMap<DocumentId, SheetDocument>,
}

impl InMemorySheetDocumentRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SheetDocumentRepository for InMemorySheetDocumentRepository {
    fn save(&mut self, document: SheetDocument) {
        self.documents.insert(document.id.clone(), document);
    }

    fn find(&self, id: &DocumentId) -> Option<SheetDocument> {
        self.documents.get(id).cloned()
    }
}
