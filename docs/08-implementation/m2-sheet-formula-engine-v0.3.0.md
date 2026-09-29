# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m2-sheet-formula-engine-v0.3.0.md
# 📌 Amac: M2 Basic Formula Engine implementation kapsam ve katman sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: FormulaCell canonical modeli, FormulaTool parser ve SheetService recursive evaluation akislarini tanimlar

Bagimli Oldugu Katman: Documentation

# M2 Sheet Basic Formula Engine

## Canonical model

CellValue yeni Formula(FormulaCell) varyantini tasir.

FormulaCell:

- expression source.
- source leading = ile saklanir.
- parsed AST persistence state degildir.
- cached result persistence state degildir.

Bu sayede canonical state formula kaynagini kaybetmez ve evaluator sonucu belge verisiyle karistirilmaz.

## Tool

FormulaTool sadece formula syntax parse eder.

Typed AST:

- Number.
- Reference.
- Unary.
- Binary.

Reference parser simple/absolute/mixed A1'i canonical zero-based CellAddress'e indirger.

FormulaTool Repo veya SheetDocument okumaz.

## Service

SheetService:

- formula set validation.
- recursive dependency resolution.
- same-worksheet cell lookup.
- empty cell numeric 0 semantigi.
- cycle detection.
- dependency depth limit.
- division-by-zero guard.
- text/boolean numeric coercion reject.
- non-finite result guard.

is kurallarini tasir.

## Controller ve View

Controller yalnizca:

- set_formula_a1.
- evaluated_cell_a1.

requestlerini Service'e iletir.

Raw CellView formula source'u Formula(String) olarak gosterebilir.

Evaluated cell request canonical formuleyi mutate etmeden Number view sonucu dondurur.

## External format siniri

CSV/XLSX minimum value-only adaptorleri formula profile'i kazanmaz.

Canonical FormulaCell export edilirse typed UnsupportedFormula doner.

XLSX input formula XML'i de mevcut minimum profile icinde strict reject olmaya devam eder.

## Sonraki M2

Formula Engine sonrasinda roadmap sirasi:

- format/filter/sort.
- basic charts.
- 100000-cell benchmark profile.

Cross-sheet reference, range/functions ve copy/fill formula rewrite ayrica planlanir.
