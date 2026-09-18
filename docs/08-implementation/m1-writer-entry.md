# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-writer-entry.md
# 📌 Amac: Desktop Writer v0.2.0 implementation baslangic sirasini ve kabul kriterlerini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Foundation sonrasi ilk urun modulu Writer icin kontrollu gelistirme giris planidir

Bagimli Oldugu Katman: Documentation

# M1 Writer Entry Plan

## Hedef surum

Writer yeni bir modul oldugu icin TurkuazLabs version standardina gore hedef `v0.2.0` olur.

## Faz 1 - Headless Writer Domain

Durum: tamamlandi.

Once UI olmadan Writer domain kurulacak:

1. WriterDocument.
2. Section.
3. Paragraph.
4. TextRun.
5. Stable NodeId.
6. CharacterStyle.
7. ParagraphStyle.
8. PageSettings.
9. ImageRef.
10. Table minimum model.

Domain type'lari DOM, Tauri veya DOCX class'larina baglanmaz.

## Faz 2 - Command Engine

Durum: temel Writer command seti ve undo/redo tamamlandi.

Belge mutation dogrudan View tarafindan yapilmaz. Ilk command seti:

- InsertText.
- DeleteRange.
- SplitParagraph.
- MergeParagraph.
- SetCharacterStyle.
- SetParagraphStyle.
- InsertImage.
- InsertTable.

Her command undo bilgisi uretebilir veya inverse operation modeli ile calisir. Undo/redo tasarimi text editor yazildiktan sonra eklenen bir yama olmayacak; ilk mutation API'sinin parcasi olacaktir.

## Faz 3 - Selection Model

Durum: logical NodeId + Unicode offset selection kontrati tamamlandi.

Selection UI pixel koordinati degildir. NodeId + logical offset tabanli selection canonical editor state icinde tutulur.

Minimum selection:

- Caret.
- Range.
- Cross-run range.
- Cross-paragraph range.

## Faz 4 - Native TKO Serializer Profile v1

Durum: tamamlandi. TKO v1 iki allowlist entry kullanir: `manifest.yml` ve `content/writer.yml`. ZIP Stored container, YAML DTO adapteri, current-schema save guard ve Local Open/Save regression testleri aktiftir.

Minimum round-trip:

1. Empty document.
2. Multi paragraph.
3. Mixed bold/italic.
4. Unicode text.
5. Image ref.
6. Basic table.
7. Page settings.

## Faz 5 - Desktop Shell

Durum: Tauri 2 + SolidJS shell, native dialog, rich-text/IME ve Local Open/Save tamamlandi; print/file-watcher gibi sinirlar sonraki M1 alt fazlarindadir.

Headless Writer contract testleri gecmeden Tauri editor UI is kurali tasimaz. Desktop shell su sorumluluklarla baslar:

- Window lifecycle.
- Open/Save dialog Tool adapter.
- Clipboard Tool adapter.
- File watcher Tool adapter.
- Printer Tool adapter boundary.
- Writer View host.

## Faz 6 - UI Framework Prototype

Durum: tamamlandi; ADR 0013 ile SolidJS secildi.

ADR 0004 geregi UI framework bugun rastgele secilmez. Ayni Writer prototype ekraninda en az su kriterler olculur:

- Large document DOM/render behavior.
- Selection stability.
- IME input.
- Keyboard event control.
- Accessibility semantics.
- Memory overhead.
- Tauri integration complexity.

Prototype sonucunda framework karari yeni ADR ile Accepted durumuna getirilir.

## Faz 7 - DOCX Minimum Profile

DOCX ilk native model degildir. Import/export adapter canonical Writer domain uzerinden calisir.

Minimum profile:

- Paragraph.
- Text run.
- Bold/italic/underline.
- Font family/size.
- Alignment.
- Page size/margin.
- Basic table.
- Basic image.

Desteklenmeyen ozellik silent drop edilmez; compatibility report uretir.

## v0.2.0 cikis bariyeri

- Windows smoke test.
- Linux smoke test.
- TKO round-trip fixtures.
- DOCX minimum profile fixtures.
- Autosave/recovery smoke test.
- External-change conflict smoke test.
- Keyboard-only basic editing test.
- Turkish + English labels.
- Performance budget baseline.
- Docs + changelog + third-party notice update.
