// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/tests/tko_package_tests.rs
// # 📌 Amac: TKO v1 ZIP+YAML serializer round-trip ve guvenlik sinirlarini regression testiyle kilitler
// # 📌 Modul - FileType: Test - Rust
// # Version: 0.2.0
// # Aciklama: Rich-text round-trip, schema, manifest, ZIP allowlist ve traversal guvenligini dogrular
// Bagimli Oldugu Katman: Service -> Tool

use std::io::{Cursor, Write};
use turkuaz_office_core::DocumentSchemaVersion;
use turkuaz_office_writer::config::constants::{TKO_MANIFEST_ENTRY, TKO_WRITER_CONTENT_ENTRY};

use turkuaz_office_writer::tools::tko_archive_tool::{TkoArchiveError, TkoArchiveTool};
use turkuaz_office_writer::{
    CharacterStylePatch, InMemoryWriterDocumentRepository, ParagraphStylePatch,
    SequentialWriterIdTool, TextAlignment, TextPosition, TextRange, TkoPackageError,
    TkoPackageService, TkoProfileError, WriterCommand, WriterEditorService,
};

fn editor() -> WriterEditorService<InMemoryWriterDocumentRepository, SequentialWriterIdTool> {
    WriterEditorService::new(
        InMemoryWriterDocumentRepository::new(),
        SequentialWriterIdTool::new(),
    )
}

#[test]
fn tko_round_trip_preserves_writer_content_and_styles() {
    let mut editor = editor();
    let created = editor.create_document("Round Trip");
    let paragraph = match &created.sections[0].blocks[0] {
        turkuaz_office_writer::Block::Paragraph(value) => value,
        _ => panic!("expected paragraph"),
    };
    let position = TextPosition {
        paragraph_id: paragraph.id.clone(),
        run_id: paragraph.runs[0].id.clone(),
        offset: 0,
    };
    let document = editor
        .execute(
            &created.id,
            WriterCommand::InsertText {
                position: position.clone(),
                text: "Turkuaz Office".to_owned(),
            },
        )
        .expect("insert text");
    let paragraph = match &document.sections[0].blocks[0] {
        turkuaz_office_writer::Block::Paragraph(value) => value,
        _ => panic!("expected paragraph"),
    };
    let range = TextRange {
        anchor: TextPosition {
            paragraph_id: paragraph.id.clone(),
            run_id: paragraph.runs[0].id.clone(),
            offset: 0,
        },
        focus: TextPosition {
            paragraph_id: paragraph.id.clone(),
            run_id: paragraph.runs[0].id.clone(),
            offset: 7,
        },
    };
    let document = editor
        .execute(
            &created.id,
            WriterCommand::ApplyCharacterStyle {
                range,
                patch: CharacterStylePatch {
                    bold: Some(true),
                    font_family: Some("Georgia".to_owned()),
                    font_size_half_points: Some(28),
                    ..CharacterStylePatch::default()
                },
            },
        )
        .expect("style text");
    let paragraph_id = match &document.sections[0].blocks[0] {
        turkuaz_office_writer::Block::Paragraph(value) => value.id.clone(),
        _ => panic!("expected paragraph"),
    };
    let document = editor
        .execute(
            &created.id,
            WriterCommand::ApplyParagraphStyle {
                paragraph_id,
                patch: ParagraphStylePatch {
                    alignment: Some(TextAlignment::Center),
                },
            },
        )
        .expect("align paragraph");

    let bytes = TkoPackageService::serialize(&document, "0.2.0").expect("serialize TKO");
    let restored = TkoPackageService::deserialize(&bytes).expect("deserialize TKO");
    assert_eq!(restored, document);
}

#[test]
fn tko_serialize_rejects_future_schema_document() {
    let mut editor = editor();
    let mut document = editor.create_document("Future Save");
    document.schema_version =
        DocumentSchemaVersion::new(DocumentSchemaVersion::current().value().saturating_add(1));

    assert!(matches!(
        TkoPackageService::serialize(&document, "0.2.0"),
        Err(TkoPackageError::Profile(TkoProfileError::FutureSchema))
    ));
}

