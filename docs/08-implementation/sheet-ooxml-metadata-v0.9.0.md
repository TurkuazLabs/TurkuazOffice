# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/sheet-ooxml-metadata-v0.9.0.md
# 📌 Amac: XLSX table ve conditional-format OOXML metadata round-trip mimarisini dokumante eder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.9.0
# Aciklama: Worksheet relationships, table parts, styles/dxfs ve desteklenen conditional rule mapping davranisini tanimlar
# Bagimli Oldugu Katman: Service | Model | Tool | Config | Sheet

# Sheet OOXML Metadata v0.9.0

## Kapsam

XLSX adapter artik yalniz cell value tasimaz. Canonical Sheet table ve desteklenen conditional-format metadata'si gercek OOXML package part'lari ile import/export edilir.

## Table parts

Her canonical SheetTable icin:

- worksheet icinde tableParts / tablePart r:id,
- xl/worksheets/_rels/sheetN.xml.rels icinde table relationship,
- xl/tables/tableN.xml table part,
- Content Types icinde table override

uretilir.

Table adi ve rectangular range canonical modele geri doner. OOXML tableColumns isimleri header satirindan turetilir; bos/uygunsuz header icin deterministic ColumnN fallback kullanilir.

## Conditional formatting

Desteklenen ortak Excel/Calc alt kumesi:

- Number Greater Than
- Number Less Than
- Number Equals
- Text Contains

Kurallar worksheet conditionalFormatting/cfRule olarak yazilir.

Semantic stiller xl/styles.xml dxfs bolumunde differential fill olarak tutulur:

- warning: FFFFC7CE
- success: FFC6EFCE
- accent: FFDDEBF7

Importta bilinen renkler canonical semantic stile map edilir. Tanimli fakat bilinmeyen differential fill Accent fallback alir.

## Guvenlik

- package/path traversal korumasi devam eder
- max table: 1024
- max conditional format: 512
- multi-range sqref bu baseline'da desteklenmez
- taninmayan conditional rule tipi sessizce kaybedilmez, strict reject edilir
- formula cell import/export mevcut profile gore strict reject kalir

## Mimari

OOXML metadata canonical modele sizmaz.

Archive Tool -> XML Tool -> XLSX Model -> XLSX Service -> Canonical SheetDocument
