# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/sheet-hybrid-office-ui-v0.5.0.md
# 📌 Amac: Turkuaz Sheet masaustu arayuzunde Excel ve LibreOffice Calc'in guclu kullanim desenlerini birlestiren standardi tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.5.0
# Aciklama: Klasik menu, compact toolbar, formula/ad kutusu, grid, sag Ozellikler dock'u ve gelecek verimlilik yolunu kaydeder
# Bagimli Oldugu Katman: View | Controller | Repo | Language | Config

# Sheet Hybrid Office UI v0.5.0

## Tasarim karari

Turkuaz Sheet herhangi bir urunun birebir arayuz kopyasi degildir.

Masaustu deneyimi iki olgun spreadsheet yaklasiminin guclu taraflarini birlestirir:

- Excel tipi hizli hucre/formul akisi ve yogun verimlilik araclari.
- LibreOffice Calc tipi klasik menu ve dock edilebilir ozellik paneli.
- Turkuaz Office'in sade, moduler ve fake komut kullanmayan UI kurali.

## Suite giris tabani

Standalone Writer/Sheet masaustu girisleri artik main dalindadir. Bu UI dilimi ortak uygulama ici modul switcher kullanmaz; dogrudan Sheet proses yuzeyini render eder.

## v0.5.0 aktif yuzey

- Klasik menu satiri.
- Compact format toolbar.
- Ad kutusu gibi secili hucre referansi.
- `fx` formula bari.
- A-Z ilk kolon projection'i.
- 100 satirlik ilk desktop viewport.
- Sagda acilip kapanabilir Ozellikler paneli.
- Ozellikler panelinden gercek kalin/italik/alti cizili, hizalama ve ondalik bicim komutlari.
- Veri menusu uzerinden filtre/siralama panelini ac-kapat.
- Alt status bar'da secim ve hesaplanmis deger gorunumu.

## Fake komut kurali

Backend veya Service davranisi olmayan Stiller, Galeri veya Islevler butonlari yalnizca goruntu icin eklenmez.

Bu moduller ancak gercek veri/model destegi eklendiginde sidebar'a acilir.

## Sonraki verimlilik sirasi

1. Multi-selection ve status bar Toplam/Ortalama/Sayim.
2. Freeze panes.
3. Table object ve otomatik filter header.
4. Conditional formatting.
5. SUM/AVERAGE/MIN/MAX/IF dahil function library.
6. Chart UI ve chart sidebar.
7. Styles ve Gallery modulleri.

## Mimari

View gercek Sheet komutlarini Controller uzerinden cagirir. Sag panel dogrudan domain/backend erisimi yapmaz. Dil metinleri merkezi Language paketinde tutulur.
