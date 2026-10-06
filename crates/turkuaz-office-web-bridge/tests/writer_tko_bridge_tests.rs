// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-web-bridge/tests/writer_tko_bridge_tests.rs
// # 📌 Amac: Web Writer TKO bridge inspect/re-encode ve stable hata mapping davranisini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.4.0
// Aciklama: Mevcut Writer TkoPackageService semantiginin Web bridge uzerinden veri kaybi olmadan yeniden kullanildigini sabitler
// Bagimli Oldugu Katman: Controller -> Service -> Writer Service -> View -> Config

use turkuaz_office_web_bridge::{
    WebWriterTkoBridgeController, WebWriterTkoBridgeError, WebWriterTkoBridgeService,
};
use turkuaz_office_writer::{
    InMemoryWriterDocumentRepository, SequentialWriterIdTool, TkoPackageService,
    WriterEditorService,
};

fn editor() -> WriterEditorService<InMemoryWriterDocumentRepository, SequentialWriterIdTool> {
    WriterEditorService::new(
        InMemoryWriterDocumentRepository::new(),
        SequentialWriterIdTool::new(),
    )
}

#[test]
fn inspect_returns_writer_metadata_without_reimplementing_tko_in_web_layer() {
    let mut editor = editor();
    let document = editor.create_document("Web Bridge");
    let bytes = TkoPackageService::serialize(&document, "0.4.0").expect("serialize fixture");

    let view = WebWriterTkoBridgeController::inspect(&bytes).expect("inspect TKO");

    assert_eq!(view.id, document.id.as_str());
    assert_eq!(view.title, "Web Bridge");
    assert_eq!(view.schema_version, document.schema_version.value());
    assert_eq!(view.revision, document.revision);
    assert_eq!(view.section_count, document.sections.len());
    assert_eq!(view.asset_count, document.assets.len());
}

#[test]
fn reencode_preserves_the_canonical_writer_document() {
    let mut editor = editor();
    let document = editor.create_document("Reencode");
    let bytes = TkoPackageService::serialize(&document, "0.2.0").expect("serialize fixture");

    let reencoded = WebWriterTkoBridgeController::reencode(&bytes, "0.4.0").expect("re-encode TKO");
    let restored = TkoPackageService::deserialize(&reencoded).expect("deserialize reencoded TKO");

    assert_eq!(restored, document);
}

#[test]
fn invalid_package_maps_to_stable_web_error_code() {
    let error = WebWriterTkoBridgeService::inspect(b"not-a-tko").expect_err("invalid package");

    assert_eq!(error, WebWriterTkoBridgeError::InvalidPackage);
    assert_eq!(error.code(), "web_tko_invalid_package");
}

#[test]
fn oversized_package_maps_to_stable_web_error_code() {
    let max_bytes = usize::try_from(TkoPackageService::MAX_PACKAGE_BYTES)
        .expect("TKO package limit must fit usize");
    let bytes = vec![0_u8; max_bytes + 1];

    let error = WebWriterTkoBridgeService::inspect(&bytes).expect_err("oversized package");

    assert_eq!(error, WebWriterTkoBridgeError::PackageTooLarge);
    assert_eq!(error.code(), "web_tko_package_too_large");
}
