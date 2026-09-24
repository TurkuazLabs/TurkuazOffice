// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/services/writer_desktop_service.rs
// # 📌 Amac: Desktop Writer requestlerini canonical Writer command, storage ve recovery servislerine cevirir
// # 📌 Modul - FileType: Service - Rust
// # Version: 0.2.0
// # Aciklama: Edit, local TKO Open/Save ve autosave recovery akislarini domain Controller uzerinden koordine eder
// Bagimli Oldugu Katman: Service -> Controller -> Service -> Repo -> Tool

use std::path::PathBuf;

use turkuaz_office_format_adapters::DocxCompatibilityReport;
use turkuaz_office_writer::{
    CharacterStyle, CharacterStylePatch, InMemoryWriterDocumentRepository, NodeId,
    ParagraphStylePatch, SequentialWriterIdTool, StyledTextRun, TextAlignment, TextPosition,
    TextRange, WriterAsset, WriterCommand, WriterController, WriterDocumentView,
    WriterEditorError,
};

use crate::config::constants::{
    ERROR_ASSET_NOT_FOUND, ERROR_INVALID_OFFSET, ERROR_PARAGRAPH_NOT_FOUND,
};
use crate::services::writer_docx_service::WriterDocxService;
use crate::services::writer_pdf_service::WriterPdfService;
use crate::services::writer_file_session_service::{
    WriterFileSessionService, WriterFileSessionStatus,
};
use crate::services::writer_recovery_service::{
    RecoveryComparison, RecoverySnapshot, WriterRecoveryService,
};
use crate::services::writer_storage_service::WriterStorageService;
use crate::tools::recovery_path_tool::RecoveryPathTool;
use crate::views::error_dto::DesktopErrorDto;

pub struct WriterDesktopService {
    controller: WriterController<InMemoryWriterDocumentRepository, SequentialWriterIdTool>,
    recovery_service: WriterRecoveryService,
    file_session_service: WriterFileSessionService,
}

impl WriterDesktopService {
    pub fn new() -> Self {
        Self::with_recovery_root(RecoveryPathTool::recovery_directory())
    }

    pub fn with_recovery_root(recovery_root: PathBuf) -> Self {
        let repository = InMemoryWriterDocumentRepository::new();
        let id_tool = SequentialWriterIdTool::new();
        let editor_service = turkuaz_office_writer::WriterEditorService::new(repository, id_tool);
        Self {
            controller: WriterController::new(editor_service),
            recovery_service: WriterRecoveryService::new(recovery_root),
            file_session_service: WriterFileSessionService::new(),
        }
    }

    pub fn create_document(&mut self) -> WriterDocumentView {
        self.file_session_service.reset_untracked();
        self.controller.create("")
    }

    pub fn import_docx(
        &mut self,
        path: &str,
    ) -> Result<(WriterDocumentView, DocxCompatibilityReport), DesktopErrorDto> {
        let imported = WriterDocxService::import(path).map_err(DesktopErrorDto::from)?;
        let compatibility = imported.compatibility;
        let view = self.controller.load_external(imported.document);
        self.file_session_service.reset_untracked();
        Ok((view, compatibility))
    }

    pub fn export_docx(
        &self,
        document_id: &str,
        path: &str,
    ) -> Result<String, DesktopErrorDto> {
        let document = self
            .controller
            .snapshot(document_id)
            .ok_or_else(|| DesktopErrorDto::from(WriterEditorError::DocumentNotFound))?;
        WriterDocxService::export(&document, path).map_err(DesktopErrorDto::from)
    }

    pub fn export_pdf(
        &self,
        document_id: &str,
        path: &str,
    ) -> Result<String, DesktopErrorDto> {
        let document = self
            .controller
            .snapshot(document_id)
            .ok_or_else(|| DesktopErrorDto::from(WriterEditorError::DocumentNotFound))?;
        WriterPdfService::new()
            .export(&document, path)
            .map_err(DesktopErrorDto::from)
    }

