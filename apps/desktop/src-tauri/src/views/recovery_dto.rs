// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/recovery_dto.rs
// # 📌 Amac: Recovery Service sonuclarini frontend icin read-only serializable DTO kontratina cevirir
// # 📌 Modul - FileType: View - Rust
// # Version: 0.2.0
// # Aciklama: Snapshot listesi, restore sonucu ve compare verisini implementation detayindan ayirir
// Bagimli Oldugu Katman: View

use serde::Serialize;

use crate::services::writer_recovery_service::{RecoveryComparison, RecoverySnapshot};
use crate::views::writer_dto::WriterDocumentDto;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoverySnapshotDto {
    pub snapshot_id: String,
    pub document_id: String,
    pub title: String,
    pub source_path: Option<String>,
    pub persisted_revision: u64,
    pub recovery_revision: u64,
    pub created_at_unix_ms: u64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryRestoreDto {
    pub snapshot: RecoverySnapshotDto,
    pub document: WriterDocumentDto,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryComparisonDto {
    pub snapshot: RecoverySnapshotDto,
    pub recovery_plain_text: String,
    pub source_plain_text: Option<String>,
    pub source_revision: Option<u64>,
}

impl From<RecoverySnapshot> for RecoverySnapshotDto {
    fn from(value: RecoverySnapshot) -> Self {
        Self {
            snapshot_id: value.snapshot_id,
            document_id: value.document_id,
            title: value.title,
            source_path: value.source_path,
            persisted_revision: value.persisted_revision,
            recovery_revision: value.recovery_revision,
            created_at_unix_ms: value.created_at_unix_ms,
        }
    }
}

impl From<RecoveryComparison> for RecoveryComparisonDto {
    fn from(value: RecoveryComparison) -> Self {
        Self {
            snapshot: value.snapshot.into(),
            recovery_plain_text: value.recovery_plain_text,
            source_plain_text: value.source_plain_text,
            source_revision: value.source_revision,
        }
    }
}