#[test]
fn tko_future_schema_package_is_rejected_on_open() {
    use turkuaz_office_writer::services::tko_package_types::{
        TkoManifestDtoV1, WriterContentDtoV1,
    };
    use turkuaz_office_writer::tools::tko_yaml_tool::TkoYamlTool;

    let mut editor = editor();
    let document = editor.create_document("Future Open");
    let bytes = TkoPackageService::serialize(&document, "0.2.0").expect("serialize TKO");
    let entries = TkoArchiveTool::decode(&bytes).expect("decode archive");
    let mut manifest: TkoManifestDtoV1 =
        TkoYamlTool::deserialize(entries.get(TKO_MANIFEST_ENTRY).expect("manifest"))
            .expect("manifest yaml");
    let mut content: WriterContentDtoV1 =
        TkoYamlTool::deserialize(entries.get(TKO_WRITER_CONTENT_ENTRY).expect("content"))
            .expect("content yaml");
    let future_schema = DocumentSchemaVersion::current().value().saturating_add(1);
    manifest.schema_version = future_schema;
    content.schema_version = future_schema;
    let manifest_bytes = TkoYamlTool::serialize(&manifest).expect("manifest yaml write");
    let content_bytes = TkoYamlTool::serialize(&content).expect("content yaml write");
    let modified = TkoArchiveTool::encode(&[
        (TKO_MANIFEST_ENTRY, manifest_bytes.as_slice()),
        (TKO_WRITER_CONTENT_ENTRY, content_bytes.as_slice()),
    ])
    .expect("encode archive");

    assert!(matches!(
        TkoPackageService::deserialize(&modified),
        Err(TkoPackageError::Profile(TkoProfileError::FutureSchema))
    ));
}

#[test]
fn tko_old_schema_package_requires_migration() {
    use turkuaz_office_writer::services::tko_package_types::{
        TkoManifestDtoV1, WriterContentDtoV1,
    };
    use turkuaz_office_writer::tools::tko_yaml_tool::TkoYamlTool;

    let mut editor = editor();
    let document = editor.create_document("Old Schema");
    let bytes = TkoPackageService::serialize(&document, "0.2.0").expect("serialize TKO");
    let entries = TkoArchiveTool::decode(&bytes).expect("decode archive");
    let mut manifest: TkoManifestDtoV1 =
        TkoYamlTool::deserialize(entries.get(TKO_MANIFEST_ENTRY).expect("manifest"))
            .expect("manifest yaml");
    let mut content: WriterContentDtoV1 =
        TkoYamlTool::deserialize(entries.get(TKO_WRITER_CONTENT_ENTRY).expect("content"))
            .expect("content yaml");
    let old_schema = DocumentSchemaVersion::current().value().saturating_sub(1);
    manifest.schema_version = old_schema;
    content.schema_version = old_schema;
    let manifest_bytes = TkoYamlTool::serialize(&manifest).expect("manifest yaml write");
    let content_bytes = TkoYamlTool::serialize(&content).expect("content yaml write");
    let modified = TkoArchiveTool::encode(&[
        (TKO_MANIFEST_ENTRY, manifest_bytes.as_slice()),
        (TKO_WRITER_CONTENT_ENTRY, content_bytes.as_slice()),
    ])
    .expect("encode archive");

    assert!(matches!(
        TkoPackageService::deserialize(&modified),
        Err(TkoPackageError::Profile(TkoProfileError::MigrationRequired))
    ));
}

