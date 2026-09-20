// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/services/document_service.rs
// # 📌 Amac: Belge olusturma ve metin degistirme is kurallarini Service katmaninda uygular
// # 📌 Modul - FileType: Core - Rust
// # Version: 0.2.0
// # Aciklama: Belge olusturma ve metin degistirme is kurallarini Service katmaninda uygular
// Bagimli Oldugu Katman: Service -> Repo -> Tool

use crate::config::constants::DEFAULT_DOCUMENT_TITLE;
use crate::repositories::document_repository::DocumentRepository;
use crate::services::document_types::{Document, DocumentId};
use crate::tools::id_tool::IdTool;

pub struct DocumentService<R, I>
where
    R: DocumentRepository,
    I: IdTool,
{
    repository: R,
    id_tool: I,
}

impl<R, I> DocumentService<R, I>
where
    R: DocumentRepository,
    I: IdTool,
{
    pub fn new(repository: R, id_tool: I) -> Self {
        Self {
            repository,
            id_tool,
        }
    }

    pub fn create_document(&mut self, title: Option<&str>) -> Document {
        let title = title
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or(DEFAULT_DOCUMENT_TITLE);
        let document = Document::new(DocumentId::new(self.id_tool.next_id()), title);
        self.repository.save(document.clone());
        document
    }

    pub fn append_text(&mut self, id: &DocumentId, text: &str) -> Option<Document> {
        let mut document = self.repository.find(id)?;
        document.text.push_str(text);
        document.revision = document.revision.saturating_add(1);
        self.repository.save(document.clone());
        Some(document)
    }

    pub fn get_document(&self, id: &DocumentId) -> Option<Document> {
        self.repository.find(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::document_repository::InMemoryDocumentRepository;
    use crate::tools::id_tool::SequentialIdTool;

    #[test]
    fn create_document_uses_default_title_when_title_is_empty() {
        let repository = InMemoryDocumentRepository::new();
        let id_tool = SequentialIdTool::new();
        let mut service = DocumentService::new(repository, id_tool);

        let document = service.create_document(Some("   "));

        assert_eq!(document.title, DEFAULT_DOCUMENT_TITLE);
        assert_eq!(
            document.schema_version.value(),
            crate::config::constants::CURRENT_DOCUMENT_SCHEMA_VERSION
        );
        assert_eq!(document.revision, 0);
    }

    #[test]
    fn append_text_updates_revision_and_persists_document() {
        let repository = InMemoryDocumentRepository::new();
        let id_tool = SequentialIdTool::new();
        let mut service = DocumentService::new(repository, id_tool);
        let created = service.create_document(Some("Test"));

        let updated = service
            .append_text(&created.id, "Merhaba")
            .expect("document should exist");
        let loaded = service
            .get_document(&created.id)
            .expect("document should be persisted");

        assert_eq!(updated.text, "Merhaba");
        assert_eq!(updated.revision, 1);
        assert_eq!(loaded, updated);
    }
}
