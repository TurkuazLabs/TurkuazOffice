// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/services/writer_pdf_service.rs
// # 📌 Amac: Canonical Writer belgesini sistem fontlariyla PDF byte'ina cevirip safe-write ile disari aktarir
// # 📌 Modul - FileType: Service - Rust
// # Version: 0.2.0
// # Aciklama: PDF path normalize, font request resolution, adapter export ve atomic dosya yazma is kurallarini koordine eder
// Bagimli Oldugu Katman: Service -> Tool

use std::path::PathBuf;

use turkuaz_office_format_adapters::config::pdf_constants::{
    MAX_PDF_OUTPUT_BYTES, PDF_FILE_EXTENSION,
};
use turkuaz_office_format_adapters::{PdfError, PdfService};
use turkuaz_office_writer::WriterDocument;

use crate::tools::local_file_tool::{LocalFileError, LocalFileTool};
use crate::tools::system_font_tool::{SystemFontError, SystemFontTool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriterPdfError {
    InvalidPath,
    OutputTooLarge,
    WriteFailed,
    Font(SystemFontError),
    Format(PdfError),
}

impl From<PdfError> for WriterPdfError {
    fn from(value: PdfError) -> Self {
        Self::Format(value)
    }
}

impl From<SystemFontError> for WriterPdfError {
    fn from(value: SystemFontError) -> Self {
        Self::Font(value)
    }
}

impl From<LocalFileError> for WriterPdfError {
    fn from(value: LocalFileError) -> Self {
        match value {
            LocalFileError::InvalidPath => Self::InvalidPath,
            LocalFileError::PackageTooLarge => Self::OutputTooLarge,
            LocalFileError::ReadFailed | LocalFileError::WriteFailed => Self::WriteFailed,
        }
    }
}

pub struct WriterPdfService {
    font_tool: SystemFontTool,
}

impl WriterPdfService {
    pub fn new() -> Self {
        Self {
            font_tool: SystemFontTool::new(),
        }
    }

    pub fn export(
        &self,
        document: &WriterDocument,
        path_text: &str,
    ) -> Result<String, WriterPdfError> {
        let path = Self::resolve_save_path(path_text)?;
        let requests = PdfService::font_requests(document)?;
        let fonts = requests
            .iter()
            .map(|request| self.font_tool.resolve(request))
            .collect::<Result<Vec<_>, _>>()?;
        let bytes = PdfService::export(document, &fonts)?;
        if bytes.len() > MAX_PDF_OUTPUT_BYTES {
            return Err(WriterPdfError::OutputTooLarge);
        }
        LocalFileTool::write_safe_replace(&path, &bytes)?;
        Ok(path.to_string_lossy().into_owned())
    }

    pub fn resolve_save_path(path_text: &str) -> Result<PathBuf, WriterPdfError> {
        if path_text.trim().is_empty() {
            return Err(WriterPdfError::InvalidPath);
        }
        let mut path = PathBuf::from(path_text);
        if path.file_name().is_none() {
            return Err(WriterPdfError::InvalidPath);
        }
        let has_pdf_extension = path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(PDF_FILE_EXTENSION));
        if !has_pdf_extension {
            path.set_extension(PDF_FILE_EXTENSION);
        }
        Ok(path)
    }
}

impl Default for WriterPdfService {
    fn default() -> Self {
        Self::new()
    }
}
