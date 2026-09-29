# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-formula-engine-test-matrix.md
# 📌 Amac: M2 Basic Formula Engine parse, evaluation ve dependency test kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Operator precedence, A1 references, cycle/depth/division ve strict numeric semantics bariyerlerini kalite kontrati yapar

Bagimli Oldugu Katman: Documentation

# Sheet Formula Engine Test Matrix

## Parser

Supported:

- leading = prefix.
- numeric literal.
- simple A1.
- absolute A1: $A$1.
- mixed A1: $A1 ve A$1.
- + - * /.
- unary + / -.
- parentheses.
- ASCII whitespace.

Rejected:

- missing = prefix.
- invalid A1.
- incomplete expression.
- expression over 4096 Unicode scalar.
- nesting depth over 64.
- operator count over 128.
- non-finite numeric literal.

## Evaluation

- operator precedence.
- parentheses.
- recursive same-worksheet formula dependency.
- empty referenced cell -> 0.
- raw formula source canonical CellValue icinde korunur.
- evaluated request Formula cell'i Number read-result olarak dondurur.
- same formula set revision no-op olur.

## Typed errors

- invalid formula.
- dependency cycle.
- dependency depth overflow.
- division by zero.
- text/boolean numeric reference.
- non-finite evaluation result.

## Adapter boundary

CSV/XLSX minimum profilleri FormulaCell'i sessiz text/cached value'a dusurmez.

Export:

- SheetCsvError::UnsupportedFormula.
- SheetXlsxError::UnsupportedFormula.

XLSX formula import mevcut minimum profile icinde UnsupportedFormula kalir.

## Regression

sheet_formula_engine_tests.rs:

- parser precedence + absolute/mixed reference.
- parser nesting/operator complexity limits.
- formula revision no-op.
- raw vs evaluated cell.
- dependency chain + empty reference.
- cycle/division/non-numeric typed errors.
- invalid formula no-mutation.
- thin Controller/View formula flow.
