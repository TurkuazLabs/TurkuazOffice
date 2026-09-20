// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/writer_document_factory_service.rs
// # 📌 Amac: Yeni Writer belgelerinin canonical ilk agacini Service katmaninda olusturur
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Bos Writer belgesini section, paragraph ve text run ile deterministik kurar
// Bagimli Oldugu Katman: Service -> Tool

use turkuaz_office_core::DocumentSchemaVersion;
use turkuaz_office_core::config::constants::DEFAULT_DOCUMENT_TITLE;

use crate::config::constants::DEFAULT_FONT_FAMILY;
use crate::services::writer_types::{
    Block, CharacterStyle, PageSettings, Paragraph, ParagraphStyle, Section, TextRun,
    WriterDocument,
};
use crate::tools::writer_id_tool::WriterIdTool;

pub struct WriterDocumentFactoryService;

impl WriterDocumentFactoryService {
    pub fn create<I>(id_tool: &I, title: impl Into<String>) -> WriterDocument
    where
        I: WriterIdTool,
    {
        let run = TextRun {
            id: id_tool.next_node_id(),
            text: String::new(),
            style: CharacterStyle {
                font_family: DEFAULT_FONT_FAMILY.to_owned(),
                ..CharacterStyle::default()
            },
        };
        let paragraph = Paragraph {
            id: id_tool.next_node_id(),
            style: ParagraphStyle::default(),
            runs: vec![run],
        };
        let section = Section {
            id: id_tool.next_node_id(),
            page_settings: PageSettings::default(),
            blocks: vec![Block::Paragraph(paragraph)],
        };

        let raw_title = title.into();
        let normalized_title = raw_title.trim();
        let title = if normalized_title.is_empty() {
            DEFAULT_DOCUMENT_TITLE.to_owned()
        } else {
            normalized_title.to_owned()
        };

        WriterDocument {
            id: id_tool.next_document_id(),
            title,
            schema_version: DocumentSchemaVersion::current(),
            revision: 0,
            sections: vec![section],
            assets: Vec::new(),
        }
    }
}
