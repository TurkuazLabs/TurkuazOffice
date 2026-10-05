// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/error_dto.rs
// # 📌 Amac: Desktop backend hatalarini Tauri frontend icin stabil error code DTO'suna map eder
// # 📌 Modul - FileType: View - Rust
// # Version: 0.6.0
// # Aciklama: Writer ve Sheet domain hatalarini implementation detayini sizdirmadan stabil serializable hata koduna map eder
// Bagimli Oldugu Katman: View

use serde::Serialize;
use turkuaz_office_format_adapters::{DocxError, PdfError, PdfWriterError};
use turkuaz_office_sheet::SheetError;
use turkuaz_office_writer::{
    TkoPackageError, TkoProfileError, WriterCommandError, WriterEditorError, WriterTemplateError,
};

use crate::config::constants::{
    ERROR_COMMAND_FAILED, ERROR_DOCUMENT_NOT_FOUND, ERROR_DOCX_INVALID, ERROR_DOCX_UNSUPPORTED,
    ERROR_EXTERNAL_CHANGE_CONFLICT, ERROR_FILE_EXTENSION_INVALID, ERROR_FILE_LOCKED,
    ERROR_FILE_PATH_INVALID, ERROR_FILE_READ_FAILED, ERROR_FILE_WRITE_FAILED,
    ERROR_NOTHING_TO_REDO, ERROR_NOTHING_TO_UNDO, ERROR_PARAGRAPH_NOT_FOUND,
    ERROR_PDF_EXPORT_FAILED, ERROR_PDF_FONT_UNAVAILABLE, ERROR_PDF_UNSUPPORTED,
    ERROR_RECENT_FILES_INVALID, ERROR_RECENT_FILES_READ_FAILED, ERROR_RECENT_FILES_WRITE_FAILED,
    ERROR_RECOVERY_INVALID, ERROR_RECOVERY_NOT_FOUND, ERROR_RECOVERY_READ_FAILED,
    ERROR_RECOVERY_WRITE_FAILED, ERROR_RUN_NOT_FOUND, ERROR_SHEET_CELL_NUMBER_NOT_FINITE,
    ERROR_SHEET_CELL_TEXT_TOO_LONG, ERROR_SHEET_CHART_CATEGORY_NOT_TEXT,
    ERROR_SHEET_CHART_NOT_FOUND, ERROR_SHEET_CHART_TOO_MANY_POINTS,
    ERROR_SHEET_CHART_VALUE_NOT_NUMERIC, ERROR_SHEET_DOCUMENT_NOT_FOUND, ERROR_SHEET_FORMULA_CYCLE,
    ERROR_SHEET_FORMULA_DEPTH_EXCEEDED, ERROR_SHEET_FORMULA_DIVISION_BY_ZERO,
    ERROR_SHEET_FORMULA_NON_NUMERIC_REFERENCE, ERROR_SHEET_FORMULA_RESULT_NOT_FINITE,
    ERROR_SHEET_INVALID_CELL_FORMAT, ERROR_SHEET_INVALID_CELL_REFERENCE,
    ERROR_SHEET_INVALID_CHART_RANGE, ERROR_SHEET_INVALID_CHART_TITLE, ERROR_SHEET_INVALID_FILTER,
    ERROR_SHEET_INVALID_FORMULA, ERROR_SHEET_INVALID_RANGE, ERROR_SHEET_INVALID_TABLE_RANGE,
    ERROR_SHEET_QUERY_TOO_LARGE, ERROR_SHEET_TABLE_NOT_FOUND, ERROR_SHEET_TABLE_RANGE_OVERLAP,
    ERROR_SHEET_WORKSHEET_NOT_FOUND, ERROR_TEMPLATE_INVALID, ERROR_TEMPLATE_NOT_FOUND,
    ERROR_TKO_FUTURE_SCHEMA, ERROR_TKO_INVALID, ERROR_TKO_MIGRATION_REQUIRED,
};
use crate::services::recent_files_service::RecentFilesError;
use crate::services::writer_docx_service::WriterDocxError;
use crate::services::writer_file_session_service::WriterFileSessionError;
use crate::services::writer_pdf_service::WriterPdfError;
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

