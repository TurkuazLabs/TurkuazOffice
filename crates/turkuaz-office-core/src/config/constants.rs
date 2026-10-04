// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/config/constants.rs
// # 📌 Amac: Magic string kullanmadan cekirdek varsayilanlarini ve native belge kontratini merkezi olarak tutar
// # 📌 Modul - FileType: Core - Rust
// # Version: 0.2.0
// # Aciklama: Belge, schema ve Web/WASM core ABI sabitlerini merkezi tanimlar
// Bagimli Oldugu Katman: Config

pub const DEFAULT_DOCUMENT_TITLE: &str = "Adsiz Belge";
pub const EMPTY_TEXT: &str = "";
pub const NATIVE_DOCUMENT_EXTENSION: &str = "tko";
pub const CURRENT_DOCUMENT_SCHEMA_VERSION: u32 = 1;
pub const WEB_CORE_ABI_VERSION: u32 = 1;
pub const WEB_CORE_BRIDGE_KIND: &str = "rust-wasm";
