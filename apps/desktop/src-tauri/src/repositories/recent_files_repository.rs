// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/repositories/recent_files_repository.rs
// # 📌 Amac: Recent files metadata listesini local state YAML dosyasinda safe-replace ile saklar
// # 📌 Modul - FileType: Repo - Rust
// # Version: 0.2.0
// # Aciklama: Service business logic'inden disk read/write, directory creation ve YAML serialization detayini ayirir
// Bagimli Oldugu Katman: Repo -> Tool

use std::path::PathBuf;

use crate::config::constants::MAX_RECENT_FILES_METADATA_BYTES;
use crate::tools::local_file_tool::{LocalFileError, LocalFileTool};
use crate::tools::recent_files_metadata_tool::{
    RecentFilesDiskDto, RecentFilesMetadataError, RecentFilesMetadataTool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecentFilesRepositoryError {
    ReadFailed,
    WriteFailed,
    MetadataInvalid,
}

impl From<RecentFilesMetadataError> for RecentFilesRepositoryError {
    fn from(_: RecentFilesMetadataError) -> Self {
        Self::MetadataInvalid
    }
}

pub struct RecentFilesRepository {
    metadata_path: PathBuf,
}

impl RecentFilesRepository {
    pub fn new(metadata_path: PathBuf) -> Self {
        Self { metadata_path }
    }

    pub fn load(&self) -> Result<RecentFilesDiskDto, RecentFilesRepositoryError> {
        if !LocalFileTool::file_exists(&self.metadata_path) {
            return Ok(RecentFilesDiskDto::default());
        }
        let bytes = LocalFileTool::read(&self.metadata_path, MAX_RECENT_FILES_METADATA_BYTES)
            .map_err(Self::read_error)?;
        RecentFilesMetadataTool::deserialize(&bytes).map_err(Into::into)
    }

    pub fn save(&self, metadata: &RecentFilesDiskDto) -> Result<(), RecentFilesRepositoryError> {
        let parent = self
            .metadata_path
            .parent()
            .ok_or(RecentFilesRepositoryError::WriteFailed)?;
        LocalFileTool::create_directory_all(parent).map_err(Self::write_error)?;
        let bytes = RecentFilesMetadataTool::serialize(metadata)?;
        if bytes.len() as u64 > MAX_RECENT_FILES_METADATA_BYTES {
            return Err(RecentFilesRepositoryError::MetadataInvalid);
        }
        LocalFileTool::write_safe_replace(&self.metadata_path, &bytes).map_err(Self::write_error)
    }

    fn read_error(error: LocalFileError) -> RecentFilesRepositoryError {
        match error {
            LocalFileError::ReadFailed => RecentFilesRepositoryError::ReadFailed,
            _ => RecentFilesRepositoryError::MetadataInvalid,
        }
    }

    fn write_error(_: LocalFileError) -> RecentFilesRepositoryError {
        RecentFilesRepositoryError::WriteFailed
    }
}
