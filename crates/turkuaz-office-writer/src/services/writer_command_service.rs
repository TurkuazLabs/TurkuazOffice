// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/writer_command_service.rs
// # 📌 Amac: Writer command is kurallarini canonical belge agaci uzerinde uygular
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Insert/delete/split/merge/range-style/image/table mutationlarini tek Service sinirinda toplar
// Bagimli Oldugu Katman: Service -> Tool

use crate::config::constants::{
    MAX_FONT_FAMILY_LENGTH, MAX_FONT_SIZE_HALF_POINTS, MAX_TABLE_DIMENSION,
    MIN_FONT_SIZE_HALF_POINTS, MIN_TABLE_DIMENSION,
};
use crate::services::writer_asset_service::{WriterAssetError, WriterAssetService};
use crate::services::writer_command::{WriterCommand, WriterCommandError};
use crate::services::writer_selection_service::WriterSelectionService;
use crate::services::writer_types::{
    Block, CharacterStyle, CharacterStylePatch, ImageBlock, Paragraph, ParagraphStyle,
    ParagraphStylePatch, StyledTextRun, Table, TableCell, TableRow, TextRun, WriterAsset,
    WriterDocument,
};
use crate::tools::writer_id_tool::WriterIdTool;

pub struct WriterCommandService;

impl WriterCommandService {
    pub fn apply<I>(
        document: &mut WriterDocument,
        command: &WriterCommand,
        id_tool: &I,
    ) -> Result<(), WriterCommandError>
    where
        I: WriterIdTool,
    {
        match command {
            WriterCommand::InsertText { position, text } => {
                if text.is_empty() {
                    return Err(WriterCommandError::EmptyCommand);
                }
                Self::insert_text(document, position, text)?;
            }
            WriterCommand::InsertStyledText {
                position,
                text,
                style,
            } => {
                if text.is_empty() {
                    return Err(WriterCommandError::EmptyCommand);
                }
                Self::validate_character_style(style)?;
                Self::insert_styled_text(document, position, text, style, id_tool)?;
            }
            WriterCommand::DeleteRange { range } => Self::delete_range(document, range)?,
            WriterCommand::ReplaceRangeWithStyledRuns { range, runs } => {
                Self::replace_range_with_styled_runs(document, range, runs, id_tool)?;
            }
            WriterCommand::SplitParagraph { position } => {
                Self::split_paragraph(document, position, id_tool)?;
            }
            WriterCommand::MergeParagraph {
                first_paragraph_id,
                second_paragraph_id,
            } => Self::merge_paragraph(document, first_paragraph_id, second_paragraph_id)?,
            WriterCommand::SetCharacterStyle { run_id, style } => {
                Self::set_character_style(document, run_id, style.clone())?;
            }
            WriterCommand::ApplyCharacterStyle { range, patch } => {
                Self::apply_character_style(document, range, patch, id_tool)?;
            }
            WriterCommand::SetParagraphStyle {
                paragraph_id,
                style,
            } => Self::set_paragraph_style(document, paragraph_id, style.clone())?,
            WriterCommand::ApplyParagraphStyle {
                paragraph_id,
                patch,
            } => Self::apply_paragraph_style(document, paragraph_id, patch)?,
            WriterCommand::InsertImage {
                after_paragraph_id,
                asset_id,
                alt_text,
                width_twips,
                height_twips,
            } => Self::insert_image(
                document,
                after_paragraph_id,
                asset_id,
                alt_text,
                *width_twips,
                *height_twips,
                id_tool,
            )?,
            WriterCommand::InsertImageData {
                after_paragraph_id,
                media_type,
                data,
                alt_text,
                width_twips,
                height_twips,
            } => Self::insert_image_data(
                document,
                after_paragraph_id,
                media_type,
                data,
                alt_text,
                *width_twips,
                *height_twips,
                id_tool,
            )?,
            WriterCommand::InsertTable {
                after_paragraph_id,
                rows,
                columns,
            } => Self::insert_table(document, after_paragraph_id, *rows, *columns, id_tool)?,
        }

        document.revision = document.revision.saturating_add(1);
        Ok(())
    }

