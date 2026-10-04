# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/m3-web-wasm-core-test-matrix.md
# 📌 Amac: M3 Web WASM Core diliminin otomatik kalite ve regression gate'lerini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.1.0
# Aciklama: wasm32 compile, ABI capability, Tool mapping ve filesystem boundary test kapsamlarini listeler

Bagimli Oldugu Katman: Documentation

# M3 Web WASM Core Test Matrix

| Alan | Otomatik kontrol |
| --- | --- |
| Rust Core native compile | `cargo check --workspace --all-targets` |
| Rust Core browser target | `cargo check -p turkuaz-office-core --target wasm32-unknown-unknown` |
| Rust capability values | `WebCoreService` unit testleri |
| ABI version | Rust constant/function + Web Tool regression testi |
| Document schema version | Rust capability + Web Tool mapping testi |
| Bridge kind | Rust capability + Web Tool mapping testi |
| Native filesystem boundary | Rust `false` capability + Web `WasmCoreTool` fail-closed testi |
| Web TypeScript contract | `npm run build` |
| Web regression tests | `npm test` |
| Static project contract | `tools/verify-project.sh` |

## Basari kriteri

- `turkuaz-office-core` wasm32 hedefinde platform-native dependency olmadan compile olur.
- Web adapter capability degerlerini Rust/WASM module kontratindan map edebilir.
- Browser Tool native filesystem erisimini kabul etmez.
- Generated JS binding mevcut degilken fallback Tool bunun tamamlanmis oldugunu iddia etmez.
