# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/performance-budgets.md
# 📌 Amac: Turkuaz Office hafiflik iddiasini olculebilir performans budgetlari ile tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Startup, RAM, large document ve save/open benchmark hedeflerini takip eder

Bagimli Oldugu Katman: Documentation

# Performance Budgets

## Neden budget

"Hafif" kelimesi olculebilir olmadikca kalite kriteri degildir. Bu dosya M1 benchmarklarinda stabil donanim profili ile olculen hedefleri tutar.

## Foundation hedefleri

Asagidaki degerler ilk engineering budget'tir; release benchmark donanimi belirlendiginde ADR olmadan daraltilabilir, gevsetme ise gerekce ister.

| Metric | M1 engineering budget |
|---|---:|
| Warm desktop launch to usable shell | <= 2.0 s |
| Idle RAM, empty Writer | <= 250 MB |
| 100-page basic Writer open | <= 3.0 s |
| 100-page basic Writer save | <= 3.0 s |
| Editor typing input latency p95 | <= 50 ms |
| Undo/redo common command p95 | <= 50 ms |
| Recovery snapshot UI stall | <= 100 ms |

## Sheet ileri hedef

100000 populated simple cells acma/scroll testi M2'de benchmark setine eklenir.

## Regression

CI her platformda tam performance benchmark kosmak zorunda degildir. Stable benchmark job release/pre-release adaylarinda calisir ve onceki baseline ile regression raporu uretir.