    fn insert_text(
        document: &mut WriterDocument,
        position: &crate::services::writer_types::TextPosition,
        text: &str,
    ) -> Result<(), WriterCommandError> {
        WriterSelectionService::validate_position(document, position)?;
        let run = Self::find_run_mut(document, &position.paragraph_id, &position.run_id)?;
        let byte_index = Self::char_offset_to_byte(&run.text, position.offset)
            .ok_or(WriterCommandError::RunNotFound)?;
        run.text.insert_str(byte_index, text);
        Ok(())
    }

    fn insert_styled_text<I>(
        document: &mut WriterDocument,
        position: &crate::services::writer_types::TextPosition,
        text: &str,
        style: &CharacterStyle,
        id_tool: &I,
    ) -> Result<(), WriterCommandError>
    where
        I: WriterIdTool,
    {
        WriterSelectionService::validate_position(document, position)?;
        let paragraph = Self::find_paragraph_mut(document, &position.paragraph_id)?;
        let run_index = paragraph
            .runs
            .iter()
            .position(|run| run.id == position.run_id)
            .ok_or(WriterCommandError::RunNotFound)?;
        let target = paragraph
            .runs
            .get(run_index)
            .cloned()
            .ok_or(WriterCommandError::RunNotFound)?;

        if target.style == *style {
            let run = paragraph
                .runs
                .get_mut(run_index)
                .ok_or(WriterCommandError::RunNotFound)?;
            let byte_index = Self::char_offset_to_byte(&run.text, position.offset)
                .ok_or(WriterCommandError::RunNotFound)?;
            run.text.insert_str(byte_index, text);
            return Ok(());
        }

        let split_byte = Self::char_offset_to_byte(&target.text, position.offset)
            .ok_or(WriterCommandError::RunNotFound)?;
        let prefix = target.text[..split_byte].to_owned();
        let suffix = target.text[split_byte..].to_owned();
        let has_prefix = !prefix.is_empty();
        let mut rebuilt = Vec::with_capacity(paragraph.runs.len().saturating_add(2));
        rebuilt.extend_from_slice(&paragraph.runs[..run_index]);

        if has_prefix {
            rebuilt.push(TextRun {
                id: target.id.clone(),
                text: prefix,
                style: target.style.clone(),
            });
        }

        rebuilt.push(TextRun {
            id: if has_prefix {
                id_tool.next_node_id()
            } else {
                target.id.clone()
            },
            text: text.to_owned(),
            style: style.clone(),
        });

        if !suffix.is_empty() {
            rebuilt.push(TextRun {
                id: id_tool.next_node_id(),
                text: suffix,
                style: target.style,
            });
        }

        rebuilt.extend_from_slice(&paragraph.runs[(run_index + 1)..]);
        paragraph.runs = Self::merge_adjacent_runs(rebuilt);
        Ok(())
    }

    fn delete_range(
        document: &mut WriterDocument,
        range: &crate::services::writer_types::TextRange,
    ) -> Result<(), WriterCommandError> {
        let (start, end) = WriterSelectionService::normalize_range(document, range)?;
        if start == end {
            return Err(WriterCommandError::EmptyCommand);
        }
        let start_order = WriterSelectionService::position_order(document, &start)?;
        let end_order = WriterSelectionService::position_order(document, &end)?;
        if start_order.0 != end_order.0 {
            return Err(WriterCommandError::CrossSectionDeleteNotSupported);
        }

        if start.paragraph_id == end.paragraph_id {
            return Self::delete_within_paragraph(document, &start, &end);
        }

        let section_index = start_order.0;
        let start_block_index = start_order.1;
        let end_block_index = end_order.1;
        let section = document
            .sections
            .get_mut(section_index)
            .ok_or(WriterCommandError::ParagraphNotFound)?;

        let mut start_paragraph = match section.blocks.get(start_block_index).cloned() {
            Some(Block::Paragraph(paragraph)) => paragraph,
            _ => return Err(WriterCommandError::ParagraphNotFound),
        };
        let end_paragraph = match section.blocks.get(end_block_index).cloned() {
            Some(Block::Paragraph(paragraph)) => paragraph,
            _ => return Err(WriterCommandError::ParagraphNotFound),
        };

        Self::truncate_paragraph_after(&mut start_paragraph, &start)?;
        let mut suffix = Self::paragraph_suffix_after(&end_paragraph, &end)?;
        start_paragraph.runs.append(&mut suffix);

        section.blocks[start_block_index] = Block::Paragraph(start_paragraph);
        drop(
            section
                .blocks
                .drain((start_block_index + 1)..=end_block_index),
        );
        Ok(())
    }

