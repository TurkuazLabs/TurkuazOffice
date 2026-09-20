// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/tko_package_service.rs
// # 📌 Amac: Canonical Writer document ile TKO v1 ZIP+YAML paket bytes arasindaki Service akisini koordine eder
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Manifest/content, optional asset index, binary asset allowlist ve paket limitlerini tek is kurali altinda birlestirir
// Bagimli Oldugu Katman: Service -> Tool -> Config

use std::collections::HashSet;

use crate::config::constants::{
    MAX_TKO_ASSET_INDEX_BYTES, MAX_TKO_CONTENT_BYTES, MAX_TKO_MANIFEST_BYTES,
    MAX_TKO_PACKAGE_BYTES, TKO_ASSET_INDEX_ENTRY, TKO_MANIFEST_ENTRY, TKO_WRITER_CONTENT_ENTRY,
};
use crate::services::tko_package_types::{
    TkoAssetEntryDtoV1, TkoAssetIndexDtoV1, TkoManifestDtoV1, WriterContentDtoV1,
};
use crate::services::tko_profile_service::{
    TkoProfileError, TkoProfileService, WriterTkoPackageV1,
};
use crate::services::writer_asset_service::{WriterAssetError, WriterAssetService};
use crate::services::writer_types::{WriterAsset, WriterDocument};
use crate::tools::tko_archive_tool::{TkoArchiveError, TkoArchiveTool};
use crate::tools::tko_yaml_tool::{TkoYamlError, TkoYamlTool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TkoPackageError {
    Archive(TkoArchiveError),
    Yaml(TkoYamlError),
    Profile(TkoProfileError),
    Asset(WriterAssetError),
    MissingManifest,
    MissingWriterContent,
    MissingAssetEntry,
    UnexpectedEntry,
    ManifestTooLarge,
    ContentTooLarge,
    AssetIndexTooLarge,
    AssetLengthMismatch,
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

impl From<WriterAssetError> for TkoPackageError {
    fn from(value: WriterAssetError) -> Self {
        Self::Asset(value)
    }
}

pub struct TkoPackageService;

impl TkoPackageService {
    pub const MAX_PACKAGE_BYTES: u64 = MAX_TKO_PACKAGE_BYTES;

    pub fn serialize(
        document: &WriterDocument,
        app_version: &str,
    ) -> Result<Vec<u8>, TkoPackageError> {
        TkoProfileService::validate_current_schema(document.schema_version)?;
        WriterAssetService::validate_document(document)?;

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

        let mut owned_entries = vec![
            (TKO_MANIFEST_ENTRY.to_owned(), manifest_bytes),
            (TKO_WRITER_CONTENT_ENTRY.to_owned(), content_bytes),
        ];

        if !document.assets.is_empty() {
            let asset_entries = document
                .assets
                .iter()
                .map(|asset| {
                    Ok(TkoAssetEntryDtoV1 {
                        id: asset.id.clone(),
                        media_type: asset.media_type.clone(),
                        entry: WriterAssetService::entry_name(&asset.id)?,
                        byte_length: asset.bytes.len() as u64,
                    })
                })
                .collect::<Result<Vec<_>, WriterAssetError>>()?;
            let index_bytes = TkoYamlTool::serialize(&TkoAssetIndexDtoV1 {
                assets: asset_entries.clone(),
            })?;
            if index_bytes.len() as u64 > MAX_TKO_ASSET_INDEX_BYTES {
                return Err(TkoPackageError::AssetIndexTooLarge);
            }
            owned_entries.push((TKO_ASSET_INDEX_ENTRY.to_owned(), index_bytes));
            for (asset, entry) in document.assets.iter().zip(asset_entries.iter()) {
                owned_entries.push((entry.entry.clone(), asset.bytes.clone()));
            }
        }

        let borrowed_entries = owned_entries
            .iter()
            .map(|(name, data)| (name.as_str(), data.as_slice()))
            .collect::<Vec<_>>();
        TkoArchiveTool::encode(&borrowed_entries).map_err(Into::into)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<WriterDocument, TkoPackageError> {
        let mut entries = TkoArchiveTool::decode(bytes)?;
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
        let mut document = TkoProfileService::restore(package)?;

        let assets = match entries.remove(TKO_ASSET_INDEX_ENTRY) {
            Some(index_bytes) => Self::decode_assets(index_bytes, &mut entries)?,
            None => Vec::new(),
        };
        if !entries.is_empty() {
            return Err(TkoPackageError::UnexpectedEntry);
        }

        document.assets = assets;
        WriterAssetService::validate_document(&document)?;
        Ok(document)
    }

    fn decode_assets(
        index_bytes: Vec<u8>,
        entries: &mut std::collections::HashMap<String, Vec<u8>>,
    ) -> Result<Vec<WriterAsset>, TkoPackageError> {
        if index_bytes.len() as u64 > MAX_TKO_ASSET_INDEX_BYTES {
            return Err(TkoPackageError::AssetIndexTooLarge);
        }
        let index: TkoAssetIndexDtoV1 = TkoYamlTool::deserialize(&index_bytes)?;
        WriterAssetService::validate_asset_count(index.assets.len())?;

        let mut seen_ids = HashSet::new();
        let mut seen_entries = HashSet::new();
        let mut assets = Vec::with_capacity(index.assets.len());
        for descriptor in index.assets {
            WriterAssetService::validate_id(&descriptor.id)?;
            let expected_entry = WriterAssetService::entry_name(&descriptor.id)?;
            if descriptor.entry != expected_entry
                || !seen_ids.insert(descriptor.id.clone())
                || !seen_entries.insert(descriptor.entry.clone())
            {
                return Err(TkoPackageError::Asset(WriterAssetError::InvalidId));
            }
            let data = entries
                .remove(&descriptor.entry)
                .ok_or(TkoPackageError::MissingAssetEntry)?;
            if data.len() as u64 != descriptor.byte_length {
                return Err(TkoPackageError::AssetLengthMismatch);
            }
            WriterAssetService::validate_image_payload(&descriptor.media_type, &data)?;
            assets.push(WriterAsset {
                id: descriptor.id,
                media_type: descriptor.media_type,
                bytes: data,
            });
        }
        Ok(assets)
    }
}