#[test]
fn tko_missing_manifest_is_rejected() {
    let mut editor = editor();
    let document = editor.create_document("Missing Manifest");
    let bytes = TkoPackageService::serialize(&document, "0.2.0").expect("serialize TKO");
    let entries = TkoArchiveTool::decode(&bytes).expect("decode archive");
    let content = entries.get(TKO_WRITER_CONTENT_ENTRY).expect("content");
    let modified = TkoArchiveTool::encode(&[(TKO_WRITER_CONTENT_ENTRY, content.as_slice())])
        .expect("encode archive");

    assert!(matches!(
        TkoPackageService::deserialize(&modified),
        Err(TkoPackageError::MissingManifest)
    ));
}

#[test]
fn tko_missing_writer_content_is_rejected() {
    let mut editor = editor();
    let document = editor.create_document("Missing Content");
    let bytes = TkoPackageService::serialize(&document, "0.2.0").expect("serialize TKO");
    let entries = TkoArchiveTool::decode(&bytes).expect("decode archive");
    let manifest = entries.get(TKO_MANIFEST_ENTRY).expect("manifest");
    let modified = TkoArchiveTool::encode(&[(TKO_MANIFEST_ENTRY, manifest.as_slice())])
        .expect("encode archive");

    assert!(matches!(
        TkoPackageService::deserialize(&modified),
        Err(TkoPackageError::MissingWriterContent)
    ));
}

#[test]
fn tko_manifest_content_mismatch_is_rejected() {
    use turkuaz_office_writer::services::tko_package_types::TkoManifestDtoV1;
    use turkuaz_office_writer::tools::tko_yaml_tool::TkoYamlTool;

    let mut editor = editor();
    let document = editor.create_document("Mismatch");
    let bytes = TkoPackageService::serialize(&document, "0.2.0").expect("serialize TKO");
    let entries = TkoArchiveTool::decode(&bytes).expect("decode archive");
    let mut manifest: TkoManifestDtoV1 =
        TkoYamlTool::deserialize(entries.get(TKO_MANIFEST_ENTRY).expect("manifest"))
            .expect("manifest yaml");
    manifest.document_id = "doc-mismatch".to_owned();
    let manifest_bytes = TkoYamlTool::serialize(&manifest).expect("manifest yaml write");
    let content = entries.get(TKO_WRITER_CONTENT_ENTRY).expect("content");
    let modified = TkoArchiveTool::encode(&[
        (TKO_MANIFEST_ENTRY, manifest_bytes.as_slice()),
        (TKO_WRITER_CONTENT_ENTRY, content.as_slice()),
    ])
    .expect("encode archive");

    assert!(matches!(
        TkoPackageService::deserialize(&modified),
        Err(TkoPackageError::Profile(
            TkoProfileError::ManifestContentMismatch
        ))
    ));
}

#[test]
fn tko_unexpected_entry_is_rejected() {
    let mut editor = editor();
    let document = editor.create_document("Extra Entry");
    let bytes = TkoPackageService::serialize(&document, "0.2.0").expect("serialize TKO");
    let entries = TkoArchiveTool::decode(&bytes).expect("decode archive");
    let manifest = entries.get(TKO_MANIFEST_ENTRY).expect("manifest");
    let content = entries.get(TKO_WRITER_CONTENT_ENTRY).expect("content");
    let modified = TkoArchiveTool::encode(&[
        (TKO_MANIFEST_ENTRY, manifest.as_slice()),
        (TKO_WRITER_CONTENT_ENTRY, content.as_slice()),
        ("unexpected.txt", b"no"),
    ])
    .expect("encode archive");
    assert!(matches!(
        TkoPackageService::deserialize(&modified),
        Err(TkoPackageError::UnexpectedEntry)
    ));
}

#[test]
fn archive_tool_rejects_traversal_entry_name() {
    let result = TkoArchiveTool::encode(&[("../manifest.yml", b"bad")]);
    assert!(result.is_err());
}

#[test]
fn archive_tool_rejects_directory_entry() {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    writer
        .add_directory("content/", options)
        .expect("directory fixture");
    writer
        .start_file(TKO_MANIFEST_ENTRY, options)
        .expect("manifest fixture");
    writer
        .write_all(b"format_version: 1\n")
        .expect("fixture write");
    let bytes = writer.finish().expect("finish fixture").into_inner();

    assert!(matches!(
        TkoArchiveTool::decode(&bytes),
        Err(TkoArchiveError::DirectoryEntryUnsupported)
    ));
}

