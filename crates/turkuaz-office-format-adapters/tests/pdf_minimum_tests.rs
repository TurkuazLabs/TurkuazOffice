// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/tests/pdf_minimum_tests.rs
// # 📌 Amac: PDF minimum font request ve strict unsupported structure davranisini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// # Version: 0.2.0
// # Aciklama: Canonical style varyantlarinin font requeste tasinmasini ve asset export reddini kilitler
// Bagimli Oldugu Katman: Service -> Model

use turkuaz_office_format_adapters::{PdfError, PdfFontKey, PdfService};
use turkuaz_office_writer::services::writer_document_factory_service::WriterDocumentFactoryService;
use turkuaz_office_writer::{Block, SequentialWriterIdTool, WriterAsset};

#[test]
fn pdf_font_requests_preserve_family_bold_and_italic_variants() {
    let ids = SequentialWriterIdTool::new();
    let mut document = WriterDocumentFactoryService::create(&ids, "PDF Fonts");
    let section = document.sections.first_mut().expect("section");
    let Block::Paragraph(paragraph) = section.blocks.first_mut().expect("paragraph") else {
        panic!("paragraph expected");
    };
    let run = paragraph.runs.first_mut().expect("run");
    run.text = "Turkce PDF".to_owned();
    run.style.font_family = "Arial".to_owned();
    run.style.bold = true;
    run.style.italic = true;

    let requests = PdfService::font_requests(&document).expect("font requests");

    assert_eq!(
        requests,
        vec![PdfFontKey {
            family: "Arial".to_owned(),
            bold: true,
            italic: true,
        }]
    );
}

#[test]
fn pdf_export_rejects_canonical_assets_in_minimum_profile() {
    let ids = SequentialWriterIdTool::new();
    let mut document = WriterDocumentFactoryService::create(&ids, "PDF Asset");
    document.assets.push(WriterAsset {
        id: "asset-pdf-test".to_owned(),
        media_type: "image/png".to_owned(),
        bytes: vec![0x89, b'P', b'N', b'G'],
    });

    assert_eq!(PdfService::font_requests(&document), Err(PdfError::UnsupportedAsset));
}
