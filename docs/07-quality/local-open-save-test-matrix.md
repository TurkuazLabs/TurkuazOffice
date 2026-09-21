# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/local-open-save-test-matrix.md
# 📌 Amac: Local Open/Save ve TKO v1 serializer icin regression ve guvenlik test matrisini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Round-trip, schema, archive guvenligi, safe-save, dirty guard ve Desktop IPC kalite bariyerlerini listeler

Bagimli Oldugu Katman: Documentation

# Local Open Save Test Matrix

## Writer TKO package

- Binary PNG/JPEG/WebP asset index + data entry round-trip korunur.
- Missing asset binary entry reddedilir.
- Rich-text, font family, font size ve paragraph alignment round-trip korunur.
- Manifest ve content document id/revision/schema eslesir.
- Current-schema disindaki in-memory belge save sirasinda reddedilir.
- Future schema package open sirasinda reddedilir.
- Old schema migration-required hatasi verir.
- Missing manifest/content reddedilir.
- Manifest/content mismatch reddedilir.
- Beklenmeyen file entry reddedilir.
- Directory entry reddedilir.
- Path traversal reddedilir.
- Duplicate entry reddedilir.
- Unsupported compression reddedilir.
- Package, manifest, content ve uncompressed byte limitleri uygulanir.

## Desktop storage

- Image asset save -> yeni Desktop Service -> open -> lazy get_asset zincirinde byte ve MIME korunur.
- Uzantisiz save yolu `.tko` ile normalize edilir.
- Save edilen belge yeni Desktop Service tarafindan tekrar acilir.
- Non-TKO open reddedilir.
- Missing/invalid path stabil error code verir.
- Write failure mevcut hedefi dogrudan truncate etmez.
- Windows replace failure durumunda backup rollback denenir.

## Frontend session

- Ctrl/Cmd+S mevcut path varsa dialog acmadan kaydeder.
- Ilk Save native save dialog acilir.
- Ctrl/Cmd+O native open dialog acilir.
- Ctrl/Cmd+N ve Open dirty document icin discard onayi ister.
- Dialog cancel belge state'ini degistirmez.
- File operation oncesinde focused paragraph flush edilir.
- Save sonrasi dirty baseline yeni revision'a tasinir.
- Open sonrasi file path ve saved revision baseline guncellenir.

## CI bariyeri

- `./tools/verify-project.sh`
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D clippy::all`
- `cargo test --workspace`
- `npm run build`
- Windows ve Linux workspace test matrix.
