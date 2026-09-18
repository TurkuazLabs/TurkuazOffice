# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/platform-strategy.md
# 📌 Amac: Desktop Web Mobile platformlarinin ayni core ile nasil gelisecegini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Desktop Web Mobile platformlarinin ayni core ile nasil gelisecegini tanimlar

Bagimli Oldugu Katman: Documentation

# Platform Stratejisi

## Desktop

Ilk hedef Windows ve Linux. Tauri 2 native shell olarak planlanir. macOS daha sonra ayni client katmanina eklenir.

## Web

Browser istemcisi TypeScript UI kullanir. Rust Core'un saf hesaplama ve document operation bolumleri WASM icin uygun tutulur. Native-only davranislar API veya browser adapter ile saglanir.

## Mobile

Android ve iOS istemcileri Tauri 2 mobile yolunu kullanabilir. UI desktop layout'un daraltilmis hali degil, ayni services uzerinde mobil UX olur.

## Ortaklik siniri

Paylasilan:

- document semantics,
- command semantics,
- validation,
- import/export domain mapping,
- sync operation format.

Platforma ozel:

- file picker,
- clipboard,
- print dialog,
- share sheet,
- secure storage,
- OS window/menu,
- permission handling.
