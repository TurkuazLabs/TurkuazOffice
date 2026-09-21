// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/writer_dto.rs
// # 📌 Amac: Writer domain View modelini Tauri IPC icin serializable DTO'lara map eder
// # 📌 Modul - FileType: View - Rust
// # Version: 0.2.0
// # Aciklama: Frontend'e read-only document, file operation, paragraph ve run alanlarini camelCase JSON olarak tasir
// Bagimli Oldugu Katman: View

use serde::{Deserialize, Serialize};
use turkuaz_office_writer::{
    CharacterStyle, ParagraphStyle, StyledTextRun, TextAlignment, WriterAsset,
    WriterDocumentView, WriterImageView, WriterPageSettingsView, WriterParagraphView,
    WriterRunView,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterFileOperationDto {
    pub document: WriterDocumentDto,
    pub path: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterCharacterStyleDto {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub font_family: String,
    pub font_size_half_points: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterCharacterStyleInputDto {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub font_family: String,
    pub font_size_half_points: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterStyledRunInputDto {
    pub text: String,
    pub style: WriterCharacterStyleInputDto,
}

impl From<WriterStyledRunInputDto> for StyledTextRun {
    fn from(run: WriterStyledRunInputDto) -> Self {
        Self {
            text: run.text,
            style: run.style.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WriterTextAlignmentDto {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterParagraphStyleDto {
    pub alignment: WriterTextAlignmentDto,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterRunDto {
    pub id: String,
    pub text: String,
    pub style: WriterCharacterStyleDto,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterParagraphDto {
    pub id: String,
    pub plain_text: String,
    pub style: WriterParagraphStyleDto,
    pub runs: Vec<WriterRunDto>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterImageDto {
    pub id: String,
    pub asset_id: String,
    pub after_paragraph_id: Option<String>,
    pub alt_text: String,
    pub width_twips: Option<u32>,
    pub height_twips: Option<u32>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterAssetDto {
    pub id: String,
    pub media_type: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterPageSettingsDto {
    pub width_twips: u32,
    pub height_twips: u32,
    pub margin_top_twips: u32,
    pub margin_right_twips: u32,
    pub margin_bottom_twips: u32,
    pub margin_left_twips: u32,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterDocumentDto {
    pub id: String,
    pub title: String,
    pub revision: u64,
    pub plain_text: String,
    pub section_count: usize,
    pub page_settings: WriterPageSettingsDto,
    pub paragraphs: Vec<WriterParagraphDto>,
    pub images: Vec<WriterImageDto>,
}

impl From<WriterCharacterStyleInputDto> for CharacterStyle {
    fn from(style: WriterCharacterStyleInputDto) -> Self {
        Self {
            bold: style.bold,
            italic: style.italic,
            underline: style.underline,
            font_family: style.font_family,
            font_size_half_points: style.font_size_half_points,
        }
    }
}

impl From<TextAlignment> for WriterTextAlignmentDto {
    fn from(alignment: TextAlignment) -> Self {
        match alignment {
            TextAlignment::Left => Self::Left,
            TextAlignment::Center => Self::Center,
            TextAlignment::Right => Self::Right,
            TextAlignment::Justify => Self::Justify,
        }
    }
}

impl From<WriterTextAlignmentDto> for TextAlignment {
    fn from(alignment: WriterTextAlignmentDto) -> Self {
        match alignment {
            WriterTextAlignmentDto::Left => Self::Left,
            WriterTextAlignmentDto::Center => Self::Center,
            WriterTextAlignmentDto::Right => Self::Right,
            WriterTextAlignmentDto::Justify => Self::Justify,
        }
    }
}

impl From<ParagraphStyle> for WriterParagraphStyleDto {
    fn from(style: ParagraphStyle) -> Self {
        Self {
            alignment: style.alignment.into(),
        }
    }
}

impl From<CharacterStyle> for WriterCharacterStyleDto {
    fn from(style: CharacterStyle) -> Self {
        Self {
            bold: style.bold,
            italic: style.italic,
            underline: style.underline,
            font_family: style.font_family,
            font_size_half_points: style.font_size_half_points,
        }
    }
}

impl From<WriterRunView> for WriterRunDto {
    fn from(view: WriterRunView) -> Self {
        Self {
            id: view.id,
            text: view.text,
            style: view.style.into(),
        }
    }
}

impl From<WriterParagraphView> for WriterParagraphDto {
    fn from(view: WriterParagraphView) -> Self {
        Self {
            id: view.id,
            plain_text: view.plain_text,
            style: view.style.into(),
            runs: view.runs.into_iter().map(WriterRunDto::from).collect(),
        }
    }
}

impl From<WriterImageView> for WriterImageDto {
    fn from(view: WriterImageView) -> Self {
        Self {
            id: view.id,
            asset_id: view.asset_id,
            after_paragraph_id: view.after_paragraph_id,
            alt_text: view.alt_text,
            width_twips: view.width_twips,
            height_twips: view.height_twips,
        }
    }
}

impl From<WriterAsset> for WriterAssetDto {
    fn from(asset: WriterAsset) -> Self {
        Self {
            id: asset.id,
            media_type: asset.media_type,
            data: asset.bytes,
        }
    }
}

impl From<WriterPageSettingsView> for WriterPageSettingsDto {
    fn from(view: WriterPageSettingsView) -> Self {
        Self {
            width_twips: view.width_twips,
            height_twips: view.height_twips,
            margin_top_twips: view.margin_top_twips,
            margin_right_twips: view.margin_right_twips,
            margin_bottom_twips: view.margin_bottom_twips,
            margin_left_twips: view.margin_left_twips,
        }
    }
}

impl From<WriterDocumentView> for WriterDocumentDto {
    fn from(view: WriterDocumentView) -> Self {
        Self {
            id: view.id,
            title: view.title,
            revision: view.revision,
            plain_text: view.plain_text,
            section_count: view.section_count,
            page_settings: view.page_settings.into(),
            paragraphs: view
                .paragraphs
                .into_iter()
                .map(WriterParagraphDto::from)
                .collect(),
            images: view
                .images
                .into_iter()
                .map(WriterImageDto::from)
                .collect(),
        }
    }
}
