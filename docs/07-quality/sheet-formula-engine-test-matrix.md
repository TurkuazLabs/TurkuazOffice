# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-formula-engine-test-matrix.md
# 📌 Amac: M2 Basic Formula Engine parser, evaluator, dependency ve error test kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Arithmetic precedence, A1 reference, SUM range, cycle/div0/type ve recursion/resource limitlerini kalite kontrati yapar

Bagimli Oldugu Katman: Documentation

# Sheet Formula Engine Test Matrix

## Canonical model

CellValue baseline:

- Text.
- Number.
- Boolean.
- Formula(source).

Formula source canonical cell degerinde korunur.

Evaluation sonucu formula source'unu mutate etmez.

EvaluatedCellValue:

- Text.
- Number.
- Boolean.

Formula sonucu M2 Basic profilinde Number'dir.

## Syntax

Formula source '=' ile baslar.

Supported:

- numeric literal.
- scientific numeric literal.
- unary + / -.
- + - * /.
- parentheses.
- same-worksheet A1 reference.
- A1:B2 range.
- SUM(arg, arg, range).

Unsupported:

- cross-sheet reference.
- absolute/mixed reference.
- named range.
- string literal.
- boolean literal.
- comparison.
- function other than SUM.

## Evaluation semantics

- operator precedence: unary -> multiply/divide -> add/subtract.
- missing referenced cell = 0.
- Number reference numeric olarak kullanilir.
- Formula reference recursively evaluate edilir.
- Text/Boolean arithmetic reference TypeMismatch.
- divide by zero typed error.
- non-finite result typed error.
- circular dependency typed error.
- range yalniz SUM icinde gecerli.

## Resource limits

- formula source max 8192 byte.
- parser nesting max 64.
- expression evaluation depth max 128.
- formula dependency depth max 128.
- SUM range max 100000 cell.

## Revision

- valid formula set revision increment eder.
- ayni formula tekrar set edilirse no-op.
- invalid formula repository'e yazilmaz.

## External format boundary

CSV/XLSX minimum formula cell'i cached/evaluated value'a sessiz flatten etmez.

- CSV export: FormulaUnsupported.
- XLSX export: UnsupportedFormula.
- XLSX formula import mevcut minimumda UnsupportedFormula.

XLSX formula round-trip ayri format-profile genisletmesi olmadan supported sayilmaz.

## Regression

sheet_formula_tests.rs:

- parser precedence.
- parentheses.
- unsupported function.
- reference arithmetic.
- unary.
- missing cell zero.
- SUM range + expression arguments.
- cycle.
- division by zero.
- type mismatch.
- range limit.
- invalid source rejection.
- same-formula revision no-op.
- Controller raw/evaluated separation.

sheet_csv_xlsx_tests.rs:

- formula export strict reject.
