# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/sheet-freeze-panes-v0.5.2.md
# 📌 Amac: Turkuaz Sheet satir/sutun dondurma davranisini ve oturum-state mimarisini dokumante eder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.5.2
# Aciklama: Excel/Calc benzeri freeze panes komutlarini Controller-Service-Repo-View zincirinde tanimlar
# Bagimli Oldugu Katman: Controller | Service | Repo | View | Language | Config

# Sheet Freeze Panes v0.5.2

## Kapsam

Turkuaz Sheet masaustu gorunumunde dort gercek komut vardir:

- Secime Gore Dondur
- Ust Satiri Dondur
- Ilk Sutunu Dondur
- Bolmeleri Coz

Komutlar Gorunum menusunde sunulur.

## Secime gore dondurma

Aktif hucre anchor kabul edilir.

Ornek: C3 aktifken Secime Gore Dondur:

- C3'un ustundeki 2 gorunen satir dondurulur.
- C3'un solundaki 2 sutun dondurulur.
- Aktif hucrenin kendisi scroll edilen alanda kalir.

Filter/sort aktifse satir freeze sayisi canonical row numarasindan degil, mevcut gorunen row projection'indaki pozisyondan hesaplanir.

## State siniri

Freeze bilgisi belge canonical modelinin parcasi degildir. Bu dilimde session View state'tir:

View -> Controller -> Service -> Repo

Belge revision'i freeze/unfreeze nedeniyle degismez.

## Grid geometrisi

Sticky offsetler magic number olarak View'e yazilmaz. Merkezi Sheet config sabitleri kullanilir:

- row height: 24 px
- column width: 88 px
- row header width: 36 px
- column header height: 24 px

Frozen bolge ile scroll edilen bolge arasinda accent sinir cizgisi gosterilir.

## Sonraki adim

Freeze state daha sonra kullanici tercihi olarak persist edilecekse belge payload'ina degil desktop preference/session metadata katmanina eklenmelidir.
