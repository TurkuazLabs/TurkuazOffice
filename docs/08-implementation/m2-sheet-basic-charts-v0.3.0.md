# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m2-sheet-basic-charts-v0.3.0.md
# 📌 Amac: M2 Basic Charts implementation kapsam ve mimari sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Canonical chart definition, Service data projection ve render-independent View akislarini tanimlar

Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View -> Language

# M2 Sheet Basic Charts

## Baseline

Desteklenen chart tipleri:

- Bar
- Line
- Pie

Chart definition canonical SheetDocument icinde saklanir. Chart metadata:

- stable ChartId
- worksheet id
- chart type
- title
- start/end row
- category column
- value column

## Data contract

Kategori kolonu text value kullanir.

Deger kolonu:

- finite number
- numeric FormulaCell

Formula degeri mevcut Formula Engine ile evaluate edilir. Chart Service kendi formula evaluatorunu tekrar etmez.

Tamamen bos source row chart point uretmez. Tek tarafi bos veya incompatible type olan row typed error verir.

## Katman siniri

Controller request aktarir.

SheetService chart lifecycle, validation ve data projection business kurallarini tasir.

SheetIdTool chart kimligi uretir.

View chart definition ve data point modellerini UI/API icin read-only tasir.

Render kutuphanesi, SVG/canvas, renk paleti veya Desktop chart editoru bu fazin disindadir.

## Resource limit

Bir chart maksimum 1000 source row kapsar.
