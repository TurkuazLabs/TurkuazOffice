// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/tools/mod.rs
// # 📌 Amac: Dis dunya adaptorleri icin Tool katmani giris noktasidir
// # 📌 Modul - FileType: Core - Rust
// # Version: 0.4.0
// # Aciklama: Native Tool modulleri ile wasm32 generated-binding export adaptorunu hedef-bazli acar
// Bagimli Oldugu Katman: Tool

pub mod id_tool;

#[cfg(target_arch = "wasm32")]
pub mod web_wasm_exports;
