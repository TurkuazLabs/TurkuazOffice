# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-desktop-shell-v0.2.0.md
# 📌 Amac: Writer v0.2.0 M1 Desktop Shell implementation kapsam ve durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Tauri 2, SolidJS, IPC bridge, paragraph input ve kalan M1 islerini ayrintili izler

Bagimli Oldugu Katman: Documentation

# M1 Desktop Shell v0.2.0

## Bu pakette tamamlananlar

- `apps/desktop` SolidJS + TypeScript + Vite frontend iskeleti.
- `apps/desktop/src-tauri` Tauri 2 Rust shell crate.
- Windows/Linux hedefli tek main window konfigurasyonu.
- Minimum capability ve explicit CSP baseline.
- Writer create, paragraph replace, split, merge, undo ve redo IPC komutlari.
- Frontend Controller -> Service -> Repo -> Tool -> View -> Language ayrimi.
- Rust Desktop Controller -> Service -> Writer Domain Controller/Service koprusu.
- Read-only structured Writer DTO: document -> paragraphs -> runs.
- Paragraph bazli textarea prototipi.
- Enter ile split, satir basi Backspace ile merge.
- UI input debounce.
- Frontend mutation serialization queue; stale IPC response sirasi engellenir.
- Blur/Undo/Redo oncesi pending paragraph commit davranisi.
- Production CSP ve local Vite websocket icin ayri development CSP.
- DOM UTF-16 selectionStart -> Writer logical Unicode scalar offset Tool adaptoru.
- Dark/light OS theme token baseline.
- Keyboard-visible focus ve semantic toolbar baseline.

## Bilincli gecici sinirlar

Paragraph editor su an plain-text textarea prototipidir. Rich text run rendering, IME composition-aware transaction engine ve selection mapping M1'in sonraki fazidir. Paragraph metni tamamen degistiginde ilgili paragraph run yapisi ilk run stilini koruyarak yeniden kurulur; rich text etkinlesmeden bu davranis kabul edilen prototip siniridir.

Save, Print ve B/I/U butonlari gorunur fakat bilerek disabled durumdadir. Kullaniciya calismayan fonksiyon varmis gibi davranilmaz.

Bundle installer bu fazda `bundle.active=false` tutulmustur. Installer/signing release fazinda etkinlestirilir.

## Siradaki alt faz

1. Gercek Rust compile/test/clippy CI dogrulamasi.
2. IME composition test harness.
3. DOM selection -> NodeId/logical offset mapper.
4. Rich text run renderer.
5. B/I/U command wiring.
6. TKO byte archive save/open.
7. Autosave/recovery desktop adapter.
8. Print preview baseline.
9. DOCX minimum profile.
