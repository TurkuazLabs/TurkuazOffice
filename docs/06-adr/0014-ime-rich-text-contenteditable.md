# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0014-ime-rich-text-contenteditable.md
# 📌 Amac: Writer Desktop rich-text editor yuzeyi, IME transaction ve DOM selection kararini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Contenteditable yalnizca platform input yuzeyi olarak kullanilir; canonical state ve format mutation Rust Writer Core'da kalir

Bagimli Oldugu Katman: Documentation

# ADR 0014 - IME-aware Rich Text Contenteditable Surface

## Durum

Accepted.

## Problem

Textarea prototipi tek bir plain-text degerini duzenleyebiliyordu. Writer ise TextRun bazli bold, italic, underline ve ileride font/renk/link gibi stilleri ayni paragraf icinde gostermek zorundadir. Ayrica Windows WebView2 ve Linux WebKitGTK IME composition eventleri sirasinda yarim composition degerlerinin canonical document history'ye yazilmasi veri ve undo kalitesini bozar.

## Karar

Desktop Writer paragraph input yuzeyi `contenteditable` tabanli olacaktir.

Bu karar DOM'u canonical model yapmaz. DOM yalnizca platform input/render adaptorudur.

Canonical akis:

1. Rust Writer Domain TextRun agacini tutar.
2. Tauri DTO TextRun style alanlarini frontend'e read-only tasir.
3. SolidJS View run'lari span olarak render eder.
4. Browser Selection DOM noktasi `DomSelectionTool` ile paragraph logical Unicode scalar offsetine cevrilir.
5. Composition basladiginda debounce commit durur.
6. Composition bittiginde tek paragraph edit transaction commit edilir.
7. Paragraph text degisikligi komple replace edilmez; Rust Desktop Service common-prefix/common-suffix minimal diff uretir.
8. B/I/U secili logical range icin typed `ApplyCharacterStyle` command uretir.
9. Command Service secim sinirlarinda run boler, style patch uygular ve ayni stile sahip komsu run'lari birlestirir.

## Selection kontrati

Desktop gecici UI selection state'i:

- `paragraphId`
- `startOffset`
- `endOffset`

tasir.

Offset birimi ADR 0010 ile aynidir: Unicode scalar count. DOM UTF-16 birimi Core'a sizmaz.

Bu milestone B/I/U icin tek paragraph selection destekler. Cross-paragraph rich-text selection bilincli olarak sonraki Writer selection engine fazina birakilmistir.

## IME kurali

`compositionstart` ile `compositionend` arasinda backend commit yapilmaz. Browser DOM composition buffer'i tamamladiktan sonra tek commit planlanir. Enter/format shortcutlari composition aktifken document commandina donusturulmez.

## Undo semantigi

Minimal diff icindeki delete + insert komutlari `execute_batch` ile tek history entry ve tek revision olarak kaydedilir. IME composition sonucu da tek paragraph commit oldugu icin tek undo adimi hedeflenir.

## Guvenlik ve veri sahipligi

Frontend DOM veya Solid signal canonical belge olarak kabul edilmez. Tauri cevabindan gelen read-model her mutation sonrasi authoritative snapshot'tir.

## Sonuclar

Olumlu:

- Rich TextRun rendering mumkun olur.
- IME yarim degerleri history'ye girmez.
- Unicode selection Windows/Linux browser farkindan ayrilir.
- Web istemcisi icin ayni logical selection kontrati tekrar kullanilabilir.
- Text editleri komsu run style bilgisini gereksiz yere silmez.

Bedeller:

- DOM selection restore kodu gerekir.
- Contenteditable browser davranislari platform smoke test ister.
- Cross-paragraph selection ve caret typing-style state henuz ayrica gelistirilmelidir.
