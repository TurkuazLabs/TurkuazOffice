// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/writer_selection_service.rs
// # 📌 Amac: Writer logical selection konumlarini dogrular ve canonical siraya getirir
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: NodeId ve Unicode scalar offset tabanli caret/range kontratini UI'dan bagimsiz tutar
// Bagimli Oldugu Katman: Service

use crate::services::writer_types::{Block, TextPosition, TextRange, WriterDocument};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectionError {
    ParagraphNotFound,
    RunNotFound,
    OffsetOutOfBounds,
    TableTextNotSupported,
}

pub struct WriterSelectionService;

impl WriterSelectionService {
    pub fn validate_position(
        document: &WriterDocument,
        position: &TextPosition,
    ) -> Result<(), SelectionError> {
        Self::position_order(document, position).map(|_| ())
    }

    pub fn normalize_range(
        document: &WriterDocument,
        range: &TextRange,
    ) -> Result<(TextPosition, TextPosition), SelectionError> {
        let anchor_order = Self::position_order(document, &range.anchor)?;
        let focus_order = Self::position_order(document, &range.focus)?;
        if anchor_order <= focus_order {
            Ok((range.anchor.clone(), range.focus.clone()))
        } else {
            Ok((range.focus.clone(), range.anchor.clone()))
        }
    }

    pub(crate) fn position_order(
        document: &WriterDocument,
        position: &TextPosition,
    ) -> Result<(usize, usize, usize, usize), SelectionError> {
        for (section_index, section) in document.sections.iter().enumerate() {
            for (block_index, block) in section.blocks.iter().enumerate() {
                let Block::Paragraph(paragraph) = block else {
                    continue;
                };
                if paragraph.id != position.paragraph_id {
                    continue;
                }
                for (run_index, run) in paragraph.runs.iter().enumerate() {
                    if run.id != position.run_id {
                        continue;
                    }
                    let char_count = run.text.chars().count();
                    if position.offset > char_count {
                        return Err(SelectionError::OffsetOutOfBounds);
                    }
                    return Ok((section_index, block_index, run_index, position.offset));
                }
                return Err(SelectionError::RunNotFound);
            }
        }
        Err(SelectionError::ParagraphNotFound)
    }
}
