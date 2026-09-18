# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/font-layout-test-matrix.md
# 📌 Amac: Writer Font/Layout baseline icin regression ve manuel test matrisini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: PageSettings, twip conversion, zoom, font fallback ve cross-platform render kontrollerini listeler

Bagimli Oldugu Katman: Documentation

# Font/Layout Test Matrix

## Otomatik kontrat testleri

- Writer View primary section page width/height/margin degerlerini twip olarak tasir.
- TKO round-trip PageSettings degerlerini kaybetmez.
- Desktop DTO page settings alanlarini camelCase JSON kontratina tasir.
- Zoom minimum %50, maksimum %200 ve adim %10 sinirinda kalir.
- Zoom session-only state'tir ve Writer revision degerini degistirmez.
- Font fallback resolver requested family degerini canonical belgeye geri yazmaz.
- Bilinmeyen font family en az generic sans-serif fallback ile render stack alir.

## Manuel Windows/Linux smoke test

- A4 default page %100 zoom'da yaklasik 793.7 x 1122.5 CSS px gorunur.
- %50, %100, %150 ve %200 zoom'da page/margin/font birlikte oransal buyur veya kuculur.
- Ctrl/Cmd `+`, `-`, `0` statusbar zoom ile ayni sonucu verir.
- Arial olmayan Linux ortaminda fallback stack belgeyi bos veya bozuk render etmez.
- Calibri olmayan ortamda Carlito/Arial/Liberation Sans fallback sirasi kullanilabilir kalir.
- Times New Roman olmayan ortamda serif fallback ile belge okunabilir kalir.
- Zoom degisimi Saved/Unsaved state'i degistirmez.
- Zoom degisimi caret ve selection'i canonical belge mutationina donusturmez.

## M1 kabul disi

- Pixel-perfect Word parity.
- Canonical glyph shaping.
- Hyphenation engine.
- Multi-page pagination.
- Embedded font packaging.
- Header/footer layout.
