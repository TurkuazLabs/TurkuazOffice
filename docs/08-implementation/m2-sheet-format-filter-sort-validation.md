# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m2-sheet-format-filter-sort-validation.md
# 📌 Amac: M2 Sheet format/filter/sort static, regression ve compiler-backed validation durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Katman sinirlari, typed validation, formula-aware query ve hosted runner durumunu ayri raporlar

Bagimli Oldugu Katman: Documentation

# M2 Sheet Format / Filter / Sort Validation

## Static contract

- CellFormat canonical Service modelindedir.
- Format storage SheetDocument icinde sparse tutulur.
- Format mutation Service katmanindadir.
- Filter/sort business logic SheetService icindedir.
- Controller format/query requestlerini yalniz Service'e aktarir.
- View format ve row-query DTO'larini tasir.
- Query limitleri config constants icindedir.

## Regression

sheet_format_filter_sort_tests.rs:

- sparse format persistence
- same-format revision no-op
- default-format cleanup
- decimal-place validation
- formula-aware numeric filter
- deterministic ascending sort
- non-mutating query revision
- Controller/View mapping

## Compiler-backed durum

workspace-ci run #49 dort job'da da checkout/source step'i baslatmadan steps=null failure verdi. Bu nedenle compiler-backed sonuc onaylanmis sayilmaz; basarili iddia icin gercek cargo test ve static verify step logu gerekir.
