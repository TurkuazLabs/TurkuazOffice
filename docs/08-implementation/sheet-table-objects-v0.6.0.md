# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/sheet-table-objects-v0.6.0.md
# 📌 Amac: Turkuaz Sheet canonical Table Object ve otomatik header filtre davranisini dokumante eder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.6.0
# Aciklama: Selection'dan tablo olusturma, canonical range metadata, scoped row-query ve XLSX kayip korumasini tanimlar
# Bagimli Oldugu Katman: Controller | Service | Repo | Tool | View | Language | Config

# Sheet Table Objects v0.6.0

## Kapsam

Turkuaz Sheet tablo nesnesini yalnizca gorsel bir bicim olarak degil canonical belge metadata'si olarak saklar.

Bir tablo:

- stabil TableId,
- worksheet kimligi,
- workbook icinde otomatik uretilen Table1, Table2 ... adi,
- header satirini ve data satirlarini kapsayan rectangular range

tasir.

## Olusturma kurallari

Tablo mevcut rectangular selection'dan olusturulur.

- En az bir header satiri ve bir data satiri gerekir.
- Ayni worksheet'te iki canonical table range'i cakisma yapamaz.
- Basarili create/remove belge revision'ini bir artirir.
- View kendi tablo metadata'sini uretmez.

Akis:

View -> Controller -> SheetSessionService -> TauriSheetTool -> Tauri Controller -> Desktop Service -> SheetController -> SheetService -> Repo

## Otomatik filtre basliklari

Canonical tablonun ilk satiri header satiridir.

Header hucrelerinde filtre dugmesi render edilir. Dugme ilgili hucreyi aktif eder ve mevcut typed filter/sort panelini acar.

Filtre/siralama icin yeni bir motor yazilmaz. Mevcut canonical row-query motoru su range ile tekrar kullanilir:

- start row = table.startRow + 1
- end row = table.endRow
- columns = table.startColumn .. table.endColumn

Bu nedenle header satiri sorguya dahil edilmez ve tablo disindaki Sheet satirlari gizlenmez.

## XLSX kayip korumasi

Mevcut XLSX adapter value-only profildedir ve OOXML table parts yazmaz.

Canonical tablolu belgeyi normal XLSX olarak export etmek tablo metadata'sini sessizce kaybedecegi icin export:

- SheetXlsxError::UnsupportedTable

ile strict reject eder.

OOXML table parts, relationships ve autofilter serialization tamamlanmadan bu guard kaldirilmaz.

## CSV

CSV tek worksheet deger akisi oldugu icin table metadata export etmez. CSV import yeni canonical belgede bos table katalogu olusturur.

## Sonraki adimlar

- tablo adini degistirme
- structured references
- total row
- table style katalogu
- OOXML table import/export
