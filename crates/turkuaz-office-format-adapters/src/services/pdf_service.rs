// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/services/pdf_service.rs
// # 📌 Amac: Canonical WriterDocument'i PDF minimum render modeline map eder ve gerekli fontlari bildirir
// # 📌 Modul - FileType: Service - Rust
// # Version: 0.2.0
// # Aciklama: Paragraph/run/page mapping, strict unsupported structure ve font completeness kurallarini koordine eder
// Bagimli Oldugu Katman: Service -> Model -> Tool

use std::collections::{BTreeMap, BTreeSet};

use turkuaz_office_writer::{Block, TextAlignment, WriterDocument};

use crate::models::pdf_model::{
    PdfAlignment, PdfDocumentModel, PdfFontData, PdfFontKey, PdfPageSettingsModel,
    PdfParagraphModel, PdfRunModel,
};
use crate::tools::pdf_writer_tool::{PdfWriterError, PdfWriterTool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PdfError {
    MultipleSectionsUnsupported,
    UnsupportedBlock,
    UnsupportedAsset,
    FontMissing(PdfFontKey),
    Writer(PdfWriterError),
}

impl From<PdfWriterError> for PdfError {
    fn from(value: PdfWriterError) -> Self {
        Self::Writer(value)
    }
}

pub struct PdfService;

impl PdfService {
    pub fn font_requests(document: &WriterDocument) -> Result<Vec<PdfFontKey>, PdfError> {
        Self::validate_structure(document)?;
        let mut requests = BTreeSet::new();
        for section in &document.sections {
            for block in &section.blocks {
                let Block::Paragraph(paragraph) = block else {
                    return Err(PdfError::UnsupportedBlock);
                };
                for run in &paragraph.runs {
                    requests.insert(PdfFontKey {
                        family: run.style.font_family.clone(),
                        bold: run.style.bold,
                        italic: run.style.italic,
                    });
                }
            }
        }
        Ok(requests.into_iter().collect())
    }

    pub fn export(document: &WriterDocument, fonts: &[PdfFontData]) -> Result<Vec<u8>, PdfError> {
        Self::validate_structure(document)?;
        let available: BTreeMap<_, _> = fonts.iter().map(|font| (font.key.clone(), font)).collect();
        for request in Self::font_requests(document)? {
            if !available.contains_key(&request) {
                return Err(PdfError::FontMissing(request));
            }
        }

        let section = document
            .sections
            .first()
            .ok_or(PdfError::MultipleSectionsUnsupported)?;
        let paragraphs = section
            .blocks
            .iter()
            .map(|block| {
                let Block::Paragraph(paragraph) = block else {
                    return Err(PdfError::UnsupportedBlock);
                };
                Ok(PdfParagraphModel {
                    alignment: Self::alignment(paragraph.style.alignment),
                    runs: paragraph
                        .runs
                        .iter()
                        .map(|run| PdfRunModel {
                            text: run.text.clone(),
                            font: PdfFontKey {
                                family: run.style.font_family.clone(),
                                bold: run.style.bold,
                                italic: run.style.italic,
                            },
                            font_size_half_points: run.style.font_size_half_points,
                            underline: run.style.underline,
                        })
                        .collect(),
                })
            })
            .collect::<Result<Vec<_>, PdfError>>()?;

        let model = PdfDocumentModel {
            title: document.title.clone(),
            page_settings: PdfPageSettingsModel {
                width_twips: section.page_settings.width_twips,
                height_twips: section.page_settings.height_twips,
                margin_top_twips: section.page_settings.margin_top_twips,
                margin_right_twips: section.page_settings.margin_right_twips,
                margin_bottom_twips: section.page_settings.margin_bottom_twips,
                margin_left_twips: section.page_settings.margin_left_twips,
            },
            paragraphs,
        };
        PdfWriterTool::encode(&model, fonts).map_err(Into::into)
    }

    fn validate_structure(document: &WriterDocument) -> Result<(), PdfError> {
        if document.sections.len() != 1 {
            return Err(PdfError::MultipleSectionsUnsupported);
        }
        if !document.assets.is_empty() {
            return Err(PdfError::UnsupportedAsset);
        }
        if document
            .sections
            .iter()
            .flat_map(|section| section.blocks.iter())
            .any(|block| !matches!(block, Block::Paragraph(_)))
        {
            return Err(PdfError::UnsupportedBlock);
        }
        Ok(())
    }

    fn alignment(value: TextAlignment) -> PdfAlignment {
        match value {
            TextAlignment::Left => PdfAlignment::Left,
            TextAlignment::Center => PdfAlignment::Center,
            TextAlignment::Right => PdfAlignment::Right,
            TextAlignment::Justify => PdfAlignment::Justify,
        }
    }
}
