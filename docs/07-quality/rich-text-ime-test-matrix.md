# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/rich-text-ime-test-matrix.md
# 📌 Amac: Writer rich-text, logical selection ve IME davranisi icin zorunlu test matrisini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Domain style mutation, minimal diff, Unicode mapping, keyboard ve platform IME kontrollerini listeler

Bagimli Oldugu Katman: Documentation

# Rich Text + IME Test Matrix

## Rust otomatik

| Alan | Fixture | Beklenti |
| --- | --- | --- |
| Style | Tek run partial B | Run iki parcaya bolunur, plain text degismez |
| Style | Multi-run ayni style | Adjacent run fragmentation compact edilir |
| Style | B/I/U patch | Yalniz patch alanlari degisir |
| Text edit | Styled paragraph sonunda insert | Mevcut styled run bilgisi kaybolmaz |
| Text edit | Unicode emoji ortasinda insert | Scalar offset korunur |
| History | Minimal diff delete+insert | Tek revision / tek undo entry |

## Frontend static/automatic

- TypeScript strict typecheck.
- Vite production build.
- `DomSelectionTool` UTF-16 -> logical mapping fixture.
- Logical -> DOM restore fixture.
- Selection signal degisince toolbar `aria-pressed` state guncellemesi.
- Backend revision sonrasi paragraph component remount olmadan selection restore.
- Composition aktifken Enter ve B/I/U command dispatch olmamasi.

## Windows manual

- Turkce klavye: i/I ve noktasiz i inputu.
- Emoji caret hareketi ve B/I/U selection.
- Microsoft Pinyin veya baska CJK IME composition fixture.
- Composition ortasinda toolbar/Enter ile yarim commit olmamasi.
- Mouse ile sec -> Bold -> selection restore.
- Shift+Arrow ile sec -> Ctrl+B/Ctrl+I/Ctrl+U.
- Undo ile tek IME commit geri alma.

## Linux manual

- WebKitGTK contenteditable input.
- IBus/Fcitx mevcutsa composition fixture.
- Wayland/X11 selection davranisi.
- Mouse ve keyboard selection restore.
- B/I/U render parity.

## Exit criteria

Rich-text input production-ready sayilmaz; Rust tests + frontend build + Windows/Linux manual IME smoke sonucu birlikte bulunmalidir.
