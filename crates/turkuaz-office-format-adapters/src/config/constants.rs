// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/config/constants.rs
// # 📌 Amac: DOCX package entry, namespace, content type ve guvenlik limitlerini merkezi tutar
// # 📌 Modul - FileType: Config - Rust
// # Version: 0.2.0
// # Aciklama: DOCX minimum profile magic string ve limitlerini Tool/Service katmanlarindan ayirir
// Bagimli Oldugu Katman: Config

pub const DOCX_FILE_EXTENSION: &str = "docx";
pub const DOCX_CONTENT_TYPES_ENTRY: &str = "[Content_Types].xml";
pub const DOCX_ROOT_RELATIONSHIPS_ENTRY: &str = "_rels/.rels";
pub const DOCX_DOCUMENT_ENTRY: &str = "word/document.xml";

pub const DOCX_WORD_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
pub const DOCX_PACKAGE_RELATIONSHIPS_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/package/2006/relationships";
pub const DOCX_CONTENT_TYPES_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/package/2006/content-types";
pub const DOCX_OFFICE_DOCUMENT_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
pub const DOCX_DOCUMENT_CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";

pub const MAX_DOCX_PACKAGE_BYTES: u64 = 32 * 1024 * 1024;
pub const MAX_DOCX_ARCHIVE_ENTRIES: usize = 512;
pub const MAX_DOCX_ENTRY_BYTES: u64 = 16 * 1024 * 1024;
pub const MAX_DOCX_DOCUMENT_XML_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_DOCX_XML_DEPTH: usize = 128;
pub const MAX_DOCX_XML_NODES: usize = 100_000;