    fn delete_within_paragraph(
        document: &mut WriterDocument,
        start: &crate::services::writer_types::TextPosition,
        end: &crate::services::writer_types::TextPosition,
    ) -> Result<(), WriterCommandError> {
        let paragraph = Self::find_paragraph_mut(document, &start.paragraph_id)?;
        let start_run_index = paragraph
            .runs
            .iter()
            .position(|run| run.id == start.run_id)
            .ok_or(WriterCommandError::RunNotFound)?;
        let end_run_index = paragraph
            .runs
            .iter()
            .position(|run| run.id == end.run_id)
            .ok_or(WriterCommandError::RunNotFound)?;

        if start_run_index == end_run_index {
            let run = paragraph
                .runs
                .get_mut(start_run_index)
                .ok_or(WriterCommandError::RunNotFound)?;
            let start_byte = Self::char_offset_to_byte(&run.text, start.offset)
                .ok_or(WriterCommandError::RunNotFound)?;
            let end_byte = Self::char_offset_to_byte(&run.text, end.offset)
                .ok_or(WriterCommandError::RunNotFound)?;
            run.text.replace_range(start_byte..end_byte, "");
            return Ok(());
        }

        {
            let start_run = paragraph
                .runs
                .get_mut(start_run_index)
                .ok_or(WriterCommandError::RunNotFound)?;
            let start_byte = Self::char_offset_to_byte(&start_run.text, start.offset)
                .ok_or(WriterCommandError::RunNotFound)?;
            start_run.text.truncate(start_byte);
        }

        let end_suffix = {
            let end_run = paragraph
                .runs
                .get(end_run_index)
                .ok_or(WriterCommandError::RunNotFound)?;
            let end_byte = Self::char_offset_to_byte(&end_run.text, end.offset)
                .ok_or(WriterCommandError::RunNotFound)?;
            let mut suffix = end_run.clone();
            suffix.text = end_run.text[end_byte..].to_owned();
            suffix
        };

        drop(paragraph.runs.drain((start_run_index + 1)..=end_run_index));
        if !end_suffix.text.is_empty() {
            paragraph.runs.insert(start_run_index + 1, end_suffix);
        }
        Ok(())
    }

