// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/lib.rs
// # 📌 Amac: Core katmanlarini dis dunyaya kontrollu olarak acar
// # 📌 Modul - FileType: Core - Rust
// # Version: 0.2.0
// # Aciklama: Core katmanlarini ve Web/WASM capability yuzeyini dis dunyaya kontrollu olarak acar
// Bagimli Oldugu Katman: Core

pub mod config;
pub mod controllers;
pub mod language;
pub mod repositories;
pub mod services;
pub mod tools;
pub mod views;

pub use controllers::document_controller::DocumentController;
pub use repositories::document_repository::{DocumentRepository, InMemoryDocumentRepository};
pub use services::document_service::DocumentService;
pub use services::document_types::{Document, DocumentId, DocumentSchemaVersion};
pub use services::schema_migration_service::{SchemaMigrationError, SchemaMigrationService};
pub use services::web_core_service::{
    WebCoreService, web_core_abi_version, web_core_bridge_kind,
    web_core_document_schema_version, web_core_native_file_system_access,
};
pub use views::document_view::DocumentView;
pub use views::web_core_view::WebCoreCapabilitiesView;
