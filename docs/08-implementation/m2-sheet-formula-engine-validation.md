# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m2-sheet-formula-engine-validation.md
# 📌 Amac: M2 Basic Formula Engine static, regression ve compiler-backed validation durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Formula layer contract, typed error coverage ve hosted runner sinirini ayri raporlar

Bagimli Oldugu Katman: Documentation

# M2 Sheet Basic Formula Engine Validation

## Static contract

- Formula parser Tool katmanindadir.
- Document/repository lookup FormulaTool icinde yoktur.
- Dependency evaluation Service katmanindadir.
- Controller business logic tasimaz.
- Formula source canonical CellValue icinde korunur.
- Parsed AST ve evaluated result canonical persistence state degildir.
- Formula limits Config katmanindadir.
- CSV/XLSX formula downgrade yapmaz.

## Regression

Covered:

- precedence.
- parentheses.
- unary operators.
- simple/absolute/mixed A1.
- recursive dependency.
- empty reference = 0.
- raw formula preservation.
- evaluated numeric read.
- revision no-op.
- invalid formula no-mutation.
- cycle.
- division by zero.
- non-numeric reference.
- adapter formula export reject.

## Compiler-backed durum

Hosted GitHub runner checkout/step baslatmadan steps=null failure verirse compiler-backed cargo sonucu onaylanmis sayilmaz.

Source/static contract ve regression kodu tamamlanir; gercek cargo fmt/check/test sonucu runner calistiginda ayrica dogrulanir.
