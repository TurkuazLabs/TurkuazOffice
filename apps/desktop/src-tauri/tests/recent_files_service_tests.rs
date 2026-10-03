// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/tests/recent_files_service_tests.rs
// # 📌 Amac: Recent files dedup, canonical path, retention ve missing-file prune davranisini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// # Version: 0.2.0
// # Aciklama: Native TKO recent metadata listesinin maksimum kayit, siralama ve cleanup kontratlarini kilitler
// Bagimli Oldugu Katman: Service -> Repo -> Tool

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use turkuaz_office_desktop_lib::config::constants::RECENT_FILES_MAX_ENTRIES;
use turkuaz_office_desktop_lib::repositories::recent_files_repository::RecentFilesRepository;
use turkuaz_office_desktop_lib::services::recent_files_service::{
    RecentFilesError, RecentFilesService,
};

fn unique_root() -> PathBuf {
    let token = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "turkuaz-office-recent-{}-{token}",
        std::process::id()
    ))
}

#[test]
fn recent_files_are_canonical_deduplicated_limited_and_pruned() {
    let root = unique_root();
    fs::create_dir_all(root.join("nested")).expect("root");
    let metadata_path = root.join("recent-files.yml");
    let service = RecentFilesService::new(RecentFilesRepository::new(metadata_path));

    let foreign_path = root.join("foreign.docx");
    fs::write(&foreign_path, b"foreign").expect("foreign fixture");
    assert_eq!(
        service.record(foreign_path.to_string_lossy().as_ref()),
        Err(RecentFilesError::InvalidPath)
    );

    let mut paths = Vec::new();
    for index in 0..(RECENT_FILES_MAX_ENTRIES + 2) {
        let path = root.join(format!("document-{index}.tko"));
        fs::write(&path, b"fixture").expect("fixture");
        service
            .record(path.to_string_lossy().as_ref())
            .expect("record");
        paths.push(path);
    }

    let list = service.list().expect("list");
    assert_eq!(list.len(), RECENT_FILES_MAX_ENTRIES);
    assert_eq!(
        list.first().expect("latest").title,
        format!("document-{}", RECENT_FILES_MAX_ENTRIES + 1)
    );

    let duplicate_path = root
        .join("nested")
        .join("..")
        .join(format!("document-{}.tko", RECENT_FILES_MAX_ENTRIES));
    let list = service
        .record(duplicate_path.to_string_lossy().as_ref())
        .expect("record duplicate");
    assert_eq!(list.len(), RECENT_FILES_MAX_ENTRIES);
    assert_eq!(
        list.first().expect("duplicate first").title,
        format!("document-{}", RECENT_FILES_MAX_ENTRIES)
    );

    let removed = paths.get(RECENT_FILES_MAX_ENTRIES).expect("removed path");
    fs::remove_file(removed).expect("remove");
    let list = service.list().expect("pruned list");
    assert!(
        list.iter()
            .all(|entry| entry.title != format!("document-{}", RECENT_FILES_MAX_ENTRIES))
    );

    let _ = fs::remove_dir_all(root);
}
