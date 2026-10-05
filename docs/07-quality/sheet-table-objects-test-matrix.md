# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-table-objects-test-matrix.md
# 📌 Amac: Sheet Table Object ve otomatik filtre header kalite kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.6.0
# Aciklama: Canonical lifecycle, overlap, scoped query, UI header ve XLSX kayip korumasini listeler
# Bagimli Oldugu Katman: Service | Controller | Tool | Repo | View | Adapter

# Sheet Table Objects Test Matrix

| Alan | Dogrulama | Beklenen |
| --- | --- | --- |
| Domain | Ilk tablo | Table1 + revision artisi |
| Domain | Ikinci tablo | Table2 |
| Domain | Tek satir range | InvalidTableRange |
| Domain | Cakisan range | TableRangeOverlap |
| Domain | Remove table | Revision artar |
| Controller | Create table | Typed SheetTableView |
| Desktop | Create/remove IPC | Canonical document doner |
| Service | Selection -> table | Selection range aynen backend'e gider |
| Service | Header filter | Sorgu yalniz data range'de calisir |
| View | Header cell | Otomatik filtre dugmesi gorunur |
| View | Table query | Table disi satirlar gorunur kalir |
| XLSX | Canonical table export | UnsupportedTable |
| Regression | Empty table catalog | Eski value-only XLSX davranisi korunur |
