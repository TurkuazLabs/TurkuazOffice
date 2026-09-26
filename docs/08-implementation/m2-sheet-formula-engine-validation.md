# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m2-sheet-formula-engine-validation.md
# 📌 Amac: M2 Basic Formula Engine static, regression, resource ve compiler-backed validation durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Parser/Service layer contract, cycle/depth/range bariyerleri ve hosted runner sinirini ayri raporlar

Bagimli Oldugu Katman: Documentation

# M2 Basic Formula Engine Validation

## Static contract

- formula source canonical CellValue varyantidir.
- FormulaParserTool Tool katmanindadir.
- parser AST Tool katmaninda kalir.
- evaluation business logic SheetFormulaService icindedir.
- SheetService document/repository coordination yapar.
- Controller yalniz Service cagirir.
- raw Formula View ile evaluated View ayridir.
- limits config/constants.rs icindedir.

## Regression

sheet_formula_tests.rs:

- arithmetic precedence.
- parentheses.
- A1 reference.
- SUM range.
- unary.
- missing reference zero.
- invalid source.
- cycle.
- divide by zero.
- type mismatch.
- range cap.
- revision no-op.
- Controller raw/evaluated split.

## Resource validation

- parser nesting.
- expression recursion.
- dependency recursion.
- SUM range cell count.
- formula length.

## External format regression

CSV/XLSX computed value ile formula source'u sessizce degistirmez.

Formula export strict typed hata verir.

## Compiler-backed durum

Hosted GitHub runner gercek cargo step baslatmadan steps=null failure verirse Formula Engine compile/test sonucu onaylanmis sayilmaz.

Static/source regression tamamlanir; compiler-backed validation pending kalir.
