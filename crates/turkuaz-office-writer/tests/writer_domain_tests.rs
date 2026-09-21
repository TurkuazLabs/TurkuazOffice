// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/tests/writer_domain_tests.rs
// # 📌 Amac: Writer v0.2.0 headless domain kabul senaryolarini test eder
// # 📌 Modul - FileType: Writer - Rust Test
// # Version: 0.2.0
// # Aciklama: Create, insert, Unicode offset, split, merge, delete, undo/redo, table ve TKO profile round-trip testlerini kapsar
// Bagimli Oldugu Katman: Service -> Repo -> Tool

use turkuaz_office_writer::WriterController;
use turkuaz_office_writer::repositories::writer_document_repository::InMemoryWriterDocumentRepository;
use turkuaz_office_writer::services::tko_profile_service::{TkoProfileError, TkoProfileService};
use turkuaz_office_writer::services::writer_command::{WriterCommand, WriterCommandError};
use turkuaz_office_writer::services::writer_command_service::WriterCommandService;
use turkuaz_office_writer::services::writer_document_factory_service::WriterDocumentFactoryService;
use turkuaz_office_writer::services::writer_editor_service::WriterEditorService;
use turkuaz_office_writer::services::writer_types::{
    Block, CharacterStyle, CharacterStylePatch, NodeId, ParagraphStylePatch, TextAlignment,
    TextPosition, TextRange, TextRun,
};
use turkuaz_office_writer::tools::writer_id_tool::{SequentialWriterIdTool, WriterIdTool};

fn service() -> WriterEditorService<InMemoryWriterDocumentRepository, SequentialWriterIdTool> {
    WriterEditorService::new(
        InMemoryWriterDocumentRepository::new(),
        SequentialWriterIdTool::new(),
    )
}

#[test]
fn new_document_has_one_section_paragraph_and_run() {
    let mut service = service();
    let document = service.create_document("Test");
    assert_eq!(document.sections.len(), 1);
    assert_eq!(document.sections[0].blocks.len(), 1);
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("first block should be paragraph");
    };
    assert_eq!(paragraph.runs.len(), 1);
    assert_eq!(document.revision, 0);
}

#[test]
fn empty_title_uses_core_default_title() {
    let mut service = service();
    let document = service.create_document("   ");
    assert_eq!(
        document.title,
        turkuaz_office_core::config::constants::DEFAULT_DOCUMENT_TITLE
    );
}

#[test]
fn insert_text_uses_unicode_scalar_offset_and_undo_redo_is_monotonic() {
    let mut service = service();
    let document = service.create_document("Unicode");
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    let position = TextPosition {
        paragraph_id: paragraph.id.clone(),
        run_id: paragraph.runs[0].id.clone(),
        offset: 0,
    };
    let updated = service
        .execute(
            &document.id,
            WriterCommand::InsertText {
                position: position.clone(),
                text: "Merhaba dunya".to_owned(),
            },
        )
        .expect("insert should succeed");
    assert_eq!(updated.plain_text(), "Merhaba dunya");
    assert_eq!(updated.revision, 1);

    let updated = service
        .execute(
            &document.id,
            WriterCommand::InsertText {
                position: TextPosition {
                    offset: 7,
                    ..position
                },
                text: "Turkuaz ".to_owned(),
            },
        )
        .expect("unicode-safe insert should succeed");
    assert_eq!(updated.plain_text(), "Merhaba Turkuaz dunya");
    assert_eq!(updated.revision, 2);

    let undone = service.undo(&document.id).expect("undo should succeed");
    assert_eq!(undone.plain_text(), "Merhaba dunya");
    assert_eq!(undone.revision, 3);

    let redone = service.redo(&document.id).expect("redo should succeed");
    assert_eq!(redone.plain_text(), "Merhaba Turkuaz dunya");
    assert_eq!(redone.revision, 4);
}

