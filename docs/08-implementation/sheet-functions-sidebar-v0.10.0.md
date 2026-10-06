# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/sheet-functions-sidebar-v0.10.0.md
# 📌 Amac: Sheet Functions sidebar v0.10.0 uygulama kontratini ve katman sinirlarini belgeler
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.10.0
# Aciklama: Destekli formula fonksiyonlarini aranabilir sag panelde sunma ve formul cubuguna guvenli taslak ekleme davranisini tanimlar
# Bagimli Oldugu Katman: Controller -> Service -> Tool -> View -> Language

# Sheet Functions Sidebar v0.10.0

## Kapsam

Bu dilim mevcut Sheet Formula Engine ve Sheet Function Library hesap motorunu degistirmez. UI, halihazirda desteklenen bes fonksiyonu kesfedilebilir hale getirir:

- SUM
- AVERAGE
- MIN
- MAX
- IF

## Akis

Function katalogu merkezi Config katmaninda tutulur. View kullanicinin secimini Controller'a iletir. Controller yalnizca Service'i cagirir. Service aktif hucre secimini dogrular ve Tool uzerinden formula taslagini alir.

View -> Controller -> Service -> Tool -> Config

View, donen taslagi formula bar'a yerlestirir ve input'a focus verir. Taslak otomatik commit edilmez. Kullanici argumanlari tamamlayip mevcut formula bar Enter/blur akisi ile canonical Sheet komutunu calistirir.

## UI

Functions paneli mevcut sag dock alanini Properties paneliyle paylasir. Iki panel ayni anda acik tutulmaz.

Panel su yuzeylerden acilabilir:

- formula bar icindeki gercek fx dugmesi
- Ekle > Fonksiyonlar paneli
- Gorunum > Fonksiyonlar paneli

Panelde lokalize fonksiyon adi, engine kimligi, imza, kisa aciklama ve Formule Ekle aksiyonu bulunur. Arama; fonksiyon kimligi, imza, lokalize ad ve aciklamayi kapsar.

## Guvenlik ve Tutarlilik

- Katalog disi function kimligi bos taslak uretir.
- Secili hucre yoksa Service taslak dondurmez.
- UI taslagi otomatik commit etmez.
- Formula edit generation guard, daha eski blur commit tamamlanmasinin daha yeni function draft state'ini temizlemesini engeller.
- Hesaplama semantigi UI katmaninda tekrar edilmez.
- Function isimleri ve taslaklari magic string olarak View icine dagitilmaz.
- TR/EN kullanici metinleri Language katmaninda tutulur.

## Sonraki Kapsam

Daha genis Excel/Calc function gruplari ayrica function-library gelistirmesi olarak ele alinacak. Bu sidebar yeni engine fonksiyonlarini katalog uzerinden gosterecek sekilde genisletilebilir.
