# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/localization.md
# 📌 Amac: Turkce ve Ingilizce baslangic destegi ile i18n ve locale sinirlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: UI labels ile belge locale verisini ayiran yerellestirme kontratini tanimlar

Bagimli Oldugu Katman: Documentation

# Localization

## Baslangic dilleri

- tr-TR
- en-US

UI language ile document locale ayni kavram degildir. UI dili kullanici tercihidir. Document locale spellcheck, number/date format ve language metadata icin belgeye ait olabilir.

## Magic string yasagi

Kullaniciya gorunen metin View icinde literal tutulmaz. Language key uzerinden cozulur.

## Locale sensitive alanlar

- Date/time.
- Decimal separator.
- Thousands separator.
- Currency.
- Measurement unit display.
- Spellcheck language.
- Sorting/collation ihtiyaci.

Canonical numeric value locale-formatted string olarak saklanmaz.

## Gelecek

RTL layout M1 zorunlulugu degildir; ancak UI component contract RTL'yi imkansiz hale getirecek sabit left/right varsayimlarina baglanmaz.


## M1 runtime implementation

Desktop supported locale listesi merkezi config'tedir:

- tr-TR
- en-US

Default UI locale tr-TR'dir.

LanguageService secili locale'i reactive Solid signal ile tasir. Turkish ve English pack'ler ayni DesktopLabelKey typed kontratini uygular.

Locale preference technical storage erisimi LanguagePreferenceTool icindedir. Validation, startup load ve save davranisi LanguagePreferenceService tarafindan yonetilir.

View katmani localStorage'a dogrudan erismez. Locale secimi WriterController uzerinden Service'e gider.

UI locale degisimi WriterDocument, TKO package veya document locale metadata'sini mutate etmez.
