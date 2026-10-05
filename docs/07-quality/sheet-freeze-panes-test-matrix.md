# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-freeze-panes-test-matrix.md
# 📌 Amac: Sheet freeze panes kalite kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.5.2
# Aciklama: Selection-based freeze, filtered row projection, sticky geometry ve unfreeze davranisini listeler
# Bagimli Oldugu Katman: Service | Repo | View | Config

# Sheet Freeze Panes Test Matrix

| Alan | Dogrulama | Beklenen |
| --- | --- | --- |
| Selection | C3 + freeze at selection | 2 row / 2 column |
| Filter | Sorted/filtered row selection | Visible row index kullanilir |
| Top row | Freeze Top Row | rows=1 columns=0 |
| First column | Freeze First Column | rows=0 columns=1 |
| Unfreeze | Unfreeze Panes | rows=0 columns=0 |
| Domain | Freeze komutu | Document revision degismez |
| View | Frozen A1 | sticky top=24px left=36px |
| View | Freeze boundary | Accent separator |
| Config | Grid geometry | Merkezi sabitler |