#[test]
fn insert_text_offset_counts_unicode_scalars_not_utf8_bytes() {
    let mut service = service();
    let document = service.create_document("UnicodeOffset");
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    let paragraph_id = paragraph.id.clone();
    let run_id = paragraph.runs[0].id.clone();
    let document = service
        .execute(
            &document.id,
            WriterCommand::InsertText {
                position: TextPosition {
                    paragraph_id: paragraph_id.clone(),
                    run_id: run_id.clone(),
                    offset: 0,
                },
                text: "A\u{0131}B".to_owned(),
            },
        )
        .expect("first insert should succeed");
    let document = service
        .execute(
            &document.id,
            WriterCommand::InsertText {
                position: TextPosition {
                    paragraph_id,
                    run_id,
                    offset: 2,
                },
                text: "X".to_owned(),
            },
        )
        .expect("scalar offset insert should succeed");
    assert_eq!(document.plain_text(), "A\u{0131}XB");
}

#[test]
fn split_merge_and_delete_range_preserve_document_flow() {
    let mut service = service();
    let document = service.create_document("Edit");
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    let paragraph_id = paragraph.id.clone();
    let run_id = paragraph.runs[0].id.clone();

    let document = service
        .execute(
            &document.id,
            WriterCommand::InsertText {
                position: TextPosition {
                    paragraph_id: paragraph_id.clone(),
                    run_id: run_id.clone(),
                    offset: 0,
                },
                text: "AlphaBeta".to_owned(),
            },
        )
        .expect("insert should succeed");
    let document = service
        .execute(
            &document.id,
            WriterCommand::SplitParagraph {
                position: TextPosition {
                    paragraph_id: paragraph_id.clone(),
                    run_id: run_id.clone(),
                    offset: 5,
                },
            },
        )
        .expect("split should succeed");
    assert_eq!(document.plain_text(), "Alpha\nBeta");

    let first = match &document.sections[0].blocks[0] {
        Block::Paragraph(paragraph) => paragraph,
        _ => panic!("first paragraph expected"),
    };
    let second = match &document.sections[0].blocks[1] {
        Block::Paragraph(paragraph) => paragraph,
        _ => panic!("second paragraph expected"),
    };
    let first_id = first.id.clone();
    let second_id = second.id.clone();
    let first_run = first.runs[0].id.clone();
    let second_run = second.runs[0].id.clone();

    let document = service
        .execute(
            &document.id,
            WriterCommand::DeleteRange {
                range: TextRange {
                    anchor: TextPosition {
                        paragraph_id: first_id.clone(),
                        run_id: first_run,
                        offset: 2,
                    },
                    focus: TextPosition {
                        paragraph_id: second_id.clone(),
                        run_id: second_run,
                        offset: 2,
                    },
                },
            },
        )
        .expect("cross paragraph delete should succeed");
    assert_eq!(document.plain_text(), "Alta");

    let Block::Paragraph(merged) = &document.sections[0].blocks[0] else {
        panic!("merged paragraph expected");
    };
    assert_eq!(merged.id, first_id);
}

#[test]
fn cross_run_delete_keeps_suffix_before_later_runs() {
    let id_tool = SequentialWriterIdTool::new();
    let mut document = WriterDocumentFactoryService::create(&id_tool, "Runs");
    let Block::Paragraph(paragraph) = &mut document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    paragraph.runs[0].text = "AA".to_owned();
    let first_id = paragraph.runs[0].id.clone();
    let middle_id = id_tool.next_node_id();
    let last_id = id_tool.next_node_id();
    paragraph.runs.push(TextRun {
        id: middle_id.clone(),
        text: "BB".to_owned(),
        style: CharacterStyle::default(),
    });
    paragraph.runs.push(TextRun {
        id: last_id,
        text: "CC".to_owned(),
        style: CharacterStyle::default(),
    });
    let paragraph_id = paragraph.id.clone();

    WriterCommandService::apply(
        &mut document,
        &WriterCommand::DeleteRange {
            range: TextRange {
                anchor: TextPosition {
                    paragraph_id: paragraph_id.clone(),
                    run_id: first_id,
                    offset: 1,
                },
                focus: TextPosition {
                    paragraph_id,
                    run_id: middle_id,
                    offset: 1,
                },
            },
        },
        &id_tool,
    )
    .expect("cross run delete should succeed");

    assert_eq!(document.plain_text(), "ABCC");
}

#[test]
fn insert_table_creates_requested_grid() {
    let mut service = service();
    let document = service.create_document("Table");
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    let document = service
        .execute(
            &document.id,
            WriterCommand::InsertTable {
                after_paragraph_id: paragraph.id.clone(),
                rows: 2,
                columns: 3,
            },
        )
        .expect("table should be inserted");
    let Block::Table(table) = &document.sections[0].blocks[1] else {
        panic!("table expected");
    };
    assert_eq!(table.rows.len(), 2);
    assert!(table.rows.iter().all(|row| row.cells.len() == 3));
}

