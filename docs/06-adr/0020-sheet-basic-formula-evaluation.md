# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0020-sheet-basic-formula-evaluation.md
# 📌 Amac: M2 Basic Formula Engine canonical representation ve evaluation sinirlarini kalici karar olarak kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Formula source storage, same-worksheet reference semantigi, strict numeric evaluation ve dependency guard kararlarini tanimlar

Bagimli Oldugu Katman: Documentation

# ADR 0020 - Sheet Basic Formula Evaluation

## Durum

Accepted.

## Karar

Canonical Sheet cell degeri FormulaCell tasiyabilir.

FormulaCell source expression'i saklar. Parse edilmis AST canonical persistence state degildir; FormulaTool tarafindan gerektiginde uretilir.

Basic Formula Engine yalnizca:

- numeric literal.
- same-worksheet cell reference.
- simple A1.
- absolute/mixed A1 parse.
- + - * /.
- unary + / -.
- parentheses.

destekler.

## Evaluation

SheetService dependency resolution sorumlusudur.

- Formula Tool storage veya repository bilmez.
- Controller formula business logic tasimaz.
- Empty referenced cell numeric 0 kabul edilir.
- Number reference numeric operand olur.
- Formula reference recursive evaluate edilir.
- Text ve Boolean implicit numeric coercion yapmaz.
- Division by zero typed error verir.
- Non-finite result typed error verir.
- Dependency cycle typed error verir.
- Dependency depth merkezi limit ile sinirlidir.

## Bilerek disarida

- Cross-worksheet reference.
- Named range.
- SUM/AVERAGE ve diger functions.
- Range expression.
- String formula.
- Date/time formula.
- Copy/fill relative reference rewrite.
- Volatile functions.
- Cached formula persistence.
- XLSX formula import/export.

Bu kapsamlar ayrica tasarlanir; basic engine sessizce bunlari taklit etmez.
