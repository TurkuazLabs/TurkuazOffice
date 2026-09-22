# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-print-preview-validation.md
# 📌 Amac: M1 Print Preview + Print Minimum implementation validation durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Static contract, frontend test ve hosted runner sinirlarini ayri ayri raporlar

Bagimli Oldugu Katman: Documentation

# M1 Print Preview Validation

## Static contract

- Print config merkezi dosyada tutulur.
- PrintTool fiziksel page rule ve system dialog cagrisini kapsar.
- Controller print logic tasimaz.
- WriterSessionService preview/print is akisini koordine eder.
- WriterSessionRepository preview layout ve print error state'ini session-only tutar.
- WriterPrintPreview yalniz render ve user action forwarding yapar.
- Ctrl+P ve Escape keyboard contracti config key sabitleri ile calisir.

## Regression

Vitest PrintTool testi canonical twip physical page size degerinin inch tabanli @page kuralina cevrildigini dogrular.

## Runtime acceptance

- Preview acildiginda mevcut WriterPage read-only render edilir.
- Print action sistem print dialogunu acar.
- Kagit ciktisinda shell chrome ve preview toolbar gizlenir.
- Preview kapatmak belge revision veya dirty state'i degistirmez.

## CI runner durumu

GitHub hosted runner joblari step baslamadan failure olursa bu durum kod compile/test failure'i olarak yorumlanmaz. Compiler-backed sonuc ancak gercek step logu bulunan run ile onaylanir.