#[test]
fn tko_profile_rejects_old_schema_until_migration_runs() {
    let mut service = service();
    let document = service.create_document("OldSchema");
    let mut package = TkoProfileService::build(&document, "0.2.0");
    package.manifest.schema_version = turkuaz_office_core::DocumentSchemaVersion::new(0);
    package.content.schema_version = turkuaz_office_core::DocumentSchemaVersion::new(0);
    let result = TkoProfileService::restore(package);
    assert_eq!(result, Err(TkoProfileError::MigrationRequired));
}

#[test]
fn tko_logical_profile_round_trip_is_lossless() {
    let mut service = service();
    let document = service.create_document("TKO");
    let package = TkoProfileService::build(&document, "0.2.0");
    let restored = TkoProfileService::restore(package).expect("profile should restore");
    assert_eq!(restored, document);
}

#[test]
fn batch_commands_create_one_revision_and_one_undo_entry() {
    let repository = InMemoryWriterDocumentRepository::new();
    let id_tool = SequentialWriterIdTool::new();
    let service = WriterEditorService::new(repository, id_tool);
    let mut controller = WriterController::new(service);
    let document = controller.create("Batch");
    let paragraph = document.paragraphs[0].clone();
    let run = paragraph.runs[0].clone();

    let document = controller
        .execute_batch(
            &document.id,
            &[
                WriterCommand::InsertText {
                    position: TextPosition {
                        paragraph_id: NodeId::new(&paragraph.id),
                        run_id: NodeId::new(&run.id),
                        offset: 0,
                    },
                    text: "Alpha".to_owned(),
                },
                WriterCommand::InsertText {
                    position: TextPosition {
                        paragraph_id: NodeId::new(&paragraph.id),
                        run_id: NodeId::new(&run.id),
                        offset: 5,
                    },
                    text: " Beta".to_owned(),
                },
            ],
        )
        .expect("batch should succeed");

    assert_eq!(document.plain_text, "Alpha Beta");
    assert_eq!(document.revision, 1);

    let document = controller
        .undo(&document.id)
        .expect("single undo should revert batch");
    assert_eq!(document.plain_text, "");
    assert_eq!(document.revision, 2);
}
#[test]
fn apply_character_style_splits_runs_and_preserves_unselected_text() {
    let id_tool = SequentialWriterIdTool::new();
    let mut document = WriterDocumentFactoryService::create(&id_tool, "Style");
    let Block::Paragraph(paragraph) = &mut document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    paragraph.runs[0].text = "AlphaBeta".to_owned();
    paragraph.runs[0].style.italic = true;
    let paragraph_id = paragraph.id.clone();
    let run_id = paragraph.runs[0].id.clone();

    WriterCommandService::apply(
        &mut document,
        &WriterCommand::ApplyCharacterStyle {
            range: TextRange {
                anchor: TextPosition {
                    paragraph_id: paragraph_id.clone(),
                    run_id: run_id.clone(),
                    offset: 5,
                },
                focus: TextPosition {
                    paragraph_id,
                    run_id,
                    offset: 9,
                },
            },
            patch: CharacterStylePatch {
                bold: Some(true),
                italic: None,
                underline: None,
                ..CharacterStylePatch::default()
            },
        },
        &id_tool,
    )
    .expect("style range should succeed");

    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    assert_eq!(paragraph.plain_text(), "AlphaBeta");
    assert_eq!(paragraph.runs.len(), 2);
    assert_eq!(paragraph.runs[0].text, "Alpha");
    assert!(!paragraph.runs[0].style.bold);
    assert!(paragraph.runs[0].style.italic);
    assert_eq!(paragraph.runs[1].text, "Beta");
    assert!(paragraph.runs[1].style.bold);
    assert!(paragraph.runs[1].style.italic);
}

