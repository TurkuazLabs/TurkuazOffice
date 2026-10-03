// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/controllers/writer_desktop_controller.rs
// # 📌 Amac: Tauri Writer IPC requestlerini alip yalnizca WriterDesktopService cagirir
// # 📌 Modul - FileType: Controller - Rust
// # Version: 0.2.0
// # Aciklama: Controller icinde is kurali tutmadan request/state sinirini yonetir ve DTO doner
// Bagimli Oldugu Katman: Controller -> Service

use std::sync::Mutex;

use tauri::State;

use crate::config::constants::ERROR_STATE_LOCK;
use crate::services::writer_desktop_service::WriterDesktopService;
use crate::views::docx_dto::WriterDocxImportDto;
use crate::views::error_dto::DesktopErrorDto;
use crate::views::file_session_dto::{WriterFileSessionDto, WriterReloadDto};
use crate::views::recent_file_dto::RecentFileDto;
use crate::views::recovery_dto::{RecoveryComparisonDto, RecoveryRestoreDto, RecoverySnapshotDto};
use crate::views::template_dto::WriterTemplateDto;
use crate::views::writer_dto::{
    WriterAssetDto, WriterCharacterStyleInputDto, WriterDocumentDto, WriterFileOperationDto,
    WriterStyledRunInputDto, WriterTextAlignmentDto,
};
use turkuaz_office_writer::CharacterStylePatch;

pub type WriterDesktopState = Mutex<WriterDesktopService>;

#[tauri::command]
pub fn writer_take_startup_file(
    state: State<'_, WriterDesktopState>,
) -> Result<Option<String>, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    Ok(service.take_startup_file())
}

#[tauri::command]
pub fn writer_list_recent_files(
    state: State<'_, WriterDesktopState>,
) -> Result<Vec<RecentFileDto>, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .list_recent_files()
        .map(|items| items.into_iter().map(RecentFileDto::from).collect())
}

#[tauri::command]
pub fn writer_record_recent_file(
    state: State<'_, WriterDesktopState>,
    path: String,
) -> Result<Vec<RecentFileDto>, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .record_recent_file(&path)
        .map(|items| items.into_iter().map(RecentFileDto::from).collect())
}

#[tauri::command]
pub fn writer_list_templates(
    state: State<'_, WriterDesktopState>,
) -> Result<Vec<WriterTemplateDto>, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .list_templates()
        .map(|items| items.into_iter().map(WriterTemplateDto::from).collect())
}

#[tauri::command]
pub fn writer_create_document_from_template(
    state: State<'_, WriterDesktopState>,
    template_id: String,
) -> Result<WriterDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .create_document_from_template(&template_id)
        .map(WriterDocumentDto::from)
}

#[tauri::command]
pub fn writer_create_document(
    state: State<'_, WriterDesktopState>,
) -> Result<WriterDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    Ok(service.create_document().into())
}

#[tauri::command]
pub fn writer_import_docx(
    state: State<'_, WriterDesktopState>,
    path: String,
) -> Result<WriterDocxImportDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    let (document, compatibility) = service.import_docx(&path)?;
    Ok(WriterDocxImportDto {
        document: document.into(),
        compatibility: compatibility.into(),
    })
}

#[tauri::command]
pub fn writer_export_docx(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    path: String,
) -> Result<String, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service.export_docx(&document_id, &path)
}

#[tauri::command]
pub fn writer_export_pdf(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    path: String,
) -> Result<String, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service.export_pdf(&document_id, &path)
}

#[tauri::command]
pub fn writer_open_document(
    state: State<'_, WriterDesktopState>,
    path: String,
) -> Result<WriterFileOperationDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    let document = service.open_document(&path)?;
    Ok(WriterFileOperationDto {
        document: document.into(),
        path,
    })
}

#[tauri::command]
pub fn writer_save_document(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    path: String,
) -> Result<WriterFileOperationDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    let (document, saved_path) = service.save_document(&document_id, &path)?;
    Ok(WriterFileOperationDto {
        document: document.into(),
        path: saved_path,
    })
}

#[tauri::command]
pub fn writer_get_file_session(
    state: State<'_, WriterDesktopState>,
    document_id: String,
) -> Result<WriterFileSessionDto, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service.file_session_status(&document_id).map(Into::into)
}

#[tauri::command]
pub fn writer_acknowledge_external_change(
    state: State<'_, WriterDesktopState>,
    document_id: String,
) -> Result<WriterFileSessionDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .acknowledge_external_change(&document_id)
        .map(Into::into)
}

#[tauri::command]
pub fn writer_reload_from_disk(
    state: State<'_, WriterDesktopState>,
    document_id: String,
) -> Result<WriterReloadDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    let (document, file_session) = service.reload_from_disk(&document_id)?;
    Ok(WriterReloadDto {
        document: document.into(),
        file_session: file_session.into(),
    })
}

