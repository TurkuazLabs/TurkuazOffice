# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-writer-domain-v0.2.0.md
# 📌 Amac: Writer v0.2.0 headless domain implementation durumunu ve sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Domain tree, command, selection, undo/redo ve TKO logical profile sonuclarini belgeler

Bagimli Oldugu Katman: Documentation

# M1 Writer Domain v0.2.0

## Tamamlanan alanlar

- Canonical WriterDocument / Section / Block / Paragraph / TextRun modeli.
- Stabil NodeId kontrati.
- CharacterStyle ve ParagraphStyle.
- A4 tabanli PageSettings varsayilani.
- ImageBlock ve minimum Table modeli.
- NodeId + Unicode scalar offset tabanli Selection.
- InsertText.
- DeleteRange; ayni section icinde cross-run ve cross-paragraph.
- SplitParagraph.
- MergeParagraph.
- SetCharacterStyle.
- SetParagraphStyle.
- InsertImage.
- InsertTable.
- Snapshot tabanli undo/redo baseline.
- Undo/redo dahil monotonik document revision.
- InMemory Writer repository.
- Controller -> Service -> Repo -> Tool -> View -> Language katman ayrimi.
- TKO v1 logical package profile; old schema migration guard ve future schema guard dahil.

## Bilincli sinirlar

### TKO archive encoding

Bu milestone TKO manifest/content semantigini freeze etmeye baslar fakat ZIP/YAML byte encoding henuz eklenmemistir. Archive Tool adapter, security limitleri ve dependency secimi Rust toolchain ile CI calistirilabildigi asamada eklenecektir.

### Text offset

Canonical offset v0.2.0 domain asamasinda Unicode scalar sayisidir. UI adapter DOM/UTF-16 offsetini canonical offset'e cevirmek zorundadir. Grapheme cluster davranisi IME prototype sonucuyla ADR seviyesinde tekrar degerlendirilecektir.

### Undo/redo

Ilk implementasyon dogruluk icin tam document snapshot kullanir. Public Service API snapshot modeline bagli degildir. Large-document benchmark sonrasi operation/inverse-command history'ye gecilebilir.

### Table text selection

Table hucre modeli canonical tree icindedir fakat table-cell selection/edit command seti bu milestone kapsaminda degildir.

### Cross-section delete

DeleteRange farkli section sinirini gecerse acik hata verir. Section merge semantigi page/layout karari ile birlikte tanimlanacaktir.

## Sonraki adim

M1'in siradaki parcasi Desktop Shell + Writer View host'tur. UI framework secimi ADR 0004 geregi IME, keyboard, accessibility, memory ve large-document prototype ile olculecektir.
