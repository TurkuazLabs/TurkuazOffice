// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/tests/docx_minimum_tests.rs
// # 📌 Amac: DOCX minimum profile import/export ve compatibility regression kapsamlarini dogrular
// # 📌 Modul - FileType: Test - Rust
// # Version: 0.2.0
// # Aciklama: Paragraph/run typography, alignment, page geometry ve unsupported feature report davranisini kilitler
// Bagimli Oldugu Katman: Service -> Tool -> Model

use turkuaz_office_format_adapters::{
    DocxArchiveTool, DocxError, DocxService, DocxUnsupportedFeature, DocxXmlError,
    DocxXmlTool,
};
use turkuaz_office_writer::services::writer_document_factory_service::WriterDocumentFactoryService;
use turkuaz_office_writer::{
    Block, SequentialWriterIdTool, TextAlignment, WriterAsset,
};

#[test]
fn docx_minimum_round_trip_preserves_text_typography_alignment_and_page_geometry() {
    let ids = SequentialWriterIdTool::new();
    let mut document = WriterDocumentFactoryService::create(&ids, "DOCX Round Trip");
    let section = document.sections.first_mut().expect("section");
    section.page_settings.width_twips = 12_000;
    section.page_settings.height_twips = 16_000;
    section.page_settings.margin_top_twips = 1_200;
    section.page_settings.margin_right_twips = 1_300;
    section.page_settings.margin_bottom_twips = 1_400;
    section.page_settings.margin_left_twips = 1_500;

    let Block::Paragraph(paragraph) = section.blocks.first_mut().expect("paragraph block") else {
        panic!("paragraph expected");
    };
    paragraph.style.alignment = TextAlignment::Center;
    let run = paragraph.runs.first_mut().expect("run");
    run.text = "Merhaba & <DOCX>\tSatir\nSon".to_owned();
    run.style.bold = true;
    run.style.italic = true;
    run.style.underline = true;
    run.style.font_family = "Calibri".to_owned();
    run.style.font_size_half_points = 24;

    let bytes = DocxService::export(&document).expect("docx export");
    let imported = DocxService::import(
        &bytes,
        "Imported DOCX",
        &SequentialWriterIdTool::new(),
    )
    .expect("docx import");

    assert!(imported.compatibility.unsupported_features.is_empty());
    let imported_section = imported.document.sections.first().expect("imported section");
    assert_eq!(imported_section.page_settings.width_twips, 12_000);
    assert_eq!(imported_section.page_settings.height_twips, 16_000);
    assert_eq!(imported_section.page_settings.margin_left_twips, 1_500);

    let Block::Paragraph(imported_paragraph) =
        imported_section.blocks.first().expect("imported paragraph")
    else {
        panic!("paragraph expected");
    };
    assert_eq!(imported_paragraph.style.alignment, TextAlignment::Center);
    let imported_run = imported_paragraph.runs.first().expect("imported run");
    assert_eq!(imported_run.text, "Merhaba & <DOCX>\tSatir\nSon");
    assert!(imported_run.style.bold);
    assert!(imported_run.style.italic);
    assert!(imported_run.style.underline);
    assert_eq!(imported_run.style.font_family, "Calibri");
    assert_eq!(imported_run.style.font_size_half_points, 24);
}

#[test]
fn docx_import_reports_table_and_hyperlink_structure_loss() {
    let content_types = DocxXmlTool::content_types_xml();
    let relationships = DocxXmlTool::root_relationships_xml();
    let document_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:tbl><w:tr><w:tc><w:p><w:r><w:t>Cell</w:t></w:r></w:p></w:tc></w:tr></w:tbl>
    <w:p><w:hyperlink><w:r><w:t>Link text</w:t></w:r></w:hyperlink></w:p>
  </w:body>
</w:document>"#;

    let bytes = DocxArchiveTool::encode(&[
        ("[Content_Types].xml", content_types.as_bytes()),
        ("_rels/.rels", relationships.as_bytes()),
        ("word/document.xml", document_xml.as_bytes()),
    ])
    .expect("fixture package");

    let imported = DocxService::import(
        &bytes,
        "Compatibility",
        &SequentialWriterIdTool::new(),
    )
    .expect("docx import");

    assert!(
        imported
            .compatibility
            .unsupported_features
            .contains(&DocxUnsupportedFeature::Table)
    );
    assert!(
        imported
            .compatibility
            .unsupported_features
            .contains(&DocxUnsupportedFeature::Hyperlink)
    );
    assert_eq!(imported.document.plain_text(), "Cell\nLink text");
}


#[test]
fn docx_export_rejects_canonical_assets_in_minimum_profile() {
    let ids = SequentialWriterIdTool::new();
    let mut document = WriterDocumentFactoryService::create(&ids, "Unsupported Asset");
    document.assets.push(WriterAsset {
        id: "asset-docx-test".to_owned(),
        media_type: "image/png".to_owned(),
        bytes: vec![0x89, b'P', b'N', b'G'],
    });

    assert_eq!(DocxService::export(&document), Err(DocxError::UnsupportedAsset));
}

#[test]
fn docx_xml_rejects_doctype() {
    let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE w:document [<!ENTITY x "unsafe">]>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body><w:p><w:r><w:t>&x;</w:t></w:r></w:p></w:body>
</w:document>"#;

    assert_eq!(
        DocxXmlTool::parse_document(xml),
        Err(DocxXmlError::DocTypeUnsupported)
    );
}
