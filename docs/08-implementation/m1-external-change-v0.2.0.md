# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-external-change-v0.2.0.md
# 📌 Amac: M1 Writer external-change ve file-lock implementation kapsamini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Rust file session, fingerprint/lock Tool, IPC, frontend poll ve protection banner akisini ozetler

Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View -> Language

# M1 External Change Protection v0.2.0

## Eklenenler

- `FileFingerprintTool`.
- `FileLockTool` cooperative sidecar lease.
- `WriterFileSessionService`.
- Backend read-only mutation guard.
- Save-before-overwrite fingerprint recheck.
- External state IPC DTO ve commandlari.
- Frontend file-session Repository state.
- 2 saniyelik external-change polling.
- Read-only contenteditable davranisi.
- File protection banner.
- Reload From Disk.
- Keep Local Version.
- Save As ribbon commandi ve Ctrl/Cmd+Shift+S.
- Statusbar lock/read-only bilgisi.

## Silent overwrite yasa kurali

Polling yalniz UX icindir. Asil guvenlik Save aninda backend tarafinda yeniden fingerprint kontroludur. Bu nedenle poll gecikse veya UI state eski olsa bile modified/missing dosya explicit kullanici karari olmadan overwrite edilmez.

## Read-only davranisi

Lock alinamayan belge okunabilir. Canonical document Rust tarafinda yuklenir ancak tum mutation commandlari backend Service guard'dan gecer.

Save As read-only belgeden yeni dosya olusturmak icin izinlidir.

## Bilinen sinir

Cooperative sidecar lock hard-crash sonrasi stale kalabilir. M1 bu durumda otomatik lock kirmaz; veri kaybi yerine read-only fallback tercih edilir.
