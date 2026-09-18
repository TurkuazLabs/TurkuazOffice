# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-autosave-recovery-v0.2.0.md
# 📌 Amac: M1 Writer autosave ve recovery implementation kapsamini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Rust recovery Service, Tauri IPC, frontend startup karar paneli ve retention akisini ozetler

Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View -> Language

# M1 Autosave Recovery v0.2.0

## Eklenenler

- WriterRecoveryService.
- Platform recovery root Tool.
- Recovery YAML metadata Tool.
- TKO tabanli recovery snapshot.
- Per-document retention.
- Recovery list/restore/compare/discard/clear IPC commandlari.
- Frontend recovery Repository state.
- Startup recovery scan.
- Recover / Compare / Discard paneli.
- 30 saniyelik autosave recovery scheduler.
- Recovery revision deduplication.
- Explicit Save sonrasi recovery cleanup.
- New/Open discard sonrasi current document recovery cleanup requesti.

## Bilincli sinir

Autosave DOM'u zorla flush etmez. Canonical Rust document snapshot'ini alir. Normal input debounce 180 ms oldugu icin IME composition ortasinda DOM'u bozacak zorunlu commit yapilmaz.

Session restore acik tab listesi ve external-change protection siradaki M1 fazlarinda genisletilecektir.
