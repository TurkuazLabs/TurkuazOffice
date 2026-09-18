// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/tko_package_service.rs
// # 📌 Amac: Canonical Writer document ile TKO v1 ZIP+YAML paket bytes arasindaki Service akisini koordine eder
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Manifest/content profile, YAML adapteri, ZIP allowlist ve boyut limitlerini tek is kurali altinda birlestirir
// Bagimli Oldugu Katman: Service -> Tool -> Config

use crate::config::constants::{
    MAX_TKO_CONTENT_BYTES, MAX_TKO_MANIFEST_BYTES, MAX_TKO_PACKAGE_BYTES, TKO_MANIFEST_ENTRY,
    TKO_WRITER_CONTENT_ENTRY,
};
use crate::services::tko_package_types::{TkoManifestDtoV1, WriterContentDtoV1};
use crate::services::tko_profile_service::{TkoProfileError, TkoProfileService, WriterTkoPackageV1};
use crate::services::writer_types::WriterDocument;
use crate::tools::tko_archive_tool::{TkoArchiveError, TkoArchiveTool};
use crate::tools::tko_yaml_tool::{TkoYamlError, TkoYamlTool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TkoPackageError {
    Archive(TkoArchiveError),
    Yaml(TkoYamlError),
    Profile(TkoProfileError),
    MissingManifest,
    MissingWriterContent,
    UnexpectedEntry,
    ManifestTooLarge,
    ContentTooLarge,
}

impl From<TkoArchiveError> for TkoPackageError {
    fn from(value: TkoArchiveError) -> Self {
        Self::Archive(value)
    }
}

impl From<TkoYamlError> for TkoPackageError {
    fn from(value: TkoYamlError) -> Self {
        Self::Yaml(value)
    }
}

impl From<TkoProfileError> for TkoPackageError {
    fn from(value: TkoProfileError) -> Self {
        Self::Profile(value)
    }
}

pub struct TkoPackageService;

impl TkoPackageService {
    pub const MAX_PACKAGE_BYTES: u64 = MAX_TKO_PACKAGE_BYTES;

    pub fn serialize(document: &WriterDocument, app_version: &str) -> Result<Vec<u8>, TkoPackageError> {
        TkoProfileService::validate_current_schema(document.schema_version)?;
        let package = TkoProfileService::build(document, app_version);
        let (manifest, content): (TkoManifestDtoV1, WriterContentDtoV1) = (&package).into();
        let manifest_bytes = TkoYamlTool::serialize(&manifest)?;
        let content_bytes = TkoYamlTool::serialize(&content)?;

        if manifest_bytes.len() as u64 > MAX_TKO_MANIFEST_BYTES {
            return Err(TkoPackageError::ManifestTooLarge);
        }
        if content_bytes.len() as u64 > MAX_TKO_CONTENT_BYTES {
            return Err(TkoPackageError::ContentTooLarge);
        }

        TkoArchiveTool::encode(&[
            (TKO_MANIFEST_ENTRY, manifest_bytes.as_slice()),
            (TKO_WRITER_CONTENT_ENTRY, content_bytes.as_slice()),
        ])
        .map_err(Into::into)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<WriterDocument, TkoPackageError> {
        let mut entries = TkoArchiveTool::decode(bytes)?;
        if entries
            .keys()
            .any(|name| name != TKO_MANIFEST_ENTRY && name != TKO_WRITER_CONTENT_ENTRY)
        {
            return Err(TkoPackageError::UnexpectedEntry);
        }

        let manifest_bytes = entries
            .remove(TKO_MANIFEST_ENTRY)
            .ok_or(TkoPackageError::MissingManifest)?;
        let content_bytes = entries
            .remove(TKO_WRITER_CONTENT_ENTRY)
            .ok_or(TkoPackageError::MissingWriterContent)?;

        if manifest_bytes.len() as u64 > MAX_TKO_MANIFEST_BYTES {
            return Err(TkoPackageError::ManifestTooLarge);
        }
        if content_bytes.len() as u64 > MAX_TKO_CONTENT_BYTES {
            return Err(TkoPackageError::ContentTooLarge);
        }

        let manifest: TkoManifestDtoV1 = TkoYamlTool::deserialize(&manifest_bytes)?;
        let content: WriterContentDtoV1 = TkoYamlTool::deserialize(&content_bytes)?;
        let package: WriterTkoPackageV1 = (manifest, content).into();
        TkoProfileService::restore(package).map_err(Into::into)
    }
}
