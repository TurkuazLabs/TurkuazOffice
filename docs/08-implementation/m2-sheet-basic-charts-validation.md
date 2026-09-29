# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m2-sheet-basic-charts-validation.md
# 📌 Amac: M2 Basic Charts static, regression ve compiler-backed validation durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Chart domain, Service/Controller/View sinirlari ve hosted runner durumunu ayri raporlar

Bagimli Oldugu Katman: Documentation

# M2 Sheet Basic Charts Validation

## Static contract

- ChartId / ChartType / SheetChart canonical Service tipleridir.
- Chart definitions SheetDocument icinde BTreeMap ile deterministic saklanir.
- Chart ID uretimi SheetIdTool icindedir.
- Lifecycle ve data projection SheetService icindedir.
- Controller business logic tasimaz.
- View render-independent ChartDataView tasir.
- Limitler config constants icindedir.

## Regression

sheet_basic_charts_tests.rs:

- canonical create + revision
- Bar/Line/Pie type surface
- formula-aware numeric data
- invalid title/range
- incompatible category/value type
- Controller/View mapping

## Compiler-backed durum

Hosted runner kaynak kod adimlarini baslatmadan steps=null failure verirse compiler-backed sonuc pending tutulur. Basarili iddia icin gercek cargo test ve static verify logu gerekir.
