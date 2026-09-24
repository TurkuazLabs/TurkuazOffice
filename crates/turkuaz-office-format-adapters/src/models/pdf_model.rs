// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/models/pdf_model.rs
// # 📌 Amac: Canonical Writer ile PDF writer arasindaki format-specific render modelini tanimlar
// # 📌 Modul - FileType: Model - Rust
// # Version: 0.2.0
// # Aciklama: Font request/data, paragraph/run, page geometry ve alignment semantigini PDF adapterine tasir
// Bagimli Oldugu Katman: Model

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PdfFontKey {
    pub family: String,
    pub bold: bool,
    pub italic: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PdfFontData {
    pub key: PdfFontKey,
    pub bytes: Vec<u8>,
    pub face_index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PdfRunModel {
    pub text: String,
    pub font: PdfFontKey,
    pub font_size_half_points: u16,
    pub underline: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PdfParagraphModel {
    pub alignment: PdfAlignment,
    pub runs: Vec<PdfRunModel>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PdfPageSettingsModel {
    pub width_twips: u32,
    pub height_twips: u32,
    pub margin_top_twips: u32,
    pub margin_right_twips: u32,
    pub margin_bottom_twips: u32,
    pub margin_left_twips: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PdfDocumentModel {
    pub title: String,
    pub page_settings: PdfPageSettingsModel,
    pub paragraphs: Vec<PdfParagraphModel>,
}
