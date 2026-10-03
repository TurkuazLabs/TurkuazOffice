// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/services/document_types.rs
// # 📌 Amac: Platformdan bagimsiz temel belge veri modelini ve schema version tipini tanimlar
// # 📌 Modul - FileType: Core - Rust
// # Version: 0.2.0
// # Aciklama: Canonical belge kimligi, schema version, revision ve temel metin modelini tasir
// Bagimli Oldugu Katman: Service

use crate::config::constants::CURRENT_DOCUMENT_SCHEMA_VERSION;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DocumentId(String);

impl DocumentId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocumentSchemaVersion(u32);

impl DocumentSchemaVersion {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn current() -> Self {
        Self(CURRENT_DOCUMENT_SCHEMA_VERSION)
    }

    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document {
    pub id: DocumentId,
    pub title: String,
    pub text: String,
    pub schema_version: DocumentSchemaVersion,
    pub revision: u64,
}

impl Document {
    pub fn new(id: DocumentId, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            text: String::new(),
            schema_version: DocumentSchemaVersion::current(),
            revision: 0,
        }
    }
}