#[test]
fn archive_tool_rejects_duplicate_entry() {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    writer
        .start_file(TKO_MANIFEST_ENTRY, options)
        .expect("first manifest fixture");
    writer.write_all(b"first").expect("first fixture write");
    let duplicate = writer.start_file(TKO_MANIFEST_ENTRY, options);

    assert!(duplicate.is_err());
}

#[test]
fn editor_load_resets_history_and_exposes_snapshot() {
    let mut source = editor();
    let document = source.create_document("Loaded");
    let mut target = editor();
    let loaded = target.load_document(document.clone());
    assert_eq!(loaded, document);
    assert_eq!(target.get_document(&document.id), Some(document.clone()));
    assert!(matches!(
        target.undo(&document.id),
        Err(turkuaz_office_writer::WriterEditorError::NothingToUndo)
    ));
}

#[test]
fn tko_round_trip_preserves_binary_image_asset() {
    let mut editor = editor();
    let document = editor.create_document("Asset Round Trip");
    let paragraph_id = match &document.sections[0].blocks[0] {
        turkuaz_office_writer::Block::Paragraph(value) => value.id.clone(),
        _ => panic!("expected paragraph"),
    };
    let png = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    let document = editor
        .execute(
            &document.id,
            WriterCommand::InsertImageData {
                after_paragraph_id: paragraph_id,
                media_type: "image/png".to_owned(),
                data: png,
                alt_text: "Round trip".to_owned(),
                width_twips: Some(1440),
                height_twips: Some(1440),
            },
        )
        .expect("image insert");

    let bytes = TkoPackageService::serialize(&document, "0.2.0").expect("serialize asset TKO");
    let entries = TkoArchiveTool::decode(&bytes).expect("decode asset archive");
    assert!(entries.contains_key(turkuaz_office_writer::config::constants::TKO_ASSET_INDEX_ENTRY));
    assert!(entries.keys().any(|name| {
        name.starts_with(turkuaz_office_writer::config::constants::TKO_ASSET_DATA_PREFIX)
    }));

    let restored = TkoPackageService::deserialize(&bytes).expect("deserialize asset TKO");
    assert_eq!(restored, document);
}

#[test]
fn tko_asset_index_missing_binary_entry_is_rejected() {
    let mut editor = editor();
    let document = editor.create_document("Missing Asset");
    let paragraph_id = match &document.sections[0].blocks[0] {
        turkuaz_office_writer::Block::Paragraph(value) => value.id.clone(),
        _ => panic!("expected paragraph"),
    };
    let document = editor
        .execute(
            &document.id,
            WriterCommand::InsertImageData {
                after_paragraph_id: paragraph_id,
                media_type: "image/png".to_owned(),
                data: vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A],
                alt_text: String::new(),
                width_twips: None,
                height_twips: None,
            },
        )
        .expect("image insert");
    let bytes = TkoPackageService::serialize(&document, "0.2.0").expect("serialize");
    let mut entries = TkoArchiveTool::decode(&bytes).expect("decode");
    let asset_entry = entries
        .keys()
        .find(|name| {
            name.starts_with(turkuaz_office_writer::config::constants::TKO_ASSET_DATA_PREFIX)
        })
        .cloned()
        .expect("asset entry");
    entries.remove(&asset_entry);

    let owned = entries.into_iter().collect::<Vec<_>>();
    let borrowed = owned
        .iter()
        .map(|(name, data)| (name.as_str(), data.as_slice()))
        .collect::<Vec<_>>();
    let modified = TkoArchiveTool::encode(&borrowed).expect("re-encode");

    assert!(matches!(
        TkoPackageService::deserialize(&modified),
        Err(TkoPackageError::MissingAssetEntry)
    ));
}
