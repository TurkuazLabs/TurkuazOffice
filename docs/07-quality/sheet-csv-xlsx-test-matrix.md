# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-csv-xlsx-test-matrix.md
# 📌 Amac: M2 Sheet CSV/XLSX parser, mapping, round-trip ve guvenlik test kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: CSV text-only semantigi, XLSX typed value round-trip, package/XML limitleri ve unsupported formula bariyerlerini kalite kontrati yapar

Bagimli Oldugu Katman: Documentation

# Sheet CSV/XLSX Test Matrix

## CSV minimum

Import:

- UTF-8.
- UTF-8 BOM kabul edilir.
- comma delimiter.
- quoted field.
- escaped quote.
- CRLF / LF / CR row ayrimi.
- empty field sparse olarak saklanmaz.
- type metadata olmadigi icin non-empty tum alanlar Text olarak import edilir.
- 001 -> Text("001"), Number'a infer edilmez.

Export:

- Sparse worksheet rectangular matrix olarak materialize edilmez.
- Output row/column gap'leri streaming yazilir.
- Text.
- finite Number.
- Boolean -> TRUE/FALSE.
- comma/quote/newline iceren alanlar quote edilir.
- embedded quote iki quote ile escape edilir.
- CRLF row separator.

Limits:

- 16 MiB CSV payload.
- 1048576 row.
- 16384 column.

## XLSX minimum

Import:

- ZIP Stored/Deflate.
- root relationship -> xl/workbook.xml validation.
- workbook.xml.
- workbook.xml.rels.
- multiple worksheet.
- inlineStr text.
- sharedStrings text.
- number.
- boolean.
- canonical A1 validation.
- duplicate cell reject.
- worksheet-name validation.
- case-insensitive duplicate worksheet-name reject.
- XML 1.0 invalid control character reject.

Export:

- valid package skeleton.
- inlineStr text output.
- number output.
- boolean output.
- multi-sheet workbook.
- deterministic sparse BTreeMap iteration.

## Strict unsupported boundary

Formula cell M2 CSV/XLSX fazinda sessiz cached-value import yapmaz.

Formula gorulurse:

SheetXlsxError::UnsupportedFormula

Basic Formula Engine canonical Sheet katmaninda sonraki M2 parca olarak tamamlandi; ancak bu CSV/XLSX minimum format profili formula round-trip'i henuz desteklemez.

Unsupported/invalid cell type typed hata ile reddedilir.

## Security

- package max 64 MiB.
- max 1024 ZIP entry.
- max entry 32 MiB.
- max 256 worksheet.
- traversal / absolute-backslash / duplicate ZIP entry reject.
- XML max 16 MiB.
- XML depth max 128.
- XML node max 2000000.
- DOCTYPE reject.
- relationship target traversal reject.

## Regression

sheet_csv_xlsx_tests.rs:

- CSV quote + CRLF parse.
- CSV text-only import.
- CSV typed export serialization.
- CSV max-grid sparse export memory regression.
- XLSX multi-sheet typed round-trip.
- sharedStrings import.
- root workbook relationship validation.
- duplicate worksheet-name reject.
- XML-invalid cell text reject.
- formula reject.
