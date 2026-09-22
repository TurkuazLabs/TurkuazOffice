// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/error_dto.rs
// # 📌 Amac: Desktop backend hatalarini Tauri frontend icin stabil error code DTO'suna map eder
// # 📌 Modul - FileType: View - Rust
// # Version: 0.2.0
// # Aciklama: Rust Debug metni veya implementation detayini UI kontratina sizdirmadan serializable hata kodu verir
// Bagimli Oldugu Katman: View

use serde::Serialize;
use turkuaz_office_format_adapters::DocxError;
use turkuaz_office_writer::{
    TkoPackageError, TkoProfileError, WriterCommandError, WriterEditorError,
};

use crate::config::constants::{
    ERROR_COMMAND_FAILED, ERROR_DOCUMENT_NOT_FOUND, ERROR_DOCX_INVALID, ERROR_DOCX_UNSUPPORTED,
    ERROR_EXTERNAL_CHANGE_CONFLICT,
    ERROR_FILE_EXTENSION_INVALID, ERROR_FILE_LOCKED, ERROR_FILE_PATH_INVALID,
    ERROR_FILE_READ_FAILED, ERROR_FILE_WRITE_FAILED, ERROR_NOTHING_TO_REDO, ERROR_NOTHING_TO_UNDO,
    ERROR_PARAGRAPH_NOT_FOUND, ERROR_RECOVERY_INVALID, ERROR_RECOVERY_NOT_FOUND,
    ERROR_RECOVERY_READ_FAILED, ERROR_RECOVERY_WRITE_FAILED, ERROR_RUN_NOT_FOUND,
    ERROR_TKO_FUTURE_SCHEMA, ERROR_TKO_INVALID, ERROR_TKO_MIGRATION_REQUIRED,
};
use crate::services::writer_docx_service::WriterDocxError;
use crate::services::writer_file_session_service::WriterFileSessionError;
use crate::services::writer_recovery_service::WriterRecoveryError;
use crate::services::writer_storage_service::WriterStorageError;

#[derive(Clone, Debug, Serialize)]
pub struct DesktopErrorDto {
    pub code: String,
}

impl DesktopErrorDto {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_owned(),
        }
    }
}

impl From<WriterEditorError> for DesktopErrorDto {
    fn from(error: WriterEditorError) -> Self {
        match error {
            WriterEditorError::DocumentNotFound => Self::new(ERROR_DOCUMENT_NOT_FOUND),
            WriterEditorError::NothingToUndo => Self::new(ERROR_NOTHING_TO_UNDO),
            WriterEditorError::NothingToRedo => Self::new(ERROR_NOTHING_TO_REDO),
            WriterEditorError::Command(command_error) => match command_error {
                WriterCommandError::ParagraphNotFound => Self::new(ERROR_PARAGRAPH_NOT_FOUND),
                WriterCommandError::RunNotFound => Self::new(ERROR_RUN_NOT_FOUND),
                _ => Self::new(ERROR_COMMAND_FAILED),
            },
        }
    }
}
impl From<WriterDocxError> for DesktopErrorDto {
    fn from(error: WriterDocxError) -> Self {
        match error {
            WriterDocxError::InvalidPath => Self::new(ERROR_FILE_PATH_INVALID),
            WriterDocxError::InvalidExtension => Self::new(ERROR_FILE_EXTENSION_INVALID),
            WriterDocxError::PackageTooLarge | WriterDocxError::ReadFailed => {
                Self::new(ERROR_FILE_READ_FAILED)
            }
            WriterDocxError::WriteFailed => Self::new(ERROR_FILE_WRITE_FAILED),
            WriterDocxError::Format(
                DocxError::MultipleSectionsUnsupported
                | DocxError::UnsupportedBlock
                | DocxError::UnsupportedAsset
                | DocxError::InvalidCharacterStyle,
            ) => Self::new(ERROR_DOCX_UNSUPPORTED),
            WriterDocxError::Format(_) => Self::new(ERROR_DOCX_INVALID),
        }
    }
}

impl From<WriterStorageError> for DesktopErrorDto {
    fn from(error: WriterStorageError) -> Self {
        match error {
            WriterStorageError::InvalidPath => Self::new(ERROR_FILE_PATH_INVALID),
            WriterStorageError::InvalidExtension => Self::new(ERROR_FILE_EXTENSION_INVALID),
            WriterStorageError::ReadFailed => Self::new(ERROR_FILE_READ_FAILED),
            WriterStorageError::WriteFailed => Self::new(ERROR_FILE_WRITE_FAILED),
            WriterStorageError::Package(TkoPackageError::Profile(
                TkoProfileError::FutureSchema,
            )) => Self::new(ERROR_TKO_FUTURE_SCHEMA),
            WriterStorageError::Package(TkoPackageError::Profile(
                TkoProfileError::MigrationRequired,
            )) => Self::new(ERROR_TKO_MIGRATION_REQUIRED),
            WriterStorageError::Package(_) => Self::new(ERROR_TKO_INVALID),
        }
    }
}

impl From<WriterRecoveryError> for DesktopErrorDto {
    fn from(error: WriterRecoveryError) -> Self {
        match error {
            WriterRecoveryError::InvalidSnapshotId
            | WriterRecoveryError::MetadataInvalid
            | WriterRecoveryError::PackageInvalid => Self::new(ERROR_RECOVERY_INVALID),
            WriterRecoveryError::SnapshotNotFound => Self::new(ERROR_RECOVERY_NOT_FOUND),
            WriterRecoveryError::ReadFailed => Self::new(ERROR_RECOVERY_READ_FAILED),
            WriterRecoveryError::WriteFailed => Self::new(ERROR_RECOVERY_WRITE_FAILED),
        }
    }
}
impl From<WriterFileSessionError> for DesktopErrorDto {
    fn from(error: WriterFileSessionError) -> Self {
        match error {
            WriterFileSessionError::FileLocked => Self::new(ERROR_FILE_LOCKED),
            WriterFileSessionError::ExternalChangeConflict => {
                Self::new(ERROR_EXTERNAL_CHANGE_CONFLICT)
            }
            WriterFileSessionError::InvalidPath => Self::new(ERROR_FILE_PATH_INVALID),
            WriterFileSessionError::ReadFailed => Self::new(ERROR_FILE_READ_FAILED),
            WriterFileSessionError::WriteFailed => Self::new(ERROR_FILE_WRITE_FAILED),
        }
    }
}
