// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/lib.rs
// # 📌 Amac: Turkuaz Office Desktop Tauri runtime composition rootunu kurar
// # 📌 Modul - FileType: Desktop - Rust
// # Version: 0.11.0
// # Aciklama: Suite launch hedefi, Writer/Sheet state, chart dahil IPC controller ve Tauri Builder kaydini merkezi baslatir
// Bagimli Oldugu Katman: Controller -> Service

use std::sync::Mutex;

use tauri::Manager;

use controllers::desktop_launch_controller::desktop_get_launch_module;
use controllers::sheet_desktop_controller::{
    sheet_clear_cell, sheet_create_chart, sheet_create_conditional_format, sheet_create_document,
    sheet_create_table, sheet_get_cell_format, sheet_get_chart_data,
    sheet_get_conditional_format_matches, sheet_get_document, sheet_get_evaluated_cell,
    sheet_get_range_summary, sheet_query_rows, sheet_remove_chart, sheet_remove_conditional_format,
    sheet_remove_table, sheet_set_boolean, sheet_set_cell_format, sheet_set_formula,
    sheet_set_number, sheet_set_text,
};
use controllers::writer_desktop_controller::{
    writer_acknowledge_external_change, writer_apply_character_style,
    writer_apply_paragraph_alignment, writer_clear_document_recovery,
    writer_compare_recovery_snapshot, writer_create_document, writer_create_document_from_template,
    writer_create_recovery_snapshot, writer_discard_recovery_snapshot, writer_export_docx,
    writer_export_pdf, writer_get_asset, writer_get_file_session, writer_import_docx,
    writer_insert_image_data, writer_list_recent_files, writer_list_recovery_snapshots,
    writer_list_templates, writer_merge_with_previous, writer_open_document,
    writer_record_recent_file, writer_redo, writer_reload_from_disk, writer_replace_paragraph_text,
    writer_replace_range_with_styled_runs, writer_restore_recovery_snapshot, writer_save_document,
    writer_split_paragraph, writer_take_startup_file, writer_undo,
};
use services::sheet_desktop_service::SheetDesktopService;
use services::writer_desktop_service::WriterDesktopService;
use tools::desktop_launch_tool::DesktopLaunchTool;

pub mod config;
pub mod controllers;
pub mod repositories;
pub mod services;
pub mod tools;
pub mod views;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(WriterDesktopService::new()))
        .manage(Mutex::new(SheetDesktopService::new()))
        .setup(|app| {
            let launch_module = DesktopLaunchTool::from_environment();
            if let Some(window) = app.get_webview_window("main") {
                window.set_title(launch_module.window_title())?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            desktop_get_launch_module,
            sheet_create_document,
            sheet_create_table,
            sheet_create_chart,
            sheet_remove_chart,
            sheet_get_chart_data,
            sheet_create_conditional_format,
            sheet_remove_conditional_format,
            sheet_remove_table,
            sheet_get_conditional_format_matches,
            sheet_get_document,
            sheet_get_cell_format,
            sheet_set_cell_format,
            sheet_query_rows,
            sheet_get_evaluated_cell,
            sheet_get_range_summary,
            sheet_set_text,
            sheet_set_number,
            sheet_set_boolean,
            sheet_set_formula,
            sheet_clear_cell,
            writer_take_startup_file,
            writer_list_recent_files,
            writer_record_recent_file,
            writer_list_templates,
            writer_create_document_from_template,
            writer_create_document,
            writer_open_document,
            writer_import_docx,
            writer_export_docx,
            writer_export_pdf,
            writer_save_document,
            writer_get_file_session,
            writer_acknowledge_external_change,
            writer_reload_from_disk,
            writer_list_recovery_snapshots,
            writer_create_recovery_snapshot,
            writer_restore_recovery_snapshot,
            writer_compare_recovery_snapshot,
            writer_discard_recovery_snapshot,
            writer_clear_document_recovery,
            writer_get_asset,
            writer_insert_image_data,
            writer_replace_paragraph_text,
            writer_replace_range_with_styled_runs,
            writer_apply_character_style,
            writer_apply_paragraph_alignment,
            writer_split_paragraph,
            writer_merge_with_previous,
            writer_undo,
            writer_redo,
        ])
        .run(tauri::generate_context!())
        .expect("desktop.tauri_runtime_failed");
}
