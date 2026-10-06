# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/m3-web-wasm-bindgen-artifact-test-matrix.md
# 📌 Amac: M3 Web generated wasm-bindgen artifact kalite kapilarini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.2.0
# Aciklama: wasm32 release build, pinned CLI, generated exportlar ve CI artifact yukleme dogrulamalarini listeler
# Bagimli Oldugu Katman: Tool | CI

# M3 Web wasm-bindgen Artifact Test Matrix

| Alan | Dogrulama | Beklenen |
| --- | --- | --- |
| Rust target | Web bridge release wasm32 build | Basarili |
| CLI | wasm-bindgen surumu | 0.2.129 |
| JS glue | generated JS dosyasi | Bos degil |
| WASM | generated bg.wasm | Bos degil |
| ABI export | web_core_abi_version | Mevcut |
| Schema export | web_core_document_schema_version | Mevcut |
| Bridge export | web_core_bridge_kind | Mevcut |
| FS export | web_core_native_file_system_access | Mevcut |
| TKO inspect export | web_writer_tko_inspect | Mevcut |
| TKO re-encode export | web_writer_tko_reencode | Mevcut |
| Core standalone feature | web-wasm-exports | Aggregate dependency'de kapali, standalone check'te acik |
| Source control | generated wasm klasoru | Ignore |
| CI | turkuaz-office-web-wasm artifact | Upload basarili |
