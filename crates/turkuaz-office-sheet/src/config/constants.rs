// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/config/constants.rs
// # 📌 Amac: Sheet cell grid limitleri, default adlar ve kimlik prefix sabitlerini merkezi tanimlar
// # 📌 Modul - FileType: Config - Rust
// Version: 0.3.0
// Aciklama: XLSX uyumlu grid sinirlari ve sparse cell model validation degerlerini magic string/sayidan ayirir
// Bagimli Oldugu Katman: Config

pub const SHEET_DOCUMENT_ID_PREFIX: &str = "sheet-document";
pub const WORKSHEET_ID_PREFIX: &str = "worksheet";

pub const DEFAULT_WORKSHEET_NAME: &str = "Sheet1";

pub const MAX_SHEET_ROWS: u32 = 1_048_576;
pub const MAX_SHEET_COLUMNS: u32 = 16_384;
pub const MAX_CELL_TEXT_LENGTH: usize = 32_767;
pub const MAX_WORKSHEET_NAME_LENGTH: usize = 31;
