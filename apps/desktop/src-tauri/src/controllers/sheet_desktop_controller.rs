// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/controllers/sheet_desktop_controller.rs
// # 📌 Amac: Tauri Sheet IPC requestlerini alip yalnizca SheetDesktopService cagirir
// # 📌 Modul - FileType: Controller - Rust
// Version: 0.7.0
// Aciklama: Sheet create/get/cell/formula/format/table/conditional-format/range-summary/query/evaluated-cell/clear request ve state sinirini business logic tasimadan yonetir
// Bagimli Oldugu Katman: Controller -> Service

use std::sync::Mutex;

use tauri::State;

use crate::config::constants::ERROR_STATE_LOCK;
use crate::services::sheet_desktop_service::SheetDesktopService;
use crate::views::error_dto::DesktopErrorDto;
use crate::views::sheet_dto::{
    SheetCellDto, SheetCellFormatDto, SheetConditionalFormatCreateRequestDto,
    SheetConditionalFormatMatchDto, SheetConditionalFormatMatchesRequestDto,
    SheetConditionalFormatRemoveRequestDto, SheetDocumentDto, SheetRangeSummaryDto,
    SheetRangeSummaryRequestDto, SheetRowQueryRequestDto, SheetRowQueryResultDto,
    SheetTableCreateRequestDto, SheetTableRemoveRequestDto,
};

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
pub fn sheet_get_cell_format(
    state: State<'_, SheetDesktopState>,
    document_id: String,
    worksheet_id: String,
    reference: String,
) -> Result<SheetCellFormatDto, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .cell_format(&document_id, &worksheet_id, &reference)
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_set_cell_format(
    state: State<'_, SheetDesktopState>,
    document_id: String,
    worksheet_id: String,
    reference: String,
    format: SheetCellFormatDto,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .set_cell_format(&document_id, &worksheet_id, &reference, format.into())
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_create_table(
    state: State<'_, SheetDesktopState>,
    request: SheetTableCreateRequestDto,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .create_table(
            &request.document_id,
            &request.worksheet_id,
            request.range.into(),
        )
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_remove_table(
    state: State<'_, SheetDesktopState>,
    request: SheetTableRemoveRequestDto,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .remove_table(&request.document_id, &request.table_id)
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_create_conditional_format(
    state: State<'_, SheetDesktopState>,
    request: SheetConditionalFormatCreateRequestDto,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .create_conditional_format(
            &request.document_id,
            &request.worksheet_id,
            request.range.into(),
            request.condition.into(),
            request.style.into(),
        )
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_remove_conditional_format(
    state: State<'_, SheetDesktopState>,
    request: SheetConditionalFormatRemoveRequestDto,
) -> Result<SheetDocumentDto, DesktopErrorDto> {
    let mut service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .remove_conditional_format(&request.document_id, &request.rule_id)
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_get_conditional_format_matches(
    state: State<'_, SheetDesktopState>,
    request: SheetConditionalFormatMatchesRequestDto,
) -> Result<Vec<SheetConditionalFormatMatchDto>, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .conditional_format_matches(
            &request.document_id,
            &request.worksheet_id,
            request.range.into(),
        )
        .map(|items| items.into_iter().map(Into::into).collect())
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_get_range_summary(
    state: State<'_, SheetDesktopState>,
    request: SheetRangeSummaryRequestDto,
) -> Result<SheetRangeSummaryDto, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .range_summary(
            &request.document_id,
            &request.worksheet_id,
            request.range.into(),
        )
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_query_rows(
    state: State<'_, SheetDesktopState>,
    request: SheetRowQueryRequestDto,
) -> Result<SheetRowQueryResultDto, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    let filter = request.filter.map(Into::into);
    service
        .query_rows(
            &request.document_id,
            &request.worksheet_id,
            request.range.into(),
            filter.as_ref(),
            request.sort.map(Into::into),
        )
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub fn sheet_get_evaluated_cell(
    state: State<'_, SheetDesktopState>,
    document_id: String,
    worksheet_id: String,
    reference: String,
) -> Result<Option<SheetCellDto>, DesktopErrorDto> {
    let service = state
        .lock()
        .map_err(|_| DesktopErrorDto::new(ERROR_STATE_LOCK))?;
    service
        .evaluated_cell(&document_id, &worksheet_id, &reference)
        .map(|cell| cell.map(Into::into))
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
