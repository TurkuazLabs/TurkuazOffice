// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/tools/local_file_tool.rs
// # 📌 Amac: Desktop yerel dosya okuma ve failure-safe replace yazma adaptorunu saglar
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Package boyut limiti, directory/list/remove, temp write, flush, replace ve Windows rollback detaylarini kapsuller
// Bagimli Oldugu Katman: Tool -> Config

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::constants::{
    CURRENT_DIRECTORY_PATH, SAFE_SAVE_BACKUP_SUFFIX, SAFE_SAVE_TEMP_SUFFIX,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocalFileError {
    InvalidPath,
    PackageTooLarge,
    ReadFailed,
    WriteFailed,
}

pub struct LocalFileTool;

impl LocalFileTool {
    pub fn create_directory_all(path: &Path) -> Result<(), LocalFileError> {
        fs::create_dir_all(path).map_err(|_| LocalFileError::WriteFailed)
    }

    pub fn directory_exists(path: &Path) -> bool {
        path.is_dir()
    }

    pub fn file_exists(path: &Path) -> bool {
        path.is_file()
    }

    pub fn list_files_with_extension(
        directory: &Path,
        extension: &str,
    ) -> Result<Vec<PathBuf>, LocalFileError> {
        let entries = fs::read_dir(directory).map_err(|_| LocalFileError::ReadFailed)?;
        let mut files = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|_| LocalFileError::ReadFailed)?;
            let path = entry.path();
            if path.is_file()
                && path
                    .extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(|value| value.eq_ignore_ascii_case(extension))
            {
                files.push(path);
            }
        }
        Ok(files)
    }

    pub fn remove_file_if_exists(path: &Path) -> Result<(), LocalFileError> {
        if !path.exists() {
            return Ok(());
        }
        fs::remove_file(path).map_err(|_| LocalFileError::WriteFailed)
    }

    pub fn read(path: &Path, max_bytes: u64) -> Result<Vec<u8>, LocalFileError> {
        let metadata = fs::metadata(path).map_err(|_| LocalFileError::ReadFailed)?;
        if !metadata.is_file() {
            return Err(LocalFileError::InvalidPath);
        }
        if metadata.len() > max_bytes {
            return Err(LocalFileError::PackageTooLarge);
        }

        let mut file = File::open(path).map_err(|_| LocalFileError::ReadFailed)?;
        let mut bytes = Vec::with_capacity(metadata.len().min(usize::MAX as u64) as usize);
        file.read_to_end(&mut bytes)
            .map_err(|_| LocalFileError::ReadFailed)?;
        if bytes.len() as u64 > max_bytes {
            return Err(LocalFileError::PackageTooLarge);
        }
        Ok(bytes)
    }

    pub fn write_safe_replace(path: &Path, bytes: &[u8]) -> Result<(), LocalFileError> {
        let parent = path
            .parent()
            .filter(|value| !value.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new(CURRENT_DIRECTORY_PATH));
        if !parent.is_dir() {
            return Err(LocalFileError::InvalidPath);
        }
        let file_name = path
            .file_name()
            .and_then(|value| value.to_str())
            .filter(|value| !value.is_empty())
            .ok_or(LocalFileError::InvalidPath)?;

        let operation_token = Self::operation_token()?;
        let temp_path =
            Self::sidecar_path(parent, file_name, &operation_token, SAFE_SAVE_TEMP_SUFFIX);
        let backup_path =
            Self::sidecar_path(parent, file_name, &operation_token, SAFE_SAVE_BACKUP_SUFFIX);

        let mut temp = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
            .map_err(|_| LocalFileError::WriteFailed)?;
        if temp.write_all(bytes).is_err() || temp.sync_all().is_err() {
            let _ = fs::remove_file(&temp_path);
            return Err(LocalFileError::WriteFailed);
        }
        drop(temp);

        let result = Self::replace_file(&temp_path, path, &backup_path);
        if result.is_err() {
            let _ = fs::remove_file(&temp_path);
        }
        result
    }

    fn operation_token() -> Result<String, LocalFileError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| LocalFileError::WriteFailed)?
            .as_nanos();
        Ok(format!("{}-{timestamp}", std::process::id()))
    }

    fn sidecar_path(parent: &Path, file_name: &str, operation_token: &str, suffix: &str) -> PathBuf {
        parent.join(format!(".{file_name}.{operation_token}.{suffix}"))
    }

    #[cfg(not(target_os = "windows"))]
    fn replace_file(
        temp_path: &Path,
        target_path: &Path,
        _backup_path: &Path,
    ) -> Result<(), LocalFileError> {
        fs::rename(temp_path, target_path).map_err(|_| LocalFileError::WriteFailed)
    }

    #[cfg(target_os = "windows")]
    fn replace_file(
        temp_path: &Path,
        target_path: &Path,
        backup_path: &Path,
    ) -> Result<(), LocalFileError> {
        if !target_path.exists() {
            return fs::rename(temp_path, target_path).map_err(|_| LocalFileError::WriteFailed);
        }

        fs::rename(target_path, backup_path).map_err(|_| LocalFileError::WriteFailed)?;
        match fs::rename(temp_path, target_path) {
            Ok(()) => {
                let _ = fs::remove_file(backup_path);
                Ok(())
            }
            Err(_) => {
                let _ = fs::rename(backup_path, target_path);
                Err(LocalFileError::WriteFailed)
            }
        }
    }
}
