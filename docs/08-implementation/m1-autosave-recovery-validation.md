# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-autosave-recovery-validation.md
# 📌 Amac: M1 autosave/recovery artifact kalite ve build validation sonucunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Static contract, TypeScript, Rust source ve runtime build bariyerlerini aciklar

Bagimli Oldugu Katman: Documentation

# M1 Autosave Recovery Validation

## Artifact ortaminda basarili kontroller

- `./tools/verify-project.sh`: BASARILI.
- `bash -n tools/verify-project.sh`: BASARILI.
- YAML/TOML/JSON config parse: BASARILI.
- ASCII teknik body standardi: BASARILI.
- TypeScript strict typecheck external module stub ile: BASARILI.
- Rust delimiter/module/static contract kontrolu: BASARILI.
- Recovery regression test source matrisi: snapshot/list/restore/compare/discard/save-cleanup/retention/traversal fixture'lari mevcut.

## Artifact sayimi

- 191 source/artifact file.
- 69 docs file.
- 64 Rust file.
- 28 TypeScript/TSX file.

## Runtime build bariyeri

Artifact ortaminda Rust toolchain veya dependency registry erisimi yoksa `cargo check/test/clippy` ve dependency-resolved Vite/Tauri build ilk baglantili CI veya yerel Windows/Linux makinesinde zorunlu calistirilir.

## Runtime senaryolari

- Gercek process crash + restart recovery.
- Windows ve Linux recovery root permissionlari.
- Retention cleanup.
- Source dosya yokken Compare.
- Explicit Save sonrasi stale snapshot temizligi.
