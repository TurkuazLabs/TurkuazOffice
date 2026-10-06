# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-basic-charts-ui-test-matrix.md
# 📌 Amac: Sheet Basic Charts Desktop UI v0.11.0 kalite kontrol matrisini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.11.0
# Aciklama: IPC, selection validation, create/select/remove, async projection, SVG render ve localization kontrollerini listeler
# Bagimli Oldugu Katman: Tool -> Service -> Controller -> Repo -> View -> Language

# Sheet Basic Charts UI Test Matrix

| Alan | Kontrol | Beklenen |
| --- | --- | --- |
| DTO | SheetDocumentDto chart listesi | Canonical chart metadata kayipsiz camelCase |
| IPC | create/remove/get chart data | Controller yalniz Desktop Service delegasyonu |
| Service | Tam iki kolon selection | Ilk kolon kategori, ikinci kolon deger |
| Service | 1 veya 3+ kolon selection | Backend cagrisi yok, invalid chart range |
| Service | Bos title | Backend cagrisi yok, invalid chart title |
| Create | Basarili chart | Document guncellenir, dirty=true |
| Select | Chart data request | Repo selectedChartId + projected points |
| Select async | Yeni chart secimi IPC tamamlanmadan once | selectedChartId hemen yeni chart olur, eski preview kalmaz |
| Async | Eski chart data response | Yeni chart secimini ezmez |
| Create async | Create beklerken daha yeni chart secimi | Create sonucu daha yeni secimi veya preview'i ezmez |
| Remove | Secili chart | Document guncellenir, chart data temizlenir |
| Mutation | Aktif chart varken cell edit | Projected chart data refresh |
| Menu | Ekle/Gorunum | Charts panel toggle gercek komut |
| Dock | Properties/Functions/Charts | Ayni anda tek sag panel |
| Renderer | Bar | Sifir baseline + finite rectangles |
| Renderer | Line | Polyline + finite points |
| Renderer | Pie | Pozitif toplam icin deterministic slices |
| Renderer extreme | +/-1e308 Bar/Line | Tum SVG koordinatlari finite kalir |
| Pie extreme | Birden fazla 1e308 | Scaled toplam ile gorunur finite dilimler |
| Empty | Chart data yok | Lokalize empty state |
| Localization | tr-TR/en-US | Tum chart panel metinleri Language katmaninda |
| Static contract | verify-project.sh | v0.11.0 dosya ve davranis kontrati |

## Manuel Smoke

1. Sheet'te A1:B3 araligina kategori ve sayisal deger girin.
2. A1:B3 araligini rectangular selection olarak secin.
3. Ekle > Grafikler panelini acin.
4. Bar secip baslik girin ve grafik olusturun.
5. Bar preview'in canonical veriyi gosterdigini kontrol edin.
6. Ayni chart icin kaynak degerlerden birini degistirin; preview'in guncellendigini kontrol edin.
7. Line ve Pie tipleriyle yeni grafikler olusturun.
8. Chart listesinden grafikler arasinda gecis yapin.
9. Secili grafigi kaldirin; listeden ve preview'den temizlendigini kontrol edin.
10. Tek kolon veya uc kolonluk selection ile create aksiyonunun devre disi kaldigini kontrol edin.
11. TR/EN dil degisiminde panel metinlerini kontrol edin.
