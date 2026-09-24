// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/tests/writer_template_tests.rs
// # 📌 Amac: Built-in Writer template katalog, canonical stil iskeleti ve editor yasam dongusunu regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// # Version: 0.2.0
// # Aciklama: Bos/Mektup/Rapor katalog sirasi, report style profili, unknown template hatasi ve editor repository kaydini kilitler
// Bagimli Oldugu Katman: Service -> Repo -> Tool

use turkuaz_office_writer::{
    Block, InMemoryWriterDocumentRepository, SequentialWriterIdTool, TextAlignment,
    WriterEditorError, WriterEditorService, WriterTemplateError, WriterTemplateService,
};

#[test]
fn built_in_template_catalog_is_stable_and_ordered() {
    let catalog = WriterTemplateService::catalog().expect("template catalog");

    assert_eq!(catalog.len(), 3);
    assert_eq!(catalog[0].id, "blank");
    assert_eq!(catalog[0].name_key, "templateBlank");
    assert!(!catalog[0].quick_create);
    assert_eq!(catalog[1].id, "letter");
    assert!(catalog[1].quick_create);
    assert_eq!(catalog[2].id, "report");
    assert!(catalog[2].quick_create);
}

#[test]
fn report_template_creates_canonical_styled_paragraphs() {
    let ids = SequentialWriterIdTool::new();
    let document = WriterTemplateService::create(&ids, "report").expect("report template");

    assert_eq!(document.revision, 0);
    assert_eq!(document.sections.len(), 1);
    assert_eq!(document.sections[0].blocks.len(), 3);

    let Block::Paragraph(title) = &document.sections[0].blocks[0] else {
        panic!("title paragraph");
    };
    assert_eq!(title.style.alignment, TextAlignment::Center);
    assert!(title.runs[0].style.bold);
    assert_eq!(title.runs[0].style.font_size_half_points, 36);

    let Block::Paragraph(subtitle) = &document.sections[0].blocks[1] else {
        panic!("subtitle paragraph");
    };
    assert_eq!(subtitle.style.alignment, TextAlignment::Center);
    assert!(subtitle.runs[0].style.italic);

    let Block::Paragraph(body) = &document.sections[0].blocks[2] else {
        panic!("body paragraph");
    };
    assert_eq!(body.style.alignment, TextAlignment::Left);
}

#[test]
fn unknown_template_is_typed_error() {
    let ids = SequentialWriterIdTool::new();

    assert_eq!(
        WriterTemplateService::create(&ids, "missing-template"),
        Err(WriterTemplateError::TemplateNotFound)
    );
}

#[test]
fn editor_template_creation_registers_normal_document_lifecycle() {
    let mut service = WriterEditorService::new(
        InMemoryWriterDocumentRepository::new(),
        SequentialWriterIdTool::new(),
    );

    let document = service
        .create_document_from_template("letter")
        .expect("letter template");
    let loaded = service
        .get_document(&document.id)
        .expect("template document repository");

    assert_eq!(loaded, document);
    assert_eq!(loaded.revision, 0);
    assert_eq!(loaded.sections[0].blocks.len(), 4);

    assert_eq!(
        service.create_document_from_template("unknown"),
        Err(WriterEditorError::Template(
            WriterTemplateError::TemplateNotFound
        ))
    );
}
