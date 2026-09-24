// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/ipc-commands.ts
// # 📌 Amac: Tauri IPC command adlarini tek merkezi kontratta toplar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.2.0
// # Aciklama: Frontend Tool katmaninda edit, local file ve recovery IPC magic string kullanilmasini engeller
// Bagimli Oldugu Katman: Config

export const IPC_COMMANDS = {
  writerListRecentFiles: "writer_list_recent_files",
  writerRecordRecentFile: "writer_record_recent_file",
  writerCreateDocument: "writer_create_document",
  writerListRecentFiles: "writer_list_recent_files",
  writerRecordRecentFile: "writer_record_recent_file",
  writerOpenDocument: "writer_open_document",
  writerImportDocx: "writer_import_docx",
  writerExportDocx: "writer_export_docx",
  writerExportPdf: "writer_export_pdf",
  writerSaveDocument: "writer_save_document",
  writerGetFileSession: "writer_get_file_session",
  writerAcknowledgeExternalChange: "writer_acknowledge_external_change",
  writerReloadFromDisk: "writer_reload_from_disk",
  writerListRecoverySnapshots: "writer_list_recovery_snapshots",
  writerCreateRecoverySnapshot: "writer_create_recovery_snapshot",
  writerRestoreRecoverySnapshot: "writer_restore_recovery_snapshot",
  writerCompareRecoverySnapshot: "writer_compare_recovery_snapshot",
  writerDiscardRecoverySnapshot: "writer_discard_recovery_snapshot",
  writerClearDocumentRecovery: "writer_clear_document_recovery",
  writerGetAsset: "writer_get_asset",
  writerInsertImageData: "writer_insert_image_data",
  writerReplaceParagraphText: "writer_replace_paragraph_text",
  writerReplaceRangeWithStyledRuns: "writer_replace_range_with_styled_runs",
  writerApplyCharacterStyle: "writer_apply_character_style",
  writerApplyParagraphAlignment: "writer_apply_paragraph_alignment",
  writerSplitParagraph: "writer_split_paragraph",
  writerMergeWithPrevious: "writer_merge_with_previous",
  writerUndo: "writer_undo",
  writerRedo: "writer_redo",
} as const;
