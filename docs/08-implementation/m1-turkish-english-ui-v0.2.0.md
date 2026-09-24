# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-turkish-english-ui-v0.2.0.md
# 📌 Amac: M1 Desktop Turkish + English UI implementation kapsam ve mimari sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Typed locale config, reactive LanguageService, persistence Tool/Service ve statusbar selector akisini tanimlar

Bagimli Oldugu Katman: Documentation

# M1 Turkish + English UI

## Mimari

Locale config:

config/localization.ts

Language packs:

language/tr.ts
language/en.ts

Pack registry:

language/language-packs.ts

Runtime:

LanguageService -> Solid locale signal.

Preference:

Writer View -> WriterController -> LanguagePreferenceService -> LanguagePreferenceTool -> local storage.

## Locale list

- tr-TR
- en-US

Default locale tr-TR'dir.

Locale listesi, default locale ve storage key merkezi config'tedir.

## Typed coverage

DesktopLabelKey tek typed label yuzeyidir.

Turkish ve English pack bu key setinin tamamini implement eder.

Template katalog name/description key'leri de ayni LanguageService uzerinden cozulur.

## Runtime behavior

Locale degistirildiginde LanguageService signal'i degisir. Mevcut JSX text() cagirilari reactive oldugu icin UI aninda yeniden render olur.

Writer canonical document veya native file session bu degisimden etkilenmez.

## Persistence

LanguagePreferenceTool localStorage teknik adaptorudur.

LanguagePreferenceService:

- startup preference load.
- supported locale validation.
- LanguageService update.
- preference save.

Storage error kullanicinin Writer ile calismasini engellemez.

## Selector

Locale selector statusbar'dadir.

View yalniz Controller API'sini kullanir:

- locale()
- localeOptions()
- setLocale()

## M1 siniri

- UI locale document locale degildir.
- System locale auto-detection yok.
- RTL zorunlu degildir.
- Per-document spellcheck locale yok.
- Date/number formatting engine bu fazin disindadir.
