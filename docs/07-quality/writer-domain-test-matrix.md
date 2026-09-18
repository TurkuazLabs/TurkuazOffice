# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/writer-domain-test-matrix.md
# 📌 Amac: Writer v0.2.0 headless domain test kapsamini ve cikis bariyerini ayrintili tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Domain tree, command, selection, history ve TKO logical profile test matrix'ini tutar

Bagimli Oldugu Katman: Documentation

# Writer Domain Test Matrix

## Belge olusturma

- Yeni belge tek Section ile baslar.
- Section en az bir Paragraph tasir.
- Paragraph en az bir TextRun ile baslar.
- Initial revision `0` olur.
- Schema version current Core schema ile aynidir.

## Text mutation

- InsertText caret konumuna text ekler.
- Offset UTF-8 byte degil Unicode scalar sayar.
- Empty insert typed hata verir.
- Same-run DeleteRange dogru araligi siler.
- Cross-run DeleteRange stil run sinirlarini tamamen gereksiz yere flatten etmez.
- Cross-run suffix, selection sonrasi bulunan run'lardan once kalir.
- Cross-paragraph DeleteRange iki paragraph kalanini birlestirir.
- Cross-section DeleteRange silent behavior yerine acik unsupported hata verir.

## Structural mutation

- SplitParagraph prefix/suffix text'i iki paragraph'a ayirir.
- Split sonrasi yeni paragraph stabil NodeId alir.
- MergeParagraph sadece adjacent paragraph icin kabul edilir.
- InsertTable rows/columns limitlerini Config'ten uygular.
- Her table cell canonical paragraph/run ile baslar.
- InsertImage asset reference'i domain block olarak ekler.

## Style

- SetCharacterStyle yalniz hedef run'i degistirir.
- SetParagraphStyle yalniz hedef paragraph'i degistirir.
- Style mutation revision ilerletir.

## Undo / redo

- Her successful command undo snapshot uretir.
- Yeni command redo stack'i temizler.
- Undo exact onceki content state'ini geri getirir.
- Redo undo edilen state'i tekrar uygular.
- Revision undo/redo ile geriye gitmez; monotonik artar.
- History Config limitini asinca en eski snapshot drop edilir.

## TKO logical profile

- Manifest document id content ile aynidir.
- Manifest revision content ile aynidir.
- Manifest schema content ile aynidir.
- Writer document kind zorunludur.
- Future schema reject edilir.
- Older schema migration calismadan restore edilmez; MigrationRequired verir.
- Bilinmeyen TKO format version reject edilir.
- Build -> restore canonical document equality saglar.

## CI bariyeri

`v0.2.0 Writer Domain` merge edilmeden once su komutlar basarili olmalidir:

```powershell
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
```

Windows ve Linux test matrix'i GitHub Actions ile calistirilir.

## Rich-text range regressionlari

- Partial run `ApplyCharacterStyle` plain texti degistirmeden run boundary olusturur.
- Ayni style'a donusen komsu run'lar compact edilir.
- `CharacterStylePatch` belirtilmeyen style alanlarini korur.
- Desktop minimal diff styled paragraph run bilgisini komple flatten etmez.
- Unicode scalar diff emoji fixture'i UTF-8/UTF-16 byte semantigine kaymaz.