#[test]
fn applying_same_style_to_adjacent_runs_compacts_fragmentation() {
    let id_tool = SequentialWriterIdTool::new();
    let mut document = WriterDocumentFactoryService::create(&id_tool, "Compact");
    let Block::Paragraph(paragraph) = &mut document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    paragraph.runs[0].text = "AB".to_owned();
    let first_id = paragraph.runs[0].id.clone();
    let second_id = id_tool.next_node_id();
    paragraph.runs.push(TextRun {
        id: second_id.clone(),
        text: "CD".to_owned(),
        style: CharacterStyle {
            bold: true,
            ..CharacterStyle::default()
        },
    });
    let paragraph_id = paragraph.id.clone();

    WriterCommandService::apply(
        &mut document,
        &WriterCommand::ApplyCharacterStyle {
            range: TextRange {
                anchor: TextPosition {
                    paragraph_id: paragraph_id.clone(),
                    run_id: first_id,
                    offset: 0,
                },
                focus: TextPosition {
                    paragraph_id,
                    run_id: second_id,
                    offset: 2,
                },
            },
            patch: CharacterStylePatch {
                bold: Some(true),
                italic: None,
                underline: None,
                ..CharacterStylePatch::default()
            },
        },
        &id_tool,
    )
    .expect("style range should succeed");

    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    assert_eq!(paragraph.runs.len(), 1);
    assert_eq!(paragraph.runs[0].text, "ABCD");
    assert!(paragraph.runs[0].style.bold);
}

#[test]
fn styled_insert_splits_run_and_preserves_neighbor_style() {
    let id_tool = SequentialWriterIdTool::new();
    let mut document = WriterDocumentFactoryService::create(&id_tool, "TypingStyle");
    let Block::Paragraph(paragraph) = &mut document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    paragraph.runs[0].text = "AB".to_owned();
    let paragraph_id = paragraph.id.clone();
    let run_id = paragraph.runs[0].id.clone();
    let styled = CharacterStyle {
        bold: true,
        font_family: "Georgia".to_owned(),
        font_size_half_points: 28,
        ..CharacterStyle::default()
    };

    WriterCommandService::apply(
        &mut document,
        &WriterCommand::InsertStyledText {
            position: TextPosition {
                paragraph_id,
                run_id,
                offset: 1,
            },
            text: "X".to_owned(),
            style: styled.clone(),
        },
        &id_tool,
    )
    .expect("styled insert should succeed");

    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    assert_eq!(paragraph.plain_text(), "AXB");
    assert_eq!(paragraph.runs.len(), 3);
    assert_eq!(paragraph.runs[0].text, "A");
    assert_eq!(paragraph.runs[1].text, "X");
    assert_eq!(paragraph.runs[1].style, styled);
    assert_eq!(paragraph.runs[2].text, "B");
    assert_eq!(paragraph.runs[0].style, CharacterStyle::default());
    assert_eq!(paragraph.runs[2].style, CharacterStyle::default());
}

#[test]
fn character_style_patch_updates_font_without_losing_inline_flags() {
    let id_tool = SequentialWriterIdTool::new();
    let mut document = WriterDocumentFactoryService::create(&id_tool, "Typography");
    let Block::Paragraph(paragraph) = &mut document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    paragraph.runs[0].text = "Turkuaz".to_owned();
    paragraph.runs[0].style.italic = true;
    let paragraph_id = paragraph.id.clone();
    let run_id = paragraph.runs[0].id.clone();

    WriterCommandService::apply(
        &mut document,
        &WriterCommand::ApplyCharacterStyle {
            range: TextRange {
                anchor: TextPosition {
                    paragraph_id: paragraph_id.clone(),
                    run_id: run_id.clone(),
                    offset: 0,
                },
                focus: TextPosition {
                    paragraph_id,
                    run_id,
                    offset: 7,
                },
            },
            patch: CharacterStylePatch {
                font_family: Some("Georgia".to_owned()),
                font_size_half_points: Some(32),
                ..CharacterStylePatch::default()
            },
        },
        &id_tool,
    )
    .expect("font patch should succeed");

    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    assert_eq!(paragraph.runs.len(), 1);
    assert_eq!(paragraph.runs[0].style.font_family, "Georgia");
    assert_eq!(paragraph.runs[0].style.font_size_half_points, 32);
    assert!(paragraph.runs[0].style.italic);
}

