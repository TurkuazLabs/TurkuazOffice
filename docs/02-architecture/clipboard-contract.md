# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/clipboard-contract.md
# 📌 Amac: Writer ve diger moduller icin platformlar arasi clipboard kontratini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Plain text, rich text, table, image ve internal fragment siralamasini tanimlar

Bagimli Oldugu Katman: Documentation

# Clipboard Contract

## Veri sirasi

Turkuaz Office copy islemi uygun oldugunda birden cok representation sunabilir:

1. Internal Turkuaz fragment.
2. HTML rich text.
3. Plain text.
4. Image payload.

Paste islemi desteklenen en zengin ve guvenli representation'i secer.

## Internal fragment

Internal fragment schema-versioned olur. Clipboard verisi repository veya service nesnesini serialize etmez; yalniz public transfer DTO kullanir.

## Dis uygulamalar

Word, LibreOffice ve browser kaynakli HTML temizlenir. Script, event handler, remote execution ve guvenilmeyen resource direktifleri kabul edilmez.

## Table

Tabular paste Writer ve Sheet tarafinda farkli Service adapterlari ile yorumlanir. Controller clipboard semantigi tasimaz.

## Test

Windows ve Linux icin copy/paste matrix fixture ile tutulur. M1 minimumu plain text + basic rich text + image'dir.


## Atomic Writer fragment mutation

M1 Writer canonical mutation kontrati `ReplaceRangeWithStyledRuns` command'ini kullanir. Fragment yalniz `text + CharacterStyle` typed run listesi tasir.

Paste, secili veya collapsed ayni-paragraf range'ini styled run listesiyle tek command olarak degistirir. Cut ayni command'e bos fragment verir. Bu nedenle her paste/cut canonical history'de tek undo adimidir.

Range disinda kalan prefix/suffix TextRun stilleri korunur. Fragment run'lari kendi canonical CharacterStyle degerleriyle yazilir. Tum paragraf bosalirsa editable bos TextRun invariant'i korunur.

Bu M1 diliminde cross-paragraph rich fragment replace desteklenmez ve acik typed hata verir. Runtime OS clipboard, sanitizer ve representation selection bu atomic command'in ustunde Tool/Service katmaninda kalir.
