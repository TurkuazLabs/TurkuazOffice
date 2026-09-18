// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/services/schema_migration_service.rs
// # 📌 Amac: Native document schema version uyumlulugunu ve migration sinirini Service katmaninda yonetir
// # 📌 Modul - FileType: Core - Rust
// # Version: 0.2.0
// # Aciklama: Current schema kontrolunu yapar ve gelecek migration zinciri icin stabil Service kontrati saglar
// Bagimli Oldugu Katman: Service

use crate::services::document_types::{Document, DocumentSchemaVersion};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchemaMigrationError {
    FutureVersion {
        document_version: u32,
        current_version: u32,
    },
    UnsupportedOlderVersion {
        document_version: u32,
        current_version: u32,
    },
}

pub struct SchemaMigrationService;

impl SchemaMigrationService {
    pub fn ensure_current(document: Document) -> Result<Document, SchemaMigrationError> {
        let document_version = document.schema_version.value();
        let current_version = DocumentSchemaVersion::current().value();

        if document_version == current_version {
            return Ok(document);
        }

        if document_version > current_version {
            return Err(SchemaMigrationError::FutureVersion {
                document_version,
                current_version,
            });
        }

        Err(SchemaMigrationError::UnsupportedOlderVersion {
            document_version,
            current_version,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::document_types::DocumentId;

    #[test]
    fn current_schema_is_accepted_without_mutation() {
        let document = Document::new(DocumentId::new("document-1"), "Test");
        let result = SchemaMigrationService::ensure_current(document.clone());
        assert_eq!(result, Ok(document));
    }

    #[test]
    fn future_schema_is_rejected() {
        let mut document = Document::new(DocumentId::new("document-1"), "Test");
        document.schema_version = DocumentSchemaVersion::new(
            DocumentSchemaVersion::current().value().saturating_add(1),
        );

        let result = SchemaMigrationService::ensure_current(document);
        assert!(matches!(result, Err(SchemaMigrationError::FutureVersion { .. })));
    }
}
