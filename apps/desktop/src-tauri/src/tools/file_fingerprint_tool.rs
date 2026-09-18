// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/tools/file_fingerprint_tool.rs
// # 📌 Amac: Yerel dosya iceriginden oturum ici degisiklik fingerprinti uretir
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Timestamp yerine dosya boyutu ve FNV-1a 64-bit icerik hashini kullanarak external-change karsilastirmasi yapar
// Bagimli Oldugu Katman: Tool -> Tool

use std::path::Path;

use crate::tools::local_file_tool::{LocalFileError, LocalFileTool};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileFingerprint {
    pub byte_length: u64,
    pub content_hash: u64,
}

pub struct FileFingerprintTool;

impl FileFingerprintTool {
    const FNV_OFFSET_BASIS: u64 = 14_695_981_039_346_656_037;
    const FNV_PRIME: u64 = 1_099_511_628_211;

    pub fn read(path: &Path, max_bytes: u64) -> Result<FileFingerprint, LocalFileError> {
        let bytes = LocalFileTool::read(path, max_bytes)?;
        Ok(Self::from_bytes(&bytes))
    }

    pub fn from_bytes(bytes: &[u8]) -> FileFingerprint {
        let mut hash = Self::FNV_OFFSET_BASIS;
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(Self::FNV_PRIME);
        }
        FileFingerprint {
            byte_length: bytes.len() as u64,
            content_hash: hash,
        }
    }
}
