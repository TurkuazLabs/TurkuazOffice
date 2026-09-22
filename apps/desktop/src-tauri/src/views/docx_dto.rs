// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/docx_dto.rs
// # 📌 Amac: DOCX import compatibility reportunu Tauri frontend icin typed DTO olarak tasir
// # 📌 Modul - FileType: View - Rust
// # Version: 0.2.0
// # Aciklama: Unsupported DOCX feature kodlarini ve imported Writer document DTO'sunu serializable hale getirir
// Bagimli Oldugu Katman: View

use serde::Serialize;
use turkuaz_office_format_adapters::{
    DocxCompatibilityReport, DocxUnsupportedFeature,
};

use crate::views::writer_dto::WriterDocumentDto;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WriterDocxUnsupportedFeatureDto {
    Table,
    Image,
    Numbering,
    Hyperlink,
    HeaderFooter,
    Comments,
    TrackedChanges,
    Fields,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterDocxCompatibilityDto {
    pub unsupported_features: Vec<WriterDocxUnsupportedFeatureDto>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterDocxImportDto {
    pub document: WriterDocumentDto,
    pub compatibility: WriterDocxCompatibilityDto,
}

impl From<DocxUnsupportedFeature> for WriterDocxUnsupportedFeatureDto {
    fn from(value: DocxUnsupportedFeature) -> Self {
        match value {
            DocxUnsupportedFeature::Table => Self::Table,
            DocxUnsupportedFeature::Image => Self::Image,
            DocxUnsupportedFeature::Numbering => Self::Numbering,
            DocxUnsupportedFeature::Hyperlink => Self::Hyperlink,
            DocxUnsupportedFeature::HeaderFooter => Self::HeaderFooter,
            DocxUnsupportedFeature::Comments => Self::Comments,
            DocxUnsupportedFeature::TrackedChanges => Self::TrackedChanges,
            DocxUnsupportedFeature::Fields => Self::Fields,
        }
    }
}

impl From<DocxCompatibilityReport> for WriterDocxCompatibilityDto {
    fn from(value: DocxCompatibilityReport) -> Self {
        Self {
            unsupported_features: value
                .unsupported_features
                .into_iter()
                .map(WriterDocxUnsupportedFeatureDto::from)
                .collect(),
        }
    }
}
