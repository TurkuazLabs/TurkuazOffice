// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/tko_package_types.rs
// # 📌 Amac: TKO v1 YAML disk DTO modelini canonical Writer domain modelinden ayirir
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Manifest ve Writer content YAML semasini serde DTO katmaninda tanimlar
// Bagimli Oldugu Katman: Service

use serde::{Deserialize, Serialize};
use turkuaz_office_core::{DocumentId, DocumentSchemaVersion};

use crate::services::tko_profile_service::{TkoManifestV1, WriterTkoPackageV1};
use crate::services::writer_types::{
    Block, CharacterStyle, ImageBlock, NodeId, PageSettings, Paragraph, ParagraphStyle, Section,
    Table, TableCell, TableRow, TextAlignment, TextRun, WriterDocument,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TkoManifestDtoV1 {
    pub format_version: u32,
    pub schema_version: u32,
    pub document_kind: String,
    pub document_id: String,
    pub revision: u64,
    pub created_by_app_version: String,
    pub required_capabilities: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TkoAssetIndexDtoV1 {
    pub assets: Vec<TkoAssetEntryDtoV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TkoAssetEntryDtoV1 {
    pub id: String,
    pub media_type: String,
    pub entry: String,
    pub byte_length: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WriterContentDtoV1 {
    pub id: String,
    pub title: String,
    pub schema_version: u32,
    pub revision: u64,
    pub sections: Vec<SectionDtoV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SectionDtoV1 {
    pub id: String,
    pub page_settings: PageSettingsDtoV1,
    pub blocks: Vec<BlockDtoV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageSettingsDtoV1 {
    pub width_twips: u32,
    pub height_twips: u32,
    pub margin_top_twips: u32,
    pub margin_right_twips: u32,
    pub margin_bottom_twips: u32,
    pub margin_left_twips: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BlockDtoV1 {
    Paragraph { paragraph: ParagraphDtoV1 },
    Table { table: TableDtoV1 },
    Image { image: ImageBlockDtoV1 },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParagraphDtoV1 {
    pub id: String,
    pub style: ParagraphStyleDtoV1,
    pub runs: Vec<TextRunDtoV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParagraphStyleDtoV1 {
    pub alignment: TextAlignmentDtoV1,
    pub space_before_twips: u32,
    pub space_after_twips: u32,
    pub first_line_indent_twips: i32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TextAlignmentDtoV1 {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextRunDtoV1 {
    pub id: String,
    pub text: String,
    pub style: CharacterStyleDtoV1,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharacterStyleDtoV1 {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub font_family: String,
    pub font_size_half_points: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableDtoV1 {
    pub id: String,
    pub rows: Vec<TableRowDtoV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableRowDtoV1 {
    pub id: String,
    pub cells: Vec<TableCellDtoV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableCellDtoV1 {
    pub id: String,
    pub paragraphs: Vec<ParagraphDtoV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageBlockDtoV1 {
    pub id: String,
    pub asset_id: String,
    pub alt_text: String,
    pub width_twips: Option<u32>,
    pub height_twips: Option<u32>,
}

impl From<&WriterTkoPackageV1> for (TkoManifestDtoV1, WriterContentDtoV1) {
    fn from(package: &WriterTkoPackageV1) -> Self {
        (
            TkoManifestDtoV1 {
                format_version: package.manifest.format_version,
                schema_version: package.manifest.schema_version.value(),
                document_kind: package.manifest.document_kind.clone(),
                document_id: package.manifest.document_id.clone(),
                revision: package.manifest.revision,
                created_by_app_version: package.manifest.created_by_app_version.clone(),
                required_capabilities: package.manifest.required_capabilities.clone(),
            },
            WriterContentDtoV1::from(&package.content),
        )
    }
}

impl From<(TkoManifestDtoV1, WriterContentDtoV1)> for WriterTkoPackageV1 {
    fn from(value: (TkoManifestDtoV1, WriterContentDtoV1)) -> Self {
        let (manifest, content) = value;
        Self {
            manifest: TkoManifestV1 {
                format_version: manifest.format_version,
                schema_version: DocumentSchemaVersion::new(manifest.schema_version),
                document_kind: manifest.document_kind,
                document_id: manifest.document_id,
                revision: manifest.revision,
                created_by_app_version: manifest.created_by_app_version,
                required_capabilities: manifest.required_capabilities,
            },
            content: content.into(),
        }
    }
}

impl From<&WriterDocument> for WriterContentDtoV1 {
    fn from(document: &WriterDocument) -> Self {
        Self {
            id: document.id.as_str().to_owned(),
            title: document.title.clone(),
            schema_version: document.schema_version.value(),
            revision: document.revision,
            sections: document.sections.iter().map(SectionDtoV1::from).collect(),
        }
    }
}

impl From<WriterContentDtoV1> for WriterDocument {
    fn from(value: WriterContentDtoV1) -> Self {
        Self {
            id: DocumentId::new(value.id),
            title: value.title,
            schema_version: DocumentSchemaVersion::new(value.schema_version),
            revision: value.revision,
            sections: value.sections.into_iter().map(Section::from).collect(),
            assets: Vec::new(),
        }
    }
}

impl From<&Section> for SectionDtoV1 {
    fn from(section: &Section) -> Self {
        Self {
            id: section.id.as_str().to_owned(),
            page_settings: PageSettingsDtoV1::from(&section.page_settings),
            blocks: section.blocks.iter().map(BlockDtoV1::from).collect(),
        }
    }
}

impl From<SectionDtoV1> for Section {
    fn from(value: SectionDtoV1) -> Self {
        Self {
            id: NodeId::new(value.id),
            page_settings: value.page_settings.into(),
            blocks: value.blocks.into_iter().map(Block::from).collect(),
        }
    }
}

impl From<&PageSettings> for PageSettingsDtoV1 {
    fn from(value: &PageSettings) -> Self {
        Self {
            width_twips: value.width_twips,
            height_twips: value.height_twips,
            margin_top_twips: value.margin_top_twips,
            margin_right_twips: value.margin_right_twips,
            margin_bottom_twips: value.margin_bottom_twips,
            margin_left_twips: value.margin_left_twips,
        }
    }
}

impl From<PageSettingsDtoV1> for PageSettings {
    fn from(value: PageSettingsDtoV1) -> Self {
        Self {
            width_twips: value.width_twips,
            height_twips: value.height_twips,
            margin_top_twips: value.margin_top_twips,
            margin_right_twips: value.margin_right_twips,
            margin_bottom_twips: value.margin_bottom_twips,
            margin_left_twips: value.margin_left_twips,
        }
    }
}

impl From<&Block> for BlockDtoV1 {
    fn from(block: &Block) -> Self {
        match block {
            Block::Paragraph(paragraph) => Self::Paragraph {
                paragraph: ParagraphDtoV1::from(paragraph),
            },
            Block::Table(table) => Self::Table {
                table: TableDtoV1::from(table),
            },
            Block::Image(image) => Self::Image {
                image: ImageBlockDtoV1::from(image),
            },
        }
    }
}

impl From<BlockDtoV1> for Block {
    fn from(value: BlockDtoV1) -> Self {
        match value {
            BlockDtoV1::Paragraph { paragraph } => Self::Paragraph(paragraph.into()),
            BlockDtoV1::Table { table } => Self::Table(table.into()),
            BlockDtoV1::Image { image } => Self::Image(image.into()),
        }
    }
}

impl From<&Paragraph> for ParagraphDtoV1 {
    fn from(value: &Paragraph) -> Self {
        Self {
            id: value.id.as_str().to_owned(),
            style: ParagraphStyleDtoV1::from(&value.style),
            runs: value.runs.iter().map(TextRunDtoV1::from).collect(),
        }
    }
}

impl From<ParagraphDtoV1> for Paragraph {
    fn from(value: ParagraphDtoV1) -> Self {
        Self {
            id: NodeId::new(value.id),
            style: value.style.into(),
            runs: value.runs.into_iter().map(TextRun::from).collect(),
        }
    }
}

impl From<&ParagraphStyle> for ParagraphStyleDtoV1 {
    fn from(value: &ParagraphStyle) -> Self {
        Self {
            alignment: value.alignment.into(),
            space_before_twips: value.space_before_twips,
            space_after_twips: value.space_after_twips,
            first_line_indent_twips: value.first_line_indent_twips,
        }
    }
}

impl From<ParagraphStyleDtoV1> for ParagraphStyle {
    fn from(value: ParagraphStyleDtoV1) -> Self {
        Self {
            alignment: value.alignment.into(),
            space_before_twips: value.space_before_twips,
            space_after_twips: value.space_after_twips,
            first_line_indent_twips: value.first_line_indent_twips,
        }
    }
}

impl From<TextAlignment> for TextAlignmentDtoV1 {
    fn from(value: TextAlignment) -> Self {
        match value {
            TextAlignment::Left => Self::Left,
            TextAlignment::Center => Self::Center,
            TextAlignment::Right => Self::Right,
            TextAlignment::Justify => Self::Justify,
        }
    }
}

impl From<TextAlignmentDtoV1> for TextAlignment {
    fn from(value: TextAlignmentDtoV1) -> Self {
        match value {
            TextAlignmentDtoV1::Left => Self::Left,
            TextAlignmentDtoV1::Center => Self::Center,
            TextAlignmentDtoV1::Right => Self::Right,
            TextAlignmentDtoV1::Justify => Self::Justify,
        }
    }
}

impl From<&TextRun> for TextRunDtoV1 {
    fn from(value: &TextRun) -> Self {
        Self {
            id: value.id.as_str().to_owned(),
            text: value.text.clone(),
            style: CharacterStyleDtoV1::from(&value.style),
        }
    }
}

impl From<TextRunDtoV1> for TextRun {
    fn from(value: TextRunDtoV1) -> Self {
        Self {
            id: NodeId::new(value.id),
            text: value.text,
            style: value.style.into(),
        }
    }
}

impl From<&CharacterStyle> for CharacterStyleDtoV1 {
    fn from(value: &CharacterStyle) -> Self {
        Self {
            bold: value.bold,
            italic: value.italic,
            underline: value.underline,
            font_family: value.font_family.clone(),
            font_size_half_points: value.font_size_half_points,
        }
    }
}

impl From<CharacterStyleDtoV1> for CharacterStyle {
    fn from(value: CharacterStyleDtoV1) -> Self {
        Self {
            bold: value.bold,
            italic: value.italic,
            underline: value.underline,
            font_family: value.font_family,
            font_size_half_points: value.font_size_half_points,
        }
    }
}

impl From<&Table> for TableDtoV1 {
    fn from(value: &Table) -> Self {
        Self {
            id: value.id.as_str().to_owned(),
            rows: value.rows.iter().map(TableRowDtoV1::from).collect(),
        }
    }
}

impl From<TableDtoV1> for Table {
    fn from(value: TableDtoV1) -> Self {
        Self {
            id: NodeId::new(value.id),
            rows: value.rows.into_iter().map(TableRow::from).collect(),
        }
    }
}

impl From<&TableRow> for TableRowDtoV1 {
    fn from(value: &TableRow) -> Self {
        Self {
            id: value.id.as_str().to_owned(),
            cells: value.cells.iter().map(TableCellDtoV1::from).collect(),
        }
    }
}

impl From<TableRowDtoV1> for TableRow {
    fn from(value: TableRowDtoV1) -> Self {
        Self {
            id: NodeId::new(value.id),
            cells: value.cells.into_iter().map(TableCell::from).collect(),
        }
    }
}

impl From<&TableCell> for TableCellDtoV1 {
    fn from(value: &TableCell) -> Self {
        Self {
            id: value.id.as_str().to_owned(),
            paragraphs: value.paragraphs.iter().map(ParagraphDtoV1::from).collect(),
        }
    }
}

impl From<TableCellDtoV1> for TableCell {
    fn from(value: TableCellDtoV1) -> Self {
        Self {
            id: NodeId::new(value.id),
            paragraphs: value.paragraphs.into_iter().map(Paragraph::from).collect(),
        }
    }
}

impl From<&ImageBlock> for ImageBlockDtoV1 {
    fn from(value: &ImageBlock) -> Self {
        Self {
            id: value.id.as_str().to_owned(),
            asset_id: value.asset_id.clone(),
            alt_text: value.alt_text.clone(),
            width_twips: value.width_twips,
            height_twips: value.height_twips,
        }
    }
}

impl From<ImageBlockDtoV1> for ImageBlock {
    fn from(value: ImageBlockDtoV1) -> Self {
        Self {
            id: NodeId::new(value.id),
            asset_id: value.asset_id,
            alt_text: value.alt_text,
            width_twips: value.width_twips,
            height_twips: value.height_twips,
        }
    }
}