    fn replace_range_with_styled_runs<I>(
        document: &mut WriterDocument,
        range: &crate::services::writer_types::TextRange,
        runs: &[StyledTextRun],
        id_tool: &I,
    ) -> Result<(), WriterCommandError>
    where
        I: WriterIdTool,
    {
        let (start, end) = WriterSelectionService::normalize_range(document, range)?;
        if start.paragraph_id != end.paragraph_id {
            return Err(WriterCommandError::CrossParagraphFragmentReplaceNotSupported);
        }

        for run in runs {
            if !run.text.is_empty() {
                Self::validate_character_style(&run.style)?;
            }
        }

        if start == end && runs.iter().all(|run| run.text.is_empty()) {
            return Err(WriterCommandError::EmptyCommand);
        }

        let paragraph = Self::find_paragraph_mut(document, &start.paragraph_id)?;
        let start_offset = Self::paragraph_offset(paragraph, &start.run_id, start.offset)?;
        let end_offset = Self::paragraph_offset(paragraph, &end.run_id, end.offset)?;
        let original_runs = std::mem::take(&mut paragraph.runs);

        let fallback_style = original_runs
            .iter()
            .scan(0usize, |cursor, run| {
                let run_start = *cursor;
                let run_end = run_start.saturating_add(run.text.chars().count());
                *cursor = run_end;
                Some((run_start, run_end, run.style.clone()))
            })
            .find(|(run_start, run_end, _)| start_offset >= *run_start && start_offset <= *run_end)
            .map(|(_, _, style)| style)
            .or_else(|| original_runs.last().map(|run| run.style.clone()))
            .unwrap_or_default();

        let mut rebuilt = Vec::with_capacity(
            original_runs
                .len()
                .saturating_add(runs.len())
                .saturating_add(2),
        );
        let mut cursor = 0usize;

        for run in &original_runs {
            let run_length = run.text.chars().count();
            let run_start = cursor;
            let run_end = cursor.saturating_add(run_length);
            cursor = run_end;

            if run_start >= start_offset {
                break;
            }

            let local_end = start_offset.saturating_sub(run_start).min(run_length);
            if local_end == 0 {
                continue;
            }
            let text =
                Self::char_slice(&run.text, 0, local_end).ok_or(WriterCommandError::RunNotFound)?;
            if !text.is_empty() {
                rebuilt.push(TextRun {
                    id: run.id.clone(),
                    text,
                    style: run.style.clone(),
                });
            }
        }

        for fragment_run in runs {
            if fragment_run.text.is_empty() {
                continue;
            }
            rebuilt.push(TextRun {
                id: id_tool.next_node_id(),
                text: fragment_run.text.clone(),
                style: fragment_run.style.clone(),
            });
        }

        cursor = 0;
        for run in &original_runs {
            let run_length = run.text.chars().count();
            let run_start = cursor;
            let run_end = cursor.saturating_add(run_length);
            cursor = run_end;

            if run_end <= end_offset {
                continue;
            }

            let local_start = end_offset.saturating_sub(run_start).min(run_length);
            let text = Self::char_slice(&run.text, local_start, run_length)
                .ok_or(WriterCommandError::RunNotFound)?;
            if !text.is_empty() {
                rebuilt.push(TextRun {
                    id: id_tool.next_node_id(),
                    text,
                    style: run.style.clone(),
                });
            }
        }

        let mut merged = Self::merge_adjacent_runs(rebuilt);
        if merged.is_empty() {
            merged.push(TextRun {
                id: original_runs
                    .first()
                    .map(|run| run.id.clone())
                    .unwrap_or_else(|| id_tool.next_node_id()),
                text: String::new(),
                style: fallback_style,
            });
        }

        paragraph.runs = merged;
        Ok(())
    }

    fn split_paragraph<I>(
        document: &mut WriterDocument,
        position: &crate::services::writer_types::TextPosition,
        id_tool: &I,
    ) -> Result<(), WriterCommandError>
    where
        I: WriterIdTool,
    {
        WriterSelectionService::validate_position(document, position)?;
        let (section_index, block_index, run_index, _) =
            WriterSelectionService::position_order(document, position)?;
        let section = document
            .sections
            .get_mut(section_index)
            .ok_or(WriterCommandError::ParagraphNotFound)?;
        let paragraph = match section.blocks.get(block_index).cloned() {
            Some(Block::Paragraph(paragraph)) => paragraph,
            _ => return Err(WriterCommandError::ParagraphNotFound),
        };

        let target_run = paragraph
            .runs
            .get(run_index)
            .ok_or(WriterCommandError::RunNotFound)?;
        let split_byte = Self::char_offset_to_byte(&target_run.text, position.offset)
            .ok_or(WriterCommandError::RunNotFound)?;

        let mut left_runs = paragraph.runs[..run_index].to_vec();
        let mut left_target = target_run.clone();
        left_target.text = target_run.text[..split_byte].to_owned();
        left_runs.push(left_target);

        let mut right_runs = Vec::new();
        let right_text = target_run.text[split_byte..].to_owned();
        let mut right_target = target_run.clone();
        right_target.id = id_tool.next_node_id();
        right_target.text = right_text;
        right_runs.push(right_target);
        right_runs.extend_from_slice(&paragraph.runs[(run_index + 1)..]);

        let mut left_paragraph = paragraph.clone();
        left_paragraph.runs = left_runs;

        let mut right_paragraph = Paragraph {
            id: id_tool.next_node_id(),
            style: paragraph.style,
            runs: right_runs,
        };

        section.blocks[block_index] = Block::Paragraph(left_paragraph);
        section
            .blocks
            .insert(block_index + 1, Block::Paragraph(right_paragraph));
        Ok(())
    }