impl From<SheetError> for DesktopErrorDto {
    fn from(error: SheetError) -> Self {
        match error {
            SheetError::DocumentNotFound => Self::new(ERROR_SHEET_DOCUMENT_NOT_FOUND),
            SheetError::WorksheetNotFound => Self::new(ERROR_SHEET_WORKSHEET_NOT_FOUND),
            SheetError::InvalidCellReference | SheetError::CellOutOfBounds => {
                Self::new(ERROR_SHEET_INVALID_CELL_REFERENCE)
            }
            SheetError::CellTextTooLong => Self::new(ERROR_SHEET_CELL_TEXT_TOO_LONG),
            SheetError::CellNumberNotFinite => Self::new(ERROR_SHEET_CELL_NUMBER_NOT_FINITE),
            SheetError::InvalidFormula => Self::new(ERROR_SHEET_INVALID_FORMULA),
            SheetError::FormulaCycle => Self::new(ERROR_SHEET_FORMULA_CYCLE),
            SheetError::FormulaDepthExceeded => Self::new(ERROR_SHEET_FORMULA_DEPTH_EXCEEDED),
            SheetError::FormulaDivisionByZero => Self::new(ERROR_SHEET_FORMULA_DIVISION_BY_ZERO),
            SheetError::FormulaNonNumericReference => {
                Self::new(ERROR_SHEET_FORMULA_NON_NUMERIC_REFERENCE)
            }
            SheetError::FormulaResultNotFinite => Self::new(ERROR_SHEET_FORMULA_RESULT_NOT_FINITE),
            SheetError::InvalidCellFormat => Self::new(ERROR_SHEET_INVALID_CELL_FORMAT),
            SheetError::InvalidRange => Self::new(ERROR_SHEET_INVALID_RANGE),
            SheetError::QueryTooLarge => Self::new(ERROR_SHEET_QUERY_TOO_LARGE),
            SheetError::InvalidFilter => Self::new(ERROR_SHEET_INVALID_FILTER),
            SheetError::TableNotFound => Self::new(ERROR_SHEET_TABLE_NOT_FOUND),
            SheetError::InvalidTableRange => Self::new(ERROR_SHEET_INVALID_TABLE_RANGE),
            SheetError::TableRangeOverlap => Self::new(ERROR_SHEET_TABLE_RANGE_OVERLAP),
            SheetError::ChartNotFound => Self::new(ERROR_SHEET_CHART_NOT_FOUND),
            SheetError::InvalidChartTitle => Self::new(ERROR_SHEET_INVALID_CHART_TITLE),
            SheetError::InvalidChartRange => Self::new(ERROR_SHEET_INVALID_CHART_RANGE),
            SheetError::ChartTooManyPoints => Self::new(ERROR_SHEET_CHART_TOO_MANY_POINTS),
            SheetError::ChartCategoryNotText => Self::new(ERROR_SHEET_CHART_CATEGORY_NOT_TEXT),
            SheetError::ChartValueNotNumeric => Self::new(ERROR_SHEET_CHART_VALUE_NOT_NUMERIC),
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
            WriterEditorError::Template(WriterTemplateError::CatalogInvalid) => {
                Self::new(ERROR_TEMPLATE_INVALID)
            }
            WriterEditorError::Template(WriterTemplateError::TemplateNotFound) => {
                Self::new(ERROR_TEMPLATE_NOT_FOUND)
            }
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

impl From<WriterPdfError> for DesktopErrorDto {
    fn from(error: WriterPdfError) -> Self {
        match error {
            WriterPdfError::InvalidPath => Self::new(ERROR_FILE_PATH_INVALID),
            WriterPdfError::OutputTooLarge | WriterPdfError::WriteFailed => {
                Self::new(ERROR_FILE_WRITE_FAILED)
            }
            WriterPdfError::Font(_) | WriterPdfError::Format(PdfError::FontMissing(_)) => {
                Self::new(ERROR_PDF_FONT_UNAVAILABLE)
            }
            WriterPdfError::Format(
                PdfError::MultipleSectionsUnsupported
                | PdfError::UnsupportedBlock
                | PdfError::UnsupportedAsset,
            ) => Self::new(ERROR_PDF_UNSUPPORTED),
            WriterPdfError::Format(PdfError::Writer(
                PdfWriterError::FontMissing
                | PdfWriterError::FontInvalid
                | PdfWriterError::GlyphMissing,
            )) => Self::new(ERROR_PDF_FONT_UNAVAILABLE),
            WriterPdfError::Format(_) => Self::new(ERROR_PDF_EXPORT_FAILED),
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

impl From<RecentFilesError> for DesktopErrorDto {
    fn from(error: RecentFilesError) -> Self {
        match error {
            RecentFilesError::InvalidPath | RecentFilesError::MetadataInvalid => {
                Self::new(ERROR_RECENT_FILES_INVALID)
            }
            RecentFilesError::ReadFailed => Self::new(ERROR_RECENT_FILES_READ_FAILED),
            RecentFilesError::WriteFailed => Self::new(ERROR_RECENT_FILES_WRITE_FAILED),
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
