# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/CONTRIBUTING.md
# 📌 Amac: Turkuaz Office gelistirme ve pull request kurallarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Layering, docs, test, roadmap ve dependency review kurallarini contributor seviyesinde toplar

Bagimli Oldugu Katman: Documentation

# Contributing

## Zorunlu kurallar

- Controller yalniz request alir ve Service cagirir.
- Is kurali Service katmanindadir.
- DB/storage Repository katmanindadir.
- Dis dunya entegrasyonu Tool katmanindadir.
- UI output View katmanindadir.
- Kullanici metni Language katmanindan gelir.
- Magic string ve inline config eklenmez.
- Roadmap disi modul implementation acilmaz.
- Mimari degisiklik docs ve gerekiyorsa ADR ile ayni PR'da gelir.

## PR kontrolu

```powershell
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
powershell -ExecutionPolicy Bypass -File .\tools\verify-project.ps1
```

## Dependency

Yeni dependency icin teknik ihtiyac, license ve security maintenance notu PR aciklamasinda bulunur.