#[tauri::command]
pub fn writer_list_recovery_snapshots(
    state: State<'_, WriterDesktopState>,
) -> Result<Vec<RecoverySnapshotDto>, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .list_recovery_snapshots()
        .map(|items| items.into_iter().map(RecoverySnapshotDto::from).collect())
}

#[tauri::command]
pub fn writer_create_recovery_snapshot(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    source_path: Option<String>,
    persisted_revision: u64,
) -> Result<RecoverySnapshotDto, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .create_recovery_snapshot(&document_id, source_path.as_deref(), persisted_revision)
        .map(RecoverySnapshotDto::from)
}

#[tauri::command]
pub fn writer_restore_recovery_snapshot(
    state: State<'_, WriterDesktopState>,
    snapshot_id: String,
) -> Result<RecoveryRestoreDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    let (document, snapshot) = service.restore_recovery_snapshot(&snapshot_id)?;
    Ok(RecoveryRestoreDto {
        snapshot: snapshot.into(),
        document: document.into(),
    })
}

#[tauri::command]
pub fn writer_compare_recovery_snapshot(
    state: State<'_, WriterDesktopState>,
    snapshot_id: String,
) -> Result<RecoveryComparisonDto, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .compare_recovery_snapshot(&snapshot_id)
        .map(RecoveryComparisonDto::from)
}

#[tauri::command]
pub fn writer_discard_recovery_snapshot(
    state: State<'_, WriterDesktopState>,
    snapshot_id: String,
) -> Result<(), DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service.discard_recovery_snapshot(&snapshot_id)
}

#[tauri::command]
pub fn writer_clear_document_recovery(
    state: State<'_, WriterDesktopState>,
    document_id: String,
) -> Result<(), DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service.clear_recovery_for_document(&document_id)
}

#[tauri::command]
pub fn writer_get_asset(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    asset_id: String,
) -> Result<WriterAssetDto, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service.get_asset(&document_id, &asset_id).map(Into::into)
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn writer_insert_image_data(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    after_paragraph_id: String,
    media_type: String,
    data: Vec<u8>,
    alt_text: String,
    width_twips: Option<u32>,
    height_twips: Option<u32>,
) -> Result<WriterDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .insert_image_data(
            &document_id,
            &after_paragraph_id,
            &media_type,
            data,
            &alt_text,
            width_twips,
            height_twips,
        )
        .map(WriterDocumentDto::from)
}

#[tauri::command]
pub fn writer_replace_paragraph_text(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    paragraph_id: String,
    text: String,
    typing_style: Option<WriterCharacterStyleInputDto>,
) -> Result<WriterDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .replace_paragraph_text_with_style(
            &document_id,
            &paragraph_id,
            &text,
            typing_style.map(Into::into),
        )
        .map(WriterDocumentDto::from)
}

#[tauri::command]
pub fn writer_replace_range_with_styled_runs(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    paragraph_id: String,
    start_offset: usize,
    end_offset: usize,
    runs: Vec<WriterStyledRunInputDto>,
) -> Result<WriterDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .replace_range_with_styled_runs(
            &document_id,
            &paragraph_id,
            start_offset,
            end_offset,
            runs.into_iter().map(Into::into).collect(),
        )
        .map(WriterDocumentDto::from)
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn writer_apply_character_style(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    paragraph_id: String,
    start_offset: usize,
    end_offset: usize,
    bold: Option<bool>,
    italic: Option<bool>,
    underline: Option<bool>,
    font_family: Option<String>,
    font_size_half_points: Option<u16>,
) -> Result<WriterDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .apply_character_style(
            &document_id,
            &paragraph_id,
            start_offset,
            end_offset,
            CharacterStylePatch {
                bold,
                italic,
                underline,
                font_family,
                font_size_half_points,
            },
        )
        .map(WriterDocumentDto::from)
}

#[tauri::command]
pub fn writer_apply_paragraph_alignment(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    paragraph_id: String,
    alignment: WriterTextAlignmentDto,
) -> Result<WriterDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .apply_paragraph_alignment(&document_id, &paragraph_id, alignment.into())
        .map(WriterDocumentDto::from)
}

#[tauri::command]
pub fn writer_split_paragraph(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    paragraph_id: String,
    offset: usize,
) -> Result<WriterDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .split_paragraph(&document_id, &paragraph_id, offset)
        .map(WriterDocumentDto::from)
}

#[tauri::command]
pub fn writer_merge_with_previous(
    state: State<'_, WriterDesktopState>,
    document_id: String,
    paragraph_id: String,
) -> Result<WriterDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .merge_with_previous(&document_id, &paragraph_id)
        .map(WriterDocumentDto::from)
}

#[tauri::command]
pub fn writer_undo(
    state: State<'_, WriterDesktopState>,
    document_id: String,
) -> Result<WriterDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service.undo(&document_id).map(WriterDocumentDto::from)
}

#[tauri::command]
pub fn writer_redo(
    state: State<'_, WriterDesktopState>,
    document_id: String,
) -> Result<WriterDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service.redo(&document_id).map(WriterDocumentDto::from)
}
