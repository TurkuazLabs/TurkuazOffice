# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/README.md
# 📌 Amac: Windows, Linux ve ileride macOS istemcisinin teknoloji, katman ve calistirma sinirlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Tauri 2 + SolidJS Writer shell, rich-text, typography, ribbon ve local TKO Open/Save baseline'ini belgeler

Bagimli Oldugu Katman: Documentation

# Desktop App

Desktop istemci Tauri 2 + SolidJS + TypeScript + Vite kullanir. Canonical Writer belge modeli Rust `turkuaz-office-writer` crate'inde kalir. Frontend sadece read-only DTO alir ve typed IPC request gonderir.

## Katmanlar

Frontend:

`View -> Controller -> Service -> Repo -> Tool -> Tauri IPC`

Rust shell:

`Tauri Controller -> Desktop Service -> Writer Controller -> Writer Service -> Repo/Tool`

## Calistirma

```powershell
cd E:\Projects\TurkuazOffice\apps\desktop
npm install --no-audit --no-fund
npm run build
npm run tauri:dev
```

Rust, Node.js ve Windows icin WebView2 build prerequisite'leri kurulu olmalidir. Linux prerequisite listesi `docs/03-development/local-development.md` icinde tutulur.

## M1 aktif baseline

- IME-aware `contenteditable` paragraph editor.
- Rich TextRun render.
- Logical Unicode selection mapping.
- B/I/U range formatting.
- Font family ve font size formatting.
- Caret typing-style session state.
- Left/center/right/justify paragraph alignment.
- Dosya, Giris, Ekle ve Gorunum tab shell'i.
- Giris ribbon icinde belge, history, font ve paragraph gruplari.
- Undo/Redo ve New Document.
- Native Open/Save dialoglari.
- Local `.tko` ZIP+YAML round-trip.
- Dirty revision guard ve discard confirmation.
- Ctrl/Cmd+O, Ctrl/Cmd+S ve Ctrl/Cmd+N file operation flow.

Autosave/recovery, external-change protection, Print, DOCX ve PDF M1 roadmap'inde sonraki parcalardir.