    fn merge_paragraph(
        document: &mut WriterDocument,
        first_id: &crate::services::writer_types::NodeId,
        second_id: &crate::services::writer_types::NodeId,
    ) -> Result<(), WriterCommandError> {
        for section in &mut document.sections {
            for index in 0..section.blocks.len().saturating_sub(1) {
                let first_matches = matches!(
                    section.blocks.get(index),
                    Some(Block::Paragraph(paragraph)) if paragraph.id == *first_id
                );
                let second_matches = matches!(
                    section.blocks.get(index + 1),
                    Some(Block::Paragraph(paragraph)) if paragraph.id == *second_id
                );
                if first_matches && second_matches {
                    let second = match section.blocks.remove(index + 1) {
                        Block::Paragraph(paragraph) => paragraph,
                        _ => return Err(WriterCommandError::ParagraphNotFound),
                    };
                    let first = match section.blocks.get_mut(index) {
                        Some(Block::Paragraph(paragraph)) => paragraph,
                        _ => return Err(WriterCommandError::ParagraphNotFound),
                    };
                    first.runs.extend(second.runs);
                    return Ok(());
                }
            }
        }
        Err(WriterCommandError::ParagraphsNotAdjacent)
    }

    fn apply_character_style<I>(
        document: &mut WriterDocument,
        range: &crate::services::writer_types::TextRange,
        patch: &CharacterStylePatch,
        id_tool: &I,
    ) -> Result<(), WriterCommandError>
    where
        I: WriterIdTool,
    {
        if patch.is_empty() {
            return Err(WriterCommandError::EmptyCommand);
        }
        Self::validate_character_style_patch(patch)?;

        let (start, end) = WriterSelectionService::normalize_range(document, range)?;
        if start == end {
            return Err(WriterCommandError::EmptyCommand);
        }
        if start.paragraph_id != end.paragraph_id {
            return Err(WriterCommandError::CrossParagraphStyleNotSupported);
        }

        let paragraph = Self::find_paragraph_mut(document, &start.paragraph_id)?;
        let start_offset = Self::paragraph_offset(paragraph, &start.run_id, start.offset)?;
        let end_offset = Self::paragraph_offset(paragraph, &end.run_id, end.offset)?;
        let original_runs = std::mem::take(&mut paragraph.runs);
        let mut rebuilt = Vec::with_capacity(original_runs.len().saturating_add(2));
        let mut cursor = 0usize;

        for run in original_runs {
            let run_length = run.text.chars().count();
            let run_start = cursor;
            let run_end = cursor.saturating_add(run_length);
            cursor = run_end;

            if end_offset <= run_start || start_offset >= run_end || run_length == 0 {
                rebuilt.push(run);
                continue;
            }

            let local_start = start_offset.saturating_sub(run_start).min(run_length);
            let local_end = end_offset.saturating_sub(run_start).min(run_length);
            let prefix = Self::char_slice(&run.text, 0, local_start)
                .ok_or(WriterCommandError::RunNotFound)?;
            let selected = Self::char_slice(&run.text, local_start, local_end)
                .ok_or(WriterCommandError::RunNotFound)?;
            let suffix = Self::char_slice(&run.text, local_end, run_length)
                .ok_or(WriterCommandError::RunNotFound)?;
            let base_style = run.style.clone();

            if !prefix.is_empty() {
                rebuilt.push(TextRun {
                    id: run.id.clone(),
                    text: prefix,
                    style: base_style.clone(),
                });
            }

            if !selected.is_empty() {
                let mut selected_style = base_style.clone();
                patch.apply_to(&mut selected_style);
                rebuilt.push(TextRun {
                    id: if local_start == 0 {
                        run.id.clone()
                    } else {
                        id_tool.next_node_id()
                    },
                    text: selected,
                    style: selected_style,
                });
            }

            if !suffix.is_empty() {
                rebuilt.push(TextRun {
                    id: id_tool.next_node_id(),
                    text: suffix,
                    style: base_style,
                });
            }
        }

        paragraph.runs = Self::merge_adjacent_runs(rebuilt);
        Ok(())
    }

