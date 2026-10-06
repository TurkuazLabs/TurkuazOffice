# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-functions-sidebar-test-matrix.md
# 📌 Amac: Sheet Functions sidebar v0.10.0 icin otomatik ve manuel kalite kontrol matrisini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.10.0
# Aciklama: Function katalogu, draft mapping, UI erisilebilirligi, localization ve mevcut formula engine entegrasyonunu dogrulayan kontrolleri listeler
# Bagimli Oldugu Katman: Tool -> Service -> Controller -> View -> Language

# Sheet Functions Sidebar Test Matrix

| Alan | Kontrol | Beklenen |
| --- | --- | --- |
| Catalog | SUM, AVERAGE, MIN, MAX, IF katalogda | Bes engine fonksiyonu tek Config kaynaginda |
| Tool | Her function kimligi draft'a map edilir | Config'teki draft aynen doner |
| Service | Secim yokken draft istenmesi | null, canonical mutasyon yok |
| Controller | Function draft istegi | Sadece Service delegasyonu |
| Formula bar | fx dugmesi | Functions panelini acar/kapatir |
| Menu | Ekle/Gorunum | Functions panel toggle gercek komut |
| Dock | Properties ve Functions | Ayni anda tek sag panel |
| Insert | Formule Ekle | Draft formula bar'a gelir, otomatik commit olmaz |
| Focus | Draft ekleme | Formula input focus olur, caret sona gider |
| Async race | Eski blur commit + yeni function draft | Eski commit tamamlaninca yeni draft temizlenmez |
| Search | ID/ad/imza/aciklama | Case-insensitive filtre |
| Localization | tr-TR/en-US | Tum gorunen panel metinleri Language katmanindan |
| Accessibility | Dugme ve sidebar | aria-label ve aria-pressed durumu mevcut |
| Regression | Formula helper unit testi | Tum katalog girdileri beklenen draft'i verir |
| Static contract | verify-project.sh | v0.10.0 dosya ve davranis kontrati dogrulanir |

## Manuel Smoke

1. Sheet ac ve bir hucre sec.
2. Formula bar'daki fx dugmesine bas; Functions panelinin Properties yerine acildigini kontrol et.
3. SUM icin Formule Ekle'ye bas; formula bar'da =SUM( taslagini ve input focusunu kontrol et.
4. Bir aralik girip formulu tamamla; mevcut engine sonucunun hesaplandigini kontrol et.
5. IF icin ayni akisi tekrarla.
6. Arama kutusunda AVG, minimum aciklamasi ve IF kimligi ile filtre davranisini kontrol et.
7. Dil secimini tr-TR ve en-US arasinda degistir; panel metinlerinin degistigini kontrol et.
8. Formula bar'da commit edilmemis bir deger varken Functions panelinden yeni draft ekle; onceki blur commit'i tamamlansa bile yeni draft'in korunmasini kontrol et.
9. Secim yokken fx ve insert akisinin canonical belgeyi degistirmedigini kontrol et.
