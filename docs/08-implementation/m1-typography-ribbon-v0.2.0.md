# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-typography-ribbon-v0.2.0.md
# 📌 Amac: Writer v0.2.0 typography, typing-style ve ribbon alt fazi implementation kapsamini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Font family/size, paragraph alignment, styled insert ve iki katmanli ribbon uygulamasini ayrintili izler

Bagimli Oldugu Katman: Documentation

# M1 Typography + Ribbon v0.2.0

## Tamamlanan domain parcasi

- `CharacterStylePatch` font family ve font size alanlariyla genisletildi.
- Font family bos deger ve maksimum uzunluk kontrolu eklendi.
- Font size minimum/maksimum half-point kontrolu eklendi.
- `InsertStyledText` Writer command eklendi.
- Styled insert mevcut run'i caret noktasinda boler ve komsu stilleri korur.
- Ayni stile sahip komsu run'lar yeniden compact edilir.
- `ParagraphStylePatch` eklendi.
- `ApplyParagraphStyle` command eklendi.
- Paragraph alignment read-model'e tasindi.

## Tamamlanan Desktop backend parcasi

- Paragraph minimal diff path opsiyonel typing style kabul eder.
- Insert diff typing style varsa `InsertStyledText` uretir.
- Character style IPC font family ve font size patch tasir.
- Paragraph alignment icin ayri typed IPC command eklendi.
- Tauri paragraph DTO alignment state tasir.

## Tamamlanan frontend parcasi

- `WriterSessionRepository` gecici typing-style signal tasir.
- Collapsed caret B/I/U artik gecici typing-style degistirir.
- Collapsed caret font family ve size ayni typing-style kontratini kullanir.
- Gercek input commit oldugunda typing-style canonical styled insert commandina donusur.
- Range selection font/B/I/U canonical style mutation olarak kalir.
- Paragraph alignment left/center/right/justify aktif edildi.
- Paragraph View backend alignment degerini CSS `text-align` ile render eder.
- Pointer ve caret navigation typing-style reset siniri eklendi.
- Font select fokusundan once pending paragraph flush edilir.
- Blur commit gecerli DOM selection yoksa son Repository selection degerini korur.
- Font family/size seciminden sonra kayitli logical selection ve editor fokusu otomatik restore edilir.

## Ribbon

Desktop Writer eski tek satir toolbar yerine iki katmanli ribbon baseline kullanir.

Tablar:

- Dosya - shell, sonraki faz.
- Giris - aktif.
- Ekle - shell, sonraki faz.
- Gorunum - shell, sonraki faz.

Aktif Giris gruplari:

- Belge: Yeni Belge, Save/Print placeholder.
- Gecmis: Undo/Redo.
- Yazi Tipi: font family, size, B/I/U.
- Paragraf: left/center/right/justify.

## Bilincli sinirlar

- Font discovery sistem font listesinden dinamik yapilmiyor; ilk baseline merkezi allow-list kullanir.
- Font fallback/layout parity ayri font-layout milestone'unda genisleyecek.
- Paragraph spacing, indent, list ve line-height UI henuz yoktur.
- Cross-paragraph rich-text selection henuz yoktur.
- Dosya/Ekle/Gorunum tablari command implementation acmaz.
- Save, Print, DOCX ve PDF halen sonraki M1 parcasi.
