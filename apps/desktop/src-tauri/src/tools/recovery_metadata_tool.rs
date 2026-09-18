// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/tools/recovery_metadata_tool.rs
// # 📌 Amac: Recovery snapshot metadata YAML disk DTO adaptorunu saglar
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Recovery metadata serde detayini Service katmanindan gizler ve TKO YAML adaptorunu yeniden kullanir
// Bagimli Oldugu Katman: Tool

use serde::{Deserialize, Serialize};
use turkuaz_office_writer::tools::tko_yaml_tool::TkoYamlTool;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecoveryMetadataDiskDto {
    pub snapshot_id: String,
    pub document_id: String,
    pub title: String,
    pub source_path: Option<String>,
    pub persisted_revision: u64,
    pub recovery_revision: u64,
    pub schema_version: u32,
    pub created_at_unix_ms: u64,
    pub created_by_app_version: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecoveryMetadataError {
    SerializeFailed,
    DeserializeFailed,
}

pub struct RecoveryMetadataTool;

impl RecoveryMetadataTool {
    pub fn serialize(metadata: &RecoveryMetadataDiskDto) -> Result<Vec<u8>, RecoveryMetadataError> {
        TkoYamlTool::serialize(metadata).map_err(|_| RecoveryMetadataError::SerializeFailed)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<RecoveryMetadataDiskDto, RecoveryMetadataError> {
        TkoYamlTool::deserialize(bytes).map_err(|_| RecoveryMetadataError::DeserializeFailed)
    }
}
