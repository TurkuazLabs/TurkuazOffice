# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-recent-files-validation.md
# 📌 Amac: M1 Recent Files static, regression ve compiler-backed validation durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Katman kontrati, path normalization, persistence ve hosted runner sinirini ayri raporlar

Bagimli Oldugu Katman: Documentation

# M1 Recent Files Validation

## Static contract

- Platform app-state root Tool katmanindadir.
- Recent YAML serialization Tool katmanindadir.
- Metadata disk read/write Repo katmanindadir.
- Dedup/retention/prune Service katmanindadir.
- Tauri Controller yalniz Service cagirir.
- Frontend Controller yalniz WriterSessionService cagirir.
- Ribbon View persistent metadata dogrudan okumaz.
- Recent file list canonical Writer document state'i degildir.

## Regression

recent_files_service_tests.rs:

- 12-entry retention.
- latest-first ordering.
- canonical duplicate dedup.
- duplicate'i basa tasima.
- missing-file prune.

writer_recent_files_integration_tests.rs:

- native save sonrasi explicit recent record.
- active native file session isolation.

## UX contract

- Startup list load.
- Open/Save record.
- normal unsaved-changes guard ile recent quick-open.
- recent metadata failure ana Open/Save sonucunu bozmaz.

## Compiler-backed durum

GitHub hosted runner gercek step baslatmadan steps=null failure verirse Recent Files compile/test sonucu onaylanmis sayilmaz. Basarili iddia icin gercek cargo/frontend step logu gerekir.
