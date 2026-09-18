// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/services/writer_recovery_service.rs
// # 📌 Amac: Writer autosave recovery snapshot, retention, restore, compare ve discard is kurallarini yonetir
// # 📌 Modul - FileType: Service - Rust
// # Version: 0.2.0
// # Aciklama: Explicit Save'i degistirmeden TKO snapshot + YAML metadata recovery hattini koordine eder
// Bagimli Oldugu Katman: Service -> Tool

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use turkuaz_office_writer::{TkoPackageService, WriterDocument};

use crate::config::constants::{
    MAX_RECOVERY_METADATA_BYTES, RECOVERY_MAX_AGE_SECONDS, RECOVERY_MAX_SNAPSHOTS_PER_DOCUMENT,
    RECOVERY_METADATA_EXTENSION, RECOVERY_SNAPSHOT_EXTENSION, RECOVERY_SNAPSHOT_PREFIX,
};
use crate::services::writer_storage_service::WriterStorageService;
use crate::tools::local_file_tool::{LocalFileError, LocalFileTool};
use crate::tools::recovery_metadata_tool::{
    RecoveryMetadataDiskDto, RecoveryMetadataError, RecoveryMetadataTool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoverySnapshot {
    pub snapshot_id: String,
    pub document_id: String,
    pub title: String,
    pub source_path: Option<String>,
    pub persisted_revision: u64,
    pub recovery_revision: u64,
    pub created_at_unix_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveryRestoreResult {
    pub snapshot: RecoverySnapshot,
    pub document: WriterDocument,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveryComparison {
    pub snapshot: RecoverySnapshot,
    pub recovery_plain_text: String,
    pub source_plain_text: Option<String>,
    pub source_revision: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriterRecoveryError {
    InvalidSnapshotId,
    SnapshotNotFound,
    MetadataInvalid,
    ReadFailed,
    WriteFailed,
    PackageInvalid,
}

impl From<LocalFileError> for WriterRecoveryError {
    fn from(value: LocalFileError) -> Self {
        match value {
            LocalFileError::ReadFailed => Self::ReadFailed,
            LocalFileError::WriteFailed | LocalFileError::InvalidPath => Self::WriteFailed,
            LocalFileError::PackageTooLarge => Self::PackageInvalid,
        }
    }
}

impl From<RecoveryMetadataError> for WriterRecoveryError {
    fn from(_: RecoveryMetadataError) -> Self {
        Self::MetadataInvalid
    }
}

pub struct WriterRecoveryService {
    root: PathBuf,
}

impl WriterRecoveryService {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn create_snapshot(
        &self,
        document: &WriterDocument,
        source_path: Option<&str>,
        persisted_revision: u64,
    ) -> Result<RecoverySnapshot, WriterRecoveryError> {
        if persisted_revision > document.revision {
            return Err(WriterRecoveryError::MetadataInvalid);
        }
        LocalFileTool::create_directory_all(&self.root)?;
        let created_at_unix_ms = Self::now_unix_ms()?;
        let snapshot_id = format!(
            "{created_at_unix_ms}-{}-{}",
            std::process::id(),
            document.revision
        );
        let snapshot = RecoverySnapshot {
            snapshot_id: snapshot_id.clone(),
            document_id: document.id.as_str().to_owned(),
            title: document.title.clone(),
            source_path: source_path.map(str::to_owned),
            persisted_revision,
            recovery_revision: document.revision,
            created_at_unix_ms,
        };
        let metadata = RecoveryMetadataDiskDto {
            snapshot_id: snapshot_id.clone(),
            document_id: snapshot.document_id.clone(),
            title: snapshot.title.clone(),
            source_path: snapshot.source_path.clone(),
            persisted_revision,
            recovery_revision: document.revision,
            schema_version: document.schema_version.value(),
            created_at_unix_ms,
            created_by_app_version: env!("CARGO_PKG_VERSION").to_owned(),
        };
        let package_bytes = TkoPackageService::serialize(document, env!("CARGO_PKG_VERSION"))
            .map_err(|_| WriterRecoveryError::PackageInvalid)?;
        let metadata_bytes = RecoveryMetadataTool::serialize(&metadata)?;
        if metadata_bytes.len() as u64 > MAX_RECOVERY_METADATA_BYTES {
            return Err(WriterRecoveryError::MetadataInvalid);
        }

        let package_path = self.package_path(&snapshot_id)?;
        let metadata_path = self.metadata_path(&snapshot_id)?;
        LocalFileTool::write_safe_replace(&package_path, &package_bytes)?;
        if let Err(error) = LocalFileTool::write_safe_replace(&metadata_path, &metadata_bytes) {
            let _ = LocalFileTool::remove_file_if_exists(&package_path);
            return Err(error.into());
        }
        let _ = self.list_snapshots();
        Ok(snapshot)
    }

    pub fn list_snapshots(&self) -> Result<Vec<RecoverySnapshot>, WriterRecoveryError> {
        let mut snapshots = self.read_snapshots()?;
        snapshots.sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));
        let now_ms = Self::now_unix_ms()?;
        let max_age_ms = RECOVERY_MAX_AGE_SECONDS.saturating_mul(1_000);
        let mut document_counts = std::collections::HashMap::<String, usize>::new();
        let mut kept = Vec::new();
        for snapshot in snapshots {
            let age = now_ms.saturating_sub(snapshot.created_at_unix_ms);
            let count = document_counts.entry(snapshot.document_id.clone()).or_insert(0);
            *count += 1;
            if age > max_age_ms || *count > RECOVERY_MAX_SNAPSHOTS_PER_DOCUMENT {
                let _ = self.discard(&snapshot.snapshot_id);
            } else {
                kept.push(snapshot);
            }
        }
        Ok(kept)
    }

    fn read_snapshots(&self) -> Result<Vec<RecoverySnapshot>, WriterRecoveryError> {
        if !LocalFileTool::directory_exists(&self.root) {
            return Ok(Vec::new());
        }
        let mut snapshots = Vec::new();
        for path in LocalFileTool::list_files_with_extension(&self.root, RECOVERY_METADATA_EXTENSION)? {
            let bytes = match LocalFileTool::read(&path, MAX_RECOVERY_METADATA_BYTES) {
                Ok(value) => value,
                Err(_) => continue,
            };
            let metadata = match RecoveryMetadataTool::deserialize(&bytes) {
                Ok(value) => value,
                Err(_) => continue,
            };
            if Self::validate_snapshot_id(&metadata.snapshot_id).is_err() {
                continue;
            }
            let package_path = match self.package_path(&metadata.snapshot_id) {
                Ok(value) => value,
                Err(_) => continue,
            };
            if !LocalFileTool::file_exists(&package_path) {
                continue;
            }
            let package_bytes = match LocalFileTool::read(&package_path, TkoPackageService::MAX_PACKAGE_BYTES) {
                Ok(value) => value,
                Err(_) => continue,
            };
            let document = match TkoPackageService::deserialize(&package_bytes) {
                Ok(value) => value,
                Err(_) => continue,
            };
            if document.id.as_str() != metadata.document_id
                || document.revision != metadata.recovery_revision
                || document.schema_version.value() != metadata.schema_version
            {
                continue;
            }
            snapshots.push(Self::snapshot_from_metadata(metadata));
        }
        Ok(snapshots)
    }

    pub fn restore(&self, snapshot_id: &str) -> Result<RecoveryRestoreResult, WriterRecoveryError> {
        let metadata = self.read_metadata(snapshot_id)?;
        let package_path = self.package_path(snapshot_id)?;
        let bytes = LocalFileTool::read(&package_path, TkoPackageService::MAX_PACKAGE_BYTES)?;
        let document = TkoPackageService::deserialize(&bytes)
            .map_err(|_| WriterRecoveryError::PackageInvalid)?;
        if document.id.as_str() != metadata.document_id
            || document.revision != metadata.recovery_revision
            || document.schema_version.value() != metadata.schema_version
        {
            return Err(WriterRecoveryError::MetadataInvalid);
        }
        Ok(RecoveryRestoreResult {
            snapshot: Self::snapshot_from_metadata(metadata),
            document,
        })
    }

    pub fn compare(&self, snapshot_id: &str) -> Result<RecoveryComparison, WriterRecoveryError> {
        let restored = self.restore(snapshot_id)?;
        let (source_plain_text, source_revision) = restored
            .snapshot
            .source_path
            .as_deref()
            .and_then(|path| WriterStorageService::open(path).ok())
            .map(|document| (Some(Self::plain_text(&document)), Some(document.revision)))
            .unwrap_or((None, None));
        Ok(RecoveryComparison {
            recovery_plain_text: Self::plain_text(&restored.document),
            snapshot: restored.snapshot,
            source_plain_text,
            source_revision,
        })
    }

    pub fn discard(&self, snapshot_id: &str) -> Result<(), WriterRecoveryError> {
        LocalFileTool::remove_file_if_exists(&self.package_path(snapshot_id)?)?;
        LocalFileTool::remove_file_if_exists(&self.metadata_path(snapshot_id)?)?;
        Ok(())
    }

    pub fn clear_document(&self, document_id: &str) -> Result<(), WriterRecoveryError> {
        for snapshot in self.list_snapshots()? {
            if snapshot.document_id == document_id {
                let _ = self.discard(&snapshot.snapshot_id);
            }
        }
        Ok(())
    }


    fn read_metadata(&self, snapshot_id: &str) -> Result<RecoveryMetadataDiskDto, WriterRecoveryError> {
        let path = self.metadata_path(snapshot_id)?;
        if !LocalFileTool::file_exists(&path) {
            return Err(WriterRecoveryError::SnapshotNotFound);
        }
        let bytes = LocalFileTool::read(&path, MAX_RECOVERY_METADATA_BYTES)?;
        let metadata = RecoveryMetadataTool::deserialize(&bytes)?;
        if metadata.snapshot_id != snapshot_id {
            return Err(WriterRecoveryError::MetadataInvalid);
        }
        Ok(metadata)
    }

    fn package_path(&self, snapshot_id: &str) -> Result<PathBuf, WriterRecoveryError> {
        Self::validate_snapshot_id(snapshot_id)?;
        Ok(self.root.join(format!(
            "{RECOVERY_SNAPSHOT_PREFIX}{snapshot_id}.{RECOVERY_SNAPSHOT_EXTENSION}"
        )))
    }

    fn metadata_path(&self, snapshot_id: &str) -> Result<PathBuf, WriterRecoveryError> {
        Self::validate_snapshot_id(snapshot_id)?;
        Ok(self.root.join(format!(
            "{RECOVERY_SNAPSHOT_PREFIX}{snapshot_id}.{RECOVERY_METADATA_EXTENSION}"
        )))
    }

    fn validate_snapshot_id(snapshot_id: &str) -> Result<(), WriterRecoveryError> {
        if snapshot_id.is_empty()
            || !snapshot_id
                .chars()
                .all(|character| character.is_ascii_digit() || character == '-')
        {
            return Err(WriterRecoveryError::InvalidSnapshotId);
        }
        Ok(())
    }

    fn snapshot_from_metadata(metadata: RecoveryMetadataDiskDto) -> RecoverySnapshot {
        RecoverySnapshot {
            snapshot_id: metadata.snapshot_id,
            document_id: metadata.document_id,
            title: metadata.title,
            source_path: metadata.source_path,
            persisted_revision: metadata.persisted_revision,
            recovery_revision: metadata.recovery_revision,
            created_at_unix_ms: metadata.created_at_unix_ms,
        }
    }

    fn now_unix_ms() -> Result<u64, WriterRecoveryError> {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| WriterRecoveryError::WriteFailed)?
            .as_millis();
        u64::try_from(millis).map_err(|_| WriterRecoveryError::WriteFailed)
    }

    fn plain_text(document: &WriterDocument) -> String {
        use turkuaz_office_writer::Block;
        let mut paragraphs = Vec::new();
        for section in &document.sections {
            for block in &section.blocks {
                if let Block::Paragraph(paragraph) = block {
                    paragraphs.push(paragraph.runs.iter().map(|run| run.text.as_str()).collect::<String>());
                }
            }
        }
        paragraphs.join("\n")
    }
}