    pub fn open_document(&mut self, path: &str) -> Result<WriterDocumentView, DesktopErrorDto> {
        let resolved_path =
            WriterStorageService::resolve_open_path(path).map_err(DesktopErrorDto::from)?;
        let document = WriterStorageService::open(path).map_err(DesktopErrorDto::from)?;
        let view = self.controller.load(document);
        self.file_session_service
            .track_open(&view.id, &resolved_path)
            .map_err(DesktopErrorDto::from)?;
        Ok(view)
    }

    pub fn save_document(
        &mut self,
        document_id: &str,
        path: &str,
    ) -> Result<(WriterDocumentView, String), DesktopErrorDto> {
        let document = self
            .controller
            .snapshot(document_id)
            .ok_or_else(|| DesktopErrorDto::from(WriterEditorError::DocumentNotFound))?;
        let target_path =
            WriterStorageService::resolve_save_path(path).map_err(DesktopErrorDto::from)?;
        let prepared = self
            .file_session_service
            .prepare_save(document_id, &target_path)
            .map_err(DesktopErrorDto::from)?;
        let saved_path =
            match WriterStorageService::save(&document, target_path.to_string_lossy().as_ref()) {
                Ok(value) => value,
                Err(error) => {
                    self.file_session_service.cancel_save(&prepared);
                    return Err(DesktopErrorDto::from(error));
                }
            };
        self.file_session_service
            .commit_save(document_id, prepared)
            .map_err(DesktopErrorDto::from)?;
        let _ = self.recovery_service.clear_document(document_id);
        Ok((WriterDocumentView::from(document), saved_path))
    }

    pub fn file_session_status(
        &self,
        document_id: &str,
    ) -> Result<WriterFileSessionStatus, DesktopErrorDto> {
        self.file_session_service
            .status(document_id)
            .map_err(DesktopErrorDto::from)
    }

    pub fn acknowledge_external_change(
        &mut self,
        document_id: &str,
    ) -> Result<WriterFileSessionStatus, DesktopErrorDto> {
        self.file_session_service
            .acknowledge_external_change(document_id)
            .map_err(DesktopErrorDto::from)
    }

    pub fn reload_from_disk(
        &mut self,
        document_id: &str,
    ) -> Result<(WriterDocumentView, WriterFileSessionStatus), DesktopErrorDto> {
        let path = self
            .file_session_service
            .current_path(document_id)
            .ok_or_else(|| DesktopErrorDto::from(crate::services::writer_file_session_service::WriterFileSessionError::InvalidPath))?;
        let document = WriterStorageService::open(path.to_string_lossy().as_ref())
            .map_err(DesktopErrorDto::from)?;
        let view = self.controller.load(document);
        self.file_session_service
            .refresh_baseline(&view.id)
            .map_err(DesktopErrorDto::from)?;
        let status = self
            .file_session_service
            .status(&view.id)
            .map_err(DesktopErrorDto::from)?;
        Ok((view, status))
    }

    pub fn create_recovery_snapshot(
        &self,
        document_id: &str,
        source_path: Option<&str>,
        persisted_revision: u64,
    ) -> Result<RecoverySnapshot, DesktopErrorDto> {
        let document = self
            .controller
            .snapshot(document_id)
            .ok_or_else(|| DesktopErrorDto::from(WriterEditorError::DocumentNotFound))?;
        self.recovery_service
            .create_snapshot(&document, source_path, persisted_revision)
            .map_err(DesktopErrorDto::from)
    }

    pub fn list_recovery_snapshots(&self) -> Result<Vec<RecoverySnapshot>, DesktopErrorDto> {
        self.recovery_service
            .list_snapshots()
            .map_err(DesktopErrorDto::from)
    }

