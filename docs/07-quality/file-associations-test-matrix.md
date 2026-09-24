# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/file-associations-test-matrix.md
# 📌 Amac: M1 native TKO file association bundle, startup argument ve cold-start open test kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Windows NSIS, Linux DEB ve startup native-file validation bariyerlerini kalite kontrati yapar

Bagimli Oldugu Katman: Documentation

# File Associations Test Matrix

## Bundle registration

- Native extension: .tko.
- MIME: application/x-turkuaz-office.
- Windows release command: tauri build --bundles nsis.
- Linux release command: tauri build --bundles deb,appimage.
- Linux association-capable package target: DEB.
- Linux AppImage portable target file association kaydi garanti etmez.

## Startup argument rules

- argv[0] Tool katmaninda atilir.
- Flag ile baslayan argumentlar aday degildir.
- Missing path reddedilir.
- Foreign extension reddedilir.
- Existing .tko path LocalFileTool ile canonicalize edilir.
- Ilk gecerli native path secilir.
- Startup path one-shot state'tir.

## Frontend cold-start flow

- Recent Files startup load devam eder.
- Recovery snapshot varsa recovery karari association path'ten once gelir.
- Recovery yoksa writer_take_startup_file IPC cagrilir.
- Startup path varsa normal Writer Open pipeline kullanilir.
- Basarili association open Recent Files'a kaydedilir.
- Invalid/corrupt TKO normal Open error kontratini kullanir.

## Platform limit

M1 File Associations cold-start installer association profilidir.

Calisan tek instance'a ikinci Explorer/file-manager acilisini teslim eden single-instance event handoff bu fazda yoktur. Bu davranis sonraki Desktop hardening kapsamidir.

## Regression

startup_file_service_tests.rs:

- flag / missing / foreign argument rejection.
- first valid TKO selection.
- canonical path.
- one-shot take.
- WriterDesktopService startup-state composition.
