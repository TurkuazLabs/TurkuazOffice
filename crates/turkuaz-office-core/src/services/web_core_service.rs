// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/services/web_core_service.rs
// # 📌 Amac: M3 Web icin WASM-compatible saf Core capability kontratini uretir
// # 📌 Modul - FileType: Core - Rust
// Version: 0.4.0
// Aciklama: OS/native API kullanmadan browser targetinda derlenebilen Core ABI ve schema capability bilgisini sunar
// Bagimli Oldugu Katman: Service -> View

use crate::config::constants::{
    CURRENT_DOCUMENT_SCHEMA_VERSION, WEB_CORE_ABI_VERSION, WEB_CORE_BRIDGE_KIND,
};
use crate::views::web_core_view::WebCoreCapabilitiesView;

#[must_use]
pub const fn web_core_abi_version() -> u32 {
    WEB_CORE_ABI_VERSION
}

#[must_use]
pub const fn web_core_document_schema_version() -> u32 {
    CURRENT_DOCUMENT_SCHEMA_VERSION
}

#[must_use]
pub const fn web_core_bridge_kind() -> &'static str {
    WEB_CORE_BRIDGE_KIND
}

#[must_use]
pub const fn web_core_native_file_system_access() -> bool {
    false
}

pub struct WebCoreService;

impl WebCoreService {
    #[must_use]
    pub const fn capabilities() -> WebCoreCapabilitiesView {
        WebCoreCapabilitiesView {
            abi_version: web_core_abi_version(),
            document_schema_version: web_core_document_schema_version(),
            bridge_kind: web_core_bridge_kind(),
            native_file_system_access: web_core_native_file_system_access(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capabilities_match_core_schema_and_browser_boundary() {
        let capabilities = WebCoreService::capabilities();

        assert_eq!(capabilities.abi_version, WEB_CORE_ABI_VERSION);
        assert_eq!(
            capabilities.document_schema_version,
            CURRENT_DOCUMENT_SCHEMA_VERSION
        );
        assert_eq!(capabilities.bridge_kind, WEB_CORE_BRIDGE_KIND);
        assert!(!capabilities.native_file_system_access);
        assert_eq!(web_core_abi_version(), capabilities.abi_version);
        assert_eq!(
            web_core_document_schema_version(),
            capabilities.document_schema_version
        );
        assert_eq!(web_core_bridge_kind(), capabilities.bridge_kind);
        assert_eq!(
            web_core_native_file_system_access(),
            capabilities.native_file_system_access
        );
    }
}