    pub fn restore_recovery_snapshot(
        &mut self,
        snapshot_id: &str,
    ) -> Result<(WriterDocumentView, RecoverySnapshot), DesktopErrorDto> {
        let restored = self
            .recovery_service
            .restore(snapshot_id)
            .map_err(DesktopErrorDto::from)?;
        let view = self.controller.load(restored.document);
        if let Some(source_path) = restored.snapshot.source_path.as_deref() {
            if let Ok(path) = WriterStorageService::resolve_open_path(source_path) {
                let _ = self.file_session_service.track_open(&view.id, &path);
            }
        } else {
            self.file_session_service.reset_untracked();
        }
        Ok((view, restored.snapshot))
    }

    pub fn compare_recovery_snapshot(
        &self,
        snapshot_id: &str,
    ) -> Result<RecoveryComparison, DesktopErrorDto> {
        self.recovery_service
            .compare(snapshot_id)
            .map_err(DesktopErrorDto::from)
    }

    pub fn discard_recovery_snapshot(&self, snapshot_id: &str) -> Result<(), DesktopErrorDto> {
        self.recovery_service
            .discard(snapshot_id)
            .map_err(DesktopErrorDto::from)
    }

    pub fn clear_recovery_for_document(&self, document_id: &str) -> Result<(), DesktopErrorDto> {
        self.recovery_service
            .clear_document(document_id)
            .map_err(DesktopErrorDto::from)
    }

    pub fn replace_paragraph_text(
        &mut self,
        document_id: &str,
        paragraph_id: &str,
        text: &str,
    ) -> Result<WriterDocumentView, DesktopErrorDto> {
        self.replace_paragraph_text_with_style(document_id, paragraph_id, text, None)
    }

    pub fn replace_paragraph_text_with_style(
        &mut self,
        document_id: &str,
        paragraph_id: &str,
        text: &str,
        typing_style: Option<CharacterStyle>,
    ) -> Result<WriterDocumentView, DesktopErrorDto> {
        self.file_session_service
            .ensure_writable(document_id)
            .map_err(DesktopErrorDto::from)?;
        let current = self.document(document_id)?;
        let paragraph = current
            .paragraphs
            .iter()
            .find(|item| item.id == paragraph_id)
            .ok_or_else(|| DesktopErrorDto::new(ERROR_PARAGRAPH_NOT_FOUND))?;

        if paragraph.plain_text == text {
            return Ok(current);
        }

        let old_chars: Vec<char> = paragraph.plain_text.chars().collect();
        let new_chars: Vec<char> = text.chars().collect();
        let prefix = Self::common_prefix_length(&old_chars, &new_chars);
        let suffix = Self::common_suffix_length(&old_chars, &new_chars, prefix);
        let old_change_end = old_chars.len().saturating_sub(suffix);
        let new_change_end = new_chars.len().saturating_sub(suffix);
        let mut commands = Vec::new();

        if old_change_end > prefix {
            commands.push(WriterCommand::DeleteRange {
                range: TextRange {
                    anchor: Self::position_at_offset(paragraph, prefix)?,
                    focus: Self::position_at_offset(paragraph, old_change_end)?,
                },
            });
        }

        if new_change_end > prefix {
            let inserted: String = new_chars[prefix..new_change_end].iter().collect();
            let position = Self::position_at_offset(paragraph, prefix)?;
            commands.push(match typing_style.clone() {
                Some(style) => WriterCommand::InsertStyledText {
                    position,
                    text: inserted,
                    style,
                },
                None => WriterCommand::InsertText {
                    position,
                    text: inserted,
                },
            });
        }

        if commands.is_empty() {
            return Ok(current);
        }

        self.controller
            .execute_batch(document_id, &commands)
            .map_err(DesktopErrorDto::from)
    }

    pub fn get_asset(
        &self,
        document_id: &str,
        asset_id: &str,
    ) -> Result<WriterAsset, DesktopErrorDto> {
        let document = self
            .controller
            .snapshot(document_id)
            .ok_or_else(|| DesktopErrorDto::from(WriterEditorError::DocumentNotFound))?;
        document
            .assets
            .into_iter()
            .find(|asset| asset.id == asset_id)
            .ok_or_else(|| DesktopErrorDto::new(ERROR_ASSET_NOT_FOUND))
    }

