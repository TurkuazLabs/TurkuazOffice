// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/tests/writer_template_desktop_tests.rs
// # 📌 Amac: Desktop template katalog ve create workflow'un native file-session izolasyonunu regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// # Version: 0.2.0
// # Aciklama: Built-in katalog metadata'sini ve template-created belgenin untracked normal Writer session olmasini kilitler
// Bagimli Oldugu Katman: Service -> Controller -> Repo -> Tool

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use turkuaz_office_desktop_lib::services::writer_desktop_service::WriterDesktopService;

fn unique_root() -> PathBuf {
    let token = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "turkuaz-office-template-{}-{token}",
        std::process::id()
    ))
}

#[test]
fn desktop_template_catalog_and_create_keep_native_session_untracked() {
    let root = unique_root();
    let recovery_root = root.join("recovery");
    let recent_metadata = root.join("recent-files.yml");
    let mut service = WriterDesktopService::with_state_roots(recovery_root, recent_metadata);

    let catalog = service.list_templates().expect("template catalog");
    assert_eq!(catalog.len(), 3);
    assert!(
        catalog
            .iter()
            .any(|item| item.id == "letter" && item.quick_create)
    );
    assert!(
        catalog
            .iter()
            .any(|item| item.id == "report" && item.quick_create)
    );

    let document = service
        .create_document_from_template("report")
        .expect("report document");
    let session = service
        .file_session_status(&document.id)
        .expect("file session");

    assert_eq!(document.paragraphs.len(), 3);
    assert!(session.path.is_none());
    assert!(!session.read_only);
    assert!(!session.lock_owned);

    let _ = std::fs::remove_dir_all(root);
}
