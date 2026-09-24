// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/tests/writer_pdf_service_tests.rs
// # 📌 Amac: Desktop PDF export, external font embedding ve native session isolation davranisini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// # Version: 0.2.0
// # Aciklama: Gercek sistem fontu ile PDF imzasi uretimini ve exportun aktif TKO session'ini degistirmedigini test eder
// Bagimli Oldugu Katman: Service -> Tool

use std::time::{SystemTime, UNIX_EPOCH};

use turkuaz_office_desktop_lib::services::writer_desktop_service::WriterDesktopService;

#[test]
fn pdf_export_writes_pdf_signature_and_keeps_native_session_untracked() {
    let mut service = WriterDesktopService::new();
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();
    let document = service
        .replace_paragraph_text(
            &document.id,
            &paragraph_id,
            "Turkuaz PDF: Cagri, Sivas ve Turkce Unicode",
        )
        .expect("paragraph replace");

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let target = std::env::temp_dir().join(format!(
        "turkuaz-office-pdf-{}-{unique}.pdf",
        std::process::id()
    ));

    let saved_path = service
        .export_pdf(&document.id, target.to_string_lossy().as_ref())
        .expect("pdf export");
    let bytes = std::fs::read(&saved_path).expect("pdf bytes");

    assert!(bytes.starts_with(b"%PDF-"));
    assert!(bytes.len() > 100);

    let session = service
        .file_session_status(&document.id)
        .expect("document session");
    assert!(session.path.is_none());
    assert!(!session.read_only);
    assert!(!session.lock_owned);

    let _ = std::fs::remove_file(saved_path);
}
