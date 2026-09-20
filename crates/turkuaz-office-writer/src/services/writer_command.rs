// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/writer_command.rs
// # 📌 Amac: Writer mutation komutlarini typed contract olarak tanimlar
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: View katmaninin belge agacini dogrudan mutate etmesini engelleyen command API'sidir
// Bagimli Oldugu Katman: Service

use crate::services::writer_selection_service::SelectionError;
use crate::services::writer_types::{
    CharacterStyle, CharacterStylePatch, NodeId, ParagraphStyle, ParagraphStylePatch,
    StyledTextRun, TextPosition, TextRange,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriterCommand {
    InsertText {
        position: TextPosition,
        text: String,
    },
    InsertStyledText {
        position: TextPosition,
        text: String,
        style: CharacterStyle,
    },
    DeleteRange {
        range: TextRange,
    },
    ReplaceRangeWithStyledRuns {
        range: TextRange,
        runs: Vec<StyledTextRun>,
    },
    SplitParagraph {
        position: TextPosition,
    },
    MergeParagraph {
        first_paragraph_id: NodeId,
        second_paragraph_id: NodeId,
    },
    SetCharacterStyle {
        run_id: NodeId,
        style: CharacterStyle,
    },
    ApplyCharacterStyle {
        range: TextRange,
        patch: CharacterStylePatch,
    },
    SetParagraphStyle {
        paragraph_id: NodeId,
        style: ParagraphStyle,
    },
    ApplyParagraphStyle {
        paragraph_id: NodeId,
        patch: ParagraphStylePatch,
    },
    InsertImage {
        after_paragraph_id: NodeId,
        asset_id: String,
        alt_text: String,
        width_twips: Option<u32>,
        height_twips: Option<u32>,
    },
    InsertImageData {
        after_paragraph_id: NodeId,
        media_type: String,
        data: Vec<u8>,
        alt_text: String,
        width_twips: Option<u32>,
        height_twips: Option<u32>,
    },
    InsertTable {
        after_paragraph_id: NodeId,
        rows: usize,
        columns: usize,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriterCommandError {
    Selection(SelectionError),
    ParagraphNotFound,
    RunNotFound,
    ParagraphsNotAdjacent,
    CrossSectionDeleteNotSupported,
    CrossParagraphStyleNotSupported,
    CrossParagraphFragmentReplaceNotSupported,
    EmptyCommand,
    InvalidTableSize,
    InvalidCharacterStyle,
    InvalidAsset,
    AssetNotFound,
    AssetLimitExceeded,
}

impl From<SelectionError> for WriterCommandError {
    fn from(value: SelectionError) -> Self {
        Self::Selection(value)
    }
}