    fn set_character_style(
        document: &mut WriterDocument,
        run_id: &crate::services::writer_types::NodeId,
        style: CharacterStyle,
    ) -> Result<(), WriterCommandError> {
        Self::validate_character_style(&style)?;
        for section in &mut document.sections {
            for block in &mut section.blocks {
                if let Block::Paragraph(paragraph) = block {
                    if let Some(run) = paragraph.runs.iter_mut().find(|run| run.id == *run_id) {
                        run.style = style;
                        return Ok(());
                    }
                }
            }
        }
        Err(WriterCommandError::RunNotFound)
    }

    fn set_paragraph_style(
        document: &mut WriterDocument,
        paragraph_id: &crate::services::writer_types::NodeId,
        style: ParagraphStyle,
    ) -> Result<(), WriterCommandError> {
        let paragraph = Self::find_paragraph_mut(document, paragraph_id)?;
        paragraph.style = style;
        Ok(())
    }

    fn apply_paragraph_style(
        document: &mut WriterDocument,
        paragraph_id: &crate::services::writer_types::NodeId,
        patch: &ParagraphStylePatch,
    ) -> Result<(), WriterCommandError> {
        if patch.is_empty() {
            return Err(WriterCommandError::EmptyCommand);
        }
        let paragraph = Self::find_paragraph_mut(document, paragraph_id)?;
        patch.apply_to(&mut paragraph.style);
        Ok(())
    }

    fn validate_character_style(style: &CharacterStyle) -> Result<(), WriterCommandError> {
        let family = style.font_family.trim();
        if family.is_empty() || family.chars().count() > MAX_FONT_FAMILY_LENGTH {
            return Err(WriterCommandError::InvalidCharacterStyle);
        }
        if !(MIN_FONT_SIZE_HALF_POINTS..=MAX_FONT_SIZE_HALF_POINTS)
            .contains(&style.font_size_half_points)
        {
            return Err(WriterCommandError::InvalidCharacterStyle);
        }
        Ok(())
    }

    fn validate_character_style_patch(
        patch: &CharacterStylePatch,
    ) -> Result<(), WriterCommandError> {
        if let Some(family) = &patch.font_family {
            let family = family.trim();
            if family.is_empty() || family.chars().count() > MAX_FONT_FAMILY_LENGTH {
                return Err(WriterCommandError::InvalidCharacterStyle);
            }
        }
        if let Some(size) = patch.font_size_half_points {
            if !(MIN_FONT_SIZE_HALF_POINTS..=MAX_FONT_SIZE_HALF_POINTS).contains(&size) {
                return Err(WriterCommandError::InvalidCharacterStyle);
            }
        }
        Ok(())
    }

    fn insert_image<I>(
        document: &mut WriterDocument,
        after_paragraph_id: &crate::services::writer_types::NodeId,
        asset_id: &str,
        alt_text: &str,
        width_twips: Option<u32>,
        height_twips: Option<u32>,
        id_tool: &I,
    ) -> Result<(), WriterCommandError>
    where
        I: WriterIdTool,
    {
        let (section_index, block_index) =
            Self::find_paragraph_block(document, after_paragraph_id)?;
        if !document.assets.iter().any(|asset| asset.id == asset_id) {
            return Err(WriterCommandError::AssetNotFound);
        }
        let image = ImageBlock {
            id: id_tool.next_node_id(),
            asset_id: asset_id.to_owned(),
            alt_text: alt_text.to_owned(),
            width_twips,
            height_twips,
        };
        document.sections[section_index]
            .blocks
            .insert(block_index + 1, Block::Image(image));
        Ok(())
    }


