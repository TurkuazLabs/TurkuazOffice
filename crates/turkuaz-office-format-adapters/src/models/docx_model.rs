// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/models/docx_model.rs
// # 📌 Amac: DOCX parser ile canonical Writer mapping arasindaki format-specific ara modeli tanimlar
// # 📌 Modul - FileType: Model - Rust
// # Version: 0.2.0
// # Aciklama: Paragraph, run, page settings ve compatibility report semantigini DOCX'e ozel tutar
// Bagimli Oldugu Katman: Model

use turkuaz_office_writer::WriterDocument;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocxAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocxRunModel {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub font_family: Option<String>,
    pub font_size_half_points: Option<u16>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocxParagraphModel {
    pub alignment: Option<DocxAlignment>,
    pub runs: Vec<DocxRunModel>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocxPageSettingsModel {
    pub width_twips: Option<u32>,
    pub height_twips: Option<u32>,
    pub margin_top_twips: Option<u32>,
    pub margin_right_twips: Option<u32>,
    pub margin_bottom_twips: Option<u32>,
    pub margin_left_twips: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DocxUnsupportedFeature {
    Table,
    Image,
    Numbering,
    HeaderFooter,
    Comments,
    TrackedChanges,
    Fields,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocxCompatibilityReport {
    pub unsupported_features: Vec<DocxUnsupportedFeature>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocxDocumentModel {
    pub paragraphs: Vec<DocxParagraphModel>,
    pub page_settings: DocxPageSettingsModel,
    pub compatibility: DocxCompatibilityReport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocxImportResult {
    pub document: WriterDocument,
    pub compatibility: DocxCompatibilityReport,
}
