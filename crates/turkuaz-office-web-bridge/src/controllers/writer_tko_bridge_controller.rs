// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-web-bridge/src/controllers/writer_tko_bridge_controller.rs
// # 📌 Amac: Web Writer TKO requestlerini Service katmanina ince Controller olarak iletir
// # 📌 Modul - FileType: Controller - Rust
// Version: 0.4.0
// Aciklama: Inspect ve re-encode requestlerinde is kurali tutmadan WebWriterTkoBridgeService metodlarini cagirir
// Bagimli Oldugu Katman: Controller -> Service

use crate::services::writer_tko_bridge_service::{
    WebWriterTkoBridgeError, WebWriterTkoBridgeService,
};
use crate::views::writer_tko_bridge_view::WebWriterTkoSummaryView;

pub struct WebWriterTkoBridgeController;

impl WebWriterTkoBridgeController {
    pub fn inspect(bytes: &[u8]) -> Result<WebWriterTkoSummaryView, WebWriterTkoBridgeError> {
        WebWriterTkoBridgeService::inspect(bytes)
    }

    pub fn reencode(
        bytes: &[u8],
        app_version: &str,
    ) -> Result<Vec<u8>, WebWriterTkoBridgeError> {
        WebWriterTkoBridgeService::reencode(bytes, app_version)
    }
}
