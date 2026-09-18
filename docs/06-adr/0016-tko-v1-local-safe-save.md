# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0016-tko-v1-local-safe-save.md
# 📌 Amac: TKO v1 byte package ve local safe-save mimari kararini kalici olarak kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: ZIP+YAML format DTO, allowlist, platform safe-replace ve dirty-guard kararlarini ADR olarak sabitler

Bagimli Oldugu Katman: Documentation

# ADR 0016 - TKO v1 Local Safe Save

## Durum

Accepted.

## Baglam

Writer domain ve TKO logical profile v1 hazirdi ancak disk byte encoding, native file dialog ve failure-safe local save henuz yoktu. Domain struct'larini serialization kutuphanesine baglamak ileride format migration ve Web/Mobile paylasimini zorlastirirdi.

## Karar

TKO v1 byte package asagidaki profile gore yazilir:

```text
sample.tko
|-- manifest.yml
`-- content/
    `-- writer.yml
```

- Container ZIP'tir.
- v1 yalniz `Stored` compression kabul eder.
- Manifest ve Writer content YAML'dir.
- Serde yalniz format DTO katmaninda kullanilir; canonical Writer domain serde derive tasimaz.
- `manifest.yml` ile `content/writer.yml` disindaki file entry v1'de reddedilir.
- Directory entry, path traversal, duplicate entry, fazla entry ve resource limit asimi reddedilir.
- Save yalniz current schema yazar; eski schema migration gerektirir ve future schema acik hata ile reddedilir.
- Manifest/content document id, revision ve schema degerleri birebir eslesmelidir.

## Local save

Desktop Service path ve TKO kurallarini yonetir. Dosya IO Tool katmaninda kalir.

- Unix ailesinde ayni klasorde temp write + sync + rename kullanilir.
- Windows'ta mevcut hedef ayni klasorde benzersiz backup'a tasinir, temp hedefe tasinir ve hata durumunda rollback denenir.
- Temp/backup sidecar adlari process id + zaman token'i ile benzersizdir.
- Hedef belgeye dogrudan truncate/write yapilmaz.
- Save yolu uzantisizsa `.tko` eklenir; relative yol current directory icinde cozulur.
- Open yalniz `.tko` kabul eder.

## Dirty guard

New/Open mevcut document revision'i son save/open baseline'indan farkliysa kullanicidan discard onayi alir. Pending DOM paragraph once Rust Core'a flush edilir; file operation mutation queue tamamlanmadan baslamaz.

## Capability

Desktop Tauri capability yalniz gerekli dialog izinlerini acar:

- `dialog:allow-open`
- `dialog:allow-save`
- `dialog:allow-message`

## Sonuclar

- Domain format kutuphanesinden bagimsiz kalir.
- Desktop local-first belge akisi gercek disk round-trip seviyesine gelir.
- TKO format version 1 kod seviyesinde tanimlidir ancak public long-term compatibility garantisi M1 stable cikisina kadar provisional kalir.
- Asset, metadata ve recovery entry'leri ileride yeni profile/ADR ile allowlist'e eklenir.
