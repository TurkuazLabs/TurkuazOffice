// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/config/sheet_constants.rs
// # 📌 Amac: Sheet CSV/XLSX extension, package entry, namespace ve guvenlik limitlerini merkezi tutar
// # 📌 Modul - FileType: Config - Rust
// Version: 0.3.0
// Aciklama: CSV parser ve SpreadsheetML ZIP/XML adaptorleri icin magic string, content type ve resource limitlerini tanimlar
// Bagimli Oldugu Katman: Config

pub const CSV_FILE_EXTENSION: &str = "csv";
pub const XLSX_FILE_EXTENSION: &str = "xlsx";

pub const MAX_CSV_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_CSV_ROWS: usize = 1_048_576;
pub const MAX_CSV_COLUMNS: usize = 16_384;

pub const XLSX_CONTENT_TYPES_ENTRY: &str = "[Content_Types].xml";
pub const XLSX_ROOT_RELATIONSHIPS_ENTRY: &str = "_rels/.rels";
pub const XLSX_WORKBOOK_ENTRY: &str = "xl/workbook.xml";
pub const XLSX_WORKBOOK_RELATIONSHIPS_ENTRY: &str = "xl/_rels/workbook.xml.rels";
pub const XLSX_SHARED_STRINGS_ENTRY: &str = "xl/sharedStrings.xml";
pub const XLSX_WORKSHEET_PREFIX: &str = "xl/worksheets/sheet";
pub const XLSX_WORKSHEET_SUFFIX: &str = ".xml";

pub const XLSX_SPREADSHEET_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
pub const XLSX_OFFICE_RELATIONSHIPS_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
pub const XLSX_PACKAGE_RELATIONSHIPS_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/package/2006/relationships";
pub const XLSX_CONTENT_TYPES_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/package/2006/content-types";
pub const XLSX_OFFICE_DOCUMENT_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
pub const XLSX_WORKSHEET_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";

pub const XLSX_WORKBOOK_CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";
pub const XLSX_WORKSHEET_CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";

pub const MAX_XLSX_PACKAGE_BYTES: u64 = 64 * 1024 * 1024;
pub const MAX_XLSX_ARCHIVE_ENTRIES: usize = 1024;
pub const MAX_XLSX_WORKSHEETS: usize = 256;
pub const MAX_XLSX_ENTRY_BYTES: u64 = 32 * 1024 * 1024;
pub const MAX_XLSX_XML_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_XLSX_XML_DEPTH: usize = 128;
pub const MAX_XLSX_XML_NODES: usize = 2_000_000;
