// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/writer_template_service.rs
// # 📌 Amac: Built-in Writer template katalogunu dogrular, listeler ve canonical belge iskeletine uygular
// # 📌 Modul - FileType: Service - Rust
// # Version: 0.2.0
// # Aciklama: YAML template config -> typed template summary -> canonical paragraph/run agaci donusumunu yonetir
// Bagimli Oldugu Katman: Service -> Tool

use std::collections::HashSet;

use serde::Deserialize;

use crate::config::constants::{
    MAX_FONT_FAMILY_LENGTH, MAX_FONT_SIZE_HALF_POINTS, MAX_TEMPLATE_ID_LENGTH,
    MAX_TEMPLATE_LANGUAGE_KEY_LENGTH, MAX_TEMPLATE_PARAGRAPHS, MIN_FONT_SIZE_HALF_POINTS,
    TEMPLATE_CATALOG_VERSION,
};
use crate::services::writer_document_factory_service::WriterDocumentFactoryService;
use crate::services::writer_types::{
    Block, CharacterStyle, Paragraph, ParagraphStyle, TextAlignment, TextRun, WriterDocument,
};
use crate::tools::tko_yaml_tool::TkoYamlTool;
use crate::tools::writer_id_tool::WriterIdTool;
use crate::tools::writer_template_catalog_tool::WriterTemplateCatalogTool;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriterTemplateSummary {
    pub id: String,
    pub name_key: String,
    pub description_key: String,
    pub quick_create: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriterTemplateError {
    CatalogInvalid,
    TemplateNotFound,
}

#[derive(Clone, Debug, Deserialize)]
struct WriterTemplateCatalogConfig {
    catalog_version: u32,
    templates: Vec<WriterTemplateConfig>,
}

#[derive(Clone, Debug, Deserialize)]
struct WriterTemplateConfig {
    id: String,
    name_key: String,
    description_key: String,
    #[serde(default)]
    quick_create: bool,
    paragraphs: Vec<WriterTemplateParagraphConfig>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
enum WriterTemplateAlignment {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct WriterTemplateParagraphConfig {
    #[serde(default)]
    text: String,
    #[serde(default)]
    alignment: WriterTemplateAlignment,
    #[serde(default)]
    bold: bool,
    #[serde(default)]
    italic: bool,
    #[serde(default)]
    underline: bool,
    font_family: Option<String>,
    font_size_half_points: Option<u16>,
    #[serde(default)]
    space_before_twips: u32,
    #[serde(default)]
    space_after_twips: u32,
    #[serde(default)]
    first_line_indent_twips: i32,
}

pub struct WriterTemplateService;

impl WriterTemplateService {
    pub fn catalog() -> Result<Vec<WriterTemplateSummary>, WriterTemplateError> {
        let catalog = Self::load_catalog()?;
        Ok(catalog
            .templates
            .into_iter()
            .map(|template| WriterTemplateSummary {
                id: template.id,
                name_key: template.name_key,
                description_key: template.description_key,
                quick_create: template.quick_create,
            })
            .collect())
    }

    pub fn create<I>(id_tool: &I, template_id: &str) -> Result<WriterDocument, WriterTemplateError>
    where
        I: WriterIdTool,
    {
        let catalog = Self::load_catalog()?;
        let template = catalog
            .templates
            .into_iter()
            .find(|template| template.id == template_id)
            .ok_or(WriterTemplateError::TemplateNotFound)?;

        let mut document = WriterDocumentFactoryService::create(id_tool, "");
        let section = document
            .sections
            .first_mut()
            .ok_or(WriterTemplateError::CatalogInvalid)?;
        section.blocks = template
            .paragraphs
            .into_iter()
            .map(|paragraph| {
                let mut character_style = CharacterStyle::default();
                character_style.bold = paragraph.bold;
                character_style.italic = paragraph.italic;
                character_style.underline = paragraph.underline;
                if let Some(font_family) = paragraph.font_family {
                    character_style.font_family = font_family;
                }
                if let Some(font_size) = paragraph.font_size_half_points {
                    character_style.font_size_half_points = font_size;
                }

                Block::Paragraph(Paragraph {
                    id: id_tool.next_node_id(),
                    style: ParagraphStyle {
                        alignment: paragraph.alignment.into(),
                        space_before_twips: paragraph.space_before_twips,
                        space_after_twips: paragraph.space_after_twips,
                        first_line_indent_twips: paragraph.first_line_indent_twips,
                    },
                    runs: vec![TextRun {
                        id: id_tool.next_node_id(),
                        text: paragraph.text,
                        style: character_style,
                    }],
                })
            })
            .collect();
        Ok(document)
    }

    fn load_catalog() -> Result<WriterTemplateCatalogConfig, WriterTemplateError> {
        let catalog: WriterTemplateCatalogConfig =
            TkoYamlTool::deserialize(WriterTemplateCatalogTool::built_in_bytes())
                .map_err(|_| WriterTemplateError::CatalogInvalid)?;
        Self::validate_catalog(&catalog)?;
        Ok(catalog)
    }

    fn validate_catalog(catalog: &WriterTemplateCatalogConfig) -> Result<(), WriterTemplateError> {
        if catalog.catalog_version != TEMPLATE_CATALOG_VERSION || catalog.templates.is_empty() {
            return Err(WriterTemplateError::CatalogInvalid);
        }

        let mut ids = HashSet::new();
        for template in &catalog.templates {
            if template.id.is_empty()
                || template.id.len() > MAX_TEMPLATE_ID_LENGTH
                || template.name_key.is_empty()
                || template.name_key.len() > MAX_TEMPLATE_LANGUAGE_KEY_LENGTH
                || template.description_key.is_empty()
                || template.description_key.len() > MAX_TEMPLATE_LANGUAGE_KEY_LENGTH
                || template.paragraphs.is_empty()
                || template.paragraphs.len() > MAX_TEMPLATE_PARAGRAPHS
                || !ids.insert(template.id.as_str())
            {
                return Err(WriterTemplateError::CatalogInvalid);
            }

            for paragraph in &template.paragraphs {
                if paragraph.font_family.as_deref().is_some_and(|family| {
                    family.trim().is_empty() || family.len() > MAX_FONT_FAMILY_LENGTH
                }) || paragraph.font_size_half_points.is_some_and(|size| {
                    !(MIN_FONT_SIZE_HALF_POINTS..=MAX_FONT_SIZE_HALF_POINTS).contains(&size)
                }) {
                    return Err(WriterTemplateError::CatalogInvalid);
                }
            }
        }
        Ok(())
    }
}

impl From<WriterTemplateAlignment> for TextAlignment {
    fn from(value: WriterTemplateAlignment) -> Self {
        match value {
            WriterTemplateAlignment::Left => Self::Left,
            WriterTemplateAlignment::Center => Self::Center,
            WriterTemplateAlignment::Right => Self::Right,
            WriterTemplateAlignment::Justify => Self::Justify,
        }
    }
}