    pub fn insert_image_data(
        &mut self,
        document_id: &str,
        after_paragraph_id: &str,
        media_type: &str,
        data: Vec<u8>,
        alt_text: &str,
        width_twips: Option<u32>,
        height_twips: Option<u32>,
    ) -> Result<WriterDocumentView, DesktopErrorDto> {
        self.file_session_service
            .ensure_writable(document_id)
            .map_err(DesktopErrorDto::from)?;
        self.controller
            .execute(
                document_id,
                WriterCommand::InsertImageData {
                    after_paragraph_id: NodeId::new(after_paragraph_id),
                    media_type: media_type.to_owned(),
                    data,
                    alt_text: alt_text.to_owned(),
                    width_twips,
                    height_twips,
                },
            )
            .map_err(DesktopErrorDto::from)
    }

    pub fn replace_range_with_styled_runs(
        &mut self,
        document_id: &str,
        paragraph_id: &str,
        start_offset: usize,
        end_offset: usize,
        runs: Vec<StyledTextRun>,
    ) -> Result<WriterDocumentView, DesktopErrorDto> {
        self.file_session_service
            .ensure_writable(document_id)
            .map_err(DesktopErrorDto::from)?;
        let current = self.document(document_id)?;
        let paragraph = current
            .paragraphs
            .iter()
            .find(|item| item.id == paragraph_id)
            .ok_or_else(|| DesktopErrorDto::new(ERROR_PARAGRAPH_NOT_FOUND))?;
        let start = start_offset.min(end_offset);
        let end = start_offset.max(end_offset);

        self.controller
            .execute(
                document_id,
                WriterCommand::ReplaceRangeWithStyledRuns {
                    range: TextRange {
                        anchor: Self::position_at_offset(paragraph, start)?,
                        focus: Self::position_at_offset(paragraph, end)?,
                    },
                    runs,
                },
            )
            .map_err(DesktopErrorDto::from)
    }

    pub fn apply_character_style(
        &mut self,
        document_id: &str,
        paragraph_id: &str,
        start_offset: usize,
        end_offset: usize,
        patch: CharacterStylePatch,
    ) -> Result<WriterDocumentView, DesktopErrorDto> {
        self.file_session_service
            .ensure_writable(document_id)
            .map_err(DesktopErrorDto::from)?;
        let current = self.document(document_id)?;
        let paragraph = current
            .paragraphs
            .iter()
            .find(|item| item.id == paragraph_id)
            .ok_or_else(|| DesktopErrorDto::new(ERROR_PARAGRAPH_NOT_FOUND))?;
        let start = start_offset.min(end_offset);
        let end = start_offset.max(end_offset);

        self.controller
            .execute(
                document_id,
                WriterCommand::ApplyCharacterStyle {
                    range: TextRange {
                        anchor: Self::position_at_offset(paragraph, start)?,
                        focus: Self::position_at_offset(paragraph, end)?,
                    },
                    patch,
                },
            )
            .map_err(DesktopErrorDto::from)
    }

    pub fn apply_paragraph_alignment(
        &mut self,
        document_id: &str,
        paragraph_id: &str,
        alignment: TextAlignment,
    ) -> Result<WriterDocumentView, DesktopErrorDto> {
        self.file_session_service
            .ensure_writable(document_id)
            .map_err(DesktopErrorDto::from)?;
        self.controller
            .execute(
                document_id,
                WriterCommand::ApplyParagraphStyle {
                    paragraph_id: NodeId::new(paragraph_id),
                    patch: ParagraphStylePatch {
                        alignment: Some(alignment),
                    },
                },
            )
            .map_err(DesktopErrorDto::from)
    }

