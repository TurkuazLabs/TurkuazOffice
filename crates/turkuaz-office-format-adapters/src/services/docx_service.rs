// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/services/docx_service.rs
// # 📌 Amac: DOCX format-specific model ile canonical WriterDocument arasinda kayip gorunur mapping yapar
// # 📌 Modul - FileType: Service - Rust
// # Version: 0.2.0
// # Aciklama: DOCX package validation, import compatibility report ve strict minimum export kurallarini koordine eder
// Bagimli Oldugu Katman: Service -> Model -> Tool

use turkuaz_office_core::DocumentSchemaVersion;
use turkuaz_office_core::config::constants::DEFAULT_DOCUMENT_TITLE;
use turkuaz_office_writer::config::constants::{
    MAX_FONT_FAMILY_LENGTH, MAX_FONT_SIZE_HALF_POINTS, MIN_FONT_SIZE_HALF_POINTS,
};
use turkuaz_office_writer::tools::writer_id_tool::WriterIdTool;
use turkuaz_office_writer::{
    Block, CharacterStyle, PageSettings, Paragraph, ParagraphStyle, Section, TextAlignment,
    TextRun, WriterDocument,
};

use crate::config::constants::{
    DOCX_CONTENT_TYPES_ENTRY, DOCX_DOCUMENT_CONTENT_TYPE, DOCX_DOCUMENT_ENTRY,
    DOCX_OFFICE_DOCUMENT_RELATIONSHIP_TYPE, DOCX_ROOT_RELATIONSHIPS_ENTRY,
};
use crate::models::docx_model::{
    DocxAlignment, DocxDocumentModel, DocxImportResult, DocxPageSettingsModel,
    DocxParagraphModel, DocxRunModel,
};
use crate::tools::docx_archive_tool::{DocxArchiveError, DocxArchiveTool};
use crate::tools::docx_xml_tool::{DocxXmlError, DocxXmlTool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocxError {
    Archive(DocxArchiveError),
    Xml(DocxXmlError),
    MissingRequiredEntry,
    InvalidPackageProfile,
    MultipleSectionsUnsupported,
    UnsupportedBlock,
    UnsupportedAsset,
    InvalidCharacterStyle,
}

impl From<DocxArchiveError> for DocxError {
    fn from(value: DocxArchiveError) -> Self {
        Self::Archive(value)
    }
}

impl From<DocxXmlError> for DocxError {
    fn from(value: DocxXmlError) -> Self {
        Self::Xml(value)
    }
}

pub struct DocxService;

impl DocxService {
    pub fn import<I>(
        bytes: &[u8],
        title: impl Into<String>,
        id_tool: &I,
    ) -> Result<DocxImportResult, DocxError>
    where
        I: WriterIdTool,
    {
        let entries = DocxArchiveTool::decode(bytes)?;
        let content_types = entries
            .get(DOCX_CONTENT_TYPES_ENTRY)
            .ok_or(DocxError::MissingRequiredEntry)?;
        let relationships = entries
            .get(DOCX_ROOT_RELATIONSHIPS_ENTRY)
            .ok_or(DocxError::MissingRequiredEntry)?;
        let document_xml = entries
            .get(DOCX_DOCUMENT_ENTRY)
            .ok_or(DocxError::MissingRequiredEntry)?;

        if !Self::contains(content_types, DOCX_DOCUMENT_CONTENT_TYPE)
            || !Self::contains(relationships, DOCX_OFFICE_DOCUMENT_RELATIONSHIP_TYPE)
            || !Self::contains(relationships, DOCX_DOCUMENT_ENTRY)
        {
            return Err(DocxError::InvalidPackageProfile);
        }

        let model = DocxXmlTool::parse_document(document_xml)?;
        let compatibility = model.compatibility.clone();
        let document = Self::to_writer(model, title.into(), id_tool)?;
        Ok(DocxImportResult {
            document,
            compatibility,
        })
    }

    pub fn export(document: &WriterDocument) -> Result<Vec<u8>, DocxError> {
        let model = Self::from_writer(document)?;
        let document_xml = DocxXmlTool::encode_document(&model);
        let content_types = DocxXmlTool::content_types_xml();
        let relationships = DocxXmlTool::root_relationships_xml();

        DocxArchiveTool::encode(&[
            (DOCX_CONTENT_TYPES_ENTRY, content_types.as_bytes()),
            (DOCX_ROOT_RELATIONSHIPS_ENTRY, relationships.as_bytes()),
            (DOCX_DOCUMENT_ENTRY, document_xml.as_bytes()),
        ])
        .map_err(Into::into)
    }

    fn to_writer<I>(
        model: DocxDocumentModel,
        title: String,
        id_tool: &I,
    ) -> Result<WriterDocument, DocxError>
    where
        I: WriterIdTool,
    {
        let mut blocks = Vec::new();
        for source_paragraph in model.paragraphs {
            let mut runs = Vec::new();
            for source_run in source_paragraph.runs {
                let mut style = CharacterStyle::default();
                style.bold = source_run.bold;
                style.italic = source_run.italic;
                style.underline = source_run.underline;
                if let Some(font_family) = source_run.font_family {
                    if font_family.is_empty() || font_family.len() > MAX_FONT_FAMILY_LENGTH {
                        return Err(DocxError::InvalidCharacterStyle);
                    }
                    style.font_family = font_family;
                }
                if let Some(size) = source_run.font_size_half_points {
                    if !(MIN_FONT_SIZE_HALF_POINTS..=MAX_FONT_SIZE_HALF_POINTS).contains(&size) {
                        return Err(DocxError::InvalidCharacterStyle);
                    }
                    style.font_size_half_points = size;
                }
                runs.push(TextRun {
                    id: id_tool.next_node_id(),
                    text: source_run.text,
                    style,
                });
            }

            if runs.is_empty() {
                runs.push(TextRun {
                    id: id_tool.next_node_id(),
                    text: String::new(),
                    style: CharacterStyle::default(),
                });
            }

            blocks.push(Block::Paragraph(Paragraph {
                id: id_tool.next_node_id(),
                style: ParagraphStyle {
                    alignment: source_paragraph
                        .alignment
                        .map(Self::to_writer_alignment)
                        .unwrap_or(TextAlignment::Left),
                    ..ParagraphStyle::default()
                },
                runs,
            }));
        }

        if blocks.is_empty() {
            blocks.push(Block::Paragraph(Paragraph {
                id: id_tool.next_node_id(),
                style: ParagraphStyle::default(),
                runs: vec![TextRun {
                    id: id_tool.next_node_id(),
                    text: String::new(),
                    style: CharacterStyle::default(),
                }],
            }));
        }

        let title = if title.trim().is_empty() {
            DEFAULT_DOCUMENT_TITLE.to_owned()
        } else {
            title.trim().to_owned()
        };

        Ok(WriterDocument {
            id: id_tool.next_document_id(),
            title,
            schema_version: DocumentSchemaVersion::current(),
            revision: 0,
            sections: vec![Section {
                id: id_tool.next_node_id(),
                page_settings: Self::to_writer_page_settings(&model.page_settings),
                blocks,
            }],
            assets: Vec::new(),
        })
    }

    fn from_writer(document: &WriterDocument) -> Result<DocxDocumentModel, DocxError> {
        if document.sections.len() != 1 {
            return Err(DocxError::MultipleSectionsUnsupported);
        }
        if !document.assets.is_empty() {
            return Err(DocxError::UnsupportedAsset);
        }

        let section = document
            .sections
            .first()
            .ok_or(DocxError::MultipleSectionsUnsupported)?;
        let mut paragraphs = Vec::new();
        for block in &section.blocks {
            let Block::Paragraph(paragraph) = block else {
                return Err(DocxError::UnsupportedBlock);
            };
            paragraphs.push(DocxParagraphModel {
                alignment: Some(Self::from_writer_alignment(paragraph.style.alignment)),
                runs: paragraph
                    .runs
                    .iter()
                    .map(|run| DocxRunModel {
                        text: run.text.clone(),
                        bold: run.style.bold,
                        italic: run.style.italic,
                        underline: run.style.underline,
                        font_family: Some(run.style.font_family.clone()),
                        font_size_half_points: Some(run.style.font_size_half_points),
                    })
                    .collect(),
            });
        }

        Ok(DocxDocumentModel {
            paragraphs,
            page_settings: DocxPageSettingsModel {
                width_twips: Some(section.page_settings.width_twips),
                height_twips: Some(section.page_settings.height_twips),
                margin_top_twips: Some(section.page_settings.margin_top_twips),
                margin_right_twips: Some(section.page_settings.margin_right_twips),
                margin_bottom_twips: Some(section.page_settings.margin_bottom_twips),
                margin_left_twips: Some(section.page_settings.margin_left_twips),
            },
            compatibility: Default::default(),
        })
    }

    fn to_writer_page_settings(source: &DocxPageSettingsModel) -> PageSettings {
        let default = PageSettings::default();
        PageSettings {
            width_twips: source.width_twips.unwrap_or(default.width_twips),
            height_twips: source.height_twips.unwrap_or(default.height_twips),
            margin_top_twips: source
                .margin_top_twips
                .unwrap_or(default.margin_top_twips),
            margin_right_twips: source
                .margin_right_twips
                .unwrap_or(default.margin_right_twips),
            margin_bottom_twips: source
                .margin_bottom_twips
                .unwrap_or(default.margin_bottom_twips),
            margin_left_twips: source
                .margin_left_twips
                .unwrap_or(default.margin_left_twips),
        }
    }

    fn to_writer_alignment(value: DocxAlignment) -> TextAlignment {
        match value {
            DocxAlignment::Left => TextAlignment::Left,
            DocxAlignment::Center => TextAlignment::Center,
            DocxAlignment::Right => TextAlignment::Right,
            DocxAlignment::Justify => TextAlignment::Justify,
        }
    }

    fn from_writer_alignment(value: TextAlignment) -> DocxAlignment {
        match value {
            TextAlignment::Left => DocxAlignment::Left,
            TextAlignment::Center => DocxAlignment::Center,
            TextAlignment::Right => DocxAlignment::Right,
            TextAlignment::Justify => DocxAlignment::Justify,
        }
    }

    fn contains(haystack: &[u8], needle: &str) -> bool {
        haystack
            .windows(needle.len())
            .any(|window| window == needle.as_bytes())
    }
}
