// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/controllers/sheet_desktop_controller.rs
// # 📌 Amac: Tauri Sheet IPC requestlerini alip yalnizca SheetDesktopService cagirir
// # 📌 Modul - FileType: Controller - Rust
// Version: 0.4.0
// Aciklama: Sheet create/get/cell/formula/clear request ve state sinirini business logic tasimadan yonetir
// Bagimli Oldugu Katman: Controller -> Service

use std::sync::Mutex;

use tauri::State;

use crate::config::constants::ERROR_STATE_LOCK;
use crate::services::sheet_desktop_service::SheetDesktopService;
use crate::views::error_dto::DesktopErrorDto;
use crate::views::sheet_dto::SheetDocumentDto;

pub type SheetDesktopState = Mutex<SheetDesktopService>;

#[tauri::command]
pub fn sheet_create_document(
    state: State<'_, SheetDesktopState>,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    Ok(service.create_document().into())
}

#[tauri::command]
pub fn sheet_get_document(
    state: State<'_, SheetDesktopState>,
    document_id: String,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .get_document(&document_id)
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_set_text(
    state: State<'_, SheetDesktopState>,
    document_id: String,
    worksheet_id: String,
    reference: String,
    value: String,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .set_text(&document_id, &worksheet_id, &reference, &value)
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_set_number(
    state: State<'_, SheetDesktopState>,
    document_id: String,
    worksheet_id: String,
    reference: String,
    value: f64,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .set_number(&document_id, &worksheet_id, &reference, value)
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_set_boolean(
    state: State<'_, SheetDesktopState>,
    document_id: String,
    worksheet_id: String,
    reference: String,
    value: bool,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .set_boolean(&document_id, &worksheet_id, &reference, value)
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_set_formula(
    state: State<'_, SheetDesktopState>,
    document_id: String,
    worksheet_id: String,
    reference: String,
    expression: String,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .set_formula(&document_id, &worksheet_id, &reference, &expression)
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_clear_cell(
    state: State<'_, SheetDesktopState>,
    document_id: String,
    worksheet_id: String,
    reference: String,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .clear_cell(&document_id, &worksheet_id, &reference)
        .map(Into::into)
        .map_err(Into::into)
}
