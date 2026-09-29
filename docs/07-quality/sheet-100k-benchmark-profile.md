# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-100k-benchmark-profile.md
# 📌 Amac: M2 Sheet 100000-cell performans workload ve olcum kontratini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: Sparse build, View projection ve sorted row query benchmarklarini CI regression testlerinden ayirir

Bagimli Oldugu Katman: Documentation

# Sheet 100000-Cell Benchmark Profile

## Dataset

- 100000 populated cell.
- Tek worksheet.
- Tek numeric column.
- Row adresleri 0..99999.
- Sparse BTreeMap canonical storage.
- Format ve chart metadata bos.

## Workload A - Sparse Build + View Projection

Olculen asamalar:

- 100000 numeric cell ile canonical BTreeMap build.
- SheetDocument -> SheetDocumentView deterministic projection.

Dogruluk bariyerleri:

- canonical cell count 100000.
- View cell count 100000.
- ilk row 0.
- son row 99999.

## Workload B - Sorted Row Query

Olculen asama:

- 100000-row range uzerinde descending numeric sort.

Dogruluk bariyerleri:

- result count 100000.
- ilk result row 99999.
- son result row 0.

## Calistirma

Benchmark normal cargo test akisinda ignored tutulur.

Explicit profil:

cargo test -p turkuaz-office-sheet --test sheet_100k_benchmark -- --ignored --nocapture

## Timing policy

Elapsed milliseconds raporlanir fakat shared/hosted CI makinesine sabit millisecond pass/fail threshold baglanmaz.

Performance regression karari ayni donanim ve ayni toolchain uzerinde tekrarlanan olcumlerle verilir. Functional correctness assert'leri benchmark icinde zorunludur.
