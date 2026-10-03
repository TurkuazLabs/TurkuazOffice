// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/tools/sheet_xlsx_archive_tool.rs
// # 📌 Amac: XLSX ZIP paketlerini traversal, duplicate ve zip-bomb limitleriyle guvenli okur/yazar
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.3.0
// Aciklama: Stored/Deflate entry destegi, package/entry boyut limiti ve safe entry-name validation uygular
// Bagimli Oldugu Katman: Tool -> Config

use std::collections::HashMap;
use std::io::{Cursor, Read, Write};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::config::sheet_constants::{
    MAX_XLSX_ARCHIVE_ENTRIES, MAX_XLSX_ENTRY_BYTES, MAX_XLSX_PACKAGE_BYTES,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SheetXlsxArchiveError {
    PackageTooLarge,
    TooManyEntries,
    UnsafeEntryName,
    DuplicateEntry,
    EntryTooLarge,
    UnsupportedCompression,
    ReadFailed,
    WriteFailed,
}

pub struct SheetXlsxArchiveTool;

impl SheetXlsxArchiveTool {
    pub fn encode(entries: &[(String, Vec<u8>)]) -> Result<Vec<u8>, SheetXlsxArchiveError> {
        if entries.len() > MAX_XLSX_ARCHIVE_ENTRIES {
            return Err(SheetXlsxArchiveError::TooManyEntries);
        }

        let cursor = Cursor::new(Vec::new());
        let mut writer = ZipWriter::new(cursor);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

        for (name, data) in entries {
            Self::validate_entry_name(name)?;
            if data.len() as u64 > MAX_XLSX_ENTRY_BYTES {
                return Err(SheetXlsxArchiveError::EntryTooLarge);
            }
            writer
                .start_file(name, options)
                .map_err(|_| SheetXlsxArchiveError::WriteFailed)?;
            writer
                .write_all(data)
                .map_err(|_| SheetXlsxArchiveError::WriteFailed)?;
        }

        let cursor = writer
            .finish()
            .map_err(|_| SheetXlsxArchiveError::WriteFailed)?;
        let bytes = cursor.into_inner();
        if bytes.len() as u64 > MAX_XLSX_PACKAGE_BYTES {
            return Err(SheetXlsxArchiveError::PackageTooLarge);
        }
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<HashMap<String, Vec<u8>>, SheetXlsxArchiveError> {
        if bytes.len() as u64 > MAX_XLSX_PACKAGE_BYTES {
            return Err(SheetXlsxArchiveError::PackageTooLarge);
        }

        let mut archive =
            ZipArchive::new(Cursor::new(bytes)).map_err(|_| SheetXlsxArchiveError::ReadFailed)?;
        if archive.len() > MAX_XLSX_ARCHIVE_ENTRIES {
            return Err(SheetXlsxArchiveError::TooManyEntries);
        }

        let mut total_uncompressed = 0_u64;
        let mut entries = HashMap::new();
        for index in 0..archive.len() {
            let mut file = archive
                .by_index(index)
                .map_err(|_| SheetXlsxArchiveError::ReadFailed)?;
            let raw_name = file.name().trim_end_matches('/').to_owned();
            Self::validate_entry_name(&raw_name)?;

            if file.is_dir() {
                continue;
            }
            if !matches!(
                file.compression(),
                CompressionMethod::Stored | CompressionMethod::Deflated
            ) {
                return Err(SheetXlsxArchiveError::UnsupportedCompression);
            }
            if file.size() > MAX_XLSX_ENTRY_BYTES {
                return Err(SheetXlsxArchiveError::EntryTooLarge);
            }

            total_uncompressed = total_uncompressed.saturating_add(file.size());
            if total_uncompressed > MAX_XLSX_PACKAGE_BYTES {
                return Err(SheetXlsxArchiveError::PackageTooLarge);
            }

            let mut data = Vec::with_capacity(file.size().min(usize::MAX as u64) as usize);
            file.read_to_end(&mut data)
                .map_err(|_| SheetXlsxArchiveError::ReadFailed)?;
            if data.len() as u64 > MAX_XLSX_ENTRY_BYTES {
                return Err(SheetXlsxArchiveError::EntryTooLarge);
            }
            if entries.insert(raw_name, data).is_some() {
                return Err(SheetXlsxArchiveError::DuplicateEntry);
            }
        }

        Ok(entries)
    }

    fn validate_entry_name(name: &str) -> Result<(), SheetXlsxArchiveError> {
        if name.is_empty()
            || name.starts_with('/')
            || name.starts_with('\\')
            || name.contains('\\')
            || name
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err(SheetXlsxArchiveError::UnsafeEntryName);
        }
        Ok(())
    }
}
