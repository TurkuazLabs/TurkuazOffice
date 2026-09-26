# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-cell-model-test-matrix.md
# 📌 Amac: M2 Sheet cell model, A1 reference ve sparse mutation test kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Grid limitleri, value validation, revision no-op ve deterministic sparse storage bariyerlerini kalite kontrati yapar

Bagimli Oldugu Katman: Documentation

# Sheet Cell Model Test Matrix

## Canonical model

- SheetDocument.
- Worksheet.
- WorksheetId.
- CellAddress.
- CellValue: text / number / boolean.
- Cell.
- Sparse BTreeMap storage.
- Document schema version.
- Monotonic revision.

## Grid

Zero-based canonical address:

- row: 0..1048575.
- column: 0..16383.

A1 Tool dis representation:

- A1 -> row 0, column 0.
- aa10 -> row 9, column 26.
- XFD1048576 -> max valid address.
- A0 / 1A / absolute-reference syntax / overflow reject.

M2 Cell Model simple A1 referansini kapsar. Absolute/mixed references Formula Engine fazinda genisletilebilir.

## Values

Canonical value set after Formula Engine follow-on phase:

- Text.
- Number.
- Boolean.
- Formula source.

Validation:

- Text max 32767 Unicode scalar.
- Number finite olmali.
- NaN / +Inf / -Inf reddedilir.

Formula ilk Cell Model slice'inda aktif degildi; sonraki M2 Basic Formula Engine parcasi CellValue::Formula source varyantini ekledi. Formula evaluation kontrati sheet-formula-engine-test-matrix.md icindedir.

## Sparse mutation

Set:

- only populated cells stored.
- changed value increments revision.
- same value is no-op.

Clear:

- cell BTreeMap entry removed.
- existing clear increments revision.
- missing clear is no-op.

## Determinism

BTreeMap row/column order View conversioninda korunur.

Bu karar sonraki CSV/XLSX serialization ve 100000-cell benchmark icin deterministic iteration baseline'idir.

## Regression

sheet_cell_model_tests.rs:

- default sparse worksheet.
- A1 boundary parse/format.
- invalid reference rejection.
- set/get/clear + revision.
- text/number/grid validation.
- thin Controller -> View flow.
