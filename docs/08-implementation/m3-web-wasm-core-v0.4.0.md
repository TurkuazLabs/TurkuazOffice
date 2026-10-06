# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-wasm-core-v0.4.0.md
# 📌 Amac: M3 Web icin compile-verified Rust WASM Core diliminin kapsam ve sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.1.0
# Aciklama: wasm32 hedefi, Core ABI capability API'si, Web Tool adapter kontrati ve sonraki binding adimini dokumante eder

Bagimli Oldugu Katman: Documentation

# M3 Web WASM Core v0.4.0

## Durum

M3 Web'in ikinci diliminde `turkuaz-office-core` crate'i `wasm32-unknown-unknown` hedefinde CI ile derlenebilir hale getirilir.

Bu dilim generated JavaScript binding veya runtime WASM loader eklemez. Amac once platformdan bagimsiz Rust Core yuzeyinin gercek browser targetinda derlenmesini ve ABI kontratinin typed olarak sabitlenmesini saglamaktir.

## Rust Core capability yuzeyi

Core tarafinda asagidaki saf fonksiyonlar tanimlidir:

- `web_core_abi_version`
- `web_core_document_schema_version`
- `web_core_bridge_kind`
- `web_core_native_file_system_access`

Bu fonksiyonlar OS, window, filesystem veya Tauri API kullanmaz.

`WebCoreService::capabilities()` ayni degerleri `WebCoreCapabilitiesView` olarak birlestirir.

## Web Tool adapteri

Web tarafinda iki Tool implementasyonu vardir:

- `BrowserCoreContractTool`: generated binding yuklenmeden once foundation capability fallback'i.
- `WasmCoreTool`: gelecekte generated Rust/WASM module'u typed `WasmCoreModule` kontratiyla adapte eder.

`WasmCoreTool` native filesystem capability `true` gelirse fail-closed hata verir.

Browser metadata storage capability Rust Core'un canonical document modeli degildir; platform adapter ozelligidir.

## CI gate

`workspace-ci` Rust quality job'u:

1. Rust `1.98.1` toolchain'i kurar.
2. `wasm32-unknown-unknown` targetini kurar.
3. Project contract'i dogrular.
4. Native workspace `cargo check` calistirir.
5. `cargo check -p turkuaz-office-core --target wasm32-unknown-unknown` calistirir.
6. Rustfmt ve Clippy gate'lerini calistirir.

Bu gate gecmeden WASM-compatible Core dilimi tamamlanmis sayilmaz.

## Bilerek kapsam disi

- `wasm-bindgen` generated JavaScript glue.
- Browser runtime WASM module loader.
- IndexedDB canonical document persistence.
- Writer/Sheet Web editor read-model entegrasyonu.
- Browser import/export.
- Service Worker / Cache Storage offline cache.

## Takip eden durum

Pinned wasm-bindgen generated artifact pipeline'i ve browser runtime loader sonraki M3 dilimlerinde tamamlanmistir. Runtime, generated binding mevcutsa `WasmCoreTool` kullanir; artifact yoksa yalniz capability foundation icin browser-contract fallback uygulanir.

Canonical belge persistence'i IndexedDB diliminde ele alinmistir.
