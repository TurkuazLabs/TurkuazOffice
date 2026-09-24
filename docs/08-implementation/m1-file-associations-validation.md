# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-file-associations-validation.md
# 📌 Amac: M1 File Associations static, regression ve compiler-backed validation durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Tauri bundle config, startup path katmanlari, frontend cold-start zinciri ve hosted runner sinirini ayri raporlar

Bagimli Oldugu Katman: Documentation

# M1 File Associations Validation

## Static contract

- bundle.fileAssociations ana Tauri config'tedir.
- .tko extension ve MIME central bundle config'tedir.
- Windows target NSIS platform config'tedir.
- Linux targets AppImage + DEB platform config'tedir.
- Process argv okuma Tool katmanindadir.
- Native startup path secimi Service katmanindadir.
- Tauri Controller yalniz take_startup_file requestini aktarir.
- Frontend Tauri Tool typed string/null IPC kullanir.
- WriterSessionService association acilisinda normal native Open pipeline'ini yeniden kullanir.

## Regression

startup_file_service_tests.rs first valid path, canonicalization, filtering ve one-shot davranisini test eder.

## Fidelity

Association path mevcut WriterStorageService tarafindan acildigi icin TKO validation, file lock ve external-change contract bypass edilmez.

## Known limit

Running-instance second-open handoff yoktur. M1 minimum cold-start odaklidir.

## Compiler-backed durum

GitHub hosted runner gercek step baslatmadan steps=null failure verirse File Associations compile/test sonucu onaylanmis sayilmaz. Basarili iddia icin gercek cargo/frontend step logu gerekir.
