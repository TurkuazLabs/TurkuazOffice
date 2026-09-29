# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m2-sheet-format-filter-sort-v0.3.0.md
# 📌 Amac: M2 Sheet format/filter/sort implementation kapsam ve mimari sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Canonical sparse format metadata ile Service-owned non-mutating filter/sort query akislarini tanimlar

Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View -> Language

# M2 Sheet Format / Filter / Sort

## Canonical format

SheetDocument cell_formats alani worksheet kimligi ve CellAddress uzerinden sparse format metadata tasir.

Baseline CellFormat:

- bold
- italic
- underline
- horizontal alignment: general/left/center/right
- decimal places: 0..12

Default format fiziksel mapte tutulmaz. Format mutation revision semantigine dahildir.

## Filter / sort

Filter ve sort business kurallari SheetService icindedir. Controller request aktarir, View typed sonucu tasir.

Baseline filter:

- non-empty
- text contains
- number greater than
- number less than
- boolean equals

Baseline sort:

- tek kolon
- ascending / descending
- deterministic row-index tie break

Formula degerleri query icin Formula Engine ile evaluate edilir.

## Non-mutating query karari

Bu faz row query sonucunu siralanmis row-index View olarak dondurur. Canonical hucre adresleri fiziksel olarak tasinmaz.

Bunun nedeni Basic Formula Engine henuz copy/fill veya relative-reference rewrite semantigi tanimlamamistir. Fiziksel row sort formula referanslarini sessizce bozmayacaktir.

## Adapter siniri

CSV/XLSX minimum adapterleri onceki value-only profillerini korur. Canonical format metadata'nin XLSX style round-trip'i bu fazin disindadir.
