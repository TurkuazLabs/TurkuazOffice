# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-clipboard-runtime-v0.2.0.md
# 📌 Amac: M1 Clipboard Minimum runtime copy/cut/paste entegrasyonunu tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Internal MIME, HTML sanitizer, plain-text fallback ve Desktop event akisini belgeler

Bagimli Oldugu Katman: Documentation

# M1 Clipboard Runtime

## Akis

`WriterParagraphEditor -> WriterController -> ClipboardService -> WriterSessionService -> TauriWriterTool -> Rust Writer Domain`

Browser clipboard erisimi `ClipboardTool` icinde, inert HTML parse erisimi `ClipboardDomTool` icinde tutulur. Clipboard Service representation secimi ve sanitizer kurallarini uygular.

## Copy

Ayni paragraf icindeki canonical secim run sinirlarina gore dilimlenir. Uc representation birlikte yazilir:

1. `application/x-turkuaz-office-fragment+json`
2. `text/html`
3. `text/plain`

Internal fragment yalniz schema version, kind ve typed styled run listesi tasir.

## Paste

Paste sirasi:

1. Gecerli Internal Turkuaz fragment.
2. Sanitized HTML.
3. Plain text.
4. Image-only payload (PNG/JPEG/WebP).

Internal fragment schema/version, run sayisi, toplam text uzunlugu, font family uzunlugu ve font size domain limitleri ile dogrulanir.

HTML path executable/resource taglarini drop eder. Basic B/I/U semantigi ile inline font family/font size/font weight/font style/text decoration degerleri canonical CharacterStyle'a map edilir. HTML hicbir zaman canonical DOM olarak saklanmaz.

Paste event'i Service tarafinda representation seciminden once consume edilir. Boylece gecersiz veya unsupported clipboard payload'i browser default contenteditable paste ile sanitizer'i bypass edemez.

Image-only payload Tool katmaninda File/ArrayBuffer'dan byte dizisine cevrilir ve `WriterSessionService.insertImageData` uzerinden canonical `InsertImageData` command'ine gider. M1 image paste block-level olarak aktif paragraph sonrasina image ekler.

## Cut

Cut once copy representationlarini yazar. Clipboard yazimi basarili olduktan sonra secili range `ReplaceRangeWithStyledRuns(..., [])` ile silinir. Tek typed command oldugu icin canonical history'de tek undo adimidir.

## Selection

Atomic fragment mutation basarili oldugunda Desktop selection caret'i `startOffset + insertedLength` noktasina collapse eder. Bu state sonraki render'da DOM selection restore icin kullanilir.

## Guvenlik ve limitler

- Clipboard payload upper bound uygulanir.
- HTML DOM parse node ve depth upper bound uygular; limit asiminda HTML reddedilip plain-text fallback denenir.
- Null byte temizlenir.
- Script/style/iframe/object/embed/svg/math/link/meta canonical text akimina girmez.
- Unknown HTML taglari semantik eklemeden yalniz guvenli text cocuklarini tasir.
- Remote resource URL'leri okunmaz veya canonical modele yazilmaz.

## Paragraph-local satir sonu kurali

M1 atomic replace yalniz tek paragraf destekledigi icin external HTML veya plain-text satir sonlari tek bosluga normalize edilir. Boylece cok satirli clipboard payload'i canonical paragraph icine ham newline sokmaz. Gercek paragraph split semantigi cross-paragraph clipboard diliminde eklenecektir.

## M1 siniri

Cross-paragraph rich fragment, image payloadlari ve table-aware adapterlar sonraki dilimlerdir.
