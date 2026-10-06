# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/desktop-ui-refresh-v0.12.0.md
# 📌 Amac: Turkuaz Office Desktop Start Center, Writer ve Sheet gorsel yenileme uygulamasini belgeler
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.12.0
# Aciklama: Onaylanan UI konseptini mevcut Controller/Service/Repo kontratlarini bozmadan Desktop View katmanina uygular
Bagimli Oldugu Katman: View -> Controller -> Repo -> Tool -> Language

# Desktop UI Refresh v0.12.0

Bu dilim Turkuaz Office masaustu deneyimini tek bir urun ailesi olarak yeniden duzenler. Degisiklik urun mantigini veya canonical document modelini degistirmez; ana hedef Start Center, Writer ve Sheet yuzeylerinin daha tutarli, daha ayirt edilebilir ve daha verimli hale gelmesidir.

## Baslangic Merkezi

Normal uygulama acilisi artik Writer'a dogrudan dusmez. Native launch hedefi varsayilan olarak `home` olur ve SolidJS composition root modern Start Center'i render eder.

Start Center:

- Writer ve Sheet icin gercek uygulama gecis kartlari sunar.
- Sunum ve PDF icin urun ailesi kartlarini gelecekteki modul olarak gosterir.
- Son Belgeler ve Sablonlar alanlarini urun tanitim dilinde sunar.
- `--module writer` ve `--module sheet` dogrudan baslatma kontratini korur.

## Uygulama kimligi

Writer ve Sheet ayni Turkuaz Office ailesinde kalir fakat ayni ikonu kullanmaz.

- Writer: mavi belge/page simgesi.
- Sheet: yesil spreadsheet/grid simgesi.
- Sunum: turuncu sunum simgesi.
- PDF: kirmizi belge simgesi.
- Start Center ve suite markasi: turkuaz ortak marka simgesi.

Ikonlar dependency-free inline SVG View olarak tutulur. Uygulama fonksiyonlari ikon dosyalarina baglanmaz.

## Writer

Writer ust menu sirasi klasik kelime islemci modeline cekilir:

`Dosya -> Duzenle -> Gorunum -> Ekle -> Bicim -> Tablo -> Araclar -> Yardim`

Mevcut calisan command toolbar korunur. Belge alani uc parcali hale gelir:

`Sayfalar -> Belge Canvas -> Ozellikler`

Sol panel aktif sayfa onizlemesini, sag panel font/paragraf ve belge bilgilerini sunar. Sag paneldeki B/I/U ve paragraph alignment butonlari mevcut WriterController komutlarini kullanir; View canonical state mutasyonu yapmaz.

## Sheet

Sheet menu sirasi spreadsheet kullanim modeline gore duzenlenir:

`Dosya -> Giris -> Ekle -> Bicim -> Veri -> Formuller -> Gorunum -> Yardim`

Mevcut cell/formula/table/query/chart komutlari korunur. Writer mavisinden ayri olarak Sheet icin yesil accent uygulanir.

## Mimari guvence

Bu dilimde:

- canonical Writer/Sheet modele yeni alan eklenmez;
- file format/schema degismez;
- Controller ve Service icine UI styling mantigi eklenmez;
- View katmani hesaplama/formula semantigini tekrar etmez;
- mevcut direct module launch ve installer shortcut davranislari korunur;
- UI metinleri typed Language kontratinda tutulur.

## Sonraki dilim

v0.12.0 sonrasinda Start Center Son Belgeler alaninin gercek recent-file read-modeli ile beslenmesi, Start Center sablon kartlarinin dogrudan template command'ina baglanmasi ve Sheet native Open/Save yetenegi ayri urun dilimleri olarak ele alinmalidir.
