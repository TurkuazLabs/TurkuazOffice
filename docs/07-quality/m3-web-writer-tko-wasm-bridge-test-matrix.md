# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/m3-web-writer-tko-wasm-bridge-test-matrix.md
# 📌 Amac: M3 Web Writer TKO WASM bridge kalite ve regression kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.4.0
# Aciklama: Native bridge semantic testleri, wasm32 compile, generated export ve Core compatibility gate'lerini listeler
# Bagimli Oldugu Katman: Tool -> Controller -> Service -> Writer Service -> View -> Config

# M3 Web Writer TKO WASM Bridge Test Matrix

| Alan | Otomatik kontrol | Beklenen |
| --- | --- | --- |
| Inspect | Rust bridge testi | TKO id/title/schema/revision/section/asset metadata kayipsiz okunur |
| Re-encode | Rust bridge testi | decode + encode sonrasi canonical WriterDocument esit kalir |
| Invalid package | Rust bridge testi | stable web_tko_invalid_package koduna map edilir |
| Package size | Rust bridge testi | 16 MiB ustu payload stable web_tko_package_too_large koduna map edilir |
| Core dependency direction | Cargo workspace | Web Bridge -> Core + Writer; Core -> Writer bagimliligi yoktur |
| Core standalone WASM | CI cargo check | web-wasm-exports feature'i ile eski Core export surface derlenir |
| Aggregate WASM | CI cargo check | turkuaz-office-web-bridge wasm32 hedefinde derlenir |
| Generated JS | build-web-wasm.sh | Core 4 export + Writer TKO 2 export generated JS icinde bulunur |
| Runtime compatibility | Web build/test | mevcut loadWebCoreTool yeni aggregate module URL'siyle tip kontratini korur |
| Workspace regression | cargo test --workspace | yeni crate native testleri Windows/Linux dahil yesil |
| Static contract | tools/verify-project.sh | aggregate module, crate ve docs zorunlu kalir |

## Basari kriteri

- Writer TKO format semantigi TypeScript'e kopyalanmaz.
- Core crate Writer'a bagimli hale gelmez.
- Tek generated browser module Core capability exportlarini korur.
- Writer TKO inspect ve re-encode ayni aggregate artifact icinde bulunur.
- Import/export View bu dilimde acilmaz.
