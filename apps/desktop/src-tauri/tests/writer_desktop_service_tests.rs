// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/tests/writer_desktop_service_tests.rs
// # 📌 Amac: Desktop Writer Service input-to-domain command koprusunu regression testleriyle dogrular
// # 📌 Modul - FileType: Test - Rust
// # Version: 0.2.0
// # Aciklama: Editor commandlari ile local TKO save/open ve error contractlarini Tauri runtime olmadan test eder
// Bagimli Oldugu Katman: Service

use turkuaz_office_desktop_lib::config::constants::{
    ERROR_FILE_EXTENSION_INVALID, ERROR_RECOVERY_INVALID,
};
use turkuaz_office_desktop_lib::services::writer_desktop_service::WriterDesktopService;
use turkuaz_office_writer::{CharacterStyle, CharacterStylePatch, TextAlignment};

#[test]
fn replace_split_merge_and_undo_flow_is_stable() {
    let mut service = WriterDesktopService::new();
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();

    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "Alpha Beta")
        .expect("replace should succeed");
    assert_eq!(document.plain_text, "Alpha Beta");
    assert_eq!(document.revision, 1);

    let document = service
        .split_paragraph(&document.id, &paragraph_id, 5)
        .expect("split should succeed");
    assert_eq!(document.paragraphs.len(), 2);
    assert_eq!(document.paragraphs[0].plain_text, "Alpha");
    assert_eq!(document.paragraphs[1].plain_text, " Beta");

    let second_id = document.paragraphs[1].id.clone();
    let document = service
        .merge_with_previous(&document.id, &second_id)
        .expect("merge should succeed");
    assert_eq!(document.paragraphs.len(), 1);
    assert_eq!(document.plain_text, "Alpha Beta");

    let document = service.undo(&document.id).expect("undo should succeed");
    assert_eq!(document.paragraphs.len(), 2);

    let document = service.redo(&document.id).expect("redo should succeed");
    assert_eq!(document.paragraphs.len(), 1);
}

#[test]
fn unicode_offset_is_logical_not_utf8_byte_based() {
    let mut service = WriterDesktopService::new();
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();

    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "A\u{1F642}B")
        .expect("unicode replace should succeed");
    let document = service
        .split_paragraph(&document.id, &paragraph_id, 2)
        .expect("unicode split should succeed");

    assert_eq!(document.paragraphs[0].plain_text, "A\u{1F642}");
    assert_eq!(document.paragraphs[1].plain_text, "B");
}
#[test]
fn inline_style_and_minimal_text_diff_preserve_run_formatting() {
    let mut service = WriterDesktopService::new();
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();

    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "Alpha Beta")
        .expect("replace should succeed");
    let document = service
        .apply_character_style(
            &document.id,
            &paragraph_id,
            6,
            10,
            CharacterStylePatch {
                bold: Some(true),
                italic: None,
                underline: None,
                ..CharacterStylePatch::default()
            },
        )
        .expect("style should succeed");
    assert_eq!(document.paragraphs[0].runs.len(), 2);
    assert!(document.paragraphs[0].runs[1].style.bold);

    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "Alpha Beta!")
        .expect("minimal diff should succeed");
    assert_eq!(document.plain_text, "Alpha Beta!");
    assert!(document.paragraphs[0].runs.iter().any(|run| run.style.bold));
}

#[test]
fn unicode_minimal_diff_uses_scalar_offsets() {
    let mut service = WriterDesktopService::new();
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();

    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "A\u{1F642}B")
        .expect("initial unicode replace should succeed");
    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "A\u{1F642}XB")
        .expect("unicode diff should succeed");

    assert_eq!(document.plain_text, "A\u{1F642}XB");
}

#[test]
fn typing_style_is_applied_only_to_inserted_diff() {
    let mut service = WriterDesktopService::new();
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();
    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "AB")
        .expect("initial text should succeed");

    let typing_style = CharacterStyle {
        bold: true,
        font_family: "Georgia".to_owned(),
        font_size_half_points: 28,
        ..CharacterStyle::default()
    };
    let document = service
        .replace_paragraph_text_with_style(
            &document.id,
            &paragraph_id,
            "AXB",
            Some(typing_style.clone()),
        )
        .expect("styled diff should succeed");

    assert_eq!(document.paragraphs[0].plain_text, "AXB");
    assert_eq!(document.paragraphs[0].runs.len(), 3);
    assert_eq!(document.paragraphs[0].runs[1].text, "X");
    assert_eq!(document.paragraphs[0].runs[1].style, typing_style);
}

#[test]
fn font_patch_and_paragraph_alignment_are_exposed_in_read_model() {
    let mut service = WriterDesktopService::new();
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();
    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "Office")
        .expect("text should succeed");
    let document = service
        .apply_character_style(
            &document.id,
            &paragraph_id,
            0,
            6,
            CharacterStylePatch {
                font_family: Some("Georgia".to_owned()),
                font_size_half_points: Some(36),
                ..CharacterStylePatch::default()
            },
        )
        .expect("font patch should succeed");
    let document = service
        .apply_paragraph_alignment(&document.id, &paragraph_id, TextAlignment::Right)
        .expect("alignment should succeed");

    assert_eq!(document.paragraphs[0].runs[0].style.font_family, "Georgia");
    assert_eq!(
        document.paragraphs[0].runs[0].style.font_size_half_points,
        36
    );
    assert_eq!(document.paragraphs[0].style.alignment, TextAlignment::Right);
}

