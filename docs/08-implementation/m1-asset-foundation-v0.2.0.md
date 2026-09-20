# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-asset-foundation-v0.2.0.md
# 📌 Amac: M1 image clipboard oncesi canonical Writer asset ve TKO binary package temelini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: WriterAsset registry, InsertImageData ve assets/index.yml guvenlik kontratini belgeler

Bagimli Oldugu Katman: Documentation

# M1 Asset Foundation

## Canonical model

WriterDocument binary asset bytes'larini `WriterAsset { id, media_type, bytes }` registry icinde tasir. ImageBlock binary data tasimaz; yalniz `asset_id` referansi kullanir.

## Atomic image command

`InsertImageData` tek Writer command icinde payload'i validate eder, collision-safe asset id uretir, asset registry'ye ekler ve ImageBlock'u hedef paragraph sonrasina ekler. Invalid signature veya limit hatasinda document mutate edilmez.

## Media allowlist

M1 image asset allowlist:

- image/png
- image/jpeg
- image/webp

MIME tek basina guvenilmez; PNG/JPEG/WebP magic signature da kontrol edilir. Tek asset 8 MiB, document basina 16 asset ve toplam TKO paket 16 MiB limiti korunur.

## TKO package

Assetsiz eski TKO v1 iki entry ile gecerlidir. Assetli paket optional olarak:

- assets/index.yml
- assets/data/<asset-id>.bin

tasir. Index id, MIME, canonical entry path ve declared byte length saklar. Deserialize sirasinda duplicate id/path, missing entry, byte-length mismatch, invalid MIME/signature, orphan asset ve missing ImageBlock reference reddedilir.

## Sonraki adim

Desktop clipboard image DataTransfer -> byte payload -> Tauri IPC -> InsertImageData baglantisi ve Writer View image rendering.
