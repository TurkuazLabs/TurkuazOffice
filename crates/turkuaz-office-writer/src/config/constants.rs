// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/config/constants.rs
// # 📌 Amac: Writer domain varsayilanlarini ve magic string olmayan sabitleri tanimlar
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Sayfa, font, history, TKO paket limitleri ve kimlik prefix sabitlerini merkezi tutar
// Bagimli Oldugu Katman: Config

pub const DEFAULT_FONT_FAMILY: &str = "Arial";
pub const DEFAULT_FONT_SIZE_HALF_POINTS: u16 = 22;
pub const MIN_FONT_SIZE_HALF_POINTS: u16 = 12;
pub const MAX_FONT_SIZE_HALF_POINTS: u16 = 192;
pub const MAX_FONT_FAMILY_LENGTH: usize = 128;
pub const DEFAULT_HISTORY_LIMIT: usize = 100;
pub const DOCUMENT_ID_PREFIX: &str = "writer-document";
pub const NODE_ID_PREFIX: &str = "node";
pub const ASSET_ID_PREFIX: &str = "asset";
pub const DEFAULT_PAGE_WIDTH_TWIPS: u32 = 11_906;
pub const DEFAULT_PAGE_HEIGHT_TWIPS: u32 = 16_838;
pub const DEFAULT_PAGE_MARGIN_TWIPS: u32 = 1_440;
pub const MIN_TABLE_DIMENSION: usize = 1;
pub const MAX_TABLE_DIMENSION: usize = 100;
pub const WRITER_DOCUMENT_KIND: &str = "writer";
pub const TKO_FILE_EXTENSION: &str = "tko";
pub const TKO_MANIFEST_ENTRY: &str = "manifest.yml";
pub const TKO_WRITER_CONTENT_ENTRY: &str = "content/writer.yml";
pub const MAX_TKO_ARCHIVE_ENTRIES: usize = 20;
pub const MAX_TKO_MANIFEST_BYTES: u64 = 64 * 1024;
pub const MAX_TKO_CONTENT_BYTES: u64 = 8 * 1024 * 1024;
pub const MAX_TKO_ASSET_INDEX_BYTES: u64 = 64 * 1024;
pub const MAX_TKO_PACKAGE_BYTES: u64 = 16 * 1024 * 1024;
pub const MAX_WRITER_ASSETS: usize = 16;
pub const MAX_WRITER_ASSET_BYTES: u64 = 8 * 1024 * 1024;
pub const MAX_WRITER_ASSET_ID_LENGTH: usize = 128;
pub const TKO_ASSET_INDEX_ENTRY: &str = "assets/index.yml";
pub const TKO_ASSET_DATA_PREFIX: &str = "assets/data/";
pub const IMAGE_MEDIA_TYPE_PNG: &str = "image/png";
pub const IMAGE_MEDIA_TYPE_JPEG: &str = "image/jpeg";
pub const IMAGE_MEDIA_TYPE_WEBP: &str = "image/webp";

pub const TEMPLATE_CATALOG_VERSION: u32 = 1;
pub const MAX_TEMPLATE_ID_LENGTH: usize = 64;
pub const MAX_TEMPLATE_LANGUAGE_KEY_LENGTH: usize = 128;
pub const MAX_TEMPLATE_PARAGRAPHS: usize = 32;
