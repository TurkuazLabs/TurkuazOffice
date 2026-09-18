# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/desktop-shell-test-matrix.md
# 📌 Amac: Writer Desktop Shell icin zorunlu test ve platform kontrol matrisini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Frontend build, Rust service bridge, keyboard, IME, accessibility ve Windows/Linux kontrollerini listeler

Bagimli Oldugu Katman: Documentation

# Desktop Shell Test Matrix

## Otomatik

| Alan | Kontrol | M1 Durum |
| --- | --- | --- |
| Frontend | TypeScript strict typecheck | Required |
| Frontend | Vite production build | Required |
| Rust | Desktop service unit tests | Required |
| Rust | Workspace cargo test | Required |
| Rust | cargo clippy | Required |
| Contract | verify-project scripts | Required |
| Security | Main capability minimum permission | Required |
| Security | CSP remote script kapali | Required |

## Manuel Windows

- App acilisi ve resize.
- Yeni belge.
- ASCII ve Turkce klavye inputu.
- Unicode emoji inputu.
- Enter split.
- Satir basi Backspace merge.
- Undo/redo.
- Tab focus order.
- High DPI 100/125/150/200 percent.
- WebView2 IME composition: Turkish ve CJK fixture zorunlu.
- Rich-text selection + B/I/U toolbar/shortcut smoke.

## Manuel Linux

- App acilisi ve resize.
- Wayland ve X11 en az birer smoke test.
- GTK/WebKit text input.
- Enter split, merge, undo/redo.
- Dark/light system theme.
- Keyboard-only toolbar navigation.
- WebKitGTK IME composition ve B/I/U rich-text selection smoke.

## Exit criteria

Desktop Shell tamamlandi sayilmasi icin frontend build, Rust workspace test/clippy ve iki platform smoke test raporu bulunmalidir. IME bilinmeyen durumdayken rich text editor production-ready sayilmaz.
