# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m2-sheet-formula-engine-v0.3.0.md
# 📌 Amac: M2 Basic Formula Engine implementation kapsam ve mimari sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Formula parser Tool, evaluation Service, canonical Formula cell ve SheetService/Controller integration'ini tanimlar

Bagimli Oldugu Katman: Documentation

# M2 Basic Formula Engine

## Mimari

Formula source:

CellValue::Formula(String)

Syntax:

FormulaParserTool -> FormulaExpression AST.

Evaluation:

SheetFormulaService -> Worksheet sparse cell map.

Document lifecycle:

SheetController -> SheetService -> SheetFormulaService / Repo / Tool.

Controller formula business logic tasimaz.

## Parser

Recursive-descent parser:

- expression.
- term.
- unary.
- primary.
- SUM.
- A1 reference.
- A1 range.

Precedence:

1. unary + / -.
2. * /.
3. + -.

Parentheses explicit grouping saglar.

## Evaluator

Formula source canonical cell'de kalir.

Evaluation read-path'tir; source veya workbook revision mutate edilmez.

Reference semantigi:

- missing -> 0.
- Number -> numeric.
- Formula -> recursive evaluation.
- Text / Boolean -> TypeMismatch.

SUM:

- expression arguments.
- range arguments.
- reversed range boundary normalize.
- range cell cap.

## Error model

Typed FormulaError:

- parser error.
- dependency depth.
- expression depth.
- circular reference.
- range outside function.
- range too large.
- type mismatch.
- division by zero.
- non-finite result.

SheetError FormulaError'i typed olarak tasir.

## View boundary

CellValueView::Formula raw source'u tasir.

EvaluatedCellValueView formula resultini Number olarak tasir.

Raw formula ile evaluated result ayni alanmis gibi davranmaz.

## External format boundary

Formula engine canonical modelde aktif olsa da CSV/XLSX M2 minimum profile formula round-trip'i otomatik olarak aktif etmez.

CSV ve XLSX export formula hucrede strict typed unsupported hata verir.

Bu karar computed value'yu yazarak formula source kaybetmeyi engeller.

## M2 siniri

Bu fazda yok:

- cross-sheet reference.
- absolute/mixed A1.
- named ranges.
- IF.
- MIN/MAX/AVERAGE.
- comparison operators.
- string functions.
- dates.
- volatile functions.
- dependency graph cache.
- incremental recalculation graph.
- XLSX formula round-trip.

Bunlar future formula/profile genisletmeleridir.
