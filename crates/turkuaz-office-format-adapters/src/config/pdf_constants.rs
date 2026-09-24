// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/config/pdf_constants.rs
// # 📌 Amac: PDF export fiziksel birim, layout ve guvenlik sabitlerini merkezi tutar
// # 📌 Modul - FileType: Config - Rust
// # Version: 0.2.0
// # Aciklama: Twip-point donusumu, line height, minimum content olcusu ve PDF boyut limitlerini tanimlar
// Bagimli Oldugu Katman: Config

pub const PDF_FILE_EXTENSION: &str = "pdf";
pub const TWIPS_PER_POINT: f32 = 20.0;
pub const PDF_LINE_HEIGHT_MULTIPLIER: f32 = 1.55;
pub const PDF_PARAGRAPH_GAP_POINTS: f32 = 3.0;
pub const PDF_TAB_SPACES: usize = 4;
pub const PDF_UNDERLINE_OFFSET_MULTIPLIER: f32 = 0.12;
pub const PDF_UNDERLINE_THICKNESS_MULTIPLIER: f32 = 0.055;
pub const MAX_PDF_OUTPUT_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_PDF_PAGES: usize = 10_000;
