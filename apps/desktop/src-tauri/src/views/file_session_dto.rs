// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/file_session_dto.rs
// # 📌 Amac: Writer file-session ve external-change durumlarini Tauri frontend'e serializable DTO olarak aktarir
// # 📌 Modul - FileType: View - Rust
// # Version: 0.2.0
// # Aciklama: Read-only, lock sahipligi, path ve disk degisikligi state kontratini implementation detayindan ayirir
// Bagimli Oldugu Katman: View

use serde::Serialize;

use crate::services::writer_file_session_service::{WriterExternalChangeState, WriterFileSessionStatus};
use crate::views::writer_dto::WriterDocumentDto;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WriterExternalChangeStateDto {
    Untracked,
    Unchanged,
    Modified,
    Missing,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterFileSessionDto {
    pub path: Option<String>,
    pub read_only: bool,
    pub lock_owned: bool,
    pub external_state: WriterExternalChangeStateDto,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterReloadDto {
    pub document: WriterDocumentDto,
    pub file_session: WriterFileSessionDto,
}

impl From<WriterExternalChangeState> for WriterExternalChangeStateDto {
    fn from(value: WriterExternalChangeState) -> Self {
        match value {
            WriterExternalChangeState::Untracked => Self::Untracked,
            WriterExternalChangeState::Unchanged => Self::Unchanged,
            WriterExternalChangeState::Modified => Self::Modified,
            WriterExternalChangeState::Missing => Self::Missing,
        }
    }
}

impl From<WriterFileSessionStatus> for WriterFileSessionDto {
    fn from(value: WriterFileSessionStatus) -> Self {
        Self {
            path: value.path,
            read_only: value.read_only,
            lock_owned: value.lock_owned,
            external_state: value.external_state.into(),
        }
    }
}
