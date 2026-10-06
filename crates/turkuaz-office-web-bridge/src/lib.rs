// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-web-bridge/src/lib.rs
// # 📌 Amac: Web WASM bridge crate public API ve katmanlarini kontrollu acar
// # 📌 Modul - FileType: WebBridge - Rust
// Version: 0.4.0
// Aciklama: Core capability ve Writer TKO bridge Controller/Service/View/Tool katmanlarini tek crate altinda toplar
// Bagimli Oldugu Katman: Controller | Service | Tool | View | Config

pub mod config;
pub mod controllers;
pub mod services;
pub mod tools;
pub mod views;

pub use controllers::writer_tko_bridge_controller::WebWriterTkoBridgeController;
pub use services::writer_tko_bridge_service::{
    WebWriterTkoBridgeError, WebWriterTkoBridgeService,
};
pub use views::writer_tko_bridge_view::WebWriterTkoSummaryView;
