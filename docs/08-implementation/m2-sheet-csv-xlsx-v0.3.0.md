# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m2-sheet-csv-xlsx-v0.3.0.md
# 📌 Amac: M2 Sheet CSV/XLSX implementation kapsam ve mimari sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: CSV Tool/Service, XLSX format model, ZIP/XML Tool ve canonical Sheet mapping katmanlarini tanimlar

Bagimli Oldugu Katman: Documentation

# M2 Sheet CSV/XLSX

## Mimari

CSV:

SheetDocument / Worksheet
-> SheetCsvService
-> SheetCsvTool
-> CSV bytes

XLSX:

SheetDocument
-> SheetXlsxService
-> XlsxWorkbookModel
-> SheetXlsxXmlTool + SheetXlsxArchiveTool
-> XLSX bytes

Import ters yonde ayni sinirlari kullanir.

Controller veya canonical Sheet domain CSV/XML/ZIP syntax bilgisi tasimaz.

## CSV semantigi

CSV type metadata tasimadigi icin import text-only'dir.

Bu karar veri koruma icindir:

001 degeri otomatik Number(1) yapilmaz.
TRUE degeri otomatik Boolean yapilmaz.

Export canonical typed degerleri stringe cevirir.

Sparse Worksheet exportu full rectangular Vec<Vec<String>> olusturmaz. Yalniz populated cell metadata'si tutulur; row/column gap'leri CSV byte akimina streaming yazilir. Bu karar XFD1048576 gibi uzak sparse adreslerde dev bos matris allocation riskini engeller.

## XLSX semantigi

M2 value-only profile:

- text.
- finite number.
- boolean.
- multiple worksheet.

Text export inlineStr kullanir.

Import hem inlineStr hem sharedStrings okur.

Bu sayede export shared-string table uretmeden basit kalirken yaygin XLSX dosyalari import edilebilir.

## Formula siniri

Formula XML gorulurse cached value kullanilarak sessiz veri kaybi/semantic downgrade yapilmaz.

Import:

SheetXlsxError::UnsupportedFormula

Basic Formula Engine canonical Sheet katmaninda sonraki M2 parca olarak tamamlanmistir. XLSX formula round-trip ise bu value-only minimum profile otomatik olarak eklenmez ve strict unsupported kalir.

## Package boundary

Zorunlu minimum parts:

- [Content_Types].xml
- _rels/.rels
- xl/workbook.xml
- xl/_rels/workbook.xml.rels
- worksheet parts

Root relationship gercek officeDocument relation type ile xl/workbook.xml'i gostermelidir.

Worksheet relationship target path'i normalize edilir ve traversal reddedilir.

## Resource limits

ZIP ve XML kaynak limitleri format adapter config katmaninda merkezi tutulur.

Canonical XLSX grid limitleri Sheet config'ten yeniden kullanilir.

## M2 siniri

Bu fazda yok:

- formula.
- cell style.
- number format.
- date/time type.
- merged cell.
- comments.
- chart.
- filter/sort state.
- images.
- named ranges.
- macros.

Bunlar CSV/XLSX minimum tamamlandi anlamina dahil degildir.
