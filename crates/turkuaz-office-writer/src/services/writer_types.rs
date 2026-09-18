// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/writer_types.rs
// # 📌 Amac: Format ve UI bagimsiz canonical Writer domain veri modelini tanimlar
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Section, paragraph, run, table, image, style, selection ve page modelini tasir
// Bagimli Oldugu Katman: Service

use turkuaz_office_core::{DocumentId, DocumentSchemaVersion};

use crate::config::constants::{
    DEFAULT_FONT_FAMILY, DEFAULT_FONT_SIZE_HALF_POINTS, DEFAULT_PAGE_HEIGHT_TWIPS,
    DEFAULT_PAGE_MARGIN_TWIPS, DEFAULT_PAGE_WIDTH_TWIPS,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(String);

impl NodeId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharacterStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub font_family: String,
    pub font_size_half_points: u16,
}

impl Default for CharacterStyle {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            font_family: DEFAULT_FONT_FAMILY.to_owned(),
            font_size_half_points: DEFAULT_FONT_SIZE_HALF_POINTS,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CharacterStylePatch {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub font_family: Option<String>,
    pub font_size_half_points: Option<u16>,
}

impl CharacterStylePatch {
    pub fn is_empty(&self) -> bool {
        self.bold.is_none()
            && self.italic.is_none()
            && self.underline.is_none()
            && self.font_family.is_none()
            && self.font_size_half_points.is_none()
    }

    pub fn apply_to(&self, style: &mut CharacterStyle) {
        if let Some(value) = self.bold {
            style.bold = value;
        }
        if let Some(value) = self.italic {
            style.italic = value;
        }
        if let Some(value) = self.underline {
            style.underline = value;
        }
        if let Some(value) = &self.font_family {
            style.font_family = value.clone();
        }
        if let Some(value) = self.font_size_half_points {
            style.font_size_half_points = value;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParagraphStyle {
    pub alignment: TextAlignment,
    pub space_before_twips: u32,
    pub space_after_twips: u32,
    pub first_line_indent_twips: i32,
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        Self {
            alignment: TextAlignment::Left,
            space_before_twips: 0,
            space_after_twips: 0,
            first_line_indent_twips: 0,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParagraphStylePatch {
    pub alignment: Option<TextAlignment>,
}

impl ParagraphStylePatch {
    pub fn is_empty(&self) -> bool {
        self.alignment.is_none()
    }

    pub fn apply_to(&self, style: &mut ParagraphStyle) {
        if let Some(value) = self.alignment {
            style.alignment = value;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageSettings {
    pub width_twips: u32,
    pub height_twips: u32,
    pub margin_top_twips: u32,
    pub margin_right_twips: u32,
    pub margin_bottom_twips: u32,
    pub margin_left_twips: u32,
}

impl Default for PageSettings {
    fn default() -> Self {
        Self {
            width_twips: DEFAULT_PAGE_WIDTH_TWIPS,
            height_twips: DEFAULT_PAGE_HEIGHT_TWIPS,
            margin_top_twips: DEFAULT_PAGE_MARGIN_TWIPS,
            margin_right_twips: DEFAULT_PAGE_MARGIN_TWIPS,
            margin_bottom_twips: DEFAULT_PAGE_MARGIN_TWIPS,
            margin_left_twips: DEFAULT_PAGE_MARGIN_TWIPS,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextRun {
    pub id: NodeId,
    pub text: String,
    pub style: CharacterStyle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StyledTextRun {
    pub text: String,
    pub style: CharacterStyle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Paragraph {
    pub id: NodeId,
    pub style: ParagraphStyle,
    pub runs: Vec<TextRun>,
}

impl Paragraph {
    pub fn plain_text(&self) -> String {
        self.runs.iter().map(|run| run.text.as_str()).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableCell {
    pub id: NodeId,
    pub paragraphs: Vec<Paragraph>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableRow {
    pub id: NodeId,
    pub cells: Vec<TableCell>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Table {
    pub id: NodeId,
    pub rows: Vec<TableRow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageBlock {
    pub id: NodeId,
    pub asset_id: String,
    pub alt_text: String,
    pub width_twips: Option<u32>,
    pub height_twips: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Block {
    Paragraph(Paragraph),
    Table(Table),
    Image(ImageBlock),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub id: NodeId,
    pub page_settings: PageSettings,
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriterDocument {
    pub id: DocumentId,
    pub title: String,
    pub schema_version: DocumentSchemaVersion,
    pub revision: u64,
    pub sections: Vec<Section>,
}

impl WriterDocument {
    pub fn plain_text(&self) -> String {
        let mut lines = Vec::new();
        for section in &self.sections {
            for block in &section.blocks {
                if let Block::Paragraph(paragraph) = block {
                    lines.push(paragraph.plain_text());
                }
            }
        }
        lines.join("\n")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextPosition {
    pub paragraph_id: NodeId,
    pub run_id: NodeId,
    pub offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextRange {
    pub anchor: TextPosition,
    pub focus: TextPosition,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selection {
    Caret(TextPosition),
    Range(TextRange),
}
