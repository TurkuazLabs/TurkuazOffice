# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/sheet-basic-charts-ui-v0.11.0.md
# 📌 Amac: Sheet Basic Charts Desktop UI v0.11.0 uygulama kontratini ve katman sinirlarini belgeler
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.11.0
# Aciklama: Canonical chart engineini Desktop IPC, Service, Controller, sag panel ve dependency-free SVG renderer ile kullanici yuzeyine baglar
# Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View -> Language

# Sheet Basic Charts UI v0.11.0

## Kapsam

v0.11.0 yeni bir chart engine yazmaz. M2 icinde mevcut olan canonical ChartId, ChartType, SheetChart, create/remove lifecycle ve chart_data projection hatti Desktop uygulamasina acilir.

Desteklenen grafik tipleri:

- Bar
- Line
- Pie

## Secim kontrati

Desktop create akisi yalnizca tam iki kolonluk rectangular selection kabul eder.

- ilk kolon: kategori
- ikinci kolon: deger
- satir araligi: secimin start/end row sinirlari
- title: bos olamaz
- point limiti: canonical Sheet katmanindaki 1000 point limiti

Frontend bu kurali kullanici affordance'i icin tekrar goruntuler; authoritative validation Service ve canonical Sheet domain tarafinda kalir.

## Katman akisi

View -> Controller -> Service -> Tool -> Tauri Controller -> Desktop Service -> Canonical Sheet Controller -> Sheet Service

Canonical state frontend tarafinda yeniden uretilmez. SheetDocumentDto artik chart listesini de tasir. chartCount compatibility alani korunur.

## Chart data state

Secilen chart icin projected ChartDataView ayri Repo signal'inda tutulur. Async chart data istekleri generation guard ile korunur. Daha eski bir istek daha yeni chart secimini ezemez.

Hucre mutation'i sonrasi aktif chart varsa chart data yeniden yuklenir. Chart kaldirilirsa secili chart ve projected data temizlenir.

## SVG renderer

Renderer ucuncu taraf chart kutuphanesi kullanmaz.

- Bar: sifir baseline destekli rectangle projection
- Line: deterministic polyline + point projection
- Pie: pozitif degerlerden slice projection; pozitif toplam yoksa empty state

Renderer yalniz sunum geometrisi hesaplar; chart semantics ve formula evaluation canonical Sheet domaininde kalir.

## UI

Charts paneli Properties ve Functions ile ayni sag dock alanini paylasir. Paneller ayni anda acik tutulmaz.

Panel:

- chart type secimi
- chart title
- selection hint
- create action
- canonical chart listesi
- secili chart SVG preview
- remove action
- stabil error-code gorunumu

## Kapsam disi

- drag/resize
- multi-series
- chart style editor
- legend editor
- axis editor
- XLSX chart OOXML round-trip
