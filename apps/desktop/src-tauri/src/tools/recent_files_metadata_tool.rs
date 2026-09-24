// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/tools/recent_files_metadata_tool.rs
// # 📌 Amac: Recent files YAML disk DTO serialize/deserialize adaptorunu saglar
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Recent list persistence serde detayini Repo/Service katmanindan gizler ve mevcut TKO YAML adaptorunu kullanir
// Bagimli Oldugu Katman: Tool

use serde::{Deserialize, Serialize};
use turkuaz_office_writer::tools::tko_yaml_tool::TkoYamlTool;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecentFileDiskDto {
    pub path: String,
    pub title: String,
    pub last_accessed_unix_ms: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecentFilesDiskDto {
    pub entries: Vec<RecentFileDiskDto>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecentFilesMetadataError {
    SerializeFailed,
    DeserializeFailed,
}

pub struct RecentFilesMetadataTool;

impl RecentFilesMetadataTool {
    pub fn serialize(metadata: &RecentFilesDiskDto) -> Result<Vec<u8>, RecentFilesMetadataError> {
        TkoYamlTool::serialize(metadata).map_err(|_| RecentFilesMetadataError::SerializeFailed)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<RecentFilesDiskDto, RecentFilesMetadataError> {
        TkoYamlTool::deserialize(bytes).map_err(|_| RecentFilesMetadataError::DeserializeFailed)
    }
}
