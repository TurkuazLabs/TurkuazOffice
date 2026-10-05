# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-function-library-test-matrix.md
# 📌 Amac: Sheet function library kalite kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.8.0
# Aciklama: Parser, range aggregate, comparison, lazy IF, cycle ve resource limit testlerini listeler
# Bagimli Oldugu Katman: Tool | Service | Controller | View

# Sheet Function Library Test Matrix

| Alan | Dogrulama | Beklenen |
| --- | --- | --- |
| Parser | SUM/AVERAGE/MIN/MAX/IF | Case-insensitive parse |
| Parser | comma + semicolon | Iki separator da kabul |
| Parser | unknown function | UnknownFunction |
| Parser | IF arg count | Tam 3 arguman |
| Range | A1:A10 | Rectangular range |
| Range | ters endpoint | Normalize edilir |
| Aggregate | number cells | Hesaba katilir |
| Aggregate | formula cells | Canonical evaluator kullanilir |
| Aggregate | text/boolean/empty | Yok sayilir |
| SUM | range + scalar | Numeric toplam |
| AVERAGE | numeric values | Numeric ortalama |
| MIN/MAX | numeric values | Dogru extremum |
| Comparison | = <> > >= < <= | 1 veya 0 |
| IF | true branch | Yalniz true branch evaluate |
| IF | false branch | Yalniz false branch evaluate |
| Cycle | self range | FormulaCycle |
| Resource | >100000 expanded cells | FormulaRangeTooLarge |
| Context | raw range scalar | FormulaRangeNotAllowed |
