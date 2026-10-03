// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/tests/startup_file_service_tests.rs
// # 📌 Amac: File association startup argument secimi, canonical path ve one-shot davranisini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// # Version: 0.2.0
// # Aciklama: Flag, foreign extension ve missing path'leri reddeder; ilk existing TKO path'i canonical olarak tek kez verir
// Bagimli Oldugu Katman: Service -> Tool

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use turkuaz_office_desktop_lib::services::startup_file_service::StartupFileService;
use turkuaz_office_desktop_lib::services::writer_desktop_service::WriterDesktopService;
use turkuaz_office_desktop_lib::tools::local_file_tool::LocalFileTool;

fn unique_root() -> PathBuf {
    let token = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "turkuaz-office-association-{}-{token}",
        std::process::id()
    ))
}

#[test]
fn startup_file_service_selects_first_existing_tko_once() {
    let root = unique_root();
    fs::create_dir_all(root.join("nested")).expect("root");

    let foreign = root.join("foreign.docx");
    fs::write(&foreign, b"foreign").expect("foreign");
    let native = root.join("native.tko");
    fs::write(&native, b"native").expect("native");
    let relative_native = root.join("nested").join("..").join("native.tko");

    let mut service = StartupFileService::from_arguments(vec![
        "--flag".to_owned(),
        root.join("missing.tko").to_string_lossy().into_owned(),
        foreign.to_string_lossy().into_owned(),
        relative_native.to_string_lossy().into_owned(),
        root.join("later.tko").to_string_lossy().into_owned(),
    ]);

    let expected = LocalFileTool::canonicalize_file(&native)
        .expect("canonical")
        .to_string_lossy()
        .into_owned();

    assert_eq!(service.take().as_deref(), Some(expected.as_str()));
    assert_eq!(service.take(), None);

    let _ = fs::remove_dir_all(root);
}

#[test]
fn writer_desktop_exposes_startup_file_as_one_shot_state() {
    let root = unique_root();
    fs::create_dir_all(&root).expect("root");
    let native = root.join("open-me.tko");
    fs::write(&native, b"native").expect("native");
    let recovery_root = root.join("recovery");
    let recent_metadata = root.join("recent-files.yml");

    let mut service = WriterDesktopService::with_state_roots_and_startup_arguments(
        recovery_root,
        recent_metadata,
        vec![native.to_string_lossy().into_owned()],
    );

    let expected = LocalFileTool::canonicalize_file(&native)
        .expect("canonical")
        .to_string_lossy()
        .into_owned();

    assert_eq!(
        service.take_startup_file().as_deref(),
        Some(expected.as_str())
    );
    assert_eq!(service.take_startup_file(), None);

    let _ = fs::remove_dir_all(root);
}
