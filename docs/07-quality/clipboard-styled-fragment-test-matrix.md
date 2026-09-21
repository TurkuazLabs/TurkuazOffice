# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/clipboard-styled-fragment-test-matrix.md
# 📌 Amac: Clipboard styled-fragment canonical mutation acceptance senaryolarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Styled paste, cut, style preservation, undo ve sinir hata senaryolarini listeler

Bagimli Oldugu Katman: Documentation

# Clipboard Styled Fragment Test Matrix

| ID | Senaryo | Beklenen |
|---|---|---|
| STYLED-01 | Ayni paragrafta styled fragment replace | Range disindaki prefix ve suffix text/stilleri korunur |
| STYLED-02 | Birden fazla farkli stilli fragment run | Her non-empty run kendi CharacterStyle degeriyle canonical belgeye girer |
| STYLED-03 | Secili tum paragrafi empty fragment ile cut | Paragraf bosalir ve tek editable bos TextRun kalir |
| STYLED-04 | Styled paste sonrasi undo | Tek undo ile command oncesi snapshot geri gelir |
| STYLED-05 | Collapsed range + empty fragment | EmptyCommand ile reddedilir |
| STYLED-06 | Cross-paragraph styled replace | Acik CrossParagraphFragmentReplaceNotSupported hatasi verir |
| STYLED-07 | Read-only file session mutation | Desktop Service ensure_writable guard islemi reddeder |
| STYLED-08 | IPC payload | DTO text + CharacterStyle disinda canonical olmayan state tasimaz |

## M1 kabul siniri

Bu matrix atomic canonical mutation dilimini kapsar. OS clipboard event, sanitizer, internal MIME ve external application interoperability matrix'i Clipboard Minimum entegrasyon diliminde ayrica kapatilacaktir.


## Desktop runtime acceptance

| Senaryo | Beklenen |
|---|---|
| Turkuaz -> Turkuaz styled copy/paste | Internal MIME secilir, run stilleri korunur |
| Browser/Word basic rich HTML paste | Script/resource taglari atilir, B/I/U/font family/font size whitelist ile map edilir |
| Internal MIME bozuk schema | HTML varsa HTML, yoksa plain text fallback kullanilir |
| Rich representation yok | Plain text insertion style ile atomic replace edilir |
| Cut | Clipboard representationlari yazilir ve secim bos fragment ile tek undo adiminda silinir |
| Read-only belge | Cut/paste canonical mutation yapmaz |
| Paste/cut sonrasi selection | Caret eklenen fragment sonuna collapse olur |

| Cok satirli plain text/HTML paste | M1 paragraph-local sinirinda satir sonlari tek bosluga normalize edilir |

| Asiri derin veya cok node'lu HTML | HTML representation reddedilir, plain text varsa fallback edilir |
| Font family icinde HTML entity/metacharacter | Copy HTML serializer CSS ve HTML baglaminda escape eder |

## Otomatik frontend regression

- Vitest 5.0.1 ile ClipboardService headless unit testleri.
- Browser DOM gerektirmeden ClipboardTool/ClipboardDomTool ve WriterSessionService sinirlari mock edilir.
- CI frontend-quality job'u build sonrasinda `npm test` calistirir.

| Image-only PNG/JPEG/WebP paste | Browser default paste consume edilir, binary payload InsertImageData ile active paragraph sonrasina eklenir |
| Unsupported/gecersiz clipboard payload | Event consume edilir; raw contenteditable default paste calismaz |