    fn insert_image_data<I>(
        document: &mut WriterDocument,
        after_paragraph_id: &crate::services::writer_types::NodeId,
        media_type: &str,
        data: &[u8],
        alt_text: &str,
        width_twips: Option<u32>,
        height_twips: Option<u32>,
        id_tool: &I,
    ) -> Result<(), WriterCommandError>
    where
        I: WriterIdTool,
    {
        if document.assets.len() >= crate::config::constants::MAX_WRITER_ASSETS {
            return Err(WriterCommandError::AssetLimitExceeded);
        }
        WriterAssetService::validate_image_payload(media_type, data)
            .map_err(Self::map_asset_error)?;
        let (section_index, block_index) =
            Self::find_paragraph_block(document, after_paragraph_id)?;

        let asset_id = (0..=crate::config::constants::MAX_WRITER_ASSETS)
            .map(|_| id_tool.next_asset_id())
            .find(|candidate| !document.assets.iter().any(|asset| asset.id == *candidate))
            .ok_or(WriterCommandError::AssetLimitExceeded)?;

        document.assets.push(WriterAsset {
            id: asset_id.clone(),
            media_type: media_type.to_owned(),
            bytes: data.to_vec(),
        });
        document.sections[section_index].blocks.insert(
            block_index + 1,
            Block::Image(ImageBlock {
                id: id_tool.next_node_id(),
                asset_id,
                alt_text: alt_text.to_owned(),
                width_twips,
                height_twips,
            }),
        );
        Ok(())
    }

    fn map_asset_error(error: WriterAssetError) -> WriterCommandError {
        match error {
            WriterAssetError::TooManyAssets => WriterCommandError::AssetLimitExceeded,
            WriterAssetError::ReferencedAssetMissing => WriterCommandError::AssetNotFound,
            _ => WriterCommandError::InvalidAsset,
        }
    }

