// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/config/constants.rs
// # 📌 Amac: Desktop Rust shell error code ve state sabitlerini merkezi tutar
// # 📌 Modul - FileType: Config - Rust
// # Version: 0.2.0
// # Aciklama: Controller, storage, recovery ve Service katmanlarinda magic string kullanilmasini engeller
// Bagimli Oldugu Katman: Config

pub const ERROR_STATE_LOCK: &str = "desktop.state_lock";
pub const ERROR_DOCUMENT_NOT_FOUND: &str = "writer.document_not_found";
pub const ERROR_PARAGRAPH_NOT_FOUND: &str = "writer.paragraph_not_found";
pub const ERROR_RUN_NOT_FOUND: &str = "writer.run_not_found";
pub const ERROR_ASSET_NOT_FOUND: &str = "writer.asset_not_found";
pub const ERROR_NOTHING_TO_UNDO: &str = "writer.nothing_to_undo";
pub const ERROR_NOTHING_TO_REDO: &str = "writer.nothing_to_redo";
pub const ERROR_COMMAND_FAILED: &str = "writer.command_failed";
pub const ERROR_INVALID_OFFSET: &str = "writer.invalid_offset";
pub const ERROR_FILE_PATH_INVALID: &str = "writer.file_path_invalid";
pub const ERROR_FILE_EXTENSION_INVALID: &str = "writer.file_extension_invalid";
pub const ERROR_FILE_READ_FAILED: &str = "writer.file_read_failed";
pub const ERROR_FILE_WRITE_FAILED: &str = "writer.file_write_failed";
pub const ERROR_TKO_INVALID: &str = "writer.tko_invalid";
pub const ERROR_TKO_FUTURE_SCHEMA: &str = "writer.tko_future_schema";
pub const ERROR_TKO_MIGRATION_REQUIRED: &str = "writer.tko_migration_required";
pub const ERROR_DOCX_INVALID: &str = "writer.docx_invalid";
pub const ERROR_DOCX_UNSUPPORTED: &str = "writer.docx_unsupported";
pub const ERROR_PDF_EXPORT_FAILED: &str = "writer.pdf_export_failed";
pub const ERROR_PDF_FONT_UNAVAILABLE: &str = "writer.pdf_font_unavailable";
pub const ERROR_PDF_UNSUPPORTED: &str = "writer.pdf_unsupported";
pub const ERROR_RECENT_FILES_READ_FAILED: &str = "writer.recent_files_read_failed";
pub const ERROR_RECENT_FILES_WRITE_FAILED: &str = "writer.recent_files_write_failed";
pub const ERROR_RECENT_FILES_INVALID: &str = "writer.recent_files_invalid";

pub const ERROR_FILE_LOCKED: &str = "writer.file_locked";
pub const ERROR_EXTERNAL_CHANGE_CONFLICT: &str = "writer.external_change_conflict";
pub const FILE_LOCK_SUFFIX: &str = "turkuaz.lock";
pub const MAX_FILE_LOCK_BYTES: u64 = 4096;

pub const ERROR_RECOVERY_INVALID: &str = "writer.recovery_invalid";
pub const ERROR_RECOVERY_NOT_FOUND: &str = "writer.recovery_not_found";
pub const ERROR_RECOVERY_READ_FAILED: &str = "writer.recovery_read_failed";
pub const ERROR_RECOVERY_WRITE_FAILED: &str = "writer.recovery_write_failed";
pub const APP_DIRECTORY_NAME: &str = "TurkuazOffice";
pub const LINUX_APP_DIRECTORY_NAME: &str = "turkuaz-office";
pub const TURKUAZLABS_DIRECTORY_NAME: &str = "TurkuazLabs";
pub const RECOVERY_DIRECTORY_NAME: &str = "recovery";
pub const RECENT_FILES_METADATA_NAME: &str = "recent-files.yml";
pub const RECENT_FILES_MAX_ENTRIES: usize = 12;
pub const MAX_RECENT_FILES_METADATA_BYTES: u64 = 65_536;
pub const RECOVERY_SNAPSHOT_PREFIX: &str = "snapshot-";
pub const RECOVERY_SNAPSHOT_EXTENSION: &str = "tko";
pub const RECOVERY_METADATA_EXTENSION: &str = "yml";
pub const RECOVERY_MAX_SNAPSHOTS_PER_DOCUMENT: usize = 5;
pub const RECOVERY_MAX_AGE_SECONDS: u64 = 604_800;
pub const MAX_RECOVERY_METADATA_BYTES: u64 = 65_536;
pub const WINDOWS_LOCAL_APP_DATA_ENV: &str = "LOCALAPPDATA";
pub const XDG_STATE_HOME_ENV: &str = "XDG_STATE_HOME";
pub const HOME_DIRECTORY_ENV: &str = "HOME";
pub const LINUX_LOCAL_STATE_SEGMENTS: &[&str] = &[".local", "state"];
pub const MACOS_APP_SUPPORT_SEGMENTS: &[&str] = &["Library", "Application Support"];

pub const SAFE_SAVE_TEMP_SUFFIX: &str = "turkuaz-save.tmp";
pub const SAFE_SAVE_BACKUP_SUFFIX: &str = "turkuaz-save.bak";

pub const CURRENT_DIRECTORY_PATH: &str = ".";
pub const STARTUP_ARGUMENT_FLAG_PREFIX: &str = "-";
pub const WINDOWS_VERBATIM_PATH_PREFIX: &str = r"\\?\";
pub const WINDOWS_VERBATIM_UNC_PATH_PREFIX: &str = r"\\?\UNC\";
pub const WINDOWS_UNC_PATH_PREFIX: &str = r"\\";
