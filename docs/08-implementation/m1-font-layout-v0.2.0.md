# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-font-layout-v0.2.0.md
# 📌 Amac: Writer M1 Font/Layout baseline implementation kapsamini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: PageSettings read-model, twip render, zoom ve font fallback implementationini ozetler

Bagimli Oldugu Katman: Documentation

# M1 Font/Layout Baseline

## Eklenen akis

Canonical `Section.PageSettings` -> Writer View -> Tauri DTO -> Desktop `WriterDocumentView.pageSettings` -> `WriterLayoutService` -> Writer Page View.

Sayfa width, height ve margin degerleri artik sabit CSS page tokenlarindan degil canonical twip alanlarindan uretilir.

## Zoom

- Repository session state'tir.
- Varsayilan %100.
- Minimum %50.
- Maksimum %200.
- Adim %10.
- Statusbar `- / % / +` kontrolleri vardir.
- Ctrl/Cmd `+`, Ctrl/Cmd `-`, Ctrl/Cmd `0` desteklenir.
- Zoom Writer revision, dirty state, save veya undo history'yi degistirmez.

## Font resolution

`FontCapabilityTool` platform/webview font capability sorgusunu izole eder. `WriterLayoutService` requested family icin config tabanli fallback stack uretir.

Baseline profilleri:

- Arial -> Liberation Sans -> Nimbus Sans -> sans-serif.
- Calibri -> Carlito -> Arial -> Liberation Sans -> sans-serif.
- Times New Roman -> Liberation Serif -> Nimbus Roman -> serif.
- Georgia -> Liberation Serif -> Nimbus Roman -> serif.
- Verdana -> DejaVu Sans -> Liberation Sans -> sans-serif.
- Courier New -> Liberation Mono -> Nimbus Mono PS -> monospace.

Canonical CharacterStyle icindeki `fontFamily` fallback sonucu ile mutate edilmez.

## Bilinen sinir

M1 page surface henuz pagination engine degildir. Ilk section page geometrysi kullanilir ve uzun content tek yuzeyde devam edebilir. DOM glyph metric canonical sayilmaz. Gercek shaping/pagination motoru daha sonraki layout engine genislemesidir.