#[test]
fn paragraph_alignment_patch_changes_only_alignment() {
    let id_tool = SequentialWriterIdTool::new();
    let mut document = WriterDocumentFactoryService::create(&id_tool, "Alignment");
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    let paragraph_id = paragraph.id.clone();

    WriterCommandService::apply(
        &mut document,
        &WriterCommand::ApplyParagraphStyle {
            paragraph_id,
            patch: ParagraphStylePatch {
                alignment: Some(TextAlignment::Center),
            },
        },
        &id_tool,
    )
    .expect("alignment patch should succeed");

    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    assert_eq!(paragraph.style.alignment, TextAlignment::Center);
    assert_eq!(paragraph.style.space_before_twips, 0);
    assert_eq!(paragraph.style.space_after_twips, 0);
}

#[test]
fn character_style_validation_rejects_blank_family_and_out_of_range_size() {
    let id_tool = SequentialWriterIdTool::new();
    let mut document = WriterDocumentFactoryService::create(&id_tool, "StyleValidation");
    let Block::Paragraph(paragraph) = &mut document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    paragraph.runs[0].text = "A".to_owned();
    let paragraph_id = paragraph.id.clone();
    let run_id = paragraph.runs[0].id.clone();
    let range = TextRange {
        anchor: TextPosition {
            paragraph_id: paragraph_id.clone(),
            run_id: run_id.clone(),
            offset: 0,
        },
        focus: TextPosition {
            paragraph_id,
            run_id,
            offset: 1,
        },
    };

    let blank_family = WriterCommandService::apply(
        &mut document,
        &WriterCommand::ApplyCharacterStyle {
            range: range.clone(),
            patch: CharacterStylePatch {
                font_family: Some("   ".to_owned()),
                ..CharacterStylePatch::default()
            },
        },
        &id_tool,
    );
    assert_eq!(blank_family, Err(WriterCommandError::InvalidCharacterStyle));

    let invalid_size = WriterCommandService::apply(
        &mut document,
        &WriterCommand::ApplyCharacterStyle {
            range,
            patch: CharacterStylePatch {
                font_size_half_points: Some(1),
                ..CharacterStylePatch::default()
            },
        },
        &id_tool,
    );
    assert_eq!(invalid_size, Err(WriterCommandError::InvalidCharacterStyle));
}

#[test]
fn paragraph_alignment_is_reversible_through_editor_history() {
    let mut service = service();
    let document = service.create_document("AlignmentHistory");
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    let paragraph_id = paragraph.id.clone();

    let aligned = service
        .execute(
            &document.id,
            WriterCommand::ApplyParagraphStyle {
                paragraph_id,
                patch: ParagraphStylePatch {
                    alignment: Some(TextAlignment::Justify),
                },
            },
        )
        .expect("alignment command should succeed");
    let Block::Paragraph(aligned_paragraph) = &aligned.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    assert_eq!(aligned_paragraph.style.alignment, TextAlignment::Justify);

    let undone = service
        .undo(&document.id)
        .expect("alignment undo should succeed");
    let Block::Paragraph(undone_paragraph) = &undone.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    assert_eq!(undone_paragraph.style.alignment, TextAlignment::Left);
}

#[test]
fn writer_view_exposes_primary_page_settings_without_pixel_conversion() {
    let repository = InMemoryWriterDocumentRepository::new();
    let id_tool = SequentialWriterIdTool::new();
    let service = WriterEditorService::new(repository, id_tool);
    let mut controller = WriterController::new(service);
    let document = controller.create("PageSettings");

    assert_eq!(
        document.page_settings.width_twips,
        turkuaz_office_writer::config::constants::DEFAULT_PAGE_WIDTH_TWIPS
    );
    assert_eq!(
        document.page_settings.height_twips,
        turkuaz_office_writer::config::constants::DEFAULT_PAGE_HEIGHT_TWIPS
    );
    assert_eq!(
        document.page_settings.margin_left_twips,
        turkuaz_office_writer::config::constants::DEFAULT_PAGE_MARGIN_TWIPS
    );
    assert_eq!(
        document.page_settings.margin_right_twips,
        turkuaz_office_writer::config::constants::DEFAULT_PAGE_MARGIN_TWIPS
    );
}

