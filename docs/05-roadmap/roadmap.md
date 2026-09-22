# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/05-roadmap/roadmap.md
# 📌 Amac: Turkuaz Office milestone sirasini ve roadmap deviation yasagini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Foundation Hardening sonrasi Desktop Writer ve takip eden platform milestone sirasini sabitler

Bagimli Oldugu Katman: Documentation

# Roadmap

## M0 - Foundation v0.1.1

Durum: Foundation Hardening tamamlandi.

- Monorepo.
- Rust Core.
- Layer contract.
- Canonical Document Model.
- Schema version type + migration service boundary.
- Native `.tko` format boundary.
- Security/untrusted document policy.
- Autosave/recovery contract.
- File locking/external change contract.
- Font/layout contract.
- Clipboard contract.
- Print preview contract.
- Localization contract.
- Accessibility/keyboard quality bar.
- Offline-first sync boundary.
- Plugin API/capability boundary.
- Performance budgets.
- CI foundation.
- Detailed docs.

## M1 - Desktop Writer v0.2.0

Durum: Devam ediyor. Headless Writer Domain, Desktop Shell, rich-text/IME, typography/ribbon, local Open/Save, autosave/recovery, external-change protection, font/layout, Clipboard Minimum ve Print Preview + Print Minimum implementation alt fazlari tamamlandi. Compiler-backed clipboard validation GitHub hosted runner tahsis sorunu nedeniyle pending tutuluyor.

### Tamamlanan M1 parcasi

- Writer document tree.
- Stable node IDs.
- Logical selection modeli.
- Typed command system.
- Undo/redo baseline.
- Image/table minimum domain modeli.
- TKO v1 logical manifest/content profile.
- Writer repository/controller/view boundary.
- Turkish + English Writer language resources.
- Domain test matrix ve validation report.
- Tauri 2 + SolidJS Desktop Shell.
- Windows + Linux shell baseline.
- IME-aware contenteditable paragraph surface.
- DOM selection -> logical Unicode scalar mapping.
- Rich TextRun rendering.
- B/I/U selection commandlari.
- Minimal diff paragraph mutation ile style-preserving input.
- Font family ve font size character style baseline.
- Caret typing-style session state ve styled insert command.
- Paragraph left/center/right/justify alignment commandlari.
- Giris ribbon baseline: belge, history, font ve paragraph gruplari.
- Local `.tko` Open/Save.
- Native `.tko` ZIP+YAML serializer profile v1.
- Native dialog Tool + minimum Tauri capability.
- Dirty revision baseline ve unsaved-changes guard.
- Platform safe-replace local save.
- Autosave recovery TKO snapshot + YAML metadata.
- Startup Recover / Compare / Discard karari.
- Per-document recovery retention ve explicit Save cleanup.
- Content fingerprint tabanli external-change detection.
- Cooperative sidecar file lock + read-only fallback.
- Backend mutation guard ve same-path overwrite conflict.
- Reload / Keep Local / Save As file protection akisi.
- Primary section PageSettings read-model ve twip tabanli Desktop page geometry.
- Session-only %50-%200 zoom ve keyboard/statusbar kontrolleri.
- Platform font fallback resolver; requested family canonical belgede korunur.
- DOM text metric canonical layout sayilmaz siniri.
- Clipboard paragraph-local atomic styled-fragment replace command ve tek-undo mutation zinciri.
- Runtime copy/cut/paste event hatti ve Internal MIME -> sanitized HTML -> plain text representation priority.
- Clipboard HTML node/depth/payload limitleri ve raw contenteditable default-paste bypass korumasi.
- PNG/JPEG/WebP image clipboard payload -> canonical WriterAsset + ImageBlock mutation hatti.
- TKO v1 optional assets/index.yml + assets/data/<asset-id>.bin binary image package profile.
- Lazy writer_get_asset IPC ve ObjectURL cleanup ile image block render baseline.
- Session-only Print Preview + system print dialog Tool adapteri ve canonical physical page geometry baseline.

### Siradaki M1 parcasi

- DOCX minimum profile.
- PDF export.
- Recent files.
- File associations.
- Template foundation.
- Turkish + English UI.
- Keyboard-only smoke test.

## M2 - Sheet v0.3.0

- Cell model.
- CSV/XLSX.
- Basic formula engine.
- Format/filter/sort.
- Basic charts.
- 100000-cell benchmark profile.

## M3 - Web v0.4.0

- Browser client.
- WASM-compatible core slice.
- Browser storage.
- Import/export.
- Offline cache boundary.

## M4 - Cloud/API v0.5.0

- Account.
- Cloud storage.
- Sync.
- Version history.
- Sharing foundation.

## M5 - Mobile v0.6.0

- Android.
- iOS.
- Mobile Writer/Sheet UX.

## M6 - Slides v0.7.0

- Slide scene model.
- Basic PPTX.
- Presentation mode.

## M7 - Collaboration v0.8.0+

- Operation protocol.
- Presence.
- Live cursors.
- Comments.
- Conflict resolution.
- CRDT/OT secimi icin ayri ADR.

## Backlog ama mimari olarak desteklenen

- Spellcheck provider abstraction.
- Advanced templates.
- Advanced PDF tools.
- Signed external plugins.
- Optional cloud collaboration.

Roadmap sirasi degisecekse once bu dosya ve ilgili ADR guncellenir; kod sessizce farkli yone gitmez.
