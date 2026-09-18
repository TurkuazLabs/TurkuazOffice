# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/native-document-format.md
# 📌 Amac: Turkuaz Office canonical native paket formatinin sinirlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: TKO v1 ZIP+YAML package, manifest/content, schema validation ve safe-save prensiplerini tarif eder

Bagimli Oldugu Katman: Documentation

# Native Document Format

## Karar

Turkuaz Office native paket uzantisi `.tko` olarak ayrilir. TKO format version 1 byte profile M1 icinde uygulanmistir; long-term public compatibility garantisi M1 stable cikisina kadar provisional kalir.

## Temel ilke

`.tko` canonical Document Model'in disk temsilidir. DOCX, XLSX, PPTX, ODT, ODS ve benzeri formatlar import/export adapteridir. Core modeli hicbir dis formatin XML agacina gore sekillenmez.

## Paket yapisi

Writer TKO v1 byte profile asagidaki iki file entry ile sinirlidir:

```text
sample.tko
|-- manifest.yml
`-- content/
    `-- writer.yml
```

- Container ZIP'tir.
- Compression v1 icin `Stored` olmak zorundadir.
- Manifest ve Writer content YAML'dir.
- Directory entry kabul edilmez.
- Beklenmeyen file entry kabul edilmez.
- Asset, metadata ve recovery payload henuz v1 allowlist'inde degildir.
- Canonical domain struct'lari serde derive tasimaz; disk semasi ayri DTO katmanindadir.

## Zorunlu metadata

- format_version
- schema_version
- document_kind
- document_id
- revision
- created_by_app_version
- required_capabilities

## Migration

M1 Local Open/Save baseline save sirasinda yalniz current schema kabul eder; eski veya future schema ile in-memory belge yazilmaz. Open sirasinda eski schema sessizce mutate edilmez ve `MigrationRequired` sonucu verir. Gercek migration uygulandiginda `SchemaMigrationService` tek yonlu fixture-tested zincir olarak devreye girer. Bilinmeyen future schema `FutureSchema` ile reddedilir.

## Safe save

Kaydetme hedef dosyanin ustune dogrudan yazmaz. Once ayni klasorde benzersiz temporary package olusturulur ve flush/sync tamamlanir. Unix ailesinde temp file hedefe rename edilir. Windows tarafinda mevcut hedef once benzersiz backup sidecar'a tasinir, temp hedefe tasinir ve ikinci adim basarisiz olursa rollback denenir. Bu nedenle platformlar icin tek bir yanlis "her yerde strict atomic" vaadi verilmez.

## Guvenlik

Paket icerigi guvenilmeyen input kabul edilir. Entry sayisi, compressed package boyutu, acilmis toplam boyut, manifest/content boyutu, directory entry, duplicate entry, unsupported compression ve path traversal security policy ile kontrol edilir.
