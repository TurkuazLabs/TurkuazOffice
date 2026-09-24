// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/lib.rs
// # 📌 Amac: Turkuaz Office Desktop Tauri runtime composition rootunu kurar
// # 📌 Modul - FileType: Desktop - Rust
// # Version: 0.2.0
// # Aciklama: Writer state, IPC controller ve Tauri Builder kaydini merkezi baslatir
// Bagimli Oldugu Katman: Controller -> Service

use std::sync::Mutex;

use controllers::writer_desktop_controller::{
    writer_acknowledge_external_change, writer_apply_character_style,
    writer_apply_paragraph_alignment, writer_clear_document_recovery,
    writer_compare_recovery_snapshot, writer_create_document, writer_create_recovery_snapshot,
    writer_discard_recovery_snapshot, writer_export_docx, writer_export_pdf, writer_get_asset,
    writer_get_file_session,
    writer_import_docx, writer_insert_image_data, writer_list_recent_files,
    writer_list_recovery_snapshots, writer_record_recent_file, writer_take_startup_file,
    writer_merge_with_previous, writer_open_document, writer_redo, writer_reload_from_disk,
    writer_replace_paragraph_text, writer_replace_range_with_styled_runs,
    writer_restore_recovery_snapshot, writer_save_document, writer_split_paragraph, writer_undo,
};
use services::writer_desktop_service::WriterDesktopService;

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
        .invoke_handler(tauri::generate_handler![
            writer_take_startup_file,
            writer_list_recent_files,
            writer_record_recent_file,
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
