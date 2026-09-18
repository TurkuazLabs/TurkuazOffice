// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/tko_profile_service.rs
// # 📌 Amac: Writer icin TKO v1 logical package profilini archive detayindan ayri tanimlar
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Manifest semantigi ve canonical Writer content round-trip sinirini saglar
// Bagimli Oldugu Katman: Service

use turkuaz_office_core::DocumentSchemaVersion;

use crate::config::constants::WRITER_DOCUMENT_KIND;
use crate::services::writer_types::WriterDocument;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TkoManifestV1 {
    pub format_version: u32,
    pub schema_version: DocumentSchemaVersion,
    pub document_kind: String,
    pub document_id: String,
    pub revision: u64,
    pub created_by_app_version: String,
    pub required_capabilities: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriterTkoPackageV1 {
    pub manifest: TkoManifestV1,
    pub content: WriterDocument,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TkoProfileError {
    UnsupportedFormatVersion,
    InvalidDocumentKind,
    MigrationRequired,
    FutureSchema,
    ManifestContentMismatch,
}

pub struct TkoProfileService;

impl TkoProfileService {
    pub const FORMAT_VERSION: u32 = 1;

    pub fn validate_current_schema(
        schema_version: DocumentSchemaVersion,
    ) -> Result<(), TkoProfileError> {
        if schema_version > DocumentSchemaVersion::current() {
            return Err(TkoProfileError::FutureSchema);
        }
        if schema_version < DocumentSchemaVersion::current() {
            return Err(TkoProfileError::MigrationRequired);
        }
        Ok(())
    }

    pub fn build(document: &WriterDocument, app_version: &str) -> WriterTkoPackageV1 {
        WriterTkoPackageV1 {
            manifest: TkoManifestV1 {
                format_version: Self::FORMAT_VERSION,
                schema_version: document.schema_version,
                document_kind: WRITER_DOCUMENT_KIND.to_owned(),
                document_id: document.id.as_str().to_owned(),
                revision: document.revision,
                created_by_app_version: app_version.to_owned(),
                required_capabilities: Vec::new(),
            },
            content: document.clone(),
        }
    }

    pub fn restore(package: WriterTkoPackageV1) -> Result<WriterDocument, TkoProfileError> {
        if package.manifest.format_version != Self::FORMAT_VERSION {
            return Err(TkoProfileError::UnsupportedFormatVersion);
        }
        if package.manifest.document_kind != WRITER_DOCUMENT_KIND {
            return Err(TkoProfileError::InvalidDocumentKind);
        }
        Self::validate_current_schema(package.manifest.schema_version)?;
        if package.manifest.document_id != package.content.id.as_str()
            || package.manifest.revision != package.content.revision
            || package.manifest.schema_version != package.content.schema_version
        {
            return Err(TkoProfileError::ManifestContentMismatch);
        }
        Ok(package.content)
    }
}
