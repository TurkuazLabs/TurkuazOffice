# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/turkish-english-ui-test-matrix.md
# 📌 Amac: M1 Desktop Turkish + English UI locale, label coverage ve preference persistence test kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Typed language pack, runtime switch, local preference ve UI/document locale ayrimi bariyerlerini sabitler

Bagimli Oldugu Katman: Documentation

# Turkish + English UI Test Matrix

## Supported locales

- tr-TR
- en-US

Default UI locale:

- tr-TR

## Typed label coverage

DesktopLabelKey tum kullaniciya gorunen Writer label kontratidir.

TR_LABELS ve EN_LABELS Readonly<Record<DesktopLabelKey, string>> kullanir. Bir key eksikse TypeScript build basarisiz olmalidir.

View katmaninda locale-specific kullanici metni tutulmaz.

## Runtime switch

LanguageService locale state'i Solid signal ile tutulur.

Locale degisimi:

- application reload gerektirmez.
- mevcut Writer document state'ini degistirmez.
- native file session'i degistirmez.
- recovery/recent/template state'ini degistirmez.

## Preference persistence

LanguagePreferenceTool browser local storage adaptorudur.

Storage key config/localization.ts icindedir.

Invalid/missing preference default tr-TR locale'e doner.

Storage read/write failure Writer kullanimini bloklamaz.

## Service boundary

View -> WriterController -> LanguagePreferenceService -> LanguagePreferenceTool + LanguageService.

View localStorage'a dogrudan erismez.

Locale validation Service katmanindadir.

## UI selector

Statusbar locale selector:

- typed locale options kullanir.
- option label'larini LanguageService ile cozer.
- tr-TR/en-US stringlerini View icinde filtrelemez.

## Document locale boundary

UI locale document locale degildir.

Bu faz canonical WriterDocument icine locale metadata eklemez.

Spellcheck/date/number document locale daha sonraki ayri ozelliktir.

## Regression

language-service.test.ts:

- Turkish default/switch.
- English label switch.
- supported locale options.

language-preference.service.test.ts:

- persisted English load.
- Turkish save.
- invalid locale rejection.
