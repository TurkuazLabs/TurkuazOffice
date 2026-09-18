# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0019-twip-page-layout-font-fallback.md
# 📌 Amac: Writer sayfa olcumu, zoom ve font fallback kararini kalici mimari karar olarak kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Canonical twip geometriyi render olceginden ve platform font resolution kararindan ayirir

Bagimli Oldugu Katman: Documentation

# ADR 0019 - Twip Page Layout ve Font Fallback

## Durum

Kabul edildi.

## Baglam

Writer belge modeli page width, height ve margin alanlarini twip olarak zaten tasiyordu. Desktop prototipi ise sayfayi sabit CSS pixel degerleriyle ciziyordu. Bu durum TKO page settings bilgisini render akisinin disinda birakiyor ve zoom davranisini belge geometrisiyle karistirma riski tasiyordu.

Font family canonical belgeye logical ad olarak yazilir. Windows ve Linux ayni font ailesine sahip olmayabilir. Fallback render karari canonical belgeyi degistirmemelidir.

## Karar

1. Canonical page geometry birimi twip olarak kalir.
2. Desktop render referansi 96 CSS pixel/inch kabul eder.
3. Device DPI canonical page geometryyi degistirmez.
4. Zoom sadece Desktop session state'tir; save, revision ve undo history'ye girmez.
5. M1 tek sayfa yuzeyi birinci section `PageSettings` degerini kullanir.
6. Requested font family canonical `CharacterStyle` icinde aynen korunur.
7. Platform font resolver yalniz render icin fallback CSS stack'i uretir.
8. Fallback sonucu canonical font family alanina geri yazilmaz.
9. Browser DOM text measurement canonical layout karari sayilmaz.
10. Gercek shaping, pagination ve golden metric tolerance ayri layout-engine fazinda genisletilecektir.

## Sonuclar

- TKO sayfa olculeri Desktop render'a gercek veri kaynagi olur.
- %50-%200 zoom belgeyi dirty yapmaz.
- Windows/Linux font farki belge semantigini degistirmez.
- CSS fallback stack'i kullanilirken requested family kaybolmaz.
- M1 henuz gercek multi-page pagination veya platform-independent shaping motoru degildir.
