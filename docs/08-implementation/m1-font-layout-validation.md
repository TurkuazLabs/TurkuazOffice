# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-font-layout-validation.md
# 📌 Amac: M1 Font/Layout baseline artifact dogrulama sonuclarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Static verifier, TypeScript, layout runtime smoke, Rust structural ve paket kalite kapilarini raporlar

Bagimli Oldugu Katman: Documentation

# M1 Font/Layout Validation

## Artifact durumu

- Toplam dosya: 207.
- Docs dosyasi: 77.
- Rust dosyasi: 68.
- Desktop TypeScript/TSX dosyasi: 31.

## Basarili kontroller

- `bash tools/verify-project.sh`: BASARILI.
- Header ve `Version: 0.2.0` kontrati: BASARILI.
- TOML parse: BASARILI.
- YAML parse: BASARILI.
- Strict JSON parse kapsamindaki config dosyalari: BASARILI.
- `bash -n tools/verify-project.sh`: BASARILI.
- Header sonrasi teknik body ASCII kontrolu: BASARILI.
- Rust delimiter/structural kontrol: BASARILI.
- Gecici external-module declaration stub'lariyla full Desktop strict TypeScript `tsc --noEmit`: BASARILI.
- `WriterLayoutService` compile + Node runtime smoke: BASARILI.
- A4 default width: yaklasik 793.73 CSS px @ %100.
- A4 default height: yaklasik 1122.53 CSS px @ %100.
- 1440 twip margin: 96 CSS px @ %100.
- Zoom clamp %50-%200, step %10 ve reset %100 runtime smoke: BASARILI.
- Calibri -> Carlito fallback fixture: BASARILI.
- Bilinmeyen family -> sans-serif fallback fixture: BASARILI.
- Writer domain regression kaynaginda primary PageSettings View testi: EKLENDI.

## Acik build bariyeri

Bu calisma ortaminda `cargo` ve `rustc` bulunmuyor. Bu nedenle asagidaki komutlar bu artifact ortaminda calistirilmis sayilmaz:

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`

Rust toolchain bulunan ilk yerel gelistirme/CI kosusunda bunlar zorunlu bariyerdir.

Container internet erisimi dependency registry build'i icin kullanilamaz. Bu nedenle gercek npm dependency agaciyla Vite/Tauri production build bu turda kosulmadi; TypeScript kendi kaynak kodu temporary declaration stub'lariyla strict seviyede dogrulandi. Stub dosyalari artifact'a dahil edilmedi.

## Bilinen layout siniri

M1 Font/Layout baseline canonical pagination veya shaping motoru degildir. Browser/webview metni render eder; DOM glyph metric canonical belge karari sayilmaz. Multi-page pagination ve cross-platform golden metric tolerance daha sonraki layout engine genislemesinde zorunlu olacaktir.
