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
