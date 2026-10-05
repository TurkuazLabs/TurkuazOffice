// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/config/constants.rs
// # 📌 Amac: Sheet grid, formula, format, table ve query limitlerini merkezi tanimlar
// # 📌 Modul - FileType: Config - Rust
// Version: 0.6.0
// Aciklama: XLSX uyumlu grid sinirlari ile formula, format ve query validation degerlerini magic string/sayidan ayirir
// Bagimli Oldugu Katman: Config

pub const SHEET_DOCUMENT_ID_PREFIX: &str = "sheet-document";
pub const WORKSHEET_ID_PREFIX: &str = "worksheet";
pub const CHART_ID_PREFIX: &str = "chart";
pub const TABLE_ID_PREFIX: &str = "table";

pub const DEFAULT_WORKSHEET_NAME: &str = "Sheet1";
pub const DEFAULT_TABLE_NAME_PREFIX: &str = "Table";

pub const MAX_SHEET_ROWS: u32 = 1_048_576;
pub const MAX_SHEET_COLUMNS: u32 = 16_384;
pub const MAX_CELL_TEXT_LENGTH: usize = 32_767;
pub const MAX_WORKSHEET_NAME_LENGTH: usize = 31;

pub const FORMULA_PREFIX: char = '=';
pub const MAX_FORMULA_LENGTH: usize = 4_096;
pub const MAX_FORMULA_PARSE_DEPTH: usize = 64;
pub const MAX_FORMULA_OPERATIONS: usize = 128;
pub const MAX_FORMULA_EVALUATION_DEPTH: usize = 64;

pub const MAX_CELL_DECIMAL_PLACES: u8 = 12;
pub const MAX_SHEET_QUERY_ROWS: usize = 100_000;

pub const MAX_CHART_TITLE_LENGTH: usize = 128;
pub const MAX_CHART_POINTS: usize = 1_000;
