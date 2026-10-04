// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/tools/web_wasm_exports.rs
// # 📌 Amac: Saf Web Core capability Service yuzeyini wasm-bindgen JavaScript exportlarina adapte eder
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.4.0
// Aciklama: Browser generated binding icin ABI/schema/bridge/native-filesystem fonksiyonlarini Tool katmanindan export eder
// Bagimli Oldugu Katman: Tool -> Service

use wasm_bindgen::prelude::wasm_bindgen;

use crate::services::web_core_service::{
    web_core_abi_version as service_abi_version, web_core_bridge_kind as service_bridge_kind,
    web_core_document_schema_version as service_document_schema_version,
    web_core_native_file_system_access as service_native_file_system_access,
};

#[wasm_bindgen(js_name = web_core_abi_version)]
#[must_use]
pub fn wasm_web_core_abi_version() -> u32 {
    service_abi_version()
}

#[wasm_bindgen(js_name = web_core_document_schema_version)]
#[must_use]
pub fn wasm_web_core_document_schema_version() -> u32 {
    service_document_schema_version()
}

#[wasm_bindgen(js_name = web_core_bridge_kind)]
#[must_use]
pub fn wasm_web_core_bridge_kind() -> String {
    service_bridge_kind().to_owned()
}

#[wasm_bindgen(js_name = web_core_native_file_system_access)]
#[must_use]
pub fn wasm_web_core_native_file_system_access() -> bool {
    service_native_file_system_access()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_exports_delegate_to_service_contract() {
        assert_eq!(wasm_web_core_abi_version(), service_abi_version());
        assert_eq!(
            wasm_web_core_document_schema_version(),
            service_document_schema_version()
        );
        assert_eq!(wasm_web_core_bridge_kind(), service_bridge_kind());
        assert_eq!(
            wasm_web_core_native_file_system_access(),
            service_native_file_system_access()
        );
    }
}