    fn insert_table<I>(
        document: &mut WriterDocument,
        after_paragraph_id: &crate::services::writer_types::NodeId,
        rows: usize,
        columns: usize,
        id_tool: &I,
    ) -> Result<(), WriterCommandError>
    where
        I: WriterIdTool,
    {
        if !(MIN_TABLE_DIMENSION..=MAX_TABLE_DIMENSION).contains(&rows)
            || !(MIN_TABLE_DIMENSION..=MAX_TABLE_DIMENSION).contains(&columns)
        {
            return Err(WriterCommandError::InvalidTableSize);
        }
        let (section_index, block_index) =
            Self::find_paragraph_block(document, after_paragraph_id)?;
        let mut table_rows = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut cells = Vec::with_capacity(columns);
            for _ in 0..columns {
                let paragraph = Paragraph {
                    id: id_tool.next_node_id(),
                    style: ParagraphStyle::default(),
                    runs: vec![TextRun {
                        id: id_tool.next_node_id(),
                        text: String::new(),
                        style: CharacterStyle::default(),
                    }],
                };
                cells.push(TableCell {
                    id: id_tool.next_node_id(),
                    paragraphs: vec![paragraph],
                });
            }
            table_rows.push(TableRow {
                id: id_tool.next_node_id(),
                cells,
            });
        }
        let table = Table {
            id: id_tool.next_node_id(),
            rows: table_rows,
        };
        document.sections[section_index]
            .blocks
            .insert(block_index + 1, Block::Table(table));
        Ok(())
    }

    fn truncate_paragraph_after(
        paragraph: &mut Paragraph,
        position: &crate::services::writer_types::TextPosition,
    ) -> Result<(), WriterCommandError> {
        let run_index = paragraph
            .runs
            .iter()
            .position(|run| run.id == position.run_id)
            .ok_or(WriterCommandError::RunNotFound)?;
        let run = paragraph
            .runs
            .get_mut(run_index)
            .ok_or(WriterCommandError::RunNotFound)?;
        let byte_index = Self::char_offset_to_byte(&run.text, position.offset)
            .ok_or(WriterCommandError::RunNotFound)?;
        run.text.truncate(byte_index);
        paragraph.runs.truncate(run_index + 1);
        Ok(())
    }

    fn paragraph_suffix_after(
        paragraph: &Paragraph,
        position: &crate::services::writer_types::TextPosition,
    ) -> Result<Vec<TextRun>, WriterCommandError> {
        let run_index = paragraph
            .runs
            .iter()
            .position(|run| run.id == position.run_id)
            .ok_or(WriterCommandError::RunNotFound)?;
        let run = paragraph
            .runs
            .get(run_index)
            .ok_or(WriterCommandError::RunNotFound)?;
        let byte_index = Self::char_offset_to_byte(&run.text, position.offset)
            .ok_or(WriterCommandError::RunNotFound)?;
        let mut result = Vec::new();
        let mut first = run.clone();
        first.text = run.text[byte_index..].to_owned();
        if !first.text.is_empty() {
            result.push(first);
        }
        result.extend_from_slice(&paragraph.runs[(run_index + 1)..]);
        Ok(result)
    }

    fn paragraph_offset(
        paragraph: &Paragraph,
        run_id: &crate::services::writer_types::NodeId,
        run_offset: usize,
    ) -> Result<usize, WriterCommandError> {
        let mut offset = 0usize;
        for run in &paragraph.runs {
            let length = run.text.chars().count();
            if run.id == *run_id {
                if run_offset > length {
                    return Err(WriterCommandError::RunNotFound);
                }
                return Ok(offset.saturating_add(run_offset));
            }
            offset = offset.saturating_add(length);
        }
        Err(WriterCommandError::RunNotFound)
    }

    fn char_slice(text: &str, start: usize, end: usize) -> Option<String> {
        if start > end || end > text.chars().count() {
            return None;
        }
        Some(text.chars().skip(start).take(end - start).collect())
    }

    fn merge_adjacent_runs(runs: Vec<TextRun>) -> Vec<TextRun> {
        let mut merged: Vec<TextRun> = Vec::with_capacity(runs.len());
        for run in runs.into_iter().filter(|run| !run.text.is_empty()) {
            if let Some(previous) = merged.last_mut() {
                if previous.style == run.style {
                    previous.text.push_str(&run.text);
                    continue;
                }
            }
            merged.push(run);
        }
        merged
    }

    fn find_run_mut<'a>(
        document: &'a mut WriterDocument,
        paragraph_id: &crate::services::writer_types::NodeId,
        run_id: &crate::services::writer_types::NodeId,
    ) -> Result<&'a mut TextRun, WriterCommandError> {
        let paragraph = Self::find_paragraph_mut(document, paragraph_id)?;
        paragraph
            .runs
            .iter_mut()
            .find(|run| run.id == *run_id)
            .ok_or(WriterCommandError::RunNotFound)
    }

    fn find_paragraph_mut<'a>(
        document: &'a mut WriterDocument,
        paragraph_id: &crate::services::writer_types::NodeId,
    ) -> Result<&'a mut Paragraph, WriterCommandError> {
        for section in &mut document.sections {
            for block in &mut section.blocks {
                if let Block::Paragraph(paragraph) = block {
                    if paragraph.id == *paragraph_id {
                        return Ok(paragraph);
                    }
                }
            }
        }
        Err(WriterCommandError::ParagraphNotFound)
    }

    fn find_paragraph_block(
        document: &WriterDocument,
        paragraph_id: &crate::services::writer_types::NodeId,
    ) -> Result<(usize, usize), WriterCommandError> {
        for (section_index, section) in document.sections.iter().enumerate() {
            for (block_index, block) in section.blocks.iter().enumerate() {
                if matches!(block, Block::Paragraph(paragraph) if paragraph.id == *paragraph_id) {
                    return Ok((section_index, block_index));
                }
            }
        }
        Err(WriterCommandError::ParagraphNotFound)
    }

    fn char_offset_to_byte(text: &str, offset: usize) -> Option<usize> {
        let char_count = text.chars().count();
        if offset > char_count {
            return None;
        }
        if offset == char_count {
            return Some(text.len());
        }
        text.char_indices().nth(offset).map(|(index, _)| index)
    }
}
