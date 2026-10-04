// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-web-wasm/src/lib.rs
// # 📌 Amac: Pure Rust Core capability API'sini browser icin wasm-bindgen exportlarina cevirir
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.4.0
// Aciklama: Business logic eklemeden Core ABI/schema/bridge capability fonksiyonlarini generated JS binding'e acar
// Bagimli Oldugu Katman: Tool -> Core

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[must_use]
pub fn web_core_abi_version() -> u32 {
    turkuaz_office_core::web_core_abi_version()
}

#[wasm_bindgen]
#[must_use]
pub fn web_core_document_schema_version() -> u32 {
    turkuaz_office_core::web_core_document_schema_version()
}

#[wasm_bindgen]
#[must_use]
pub fn web_core_bridge_kind() -> String {
    turkuaz_office_core::web_core_bridge_kind().to_owned()
}

#[wasm_bindgen]
#[must_use]
pub fn web_core_native_file_system_access() -> bool {
    turkuaz_office_core::web_core_native_file_system_access()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binding_exports_delegate_to_pure_core() {
        assert_eq!(
            web_core_abi_version(),
            turkuaz_office_core::web_core_abi_version()
        );
        assert_eq!(
            web_core_document_schema_version(),
            turkuaz_office_core::web_core_document_schema_version()
        );
        assert_eq!(
            web_core_bridge_kind(),
            turkuaz_office_core::web_core_bridge_kind()
        );
        assert_eq!(
            web_core_native_file_system_access(),
            turkuaz_office_core::web_core_native_file_system_access()
        );
    }
}