    pub fn split_paragraph(
        &mut self,
        document_id: &str,
        paragraph_id: &str,
        offset: usize,
    ) -> Result<WriterDocumentView, DesktopErrorDto> {
        self.file_session_service
            .ensure_writable(document_id)
            .map_err(DesktopErrorDto::from)?;
        let current = self.document(document_id)?;
        let paragraph = current
            .paragraphs
            .iter()
            .find(|item| item.id == paragraph_id)
            .ok_or_else(|| DesktopErrorDto::new(ERROR_PARAGRAPH_NOT_FOUND))?;
        let position = Self::position_at_offset(paragraph, offset)?;

        self.controller
            .execute(document_id, WriterCommand::SplitParagraph { position })
            .map_err(DesktopErrorDto::from)
    }

    pub fn merge_with_previous(
        &mut self,
        document_id: &str,
        paragraph_id: &str,
    ) -> Result<WriterDocumentView, DesktopErrorDto> {
        self.file_session_service
            .ensure_writable(document_id)
            .map_err(DesktopErrorDto::from)?;
        let current = self.document(document_id)?;
        let paragraph_index = current
            .paragraphs
            .iter()
            .position(|item| item.id == paragraph_id)
            .ok_or_else(|| DesktopErrorDto::new(ERROR_PARAGRAPH_NOT_FOUND))?;
        let previous = paragraph_index
            .checked_sub(1)
            .and_then(|index| current.paragraphs.get(index))
            .ok_or_else(|| DesktopErrorDto::new(ERROR_PARAGRAPH_NOT_FOUND))?;

        self.controller
            .execute(
                document_id,
                WriterCommand::MergeParagraph {
                    first_paragraph_id: NodeId::new(&previous.id),
                    second_paragraph_id: NodeId::new(paragraph_id),
                },
            )
            .map_err(DesktopErrorDto::from)
    }

    pub fn undo(&mut self, document_id: &str) -> Result<WriterDocumentView, DesktopErrorDto> {
        self.file_session_service
            .ensure_writable(document_id)
            .map_err(DesktopErrorDto::from)?;
        self.controller
            .undo(document_id)
            .map_err(DesktopErrorDto::from)
    }

    pub fn redo(&mut self, document_id: &str) -> Result<WriterDocumentView, DesktopErrorDto> {
        self.file_session_service
            .ensure_writable(document_id)
            .map_err(DesktopErrorDto::from)?;
        self.controller
            .redo(document_id)
            .map_err(DesktopErrorDto::from)
    }

    fn document(&self, document_id: &str) -> Result<WriterDocumentView, DesktopErrorDto> {
        self.controller
            .get(document_id)
            .ok_or_else(|| DesktopErrorDto::from(WriterEditorError::DocumentNotFound))
    }

    fn common_prefix_length(left: &[char], right: &[char]) -> usize {
        left.iter()
            .zip(right.iter())
            .take_while(|(left_char, right_char)| left_char == right_char)
            .count()
    }

    fn common_suffix_length(left: &[char], right: &[char], prefix: usize) -> usize {
        let left_remaining = left.len().saturating_sub(prefix);
        let right_remaining = right.len().saturating_sub(prefix);
        left.iter()
            .rev()
            .take(left_remaining)
            .zip(right.iter().rev().take(right_remaining))
            .take_while(|(left_char, right_char)| left_char == right_char)
            .count()
    }

    fn position_at_offset(
        paragraph: &turkuaz_office_writer::WriterParagraphView,
        offset: usize,
    ) -> Result<TextPosition, DesktopErrorDto> {
        let paragraph_length = paragraph.plain_text.chars().count();
        if offset > paragraph_length {
            return Err(DesktopErrorDto::new(ERROR_INVALID_OFFSET));
        }

        let mut remaining = offset;
        for run in &paragraph.runs {
            let run_length = run.text.chars().count();
            if remaining <= run_length {
                return Ok(TextPosition {
                    paragraph_id: NodeId::new(&paragraph.id),
                    run_id: NodeId::new(&run.id),
                    offset: remaining,
                });
            }
            remaining -= run_length;
        }

        Err(DesktopErrorDto::new(ERROR_INVALID_OFFSET))
    }
}

impl Default for WriterDesktopService {
    fn default() -> Self {
        Self::new()
    }
}