#[test]
fn replace_range_with_styled_runs_preserves_neighbor_styles_and_undoes_once() {
    let mut service = service();
    let document = service.create_document("StyledFragment");
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    let paragraph_id = paragraph.id.clone();
    let initial_run_id = paragraph.runs[0].id.clone();

    let document = service
        .execute(
            &document.id,
            WriterCommand::InsertText {
                position: TextPosition {
                    paragraph_id: paragraph_id.clone(),
                    run_id: initial_run_id.clone(),
                    offset: 0,
                },
                text: "HelloWorld".to_owned(),
            },
        )
        .expect("seed text should succeed");

    let document = service
        .execute(
            &document.id,
            WriterCommand::ApplyCharacterStyle {
                range: TextRange {
                    anchor: TextPosition {
                        paragraph_id: paragraph_id.clone(),
                        run_id: initial_run_id.clone(),
                        offset: 5,
                    },
                    focus: TextPosition {
                        paragraph_id: paragraph_id.clone(),
                        run_id: initial_run_id,
                        offset: 10,
                    },
                },
                patch: CharacterStylePatch {
                    bold: Some(true),
                    ..CharacterStylePatch::default()
                },
            },
        )
        .expect("style split should succeed");

    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    assert_eq!(paragraph.runs.len(), 2);
    let left_run_id = paragraph.runs[0].id.clone();
    let right_run_id = paragraph.runs[1].id.clone();

    let italic = CharacterStyle {
        italic: true,
        ..CharacterStyle::default()
    };
    let underlined = CharacterStyle {
        underline: true,
        ..CharacterStyle::default()
    };

    let replaced = service
        .execute(
            &document.id,
            WriterCommand::ReplaceRangeWithStyledRuns {
                range: TextRange {
                    anchor: TextPosition {
                        paragraph_id: paragraph_id.clone(),
                        run_id: left_run_id,
                        offset: 3,
                    },
                    focus: TextPosition {
                        paragraph_id: paragraph_id.clone(),
                        run_id: right_run_id,
                        offset: 3,
                    },
                },
                runs: vec![
                    turkuaz_office_writer::StyledTextRun {
                        text: "X".to_owned(),
                        style: italic.clone(),
                    },
                    turkuaz_office_writer::StyledTextRun {
                        text: "Y".to_owned(),
                        style: underlined.clone(),
                    },
                ],
            },
        )
        .expect("styled fragment replace should succeed");

    assert_eq!(replaced.plain_text(), "HelXYld");
    let Block::Paragraph(paragraph) = &replaced.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    assert_eq!(paragraph.runs.len(), 4);
    assert_eq!(paragraph.runs[0].text, "Hel");
    assert_eq!(paragraph.runs[0].style, CharacterStyle::default());
    assert_eq!(paragraph.runs[1].text, "X");
    assert_eq!(paragraph.runs[1].style, italic);
    assert_eq!(paragraph.runs[2].text, "Y");
    assert_eq!(paragraph.runs[2].style, underlined);
    assert_eq!(paragraph.runs[3].text, "ld");
    assert!(paragraph.runs[3].style.bold);

    let undone = service
        .undo(&document.id)
        .expect("single undo should restore pre-paste state");
    assert_eq!(undone.plain_text(), "HelloWorld");
    let Block::Paragraph(paragraph) = &undone.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    assert_eq!(paragraph.runs.len(), 2);
    assert!(!paragraph.runs[0].style.bold);
    assert!(paragraph.runs[1].style.bold);
}

#[test]
fn replace_range_with_empty_fragment_keeps_editable_empty_run() {
    let mut service = service();
    let document = service.create_document("CutFragment");
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    let paragraph_id = paragraph.id.clone();
    let run_id = paragraph.runs[0].id.clone();

    let document = service
        .execute(
            &document.id,
            WriterCommand::InsertText {
                position: TextPosition {
                    paragraph_id: paragraph_id.clone(),
                    run_id: run_id.clone(),
                    offset: 0,
                },
                text: "Kesilecek".to_owned(),
            },
        )
        .expect("seed text should succeed");

    let cut = service
        .execute(
            &document.id,
            WriterCommand::ReplaceRangeWithStyledRuns {
                range: TextRange {
                    anchor: TextPosition {
                        paragraph_id: paragraph_id.clone(),
                        run_id: run_id.clone(),
                        offset: 0,
                    },
                    focus: TextPosition {
                        paragraph_id,
                        run_id,
                        offset: 9,
                    },
                },
                runs: Vec::new(),
            },
        )
        .expect("cut fragment should succeed");

    assert_eq!(cut.plain_text(), "");
    let Block::Paragraph(paragraph) = &cut.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    assert_eq!(paragraph.runs.len(), 1);
    assert_eq!(paragraph.runs[0].text, "");
}

