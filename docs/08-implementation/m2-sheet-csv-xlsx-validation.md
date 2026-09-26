# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m2-sheet-csv-xlsx-validation.md
# 📌 Amac: M2 Sheet CSV/XLSX static, regression, security ve compiler-backed validation durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Format adapter katmanlari, strict unsupported contract ve hosted runner sinirini ayri raporlar

Bagimli Oldugu Katman: Documentation

# M2 Sheet CSV/XLSX Validation

## Static contract

- format-adapters crate Sheet domain'e dependency tasir.
- Sheet domain format adapter crate'ine dependency tasimaz.
- CSV syntax Tool katmanindadir.
- CSV canonical mapping Service katmanindadir.
- XLSX ZIP Tool katmanindadir.
- SpreadsheetML XML Tool katmanindadir.
- XLSX format-specific model canonical Sheet modelinden ayridir.
- XLSX canonical mapping Service katmanindadir.
- magic string ve resource limitler sheet_constants.rs icindedir.

## CSV regression

- quote.
- escaped quote.
- CRLF.
- text-only import.
- typed export.
- sparse empty field.

## XLSX regression

- text/number/boolean.
- multi-sheet round-trip.
- sharedStrings import.
- root workbook relationship validation.
- formula strict reject.

## Security validation

- ZIP package/entry/count limits.
- unsafe path reject.
- duplicate entry reject.
- XML size/depth/node limits.
- DOCTYPE reject.
- relationship target traversal reject.

## Quick-XML API validation

Implementation quick-xml 0.42 UTF-8 API contractina gore yazilmistir.

Element/attribute names str tabanli; GeneralRef str tabanlidir.

## Compiler-backed durum

Hosted GitHub runner gercek cargo step baslatmadan steps=null failure verirse CSV/XLSX compile/test sonucu onaylanmis sayilmaz.

Static/source regression tamamlanir; compiler-backed validation pending kalir.
