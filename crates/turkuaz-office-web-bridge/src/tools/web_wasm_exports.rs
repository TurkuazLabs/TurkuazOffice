// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-web-bridge/src/tools/web_wasm_exports.rs
// # 📌 Amac: Core capability ve Writer TKO bridge yuzeyini wasm-bindgen JavaScript exportlarina adapte eder
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.4.0
// Aciklama: Mevcut Core ABI fonksiyonlarini korur; Writer TKO inspect ve re-encode fonksiyonlarini ayni browser WASM modulune ekler
// Bagimli Oldugu Katman: Tool -> Controller -> Service -> View -> Config

use wasm_bindgen::prelude::{JsValue, wasm_bindgen};

use crate::config::constants::WEB_TKO_ERROR_JSON_SERIALIZATION;
use crate::WebWriterTkoBridgeController;

#[wasm_bindgen(js_name = web_core_abi_version)]
#[must_use]
pub fn wasm_web_core_abi_version() -> u32 {
    turkuaz_office_core::web_core_abi_version()
}

#[wasm_bindgen(js_name = web_core_document_schema_version)]
#[must_use]
pub fn wasm_web_core_document_schema_version() -> u32 {
    turkuaz_office_core::web_core_document_schema_version()
}

#[wasm_bindgen(js_name = web_core_bridge_kind)]
#[must_use]
pub fn wasm_web_core_bridge_kind() -> String {
    turkuaz_office_core::web_core_bridge_kind().to_owned()
}

#[wasm_bindgen(js_name = web_core_native_file_system_access)]
#[must_use]
pub fn wasm_web_core_native_file_system_access() -> bool {
    turkuaz_office_core::web_core_native_file_system_access()
}

#[wasm_bindgen(js_name = web_writer_tko_inspect)]
pub fn wasm_web_writer_tko_inspect(bytes: &[u8]) -> Result<String, JsValue> {
    let view = WebWriterTkoBridgeController::inspect(bytes)
        .map_err(|error| JsValue::from_str(error.code()))?;
    serde_json::to_string(&view)
        .map_err(|_| JsValue::from_str(WEB_TKO_ERROR_JSON_SERIALIZATION))
}

#[wasm_bindgen(js_name = web_writer_tko_reencode)]
pub fn wasm_web_writer_tko_reencode(
    bytes: &[u8],
    app_version: &str,
) -> Result<Vec<u8>, JsValue> {
    WebWriterTkoBridgeController::reencode(bytes, app_version)
        .map_err(|error| JsValue::from_str(error.code()))
}
