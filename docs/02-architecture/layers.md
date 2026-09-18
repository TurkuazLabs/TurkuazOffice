# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/layers.md
# 📌 Amac: Controller Service Repo Tool View Language katmanlarinin kesin sorumluluklarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Controller Service Repo Tool View Language katmanlarinin kesin sorumluluklarini tanimlar

Bagimli Oldugu Katman: Documentation

# Katman Kurallari

## Controller

Request alir, inputu servis kontratina tasir, Service cagirir ve View DTO dondurur. Is kurali, dosya islemi veya format parsing yapamaz.

## Service

Tum is kurali burada bulunur. Document operation, validation, command ve orchestration bu katmandadir.

## Repository

Kalici veya gecici storage islemlerini soyutlar. Service storage teknolojisini bilmez.

## Tool

Dis dunya adaptoru. File picker, native clipboard, PDF engine, format parser, network veya platform API gibi detaylar Tool kontrati arkasinda tutulur.

## View

UI'ya tasinacak cikti modelidir. UI framework tipi Core'a sizamaz.

## Language

Kullaniciya gorunen label key ve localization kontratlarinin merkezidir.

## Yasaklar

- Controller icinde business rule.
- Service icinde Tauri, browser veya Android SDK detayi.
- View icinde repository erisimi.
- Hard-coded magic string.
- Inline environment configuration.