fn temp_file_stem(name: &str) -> std::path::PathBuf {
    let token = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "turkuaz-office-{name}-{}-{token}",
        std::process::id()
    ))
}

#[test]
fn local_tko_save_open_round_trip_preserves_document() {
    let mut service = WriterDesktopService::new();
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();
    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "Local TKO")
        .expect("replace should succeed");

    let stem = temp_file_stem("round-trip");
    let path_text = stem.to_string_lossy().into_owned();
    let (_, saved_path) = service
        .save_document(&document.id, &path_text)
        .expect("save should succeed");
    assert!(saved_path.ends_with(".tko"));

    let mut reopened_service = WriterDesktopService::new();
    let reopened = reopened_service
        .open_document(&saved_path)
        .expect("open should succeed");
    assert_eq!(reopened.plain_text, "Local TKO");
    assert_eq!(reopened.revision, document.revision);

    let _ = std::fs::remove_file(saved_path);
}

#[test]
fn local_open_rejects_non_tko_extension() {
    let mut service = WriterDesktopService::new();
    let path = temp_file_stem("invalid-extension").with_extension("txt");
    std::fs::write(&path, b"not a tko").expect("fixture write");
    let error = service
        .open_document(path.to_string_lossy().as_ref())
        .expect_err("non-TKO extension should fail");
    assert_eq!(error.code, ERROR_FILE_EXTENSION_INVALID);
    let _ = std::fs::remove_file(path);
}

#[test]
fn recovery_snapshot_can_be_listed_restored_and_discarded() {
    let recovery_root = temp_file_stem("recovery-round-trip");
    std::fs::create_dir_all(&recovery_root).expect("recovery root");
    let mut service = WriterDesktopService::with_recovery_root(recovery_root.clone());
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();
    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "Recover me")
        .expect("edit should succeed");
    let snapshot = service
        .create_recovery_snapshot(&document.id, None, 0)
        .expect("snapshot should succeed");

    let listed = service
        .list_recovery_snapshots()
        .expect("list should succeed");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].snapshot_id, snapshot.snapshot_id);

    let mut restored_service = WriterDesktopService::with_recovery_root(recovery_root.clone());
    let (restored, restored_snapshot) = restored_service
        .restore_recovery_snapshot(&snapshot.snapshot_id)
        .expect("restore should succeed");
    assert_eq!(restored.plain_text, "Recover me");
    assert_eq!(restored_snapshot.recovery_revision, document.revision);

    restored_service
        .discard_recovery_snapshot(&snapshot.snapshot_id)
        .expect("discard should succeed");
    assert!(
        restored_service
            .list_recovery_snapshots()
            .expect("list after discard")
            .is_empty()
    );
    let _ = std::fs::remove_dir_all(recovery_root);
}

#[test]
fn recovery_compare_uses_saved_source_when_available() {
    let recovery_root = temp_file_stem("recovery-compare");
    std::fs::create_dir_all(&recovery_root).expect("recovery root");
    let mut service = WriterDesktopService::with_recovery_root(recovery_root.clone());
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();
    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "Disk copy")
        .expect("disk text");
    let save_stem = temp_file_stem("recovery-source");
    let (_, saved_path) = service
        .save_document(&document.id, save_stem.to_string_lossy().as_ref())
        .expect("source save");
    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "Disk copy plus recovery")
        .expect("recovery text");
    let snapshot = service
        .create_recovery_snapshot(&document.id, Some(&saved_path), 1)
        .expect("snapshot");
    let comparison = service
        .compare_recovery_snapshot(&snapshot.snapshot_id)
        .expect("compare");

    assert_eq!(comparison.source_plain_text.as_deref(), Some("Disk copy"));
    assert_eq!(comparison.recovery_plain_text, "Disk copy plus recovery");
    assert_eq!(comparison.source_revision, Some(1));

    let _ = std::fs::remove_file(saved_path);
    let _ = std::fs::remove_dir_all(recovery_root);
}

#[test]
fn explicit_save_clears_recovery_snapshots_for_document() {
    let recovery_root = temp_file_stem("recovery-clear-on-save");
    std::fs::create_dir_all(&recovery_root).expect("recovery root");
    let mut service = WriterDesktopService::with_recovery_root(recovery_root.clone());
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();
    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "Dirty")
        .expect("edit");
    service
        .create_recovery_snapshot(&document.id, None, 0)
        .expect("snapshot");
    assert_eq!(service.list_recovery_snapshots().expect("list").len(), 1);

    let save_stem = temp_file_stem("recovery-save-clear");
    let (_, saved_path) = service
        .save_document(&document.id, save_stem.to_string_lossy().as_ref())
        .expect("save");
    assert!(
        service
            .list_recovery_snapshots()
            .expect("list after save")
            .is_empty()
    );

    let _ = std::fs::remove_file(saved_path);
    let _ = std::fs::remove_dir_all(recovery_root);
}

