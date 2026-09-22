// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/tests/writer_docx_service_tests.rs
// # 📌 Amac: Desktop DOCX import/export akisini ve foreign format session sinirini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// # Version: 0.2.0
// # Aciklama: DOCX export/import, fresh canonical ID ve untracked TKO file session davranisini test eder
// Bagimli Oldugu Katman: Service -> Tool

use std::time::{SystemTime, UNIX_EPOCH};

use turkuaz_office_desktop_lib::services::writer_desktop_service::WriterDesktopService;

#[test]
fn docx_export_import_rekeys_document_and_keeps_native_session_untracked() {
    let mut service = WriterDesktopService::new();
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();
    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "Desktop DOCX round trip")
        .expect("paragraph replace");

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let target = std::env::temp_dir().join(format!(
        "turkuaz-office-docx-{}-{unique}.docx",
        std::process::id()
    ));

    let saved_path = service
        .export_docx(&document.id, target.to_string_lossy().as_ref())
        .expect("docx export");
    let (imported, compatibility) = service.import_docx(&saved_path).expect("docx import");

    assert_ne!(imported.id, document.id);
    assert_eq!(imported.plain_text, "Desktop DOCX round trip");
    assert!(compatibility.unsupported_features.is_empty());

    let session = service
        .file_session_status(&imported.id)
        .expect("imported session");
    assert!(session.path.is_none());
    assert!(!session.read_only);
    assert!(!session.lock_owned);

    let _ = std::fs::remove_file(saved_path);
}
