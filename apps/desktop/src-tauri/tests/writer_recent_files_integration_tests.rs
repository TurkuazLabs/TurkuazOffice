// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/tests/writer_recent_files_integration_tests.rs
// # 📌 Amac: WriterDesktopService recent kaydi ile native TKO file session arasindaki entegrasyonu dogrular
// # 📌 Modul - FileType: Test - Rust
// # Version: 0.2.0
// # Aciklama: Basarili native save sonrasi explicit recent record'un belge session state'ini degistirmedigini kilitler
// Bagimli Oldugu Katman: Service -> Repo -> Tool

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use turkuaz_office_desktop_lib::services::writer_desktop_service::WriterDesktopService;

fn unique_root() -> PathBuf {
    let token = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "turkuaz-office-recent-integration-{}-{token}",
        std::process::id()
    ))
}

#[test]
fn recent_record_does_not_change_native_file_session() {
    let root = unique_root();
    let recovery_root = root.join("recovery");
    let recent_metadata = root.join("recent-files.yml");
    fs::create_dir_all(&root).expect("root");

    let mut service = WriterDesktopService::with_state_roots(recovery_root, recent_metadata);
    let document = service.create_document();
    let target = root.join("saved-document.tko");

    let (_, saved_path) = service
        .save_document(&document.id, target.to_string_lossy().as_ref())
        .expect("save");
    let before = service
        .file_session_status(&document.id)
        .expect("session before");

    let recent = service
        .record_recent_file(&saved_path)
        .expect("record recent");
    let after = service
        .file_session_status(&document.id)
        .expect("session after");

    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].title, "saved-document");
    assert_eq!(before, after);
    assert_eq!(after.path.as_deref(), Some(saved_path.as_str()));

    let _ = fs::remove_dir_all(root);
}
