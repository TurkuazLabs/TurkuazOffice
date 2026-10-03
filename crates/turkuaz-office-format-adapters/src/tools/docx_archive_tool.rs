// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/tools/docx_archive_tool.rs
// # 📌 Amac: DOCX ZIP paketlerini traversal ve zip-bomb limitleri ile guvenli okur/yazar
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Stored/Deflate entry destegi, path validation, entry count ve acilmis boyut limitlerini uygular
// Bagimli Oldugu Katman: Tool -> Config

use std::collections::HashMap;
use std::io::{Cursor, Read, Write};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::config::constants::{
    MAX_DOCX_ARCHIVE_ENTRIES, MAX_DOCX_ENTRY_BYTES, MAX_DOCX_PACKAGE_BYTES,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocxArchiveError {
    PackageTooLarge,
    TooManyEntries,
    UnsafeEntryName,
    DuplicateEntry,
    EntryTooLarge,
    UnsupportedCompression,
    ReadFailed,
    WriteFailed,
}

pub struct DocxArchiveTool;

impl DocxArchiveTool {
    pub fn encode(entries: &[(&str, &[u8])]) -> Result<Vec<u8>, DocxArchiveError> {
        if entries.len() > MAX_DOCX_ARCHIVE_ENTRIES {
            return Err(DocxArchiveError::TooManyEntries);
        }

        let cursor = Cursor::new(Vec::new());
        let mut writer = ZipWriter::new(cursor);
        let options =
            SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

        for (name, data) in entries {
            Self::validate_entry_name(name)?;
            if data.len() as u64 > MAX_DOCX_ENTRY_BYTES {
                return Err(DocxArchiveError::EntryTooLarge);
            }
            writer
                .start_file(*name, options)
                .map_err(|_| DocxArchiveError::WriteFailed)?;
            writer
                .write_all(data)
                .map_err(|_| DocxArchiveError::WriteFailed)?;
        }

        let cursor = writer
            .finish()
            .map_err(|_| DocxArchiveError::WriteFailed)?;
        let bytes = cursor.into_inner();
        if bytes.len() as u64 > MAX_DOCX_PACKAGE_BYTES {
            return Err(DocxArchiveError::PackageTooLarge);
        }
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<HashMap<String, Vec<u8>>, DocxArchiveError> {
        if bytes.len() as u64 > MAX_DOCX_PACKAGE_BYTES {
            return Err(DocxArchiveError::PackageTooLarge);
        }

        let cursor = Cursor::new(bytes);
        let mut archive =
            ZipArchive::new(cursor).map_err(|_| DocxArchiveError::ReadFailed)?;
        if archive.len() > MAX_DOCX_ARCHIVE_ENTRIES {
            return Err(DocxArchiveError::TooManyEntries);
        }

        let mut total_uncompressed = 0_u64;
        let mut entries = HashMap::new();
        for index in 0..archive.len() {
            let mut file = archive
                .by_index(index)
                .map_err(|_| DocxArchiveError::ReadFailed)?;
            let raw_name = file.name().trim_end_matches('/').to_owned();
            Self::validate_entry_name(&raw_name)?;

            if file.is_dir() {
                continue;
            }
            if !matches!(
                file.compression(),
                CompressionMethod::Stored | CompressionMethod::Deflated
            ) {
                return Err(DocxArchiveError::UnsupportedCompression);
            }
            if file.size() > MAX_DOCX_ENTRY_BYTES {
                return Err(DocxArchiveError::EntryTooLarge);
            }

            total_uncompressed = total_uncompressed.saturating_add(file.size());
            if total_uncompressed > MAX_DOCX_PACKAGE_BYTES {
                return Err(DocxArchiveError::PackageTooLarge);
            }

            let mut data = Vec::with_capacity(file.size().min(usize::MAX as u64) as usize);
            file.read_to_end(&mut data)
                .map_err(|_| DocxArchiveError::ReadFailed)?;
            if data.len() as u64 > MAX_DOCX_ENTRY_BYTES {
                return Err(DocxArchiveError::EntryTooLarge);
            }
            if entries.insert(raw_name, data).is_some() {
                return Err(DocxArchiveError::DuplicateEntry);
            }
        }

        Ok(entries)
    }

    fn validate_entry_name(name: &str) -> Result<(), DocxArchiveError> {
        if name.is_empty()
            || name.starts_with('/')
            || name.starts_with('\\')
            || name.contains('\\')
            || name
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err(DocxArchiveError::UnsafeEntryName);
        }
        Ok(())
    }
}
