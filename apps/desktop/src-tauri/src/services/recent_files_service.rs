// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/services/recent_files_service.rs
// # 📌 Amac: Native TKO recent file dedup, retention, prune ve siralama is kurallarini yonetir
// # 📌 Modul - FileType: Service - Rust
// # Version: 0.2.0
// # Aciklama: Kalici Repo ile filesystem Tool arasinda maksimum kayit, missing-file cleanup ve last-access semantigini uygular
// Bagimli Oldugu Katman: Service -> Repo -> Tool

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use turkuaz_office_writer::config::constants::TKO_FILE_EXTENSION;

use crate::config::constants::RECENT_FILES_MAX_ENTRIES;
use crate::repositories::recent_files_repository::{
    RecentFilesRepository, RecentFilesRepositoryError,
};
use crate::tools::local_file_tool::LocalFileTool;
use crate::tools::recent_files_metadata_tool::{RecentFileDiskDto, RecentFilesDiskDto};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecentFileEntry {
    pub path: String,
    pub title: String,
    pub last_accessed_unix_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecentFilesError {
    InvalidPath,
    ReadFailed,
    WriteFailed,
    MetadataInvalid,
}

impl From<RecentFilesRepositoryError> for RecentFilesError {
    fn from(value: RecentFilesRepositoryError) -> Self {
        match value {
            RecentFilesRepositoryError::ReadFailed => Self::ReadFailed,
            RecentFilesRepositoryError::WriteFailed => Self::WriteFailed,
            RecentFilesRepositoryError::MetadataInvalid => Self::MetadataInvalid,
        }
    }
}

pub struct RecentFilesService {
    repository: RecentFilesRepository,
}

impl RecentFilesService {
    pub fn new(repository: RecentFilesRepository) -> Self {
        Self { repository }
    }

    pub fn list(&self) -> Result<Vec<RecentFileEntry>, RecentFilesError> {
        let metadata = self.repository.load()?;
        let original = metadata.entries;
        let normalized = Self::normalize(original.clone());
        if normalized != original {
            self.repository.save(&RecentFilesDiskDto {
                entries: normalized.clone(),
            })?;
        }
        Ok(normalized.into_iter().map(Into::into).collect())
    }

    pub fn record(&self, path_text: &str) -> Result<Vec<RecentFileEntry>, RecentFilesError> {
        let path = LocalFileTool::canonicalize_file(Path::new(path_text))
            .map_err(|_| RecentFilesError::InvalidPath)?;
        if !Self::is_tko(&path) {
            return Err(RecentFilesError::InvalidPath);
        }
        let canonical_path = path.to_string_lossy().into_owned();
        let title = path
            .file_stem()
            .and_then(|value| value.to_str())
            .filter(|value| !value.is_empty())
            .ok_or(RecentFilesError::InvalidPath)?
            .to_owned();
        let timestamp = Self::now_unix_ms()?;
        let mut entries = self.repository.load()?.entries;
        entries.retain(|entry| !Self::same_path(&entry.path, &canonical_path));
        entries.insert(
            0,
            RecentFileDiskDto {
                path: canonical_path,
                title,
                last_accessed_unix_ms: timestamp,
            },
        );
        let entries = Self::normalize(entries);
        self.repository.save(&RecentFilesDiskDto {
            entries: entries.clone(),
        })?;
        Ok(entries.into_iter().map(Into::into).collect())
    }

    fn normalize(entries: Vec<RecentFileDiskDto>) -> Vec<RecentFileDiskDto> {
        let mut entries = entries
            .into_iter()
            .filter_map(|entry| {
                if entry.path.trim().is_empty() {
                    return None;
                }
                let canonical = LocalFileTool::canonicalize_file(Path::new(&entry.path)).ok()?;
                if !Self::is_tko(&canonical) {
                    return None;
                }
                let title = canonical
                    .file_stem()
                    .and_then(|value| value.to_str())
                    .filter(|value| !value.is_empty())?
                    .to_owned();
                Some(RecentFileDiskDto {
                    path: canonical.to_string_lossy().into_owned(),
                    title,
                    last_accessed_unix_ms: entry.last_accessed_unix_ms,
                })
            })
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| std::cmp::Reverse(entry.last_accessed_unix_ms));
        let mut unique = Vec::new();
        for entry in entries {
            if unique
                .iter()
                .any(|existing: &RecentFileDiskDto| Self::same_path(&existing.path, &entry.path))
            {
                continue;
            }
            unique.push(entry);
            if unique.len() >= RECENT_FILES_MAX_ENTRIES {
                break;
            }
        }
        unique
    }

    fn is_tko(path: &Path) -> bool {
        path.extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(TKO_FILE_EXTENSION))
    }

    #[cfg(target_os = "windows")]
    fn same_path(left: &str, right: &str) -> bool {
        left.eq_ignore_ascii_case(right)
    }

    #[cfg(not(target_os = "windows"))]
    fn same_path(left: &str, right: &str) -> bool {
        left == right
    }

    fn now_unix_ms() -> Result<u64, RecentFilesError> {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| RecentFilesError::WriteFailed)?
            .as_millis();
        u64::try_from(millis).map_err(|_| RecentFilesError::WriteFailed)
    }
}

impl From<RecentFileDiskDto> for RecentFileEntry {
    fn from(value: RecentFileDiskDto) -> Self {
        Self {
            path: value.path,
            title: value.title,
            last_accessed_unix_ms: value.last_accessed_unix_ms,
        }
    }
}
