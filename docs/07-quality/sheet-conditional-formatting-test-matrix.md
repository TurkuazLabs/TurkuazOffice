# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-conditional-formatting-test-matrix.md
# 📌 Amac: Sheet conditional formatting kalite kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.7.0
# Aciklama: Canonical lifecycle, formula match, priority, UI projection ve XLSX kayip korumasini listeler
# Bagimli Oldugu Katman: Service | Controller | Tool | Repo | View | Adapter

# Sheet Conditional Formatting Test Matrix

| Alan | Dogrulama | Beklenen |
| --- | --- | --- |
| Domain | Create rule | Canonical metadata + revision artisi |
| Domain | Remove rule | Metadata silinir + revision artisi |
| Domain | Invalid numeric | InvalidConditionalFormat |
| Domain | Empty text contains | InvalidConditionalFormat |
| Formula | Numeric formula sonucu | Numeric rule'a katilir |
| Formula | Evaluation error | Yalniz ilgili hucre no-match |
| Priority | Iki matching rule | En dusuk priority kazanir |
| Priority | Ilk rule silinir | Sonraki matching rule gorunur |
| Desktop | Selection range -> create | Typed IPC request |
| Desktop | Single cell -> create | Tek hucre range |
| Desktop | Cell mutation | Match projection yenilenir |
| View | warning/success/accent | Semantic CSS class |
| View | Status | Canonical rule sayisi gorunur |
| XLSX | Rule metadata export | UnsupportedConditionalFormat |
