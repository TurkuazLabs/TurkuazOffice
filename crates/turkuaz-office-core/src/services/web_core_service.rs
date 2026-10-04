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

pub struct WebCoreService;

impl WebCoreService {
    #[must_use]
    pub const fn capabilities() -> WebCoreCapabilitiesView {
        WebCoreCapabilitiesView {
            abi_version: WEB_CORE_ABI_VERSION,
            document_schema_version: CURRENT_DOCUMENT_SCHEMA_VERSION,
            bridge_kind: WEB_CORE_BRIDGE_KIND,
            native_file_system_access: false,
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
    }
}