#[test]
fn insert_image_data_registers_asset_and_block_atomically() {
    let mut service = service();
    let document = service.create_document("ImageAsset");
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    let paragraph_id = paragraph.id.clone();
    let png = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

    let document = service
        .execute(
            &document.id,
            WriterCommand::InsertImageData {
                after_paragraph_id: paragraph_id,
                media_type: "image/png".to_owned(),
                data: png.clone(),
                alt_text: "Clipboard image".to_owned(),
                width_twips: None,
                height_twips: None,
            },
        )
        .expect("image data insert should succeed");

    assert_eq!(document.assets.len(), 1);
    assert_eq!(document.assets[0].media_type, "image/png");
    assert_eq!(document.assets[0].bytes, png);
    let Block::Image(image) = &document.sections[0].blocks[1] else {
        panic!("image block expected");
    };
    assert_eq!(image.asset_id, document.assets[0].id);
}

#[test]
fn insert_image_data_rejects_invalid_signature_without_mutation() {
    let mut service = service();
    let document = service.create_document("BadImage");
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };

    let result = service.execute(
        &document.id,
        WriterCommand::InsertImageData {
            after_paragraph_id: paragraph.id.clone(),
            media_type: "image/png".to_owned(),
            data: b"not-png".to_vec(),
            alt_text: String::new(),
            width_twips: None,
            height_twips: None,
        },
    );
    assert!(matches!(
        result,
        Err(turkuaz_office_writer::WriterEditorError::Command(
            WriterCommandError::InvalidAsset
        ))
    ));
    let unchanged = service
        .get_document(&document.id)
        .expect("document should remain available");
    assert!(unchanged.assets.is_empty());
    assert_eq!(unchanged.sections[0].blocks.len(), 1);
}

#[test]
fn insert_image_data_undo_redo_restores_asset_and_block() {
    let mut service = service();
    let document = service.create_document("ImageHistory");
    let paragraph_id = match &document.sections[0].blocks[0] {
        Block::Paragraph(paragraph) => paragraph.id.clone(),
        _ => panic!("paragraph expected"),
    };
    let document = service
        .execute(
            &document.id,
            WriterCommand::InsertImageData {
                after_paragraph_id: paragraph_id,
                media_type: "image/png".to_owned(),
                data: vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A],
                alt_text: "History image".to_owned(),
                width_twips: None,
                height_twips: None,
            },
        )
        .expect("image insert");
    assert_eq!(document.assets.len(), 1);
    assert!(matches!(document.sections[0].blocks[1], Block::Image(_)));

    let undone = service.undo(&document.id).expect("image undo");
    assert!(undone.assets.is_empty());
    assert_eq!(undone.sections[0].blocks.len(), 1);

    let redone = service.redo(&document.id).expect("image redo");
    assert_eq!(redone.assets.len(), 1);
    assert!(matches!(&redone.sections[0].blocks[1], Block::Image(_)));
}

#[test]
fn image_insert_undo_redo_restores_asset_and_block_together() {
    let mut service = service();
    let document = service.create_document("ImageHistory");
    let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
        panic!("paragraph expected");
    };
    let paragraph_id = paragraph.id.clone();
    let png = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

    let inserted = service
        .execute(
            &document.id,
            WriterCommand::InsertImageData {
                after_paragraph_id: paragraph_id,
                media_type: "image/png".to_owned(),
                data: png.clone(),
                alt_text: "History".to_owned(),
                width_twips: None,
                height_twips: None,
            },
        )
        .expect("image insert");
    assert_eq!(inserted.assets.len(), 1);
    assert!(matches!(&inserted.sections[0].blocks[1], Block::Image(_)));

    let undone = service.undo(&document.id).expect("undo image insert");
    assert!(undone.assets.is_empty());
    assert_eq!(undone.sections[0].blocks.len(), 1);

    let redone = service.redo(&document.id).expect("redo image insert");
    assert_eq!(redone.assets.len(), 1);
    assert_eq!(redone.assets[0].bytes, png);
    assert!(matches!(redone.sections[0].blocks[1], Block::Image(_)));
}
