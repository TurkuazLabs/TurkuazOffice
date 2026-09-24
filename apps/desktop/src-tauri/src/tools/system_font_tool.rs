// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/tools/system_font_tool.rs
// # 📌 Amac: PDF export icin Windows/Linux sistem fontlarini requested family ve style ile cozer
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: fontdb ile font face bulur, fallback zincirini uygular ve format adapterine yalniz byte + face index verir
// Bagimli Oldugu Katman: Tool -> Config

use fontdb::{Database, Family, Query, Stretch, Style, Weight};
use turkuaz_office_format_adapters::{PdfFontData, PdfFontKey};

use crate::config::constants::PDF_FONT_FALLBACK_FAMILIES;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SystemFontError {
    FontNotFound,
    FontDataUnavailable,
}

pub struct SystemFontTool {
    database: Database,
}

impl SystemFontTool {
    pub fn new() -> Self {
        let mut database = Database::new();
        database.load_system_fonts();
        Self { database }
    }

    pub fn resolve(&self, key: &PdfFontKey) -> Result<PdfFontData, SystemFontError> {
        let weight = if key.bold {
            Weight::BOLD
        } else {
            Weight::NORMAL
        };
        let style = if key.italic {
            Style::Italic
        } else {
            Style::Normal
        };

        let mut families = Vec::with_capacity(PDF_FONT_FALLBACK_FAMILIES.len() + 1);
        families.push(key.family.as_str());
        families.extend(PDF_FONT_FALLBACK_FAMILIES.iter().copied());

        for family in families {
            let family_query = [Family::Name(family)];
            let id = self.database.query(&Query {
                families: &family_query,
                weight,
                stretch: Stretch::Normal,
                style,
            });
            let Some(id) = id else {
                continue;
            };
            let Some((bytes, face_index)) = self
                .database
                .with_face_data(id, |data, index| (data.to_vec(), index))
            else {
                continue;
            };
            return Ok(PdfFontData {
                key: key.clone(),
                bytes,
                face_index,
            });
        }

        Err(SystemFontError::FontNotFound)
    }
}

impl Default for SystemFontTool {
    fn default() -> Self {
        Self::new()
    }
}
