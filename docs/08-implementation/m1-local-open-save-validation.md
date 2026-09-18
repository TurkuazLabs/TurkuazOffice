# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-local-open-save-validation.md
# 📌 Amac: M1 Local Open/Save artifact kalite ve build validation sonucunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Static contract, TypeScript, config, archive test source ve runtime build bariyerlerini aciklar

Bagimli Oldugu Katman: Documentation

# M1 Local Open Save Validation

## Artifact ortaminda basarili kontroller

- `./tools/verify-project.sh`: BASARILI.
- `bash -n tools/verify-project.sh`: BASARILI.
- YAML/TOML/package JSON parse: BASARILI.
- Teknik body ASCII standardi: BASARILI.
- TypeScript/TSX strict typecheck: BASARILI; external dependency'ler gecici declaration stub ile temsil edildi.
- Rust source delimiter/module/static contract kontrolu: BASARILI.
- TKO test source matrisi: round-trip, current-schema save guard, future/old schema, missing manifest/content, mismatch, unexpected/duplicate entry, directory ve traversal fixture'lari mevcut.
- Desktop service test source matrisi: local save/open round-trip, extension normalization ve stabil invalid-extension error code fixture'lari mevcut.

## Runtime build bariyeri

Bu calisma ortaminda Rust toolchain yoktur. Frontend package registry de dependency install icin erisilebilir degildir. Bu nedenle asagidaki kontroller artifact uretim ortaminda gercek dependency graph ile calistirilamamistir:

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets -- -D clippy::all`
- `cargo test --workspace`
- `npm install`
- `npm run build`
- `npm run tauri:build`

Bunlar GitHub CI ve yerel Windows/Linux gelistirme makinesinde zorunlu bariyer olarak kalir. Rust integration test source'larinin mevcut olmasi burada gercek test binary'sinin calistigi anlamina gelmez. Registry erisimi olmadigi icin `Cargo.lock` ve `package-lock.json` elle uretilmez; ilk baglantili build ortaminda gercek resolver ile uretilip commit edilmelidir.

## Runtime ortaminda ozellikle dogrulanacak senaryolar

- Windows safe-replace rollback.
- Linux rename replace.
- Tauri native open/save/confirm dialog capability.
- Gercek dependency graph ile TKO ZIP/YAML round-trip testleri.
- IME input sonrasi Ctrl+S flush.
- Dirty document Open/New confirmation.
- Native file dialog cancel state stability.

## Paketleme bariyeri

Final ZIP olusturulduktan sonra archive CRC/integrity ve SHA-256 dis artifact kontrolu ayrica calistirilir.
