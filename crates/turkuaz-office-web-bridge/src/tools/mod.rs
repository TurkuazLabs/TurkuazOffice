// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-web-bridge/src/tools/mod.rs
// # 📌 Amac: Web bridge dis dunya adaptorlerini disari acar
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.4.0
// Aciklama: wasm-bindgen export Tool'unu yalniz wasm32 hedefinde etkinlestirir
// Bagimli Oldugu Katman: Tool

#[cfg(target_arch = "wasm32")]
pub mod web_wasm_exports;
