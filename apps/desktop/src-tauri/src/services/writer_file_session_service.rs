// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/services/writer_file_session_service.rs
// # 📌 Amac: Acik Writer dosyasinin fingerprint, lock, read-only ve external-change durumunu yonetir
// # 📌 Modul - FileType: Service - Rust
// # Version: 0.2.0
// # Aciklama: Save conflict, cooperative lock sahipligi ve dis dosya degisikligi kararlarini Tool katmani uzerinden koordine eder
// Bagimli Oldugu Katman: Service -> Tool

use std::path::{Path, PathBuf};

use turkuaz_office_writer::TkoPackageService;

use crate::tools::file_fingerprint_tool::{FileFingerprint, FileFingerprintTool};
use crate::tools::file_lock_tool::{FileLockError, FileLockLease, FileLockTool};
use crate::tools::local_file_tool::{LocalFileError, LocalFileTool};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriterExternalChangeState {
    Untracked,
    Unchanged,
    Modified,
    Missing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriterFileSessionStatus {
    pub path: Option<String>,
    pub read_only: bool,
    pub lock_owned: bool,
    pub external_state: WriterExternalChangeState,
}

#[derive(Debug)]
pub enum WriterFileSessionError {
    FileLocked,
    ExternalChangeConflict,
    InvalidPath,
    ReadFailed,
    WriteFailed,
}

#[derive(Debug)]
pub struct PreparedFileSave {
    pub target_path: PathBuf,
    pub new_lease: Option<FileLockLease>,
    pub same_path: bool,
}

#[derive(Debug)]
struct WriterFileSession {
    document_id: String,
    path: PathBuf,
    fingerprint: Option<FileFingerprint>,
    lease: Option<FileLockLease>,
    read_only: bool,
}

#[derive(Default)]
pub struct WriterFileSessionService {
    current: Option<WriterFileSession>,
}

impl WriterFileSessionService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset_untracked(&mut self) {
        self.release_current_lock();
        self.current = None;
    }

    pub fn track_open(
        &mut self,
        document_id: &str,
        path: &Path,
    ) -> Result<(), WriterFileSessionError> {
        let fingerprint = Some(Self::fingerprint(path)?);
        if let Some(current) = self
            .current
            .as_mut()
            .filter(|item| Self::same_path(&item.path, path))
        {
            current.document_id = document_id.to_owned();
            current.path = path.to_path_buf();
            current.fingerprint = fingerprint;
            return Ok(());
        }
        let (lease, read_only) = match FileLockTool::acquire(path) {
            Ok(lease) => (Some(lease), false),
            Err(FileLockError::Locked | FileLockError::ReadFailed | FileLockError::WriteFailed) => {
                (None, true)
            }
            Err(FileLockError::InvalidPath) => return Err(WriterFileSessionError::InvalidPath),
        };
        self.release_current_lock();
        self.current = Some(WriterFileSession {
            document_id: document_id.to_owned(),
            path: path.to_path_buf(),
            fingerprint,
            lease,
            read_only,
        });
        Ok(())
    }

    pub fn status(
        &self,
        document_id: &str,
    ) -> Result<WriterFileSessionStatus, WriterFileSessionError> {
        let Some(session) = self
            .current
            .as_ref()
            .filter(|item| item.document_id == document_id)
        else {
            return Ok(WriterFileSessionStatus {
                path: None,
                read_only: false,
                lock_owned: false,
                external_state: WriterExternalChangeState::Untracked,
            });
        };
        Ok(WriterFileSessionStatus {
            path: Some(session.path.to_string_lossy().into_owned()),
            read_only: session.read_only,
            lock_owned: session.lease.is_some(),
            external_state: Self::external_state(session)?,
        })
    }

    pub fn ensure_writable(&self, document_id: &str) -> Result<(), WriterFileSessionError> {
        if let Some(session) = self
            .current
            .as_ref()
            .filter(|item| item.document_id == document_id)
            && session.read_only
        {
            return Err(WriterFileSessionError::FileLocked);
        }
        Ok(())
    }

    pub fn prepare_save(
        &self,
        document_id: &str,
        target_path: &Path,
    ) -> Result<PreparedFileSave, WriterFileSessionError> {
        let same_path = self
            .current
            .as_ref()
            .filter(|item| item.document_id == document_id)
            .is_some_and(|item| Self::same_path(&item.path, target_path));

        if same_path {
            let session = self
                .current
                .as_ref()
                .filter(|item| item.document_id == document_id)
                .ok_or(WriterFileSessionError::InvalidPath)?;
            if session.read_only {
                return Err(WriterFileSessionError::FileLocked);
            }
            if Self::external_state(session)? != WriterExternalChangeState::Unchanged {
                return Err(WriterFileSessionError::ExternalChangeConflict);
            }
            return Ok(PreparedFileSave {
                target_path: target_path.to_path_buf(),
                new_lease: None,
                same_path: true,
            });
        }

        let new_lease = FileLockTool::acquire(target_path).map_err(Self::map_lock_error)?;
        Ok(PreparedFileSave {
            target_path: target_path.to_path_buf(),
            new_lease: Some(new_lease),
            same_path: false,
        })
    }

    pub fn cancel_save(&self, prepared: &PreparedFileSave) {
        if let Some(lease) = &prepared.new_lease {
            let _ = FileLockTool::release(lease);
        }
    }

    pub fn commit_save(
        &mut self,
        document_id: &str,
        prepared: PreparedFileSave,
    ) -> Result<(), WriterFileSessionError> {
        let fingerprint = Some(Self::fingerprint(&prepared.target_path)?);
        if prepared.same_path
            && let Some(session) = self
                .current
                .as_mut()
                .filter(|item| item.document_id == document_id)
        {
            session.fingerprint = fingerprint;
            return Ok(());
        }

        let lease = prepared.new_lease;
        self.release_current_lock();
        self.current = Some(WriterFileSession {
            document_id: document_id.to_owned(),
            path: prepared.target_path,
            fingerprint,
            lease,
            read_only: false,
        });
        Ok(())
    }

    pub fn acknowledge_external_change(
        &mut self,
        document_id: &str,
    ) -> Result<WriterFileSessionStatus, WriterFileSessionError> {
        let session = self
            .current
            .as_mut()
            .filter(|item| item.document_id == document_id)
            .ok_or(WriterFileSessionError::InvalidPath)?;
        session.fingerprint = Self::fingerprint_optional(&session.path)?;
        self.status(document_id)
    }

    pub fn refresh_baseline(&mut self, document_id: &str) -> Result<(), WriterFileSessionError> {
        let session = self
            .current
            .as_mut()
            .filter(|item| item.document_id == document_id)
            .ok_or(WriterFileSessionError::InvalidPath)?;
        session.fingerprint = Some(Self::fingerprint(&session.path)?);
        Ok(())
    }

    pub fn current_path(&self, document_id: &str) -> Option<PathBuf> {
        self.current
            .as_ref()
            .filter(|item| item.document_id == document_id)
            .map(|item| item.path.clone())
    }

    fn external_state(
        session: &WriterFileSession,
    ) -> Result<WriterExternalChangeState, WriterFileSessionError> {
        let current = Self::fingerprint_optional(&session.path)?;
        Ok(match (session.fingerprint, current) {
            (None, None) => WriterExternalChangeState::Unchanged,
            (Some(_), None) => WriterExternalChangeState::Missing,
            (None, Some(_)) => WriterExternalChangeState::Modified,
            (Some(expected), Some(actual)) if expected == actual => {
                WriterExternalChangeState::Unchanged
            }
            (Some(_), Some(_)) => WriterExternalChangeState::Modified,
        })
    }

    fn fingerprint(path: &Path) -> Result<FileFingerprint, WriterFileSessionError> {
        FileFingerprintTool::read(path, TkoPackageService::MAX_PACKAGE_BYTES)
            .map_err(Self::map_local_error)
    }

    fn fingerprint_optional(
        path: &Path,
    ) -> Result<Option<FileFingerprint>, WriterFileSessionError> {
        if !LocalFileTool::file_exists(path) {
            return Ok(None);
        }
        Self::fingerprint(path).map(Some)
    }

    fn same_path(left: &Path, right: &Path) -> bool {
        Self::identity(left) == Self::identity(right)
    }

    fn identity(path: &Path) -> PathBuf {
        if let Ok(value) = std::fs::canonicalize(path) {
            return value;
        }
        if path.is_absolute() {
            return path.to_path_buf();
        }
        std::env::current_dir().map_or_else(|_| path.to_path_buf(), |current| current.join(path))
    }

    fn release_current_lock(&mut self) {
        if let Some(lease) = self.current.as_ref().and_then(|item| item.lease.as_ref()) {
            let _ = FileLockTool::release(lease);
        }
    }

    fn map_local_error(error: LocalFileError) -> WriterFileSessionError {
        match error {
            LocalFileError::InvalidPath => WriterFileSessionError::InvalidPath,
            LocalFileError::PackageTooLarge | LocalFileError::ReadFailed => {
                WriterFileSessionError::ReadFailed
            }
            LocalFileError::WriteFailed => WriterFileSessionError::WriteFailed,
        }
    }

    fn map_lock_error(error: FileLockError) -> WriterFileSessionError {
        match error {
            FileLockError::Locked => WriterFileSessionError::FileLocked,
            FileLockError::InvalidPath => WriterFileSessionError::InvalidPath,
            FileLockError::ReadFailed => WriterFileSessionError::ReadFailed,
            FileLockError::WriteFailed => WriterFileSessionError::WriteFailed,
        }
    }
}

impl Drop for WriterFileSessionService {
    fn drop(&mut self) {
        self.release_current_lock();
    }
}
