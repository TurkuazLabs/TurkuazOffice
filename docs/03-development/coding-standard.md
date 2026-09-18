# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/03-development/coding-standard.md
# 📌 Amac: TurkuazLabs kod ve katman standardini proje icin uygulanabilir kurallara cevirir
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: TurkuazLabs kod ve katman standardini proje icin uygulanabilir kurallara cevirir

Bagimli Oldugu Katman: Documentation

# Kod Standardi

## Zorunlu header

Her teknik dosya; tam dosya yolu, amac, modul/file type, version, aciklama ve bagimli katmani belirtmelidir.

## ASCII Turkce

Kod identifier, yorum ve dokuman teknik metinleri ASCII Turkce ile yazilir. Dis standartlarda zorunlu isimler degistirilmez.

## Layered architecture

Controller -> Service -> Repo/Model -> Tool -> View -> Language prensibi korunur.

## Magic string

Tekrar eden veya davranis belirleyen sabitler `config` veya typed constant uzerinden gelir.

## Inline config

Port, endpoint, autosave interval, feature switch gibi config degerleri kod icine gomulmez.

## Versioning

- Patch: bug fix.
- Minor: modul veya geriye uyumlu belirgin yetenek.
- Major: mimari/public contract kirilmasi.

## Review

Yeni kod icin test, docs etkisi ve katman ihlali kontrol edilir.
