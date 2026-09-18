// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/tools/file_lock_tool.rs
// # 📌 Amac: Turkuaz Office surecleri arasinda cooperative sidecar dosya kilidi saglar
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: create_new ile lock sahipligi alir, token dogrulamasi ile yalniz kendi kilidini serbest birakir
// Bagimli Oldugu Katman: Tool -> Config

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::constants::{FILE_LOCK_SUFFIX, MAX_FILE_LOCK_BYTES};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileLockLease {
    pub lock_path: PathBuf,
    pub token: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileLockError {
    InvalidPath,
    Locked,
    ReadFailed,
    WriteFailed,
}

pub struct FileLockTool;

impl FileLockTool {
    pub fn acquire(target_path: &Path) -> Result<FileLockLease, FileLockError> {
        let lock_path = Self::lock_path(target_path)?;
        let token = Self::token()?;
        let mut file = match OpenOptions::new().write(true).create_new(true).open(&lock_path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(FileLockError::Locked);
            }
            Err(_) => return Err(FileLockError::WriteFailed),
        };
        if file.write_all(token.as_bytes()).is_err() || file.sync_all().is_err() {
            let _ = fs::remove_file(&lock_path);
            return Err(FileLockError::WriteFailed);
        }
        Ok(FileLockLease { lock_path, token })
    }

    pub fn release(lease: &FileLockLease) -> Result<(), FileLockError> {
        if !lease.lock_path.exists() {
            return Ok(());
        }
        let mut file = fs::File::open(&lease.lock_path).map_err(|_| FileLockError::ReadFailed)?;
        let metadata = file.metadata().map_err(|_| FileLockError::ReadFailed)?;
        if metadata.len() > MAX_FILE_LOCK_BYTES {
            return Err(FileLockError::ReadFailed);
        }
        let mut token = String::new();
        file.read_to_string(&mut token).map_err(|_| FileLockError::ReadFailed)?;
        if token != lease.token {
            return Err(FileLockError::Locked);
        }
        fs::remove_file(&lease.lock_path).map_err(|_| FileLockError::WriteFailed)
    }

    pub fn lock_path(target_path: &Path) -> Result<PathBuf, FileLockError> {
        let parent = target_path.parent().ok_or(FileLockError::InvalidPath)?;
        let file_name = target_path
            .file_name()
            .and_then(|value| value.to_str())
            .filter(|value| !value.is_empty())
            .ok_or(FileLockError::InvalidPath)?;
        Ok(parent.join(format!(".{file_name}.{FILE_LOCK_SUFFIX}")))
    }

    fn token() -> Result<String, FileLockError> {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| FileLockError::WriteFailed)?
            .as_millis();
        Ok(format!("{}:{created_at}", std::process::id()))
    }
}
