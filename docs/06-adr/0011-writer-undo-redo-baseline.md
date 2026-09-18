# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0011-writer-undo-redo-baseline.md
# 📌 Amac: Writer undo/redo ilk implementasyon stratejisini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Snapshot history baseline ve monotonik revision kararini belgeler

Bagimli Oldugu Katman: Documentation

# ADR 0011 - Writer Undo Redo Baseline

## Durum

Accepted as M1 baseline.

## Karar

Ilk Writer headless domain undo/redo mekanizmasi tam WriterDocument snapshot history kullanir. History limiti config katmanindan gelir.

Undo veya redo document revision degerini eski snapshot degerine geri sarmaz. Her undo/redo yeni bir mutation kabul edilir ve current revision + 1 kullanir.

## Neden

Bu model ilk milestone'da command dogrulugunu ve exact state restore davranisini basitlestirir. Autosave/sync revision monotonikligini korur.

## Gelecek optimizasyon

Large-document performance budget snapshot maliyetini asarsa internal implementation inverse command veya operation log'a tasinabilir. WriterEditorService public kontrati degismemelidir.
