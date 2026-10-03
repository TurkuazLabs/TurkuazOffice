// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/writer_asset_service.rs
// # 📌 Amac: Writer binary image asset kimlik, MIME, signature, boyut ve referans kurallarini dogrular
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: PNG/JPEG/WebP asset validation ve canonical TKO asset entry adini Service katmaninda saglar
// Bagimli Oldugu Katman: Service -> Config

use std::collections::HashSet;

use crate::config::constants::{
    IMAGE_MEDIA_TYPE_JPEG, IMAGE_MEDIA_TYPE_PNG, IMAGE_MEDIA_TYPE_WEBP, MAX_WRITER_ASSET_BYTES,
    MAX_WRITER_ASSET_ID_LENGTH, MAX_WRITER_ASSETS, TKO_ASSET_DATA_PREFIX,
};
use crate::services::writer_types::{Block, WriterDocument};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriterAssetError {
    InvalidId,
    UnsupportedMediaType,
    EmptyData,
    AssetTooLarge,
    InvalidSignature,
    TooManyAssets,
    DuplicateAssetId,
    ReferencedAssetMissing,
    UnreferencedAsset,
}

pub struct WriterAssetService;

impl WriterAssetService {
    pub fn validate_asset_count(count: usize) -> Result<(), WriterAssetError> {
        if count > MAX_WRITER_ASSETS {
            return Err(WriterAssetError::TooManyAssets);
        }
        Ok(())
    }

    pub fn validate_id(id: &str) -> Result<(), WriterAssetError> {
        if id.is_empty()
            || id.chars().count() > MAX_WRITER_ASSET_ID_LENGTH
            || !id
                .chars()
                .all(|value| value.is_ascii_alphanumeric() || value == '-' || value == '_')
        {
            return Err(WriterAssetError::InvalidId);
        }
        Ok(())
    }

    pub fn entry_name(id: &str) -> Result<String, WriterAssetError> {
        Self::validate_id(id)?;
        Ok(format!("{TKO_ASSET_DATA_PREFIX}{id}.bin"))
    }

    pub fn validate_image_payload(media_type: &str, data: &[u8]) -> Result<(), WriterAssetError> {
        if data.is_empty() {
            return Err(WriterAssetError::EmptyData);
        }
        if data.len() as u64 > MAX_WRITER_ASSET_BYTES {
            return Err(WriterAssetError::AssetTooLarge);
        }
        let signature_valid = match media_type {
            IMAGE_MEDIA_TYPE_PNG => {
                data.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A])
            }
            IMAGE_MEDIA_TYPE_JPEG => data.starts_with(&[0xFF, 0xD8, 0xFF]),
            IMAGE_MEDIA_TYPE_WEBP => {
                data.len() >= 12 && data.starts_with(b"RIFF") && &data[8..12] == b"WEBP"
            }
            _ => return Err(WriterAssetError::UnsupportedMediaType),
        };
        if !signature_valid {
            return Err(WriterAssetError::InvalidSignature);
        }
        Ok(())
    }

    pub fn validate_document(document: &WriterDocument) -> Result<(), WriterAssetError> {
        Self::validate_asset_count(document.assets.len())?;
        let mut ids = HashSet::new();
        for asset in &document.assets {
            Self::validate_id(&asset.id)?;
            if !ids.insert(asset.id.as_str()) {
                return Err(WriterAssetError::DuplicateAssetId);
            }
            Self::validate_image_payload(&asset.media_type, &asset.bytes)?;
        }

        let mut referenced = HashSet::new();
        for section in &document.sections {
            for block in &section.blocks {
                if let Block::Image(image) = block {
                    if !ids.contains(image.asset_id.as_str()) {
                        return Err(WriterAssetError::ReferencedAssetMissing);
                    }
                    referenced.insert(image.asset_id.as_str());
                }
            }
        }
        if document
            .assets
            .iter()
            .any(|asset| !referenced.contains(asset.id.as_str()))
        {
            return Err(WriterAssetError::UnreferencedAsset);
        }
        Ok(())
    }
}
