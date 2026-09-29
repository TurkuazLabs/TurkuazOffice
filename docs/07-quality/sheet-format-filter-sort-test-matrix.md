# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-format-filter-sort-test-matrix.md
# 📌 Amac: M2 Sheet format/filter/sort regression ve kaynak-limit kabul matrisini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Canonical format metadata, formula-aware filter, deterministic sort ve non-mutating query kurallarini kilitler

Bagimli Oldugu Katman: Documentation

# Sheet Format / Filter / Sort Test Matrix

## Format

- Cell format canonical document metadata'sidir.
- Bold, italic, underline, horizontal alignment ve decimal places baseline kapsamdadir.
- Default format sparse mapte tutulmaz.
- Ayni format tekrar uygulanirsa revision artmaz.
- Decimal places 0..12 araligindadir.

## Filter

- Non-empty, text contains, number greater/less ve boolean equals baseline kosullaridir.
- Formula hucreleri query sirasinda mevcut Formula Engine ile evaluate edilir.
- Non-finite numeric filter girdisi reddedilir.
- Filter kolonu query range disindaysa typed error doner.

## Sort

- Ascending ve descending tek kolon sort baseline'dir.
- Query canonical cell yerlesimini mutate etmez.
- Esit degerlerde original row index deterministic tie-break'tir.
- Mixed value type sirasi deterministic tutulur.

## Resource limits

- Query range maksimum 100000 satirdir.
- Grid disi veya ters range reddedilir.
- Query sonucu yalniz sparse range icinde veri bulunan satirlari kapsar.
