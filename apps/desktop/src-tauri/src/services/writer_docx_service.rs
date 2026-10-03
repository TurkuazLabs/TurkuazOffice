// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/services/writer_docx_service.rs
// # 📌 Amac: Desktop DOCX import/export dosya yolu ve safe-write is kurallarini koordine eder
// # 📌 Modul - FileType: Service - Rust
// # Version: 0.2.0
// # Aciklama: DOCX adapterini LocalFileTool ile baglar; foreign format kaynagini native TKO file session'dan ayirir
// Bagimli Oldugu Katman: Service -> Tool

use std::path::{Path, PathBuf};

use turkuaz_office_format_adapters::config::constants::{
    DOCX_FILE_EXTENSION, MAX_DOCX_PACKAGE_BYTES,
};
use turkuaz_office_format_adapters::{DocxError, DocxImportResult, DocxService};
use turkuaz_office_writer::{SequentialWriterIdTool, WriterDocument};

use crate::tools::local_file_tool::{LocalFileError, LocalFileTool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriterDocxError {
    InvalidPath,
    InvalidExtension,
    PackageTooLarge,
    ReadFailed,
    WriteFailed,
    Format(DocxError),
}

impl From<DocxError> for WriterDocxError {
    fn from(value: DocxError) -> Self {
        Self::Format(value)
    }
}

impl From<LocalFileError> for WriterDocxError {
    fn from(value: LocalFileError) -> Self {
        match value {
            LocalFileError::InvalidPath => Self::InvalidPath,
            LocalFileError::PackageTooLarge => Self::PackageTooLarge,
            LocalFileError::ReadFailed => Self::ReadFailed,
            LocalFileError::WriteFailed => Self::WriteFailed,
        }
    }
}

pub struct WriterDocxService;

impl WriterDocxService {
    pub fn import(path_text: &str) -> Result<DocxImportResult, WriterDocxError> {
        let path = Self::resolve_open_path(path_text)?;
        let bytes = LocalFileTool::read(&path, MAX_DOCX_PACKAGE_BYTES)?;
        let title = path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        DocxService::import(&bytes, title, &SequentialWriterIdTool::new()).map_err(Into::into)
    }

    pub fn export(document: &WriterDocument, path_text: &str) -> Result<String, WriterDocxError> {
        let path = Self::resolve_save_path(path_text)?;
        let bytes = DocxService::export(document)?;
        LocalFileTool::write_safe_replace(&path, &bytes)?;
        Ok(path.to_string_lossy().into_owned())
    }

    pub fn resolve_open_path(path_text: &str) -> Result<PathBuf, WriterDocxError> {
        let path = Self::base_path(path_text)?;
        if !Self::has_docx_extension(&path) {
            return Err(WriterDocxError::InvalidExtension);
        }
        Ok(path)
    }

    pub fn resolve_save_path(path_text: &str) -> Result<PathBuf, WriterDocxError> {
        let mut path = Self::base_path(path_text)?;
        if !Self::has_docx_extension(&path) {
            path.set_extension(DOCX_FILE_EXTENSION);
        }
        Ok(path)
    }

    fn base_path(path_text: &str) -> Result<PathBuf, WriterDocxError> {
        if path_text.trim().is_empty() {
            return Err(WriterDocxError::InvalidPath);
        }
        let path = PathBuf::from(path_text);
        if path.file_name().is_none() {
            return Err(WriterDocxError::InvalidPath);
        }
        Ok(path)
    }

    fn has_docx_extension(path: &Path) -> bool {
        path.extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(DOCX_FILE_EXTENSION))
    }
}
