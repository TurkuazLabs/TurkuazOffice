// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/tools/tko_archive_tool.rs
// # 📌 Amac: TKO ZIP paketini guvenli limitlerle olusturur ve okur
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: ZIP path traversal, entry sayisi ve acilmis boyut limitlerini Tool katmaninda uygular
// Bagimli Oldugu Katman: Tool -> Config

use std::collections::HashMap;
use std::io::{Cursor, Read, Write};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::config::constants::{MAX_TKO_ARCHIVE_ENTRIES, MAX_TKO_PACKAGE_BYTES};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TkoArchiveError {
    PackageTooLarge,
    TooManyEntries,
    UnsafeEntryName,
    DuplicateEntry,
    EntryTooLarge,
    DirectoryEntryUnsupported,
    UnsupportedCompression,
    ReadFailed,
    WriteFailed,
}

pub struct TkoArchiveTool;

impl TkoArchiveTool {
    pub fn encode(entries: &[(&str, &[u8])]) -> Result<Vec<u8>, TkoArchiveError> {
        if entries.len() > MAX_TKO_ARCHIVE_ENTRIES {
            return Err(TkoArchiveError::TooManyEntries);
        }

        let cursor = Cursor::new(Vec::new());
        let mut writer = ZipWriter::new(cursor);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

        for (name, data) in entries {
            Self::validate_entry_name(name)?;
            writer
                .start_file(*name, options)
                .map_err(|_| TkoArchiveError::WriteFailed)?;
            writer
                .write_all(data)
                .map_err(|_| TkoArchiveError::WriteFailed)?;
        }

        let cursor = writer.finish().map_err(|_| TkoArchiveError::WriteFailed)?;
        let bytes = cursor.into_inner();
        if bytes.len() as u64 > MAX_TKO_PACKAGE_BYTES {
            return Err(TkoArchiveError::PackageTooLarge);
        }
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<HashMap<String, Vec<u8>>, TkoArchiveError> {
        if bytes.len() as u64 > MAX_TKO_PACKAGE_BYTES {
            return Err(TkoArchiveError::PackageTooLarge);
        }

        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).map_err(|_| TkoArchiveError::ReadFailed)?;
        if archive.len() > MAX_TKO_ARCHIVE_ENTRIES {
            return Err(TkoArchiveError::TooManyEntries);
        }

        let mut total_uncompressed = 0_u64;
        let mut entries = HashMap::new();
        for index in 0..archive.len() {
            let mut file = archive
                .by_index(index)
                .map_err(|_| TkoArchiveError::ReadFailed)?;
            let name = file.name().trim_end_matches('/').to_owned();
            Self::validate_entry_name(&name)?;
            if file.is_dir() {
                return Err(TkoArchiveError::DirectoryEntryUnsupported);
            }
            if file.compression() != CompressionMethod::Stored {
                return Err(TkoArchiveError::UnsupportedCompression);
            }
            total_uncompressed = total_uncompressed.saturating_add(file.size());
            if total_uncompressed > MAX_TKO_PACKAGE_BYTES {
                return Err(TkoArchiveError::EntryTooLarge);
            }

            let mut data = Vec::with_capacity(file.size().min(usize::MAX as u64) as usize);
            file.read_to_end(&mut data)
                .map_err(|_| TkoArchiveError::ReadFailed)?;
            if data.len() as u64 > MAX_TKO_PACKAGE_BYTES {
                return Err(TkoArchiveError::EntryTooLarge);
            }
            let declared_size = file.size();
            total_uncompressed = total_uncompressed
                .saturating_sub(declared_size)
                .saturating_add(data.len() as u64);
            if total_uncompressed > MAX_TKO_PACKAGE_BYTES {
                return Err(TkoArchiveError::EntryTooLarge);
            }
            if entries.insert(name, data).is_some() {
                return Err(TkoArchiveError::DuplicateEntry);
            }
        }
        Ok(entries)
    }

    fn validate_entry_name(name: &str) -> Result<(), TkoArchiveError> {
        if name.is_empty()
            || name.starts_with('/')
            || name.starts_with('\\')
            || name.contains('\\')
            || name
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err(TkoArchiveError::UnsafeEntryName);
        }
        Ok(())
    }
}
