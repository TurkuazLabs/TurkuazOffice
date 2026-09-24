// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/recent_file_dto.rs
// # 📌 Amac: Recent native TKO file kaydini frontend icin serializable typed DTO olarak tasir
// # 📌 Modul - FileType: View - Rust
// # Version: 0.2.0
// # Aciklama: Path, display title ve last-access timestamp alanlarini camelCase IPC kontratina cevirir
// Bagimli Oldugu Katman: View

use serde::Serialize;

use crate::services::recent_files_service::RecentFileEntry;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentFileDto {
    pub path: String,
    pub title: String,
    pub last_accessed_unix_ms: u64,
}

impl From<RecentFileEntry> for RecentFileDto {
    fn from(value: RecentFileEntry) -> Self {
        Self {
            path: value.path,
            title: value.title,
            last_accessed_unix_ms: value.last_accessed_unix_ms,
        }
    }
}
