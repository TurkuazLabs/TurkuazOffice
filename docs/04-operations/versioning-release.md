# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/04-operations/versioning-release.md
# 📌 Amac: SemVer ve platform release akisini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: SemVer ve platform release akisini tanimlar

Bagimli Oldugu Katman: Documentation

# Versioning ve Release

## Surum modeli

Platform umbrella version `Turkuaz Office x.y.z` olur. Alt bilesenler kendi teknik surumunu tasiyabilir ancak public release matrisi tek yerde tutulur.

## Degisim tipi

Patch = bug.

Minor = modul veya geriye uyumlu yeni yetenek.

Major = mimari/public contract kirilmasi.

## Release gates

- format check,
- clippy,
- unit/integration test,
- docs check,
- supported platform smoke test,
- artifact checksum,
- changelog.

## Branch

Foundation icin trunk/main modeli yeterlidir. Release tag imzalama daha sonra operasyon milestone'unda acilir.