#[test]
fn recovery_retention_keeps_latest_five_snapshots() {
    let recovery_root = temp_file_stem("recovery-retention");
    std::fs::create_dir_all(&recovery_root).expect("recovery root");
    let mut service = WriterDesktopService::with_recovery_root(recovery_root.clone());
    let document = service.create_document();
    let document_id = document.id.clone();
    let paragraph_id = document.paragraphs[0].id.clone();

    for index in 0..6 {
        let document = service
            .replace_paragraph_text(&document_id, &paragraph_id, &format!("Revision {index}"))
            .expect("edit");
        service
            .create_recovery_snapshot(&document.id, None, 0)
            .expect("snapshot");
    }

    assert_eq!(service.list_recovery_snapshots().expect("list").len(), 5);
    let _ = std::fs::remove_dir_all(recovery_root);
}

#[test]
fn recovery_snapshot_id_rejects_path_traversal() {
    let recovery_root = temp_file_stem("recovery-traversal");
    std::fs::create_dir_all(&recovery_root).expect("recovery root");
    let service = WriterDesktopService::with_recovery_root(recovery_root.clone());
    let error = service
        .discard_recovery_snapshot("../outside")
        .expect_err("traversal snapshot id must fail");
    assert_eq!(error.code, ERROR_RECOVERY_INVALID);
    let _ = std::fs::remove_dir_all(recovery_root);
}

#[test]
fn second_process_style_session_opens_locked_document_read_only() {
    use turkuaz_office_desktop_lib::config::constants::ERROR_FILE_LOCKED;

    let save_stem = temp_file_stem("lock-read-only");
    let saved_path;
    {
        let mut owner = WriterDesktopService::new();
        let document = owner.create_document();
        let paragraph_id = document.paragraphs[0].id.clone();
        let document = owner
            .replace_paragraph_text(&document.id, &paragraph_id, "Owner")
            .expect("owner edit");
        let (_, path) = owner
            .save_document(&document.id, save_stem.to_string_lossy().as_ref())
            .expect("owner save");
        saved_path = path;

        let mut reader = WriterDesktopService::new();
        let opened = reader.open_document(&saved_path).expect("read-only open");
        let status = reader.file_session_status(&opened.id).expect("status");
        assert!(status.read_only);
        assert!(!status.lock_owned);

        let error = reader
            .replace_paragraph_text(&opened.id, &opened.paragraphs[0].id, "Blocked")
            .expect_err("read-only mutation must fail");
        assert_eq!(error.code, ERROR_FILE_LOCKED);
    }
    let _ = std::fs::remove_file(saved_path);
}

#[test]
fn external_change_blocks_same_path_save_until_explicit_acknowledge() {
    use turkuaz_office_desktop_lib::config::constants::ERROR_EXTERNAL_CHANGE_CONFLICT;
    use turkuaz_office_desktop_lib::services::writer_file_session_service::WriterExternalChangeState;

    let save_stem = temp_file_stem("external-change-conflict");
    let mut service = WriterDesktopService::new();
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();
    let document = service
        .replace_paragraph_text(&document.id, &paragraph_id, "Local")
        .expect("edit");
    let (_, saved_path) = service
        .save_document(&document.id, save_stem.to_string_lossy().as_ref())
        .expect("initial save");

    let mut bytes = std::fs::read(&saved_path).expect("read fixture");
    bytes.extend_from_slice(b"external-change");
    std::fs::write(&saved_path, bytes).expect("external write");

    let status = service
        .file_session_status(&document.id)
        .expect("modified status");
    assert_eq!(status.external_state, WriterExternalChangeState::Modified);
    let error = service
        .save_document(&document.id, &saved_path)
        .expect_err("silent overwrite must fail");
    assert_eq!(error.code, ERROR_EXTERNAL_CHANGE_CONFLICT);

    service
        .acknowledge_external_change(&document.id)
        .expect("explicit keep-local acknowledgement");
    service
        .save_document(&document.id, &saved_path)
        .expect("save after acknowledgement");
    let status = service
        .file_session_status(&document.id)
        .expect("clean status");
    assert_eq!(status.external_state, WriterExternalChangeState::Unchanged);

    drop(service);
    let _ = std::fs::remove_file(saved_path);
}

#[test]
fn image_asset_insert_and_fetch_are_exposed_by_desktop_service() {
    let mut service = WriterDesktopService::new();
    let document = service.create_document();
    let paragraph_id = document.paragraphs[0].id.clone();
    let png = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

    let document = service
        .insert_image_data(
            &document.id,
            &paragraph_id,
            "image/png",
            png.clone(),
            "Clipboard image",
            None,
            None,
        )
        .expect("image insert");
    assert_eq!(document.images.len(), 1);

    let asset = service
        .get_asset(&document.id, &document.images[0].asset_id)
        .expect("asset fetch");
    assert_eq!(asset.media_type, "image/png");
    assert_eq!(asset.bytes, png);
}
