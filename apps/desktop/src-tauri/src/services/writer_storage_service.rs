// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/services/writer_storage_service.rs
// # 📌 Amac: Desktop Writer local TKO open/save is kurallarini koordine eder
// # 📌 Modul - FileType: Service - Rust
// # Version: 0.2.0
// # Aciklama: Dosya yolu, TKO extension, package serialization ve safe-replace akisini Tool katmani uzerinden yonetir
// Bagimli Oldugu Katman: Service -> Tool

use std::path::{Path, PathBuf};

use turkuaz_office_writer::config::constants::TKO_FILE_EXTENSION;
use turkuaz_office_writer::{TkoArchiveError, TkoPackageError, TkoPackageService, WriterDocument};

use crate::tools::local_file_tool::{LocalFileError, LocalFileTool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriterStorageError {
    InvalidPath,
    InvalidExtension,
    ReadFailed,
    WriteFailed,
    Package(TkoPackageError),
}

impl From<TkoPackageError> for WriterStorageError {
    fn from(value: TkoPackageError) -> Self {
        Self::Package(value)
    }
}

impl From<LocalFileError> for WriterStorageError {
    fn from(value: LocalFileError) -> Self {
        match value {
            LocalFileError::InvalidPath => Self::InvalidPath,
            LocalFileError::PackageTooLarge => {
                Self::Package(TkoPackageError::Archive(TkoArchiveError::PackageTooLarge))
            }
            LocalFileError::ReadFailed => Self::ReadFailed,
            LocalFileError::WriteFailed => Self::WriteFailed,
        }
    }
}

pub struct WriterStorageService;

impl WriterStorageService {
    pub fn open(path_text: &str) -> Result<WriterDocument, WriterStorageError> {
        let path = Self::resolve_open_path(path_text)?;
        let bytes = LocalFileTool::read(&path, TkoPackageService::MAX_PACKAGE_BYTES)?;
        TkoPackageService::deserialize(&bytes).map_err(Into::into)
    }

    pub fn save(document: &WriterDocument, path_text: &str) -> Result<String, WriterStorageError> {
        let path = Self::resolve_save_path(path_text)?;
        let bytes = TkoPackageService::serialize(document, env!("CARGO_PKG_VERSION"))?;
        LocalFileTool::write_safe_replace(&path, &bytes)?;
        Ok(path.to_string_lossy().into_owned())
    }

    pub fn resolve_open_path(path_text: &str) -> Result<PathBuf, WriterStorageError> {
        let path = Self::base_path(path_text)?;
        if !Self::has_tko_extension(&path) {
            return Err(WriterStorageError::InvalidExtension);
        }
        Ok(path)
    }

    pub fn resolve_save_path(path_text: &str) -> Result<PathBuf, WriterStorageError> {
        let mut path = Self::base_path(path_text)?;
        if !Self::has_tko_extension(&path) {
            path.set_extension(TKO_FILE_EXTENSION);
        }
        Ok(path)
    }

    fn base_path(path_text: &str) -> Result<PathBuf, WriterStorageError> {
        if path_text.trim().is_empty() {
            return Err(WriterStorageError::InvalidPath);
        }
        let path = PathBuf::from(path_text);
        if path.file_name().is_none() {
            return Err(WriterStorageError::InvalidPath);
        }
        Ok(path)
    }

    fn has_tko_extension(path: &Path) -> bool {
        path.extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(TKO_FILE_EXTENSION))
    }
}
