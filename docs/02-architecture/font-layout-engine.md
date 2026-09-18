# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/font-layout-engine.md
# 📌 Amac: Writer font olcumu, fallback, zoom ve sayfa layout sinirlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Platformlar arasi font ve layout farklarini kontrol altina alan canonical/render kontratini tanimlar

Bagimli Oldugu Katman: Documentation

# Font ve Layout Engine

## Hedef

Writer belge gorunumu Windows ve Linux arasinda olabildigince stabil kalmalidir. Piksel seviyesinde Microsoft Word kopyasi hedef degildir; belge semantigi ve fiziksel sayfa olculeri korunur.

## Canonical geometri

Belge page width, height ve margin degerlerini twip ile saklar. Bir inch 1440 twip'tir. Page geometry platform DPI'sindan ve UI zoom seviyesinden bagimsizdir.

Desktop render baseline 96 CSS pixel/inch referansi kullanir. Bu deger yalniz render adapteridir; canonical belgeye pixel yazilmaz.

## Zoom

Zoom bir belge ozelligi degildir. Desktop Repository session state'idir.

- Varsayilan: %100.
- Minimum: %50.
- Maksimum: %200.
- Adim: %10.

Zoom page geometry ve font render boyutuna ayni oranla uygulanir. Zoom degisikligi revision, dirty state, save veya undo/redo history uretmez.

## Font resolution

Font ismi dogrudan platform font path'i olarak saklanmaz. Belge requested logical family ve style ister. Platform adapter render icin uygun stack'i cozer.

Fallback sirasi:

1. Requested family.
2. Config ile tanimli uyumlu veya yakin fallback family.
3. Sistem generic sans-serif/serif/monospace fallback.

Fallback sonucu canonical `CharacterStyle.font_family` degerini degistirmez. Bu sayede Linux'ta fallback ile acilan belge Windows'ta kaydedildiginde orijinal font family bilgisini kaybetmez.

## Olcum ve shaping

M1 Desktop surface browser/webview layout motorunu render icin kullanir. DOM olcumu canonical layout karari sayilmaz.

Gelecekte text shaping ve glyph metric hesaplari tek layout interface arkasina alinacaktir. Web, Desktop ve Mobile ayni paragraph/layout semantigini uygulamalidir.

## Sayfa modeli

Page size ve margin canonical modelde fiziksel birimle tutulur. Paragraph spacing ve indent canonical modelde twip olarak zaten vardir; M1 font/layout baseline bu alanlarin tumunu henuz UI'da materialize etmez.

M1 tek page surface birinci section PageSettings degerini kullanir. Multi-page pagination, section break, header/footer reserve ve page-break motoru sonraki layout genislemesidir.

## DPI

Document dimension DPI'dan bagimsiz saklanir. Device DPI yalniz webview fiziksel render yogunlugunu etkiler; canonical geometry veya TKO verisini degistirmez.

## Test

Golden fixture setinde ayni belgenin Windows ve Linux render metrikleri tolerance ile karsilastirilacaktir. Tolerance degeri shaping/pagination motoru aktif oldugunda sabitlenecektir.
