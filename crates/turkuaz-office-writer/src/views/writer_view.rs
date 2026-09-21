// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/views/writer_view.rs
// # 📌 Amac: Writer canonical domain verisini UI ve platform kopruleri icin read-only DTO'ya map eder
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Paragraph, run, image block ve page metadata'sini binary asset bytes tasimadan View katmanina acar
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
pub struct WriterImageView {
    pub id: String,
    pub asset_id: String,
    pub after_paragraph_id: Option<String>,
    pub alt_text: String,
    pub width_twips: Option<u32>,
    pub height_twips: Option<u32>,
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
    pub images: Vec<WriterImageView>,
}

impl From<WriterDocument> for WriterDocumentView {
    fn from(document: WriterDocument) -> Self {
        let mut paragraphs = Vec::new();
        let mut images = Vec::new();

        for section in &document.sections {
            let mut previous_paragraph_id: Option<String> = None;
            for block in &section.blocks {
                match block {
                    Block::Paragraph(paragraph) => {
                        let paragraph_id = paragraph.id.as_str().to_owned();
                        paragraphs.push(WriterParagraphView {
                            id: paragraph_id.clone(),
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
                        });
                        previous_paragraph_id = Some(paragraph_id);
                    }
                    Block::Image(image) => images.push(WriterImageView {
                        id: image.id.as_str().to_owned(),
                        asset_id: image.asset_id.clone(),
                        after_paragraph_id: previous_paragraph_id.clone(),
                        alt_text: image.alt_text.clone(),
                        width_twips: image.width_twips,
                        height_twips: image.height_twips,
                    }),
                    Block::Table(_) => {}
                }
            }
        }

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
            images,
        }
    }
}
