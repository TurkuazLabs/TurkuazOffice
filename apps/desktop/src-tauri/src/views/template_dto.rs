// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/template_dto.rs
// # 📌 Amac: Writer template katalog metadata'sini frontend icin serializable typed DTO olarak tasir
// # 📌 Modul - FileType: View - Rust
// # Version: 0.2.0
// # Aciklama: Stable template id ile Language katmani name/description key'lerini camelCase IPC kontratina cevirir
// Bagimli Oldugu Katman: View

use serde::Serialize;
use turkuaz_office_writer::WriterTemplateSummary;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterTemplateDto {
    pub id: String,
    pub name_key: String,
    pub description_key: String,
}

impl From<WriterTemplateSummary> for WriterTemplateDto {
    fn from(value: WriterTemplateSummary) -> Self {
        Self {
            id: value.id,
            name_key: value.name_key,
            description_key: value.description_key,
        }
    }
}
