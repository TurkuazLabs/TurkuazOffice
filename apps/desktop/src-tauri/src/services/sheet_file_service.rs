// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/services/sheet_file_service.rs
// # 📌 Amac: Sheet CSV/XLSX yerel import/export akislarini format adapter ve LocalFileTool uzerinden koordine eder
// # 📌 Modul - FileType: Service - Rust
// Version: 0.4.0
// Aciklama: Uzanti, dosya boyutu, canonical import/export ve failure-safe write kurallarini Desktop katmaninda merkezilestirir
// Bagimli Oldugu Katman: Service -> Tool -> FormatAdapter -> Sheet

use std::path::{Path, PathBuf};

use turkuaz_office_format_adapters::config::sheet_constants::{
    CSV_FILE_EXTENSION, MAX_CSV_BYTES, MAX_XLSX_PACKAGE_BYTES, XLSX_FILE_EXTENSION,
};
use turkuaz_office_format_adapters::{
    SheetCsvError, SheetCsvService, SheetXlsxError, SheetXlsxService,
};
use turkuaz_office_sheet::{SheetDocument, SheetIdTool};

use crate::tools::local_file_tool::{LocalFileError, LocalFileTool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SheetFileError {
    InvalidPath,
    InvalidExtension,
    ReadFailed,
    WriteFailed,
    ResourceLimit,
    WorksheetNotFound,
    Csv(SheetCsvError),
    Xlsx(SheetXlsxError),
}

pub struct SheetFileService;

impl SheetFileService {
    pub fn import_csv<I>(
        id_tool: &I,
        path: &str,
    ) -> Result<SheetDocument, SheetFileError>
    where
        I: SheetIdTool,
    {
        let path = Self::validated_path(path, CSV_FILE_EXTENSION)?;
        let max_bytes =
            u64::try_from(MAX_CSV_BYTES).map_err(|_| SheetFileError::ResourceLimit)?;
        let bytes = LocalFileTool::read(&path, max_bytes).map_err(Self::map_local_error)?;
        SheetCsvService::import(id_tool, Self::title(&path)?, &bytes).map_err(SheetFileError::Csv)
    }

    pub fn import_xlsx<I>(
        id_tool: &I,
        path: &str,
    ) -> Result<SheetDocument, SheetFileError>
    where
        I: SheetIdTool,
    {
        let path = Self::validated_path(path, XLSX_FILE_EXTENSION)?;
        let bytes =
            LocalFileTool::read(&path, MAX_XLSX_PACKAGE_BYTES).map_err(Self::map_local_error)?;
        SheetXlsxService::import(id_tool, Self::title(&path)?, &bytes)
            .map_err(SheetFileError::Xlsx)
    }

    pub fn export_csv(
        document: &SheetDocument,
        worksheet_id: &str,
        path: &str,
    ) -> Result<String, SheetFileError> {
        let path = Self::validated_path(path, CSV_FILE_EXTENSION)?;
        let worksheet = document
            .worksheets
            .iter()
            .find(|worksheet| worksheet.id.as_str() == worksheet_id)
            .ok_or(SheetFileError::WorksheetNotFound)?;
        let bytes = SheetCsvService::export_worksheet(worksheet).map_err(SheetFileError::Csv)?;
        LocalFileTool::write_safe_replace(&path, &bytes).map_err(Self::map_local_error)?;
        Ok(path.to_string_lossy().into_owned())
    }

    pub fn export_xlsx(
        document: &SheetDocument,
        path: &str,
    ) -> Result<String, SheetFileError> {
        let path = Self::validated_path(path, XLSX_FILE_EXTENSION)?;
        let bytes = SheetXlsxService::export(document).map_err(SheetFileError::Xlsx)?;
        LocalFileTool::write_safe_replace(&path, &bytes).map_err(Self::map_local_error)?;
        Ok(path.to_string_lossy().into_owned())
    }

    fn validated_path(path: &str, extension: &str) -> Result<PathBuf, SheetFileError> {
        let path = PathBuf::from(path.trim());
        if path.as_os_str().is_empty() {
            return Err(SheetFileError::InvalidPath);
        }
        let matches_extension = path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(extension));
        if !matches_extension {
            return Err(SheetFileError::InvalidExtension);
        }
        Ok(path)
    }

    fn title(path: &Path) -> Result<String, SheetFileError> {
        path.file_stem()
            .and_then(|value| value.to_str())
            .filter(|value| !value.trim().is_empty())
            .map(str::to_owned)
            .ok_or(SheetFileError::InvalidPath)
    }

    fn map_local_error(error: LocalFileError) -> SheetFileError {
        match error {
            LocalFileError::InvalidPath => SheetFileError::InvalidPath,
            LocalFileError::PackageTooLarge => SheetFileError::ResourceLimit,
            LocalFileError::ReadFailed => SheetFileError::ReadFailed,
            LocalFileError::WriteFailed => SheetFileError::WriteFailed,
        }
    }
}
