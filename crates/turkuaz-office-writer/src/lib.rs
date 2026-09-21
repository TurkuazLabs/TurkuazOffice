// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/lib.rs
// # 📌 Amac: Writer modulunun kontrollu public API yuzeyini tanimlar
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Controller, Service, Repo, Tool, View, Language ve Config katmanlarini disari acar
// Bagimli Oldugu Katman: Writer

pub mod config;
pub mod controllers;
pub mod language;
pub mod repositories;
pub mod services;
pub mod tools;
pub mod views;

pub use controllers::writer_controller::WriterController;
pub use repositories::writer_document_repository::{
    InMemoryWriterDocumentRepository, WriterDocumentRepository,
};
pub use services::tko_package_service::{TkoPackageError, TkoPackageService};
pub use services::tko_profile_service::{TkoProfileError, TkoProfileService};
pub use services::writer_asset_service::{WriterAssetError, WriterAssetService};
pub use services::writer_command::{WriterCommand, WriterCommandError};
pub use services::writer_editor_service::{WriterEditorError, WriterEditorService};
pub use services::writer_selection_service::{SelectionError, WriterSelectionService};
pub use services::writer_types::{
    Block, CharacterStyle, CharacterStylePatch, ImageBlock, NodeId, PageSettings, Paragraph,
    ParagraphStyle, ParagraphStylePatch, Section, Selection, StyledTextRun, Table, TableCell,
    TableRow, TextAlignment, TextPosition, TextRange, TextRun, WriterAsset, WriterDocument,
};
pub use tools::tko_archive_tool::TkoArchiveError;
pub use tools::writer_id_tool::{SequentialWriterIdTool, WriterIdTool};
pub use views::writer_view::{
    WriterDocumentView, WriterImageView, WriterPageSettingsView, WriterParagraphView,
    WriterRunView,
};
