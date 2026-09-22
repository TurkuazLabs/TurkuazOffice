// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/lib.rs
// # 📌 Amac: External format adapter crate public API yuzeyini tanimlar
// # 📌 Modul - FileType: FormatAdapters - Rust
// # Version: 0.2.0
// # Aciklama: DOCX model, Service ve Tool katmanlarini kontrollu olarak disari acar
// Bagimli Oldugu Katman: Service -> Model -> Tool -> Config

pub mod config;
pub mod models;
pub mod services;
pub mod tools;

pub use models::docx_model::{
    DocxAlignment, DocxCompatibilityReport, DocxDocumentModel, DocxImportResult,
    DocxPageSettingsModel, DocxParagraphModel, DocxRunModel, DocxUnsupportedFeature,
};
pub use services::docx_service::{DocxError, DocxService};
pub use tools::docx_archive_tool::{DocxArchiveError, DocxArchiveTool};
pub use tools::docx_xml_tool::{DocxXmlError, DocxXmlTool};
