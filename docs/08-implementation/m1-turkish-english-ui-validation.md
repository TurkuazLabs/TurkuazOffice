# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-turkish-english-ui-validation.md
# 📌 Amac: M1 Turkish + English UI static, frontend regression ve compiler-backed validation durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Typed label coverage, reactive locale, persistence boundary ve hosted runner durumunu ayri raporlar

Bagimli Oldugu Katman: Documentation

# M1 Turkish + English UI Validation

## Static contract

- Supported locale list config katmanindadir.
- Turkish ve English packs Language katmanindadir.
- Her iki pack DesktopLabelKey Record kontratini implement eder.
- LanguageService reactive locale signal tasir.
- Preference storage Tool katmanindadir.
- Locale validation/persistence Service katmanindadir.
- Controller yalniz locale requestlerini aktarir.
- Statusbar View localStorage'a erismez.

## Regression

language-service.test.ts:

- runtime tr-TR -> en-US switch.
- label switch.
- locale option coverage.

language-preference.service.test.ts:

- persisted en-US load.
- tr-TR persistence.
- invalid locale rejection.

## Data isolation

Locale degisimi canonical WriterDocument veya TKO package verisini mutate etmez.

UI locale persistence browser local preference'tir.

## Compiler-backed durum

GitHub hosted runner gercek step baslatmadan steps=null failure verirse Turkish + English UI build/test sonucu onaylanmis sayilmaz. Basarili iddia icin gercek frontend-quality step logu gerekir.
