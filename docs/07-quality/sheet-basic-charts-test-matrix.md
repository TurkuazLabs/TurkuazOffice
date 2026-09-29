# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-basic-charts-test-matrix.md
# 📌 Amac: M2 Basic Charts lifecycle, data projection ve validation test kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Bar/Line/Pie canonical chart tanimi, formula-aware numeric data ve typed error bariyerlerini kilitler

Bagimli Oldugu Katman: Documentation

# Sheet Basic Charts Test Matrix

## Canonical model

- ChartId deterministic Tool ile uretilir.
- ChartType: bar, line, pie.
- Chart definition SheetDocument icinde canonical metadata olarak saklanir.
- Definition worksheet, title, row range, category column ve value column tasir.
- Create/remove revision artirir.

## Validation

- Title bos olamaz ve maksimum 128 karakterdir.
- Row range ters veya grid disi olamaz.
- Category ve value kolonu ayni olamaz.
- Chart maksimum 1000 point kapsar.
- Missing worksheet typed error doner.

## Data projection

- Category cell text olmalidir.
- Value cell finite number veya numeric formula olabilir.
- Formula value mevcut Basic Formula Engine ile evaluate edilir.
- Tamamen bos satirlar chart data icinde atlanir.
- Partial/mismatched row typed error doner.

## Controller / View

- Controller chart create/remove/data requestlerini yalniz Service'e aktarir.
- SheetDocumentView deterministic chart listesi tasir.
- ChartDataView render-independent point listesi tasir.
