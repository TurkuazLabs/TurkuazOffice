// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/views/web_core_view.rs
// # 📌 Amac: Browser/WASM istemcisine aktarilacak platformdan bagimsiz Core capability read-modelini tanimlar
// # 📌 Modul - FileType: Core - Rust
// Version: 0.4.0
// Aciklama: ABI, document schema ve native-filesystem sinirini sade typed View olarak sunar
// Bagimli Oldugu Katman: View

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WebCoreCapabilitiesView {
    pub abi_version: u32,
    pub document_schema_version: u32,
    pub bridge_kind: &'static str,
    pub native_file_system_access: bool,
}
