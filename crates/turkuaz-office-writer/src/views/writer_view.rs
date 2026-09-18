// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/views/writer_view.rs
// # 📌 Amac: Writer canonical domain verisini UI ve platform kopruleri icin read-only DTO'ya map eder
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Document, paragraph ve run kimliklerini mutation yetkisi vermeden View katmanina tasir
// Bagimli Oldugu Katman: View

use crate::services::writer_types::{
    Block, CharacterStyle, PageSettings, ParagraphStyle, WriterDocument,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriterRunView {
    pub id: String,
    pub text: String,
    pub style: CharacterStyle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriterParagraphView {
    pub id: String,
    pub plain_text: String,
    pub style: ParagraphStyle,
    pub runs: Vec<WriterRunView>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriterPageSettingsView {
    pub width_twips: u32,
    pub height_twips: u32,
    pub margin_top_twips: u32,
    pub margin_right_twips: u32,
    pub margin_bottom_twips: u32,
    pub margin_left_twips: u32,
}

impl From<PageSettings> for WriterPageSettingsView {
    fn from(settings: PageSettings) -> Self {
        Self {
            width_twips: settings.width_twips,
            height_twips: settings.height_twips,
            margin_top_twips: settings.margin_top_twips,
            margin_right_twips: settings.margin_right_twips,
            margin_bottom_twips: settings.margin_bottom_twips,
            margin_left_twips: settings.margin_left_twips,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriterDocumentView {
    pub id: String,
    pub title: String,
    pub revision: u64,
    pub plain_text: String,
    pub section_count: usize,
    pub page_settings: WriterPageSettingsView,
    pub paragraphs: Vec<WriterParagraphView>,
}

impl From<WriterDocument> for WriterDocumentView {
    fn from(document: WriterDocument) -> Self {
        let paragraphs = document
            .sections
            .iter()
            .flat_map(|section| section.blocks.iter())
            .filter_map(|block| match block {
                Block::Paragraph(paragraph) => Some(WriterParagraphView {
                    id: paragraph.id.as_str().to_owned(),
                    plain_text: paragraph.plain_text(),
                    style: paragraph.style.clone(),
                    runs: paragraph
                        .runs
                        .iter()
                        .map(|run| WriterRunView {
                            id: run.id.as_str().to_owned(),
                            text: run.text.clone(),
                            style: run.style.clone(),
                        })
                        .collect(),
                }),
                Block::Table(_) | Block::Image(_) => None,
            })
            .collect();

        let page_settings = document
            .sections
            .first()
            .map(|section| section.page_settings.clone())
            .unwrap_or_default();

        Self {
            id: document.id.as_str().to_owned(),
            title: document.title.clone(),
            revision: document.revision,
            plain_text: document.plain_text(),
            section_count: document.sections.len(),
            page_settings: page_settings.into(),
            paragraphs,
        }
    }
}
