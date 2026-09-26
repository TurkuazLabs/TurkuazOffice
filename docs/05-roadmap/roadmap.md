# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/05-roadmap/roadmap.md
# 📌 Amac: Turkuaz Office milestone sirasini ve roadmap deviation yasagini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
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

Durum: Feature kapsaminda tamamlandi; compiler-backed final validation pending. Headless Writer Domain, Desktop Shell, rich-text/IME, typography/ribbon, local Open/Save, autosave/recovery, external-change protection, font/layout, Clipboard Minimum, Print Preview + Print Minimum, DOCX Minimum, PDF Export Minimum, Recent Files, File Associations Minimum, Template Foundation, Turkish + English UI ve Keyboard-Only Smoke implementation alt fazlari tamamlandi. Compiler-backed son M1 validationlari GitHub hosted runner tahsis sorunu nedeniyle pending tutuluyor.

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
- DOCX paragraph/run/page minimum import-export adapteri, compatibility report ve strict unsupported export bariyeri.
- PDF paragraph/run/page minimum export adapteri, embedded system font, multi-page text flow ve strict unsupported export bariyeri.
- Native TKO Recent Files persistence, canonical path dedup, missing-file prune ve ribbon quick-open baseline.
- Native TKO Windows NSIS + Linux DEB file association ve cold-start open baseline.
- Built-in YAML Writer template catalog, Bos/Mektup/Rapor skeleton ve ribbon quick-create baseline.
- Runtime tr-TR/en-US Desktop UI, typed language pack coverage ve kalici locale preference baseline.
- Typed keyboard shortcut resolver, aria-keyshortcuts ve keyboard-only smoke baseline.

### M1 kapanis durumu

M1 feature kapsaminda planlanan alt fazlar tamamlandi.

Hosted GitHub runner job'lari steps=null ile checkout oncesi dustugu icin compiler-backed final validation pending tutulur. Bu altyapi durumu M1 source kapsamindan ayri izlenir.

## M2 - Sheet v0.3.0

Durum: Devam ediyor. Cell Model, CSV/XLSX ve Basic Formula Engine implementation parcalari tamamlandi. Compiler-backed validation hosted runner steps=null sorunu nedeniyle pending tutuluyor.

### Tamamlanan M2 parcasi

- Yeni turkuaz-office-sheet workspace crate.
- Controller -> Service -> Repo -> Tool -> View -> Language katman iskeleti.
- Sparse SheetDocument / Worksheet / CellAddress / CellValue canonical model.
- Text / finite number / boolean cell value baseline.
- XLSX grid limitleri: 1048576 satir x 16384 kolon.
- Simple A1 parse/format Tool: A1..XFD1048576.
- Sparse BTreeMap set/get/clear mutation.
- Same-value ve missing-clear revision no-op.
- Typed cell/grid validation hatalari.
- Thin SheetController ve deterministic Sheet View.
- Cell model regression test matrisi.
- CSV UTF-8 text-only import ve typed value export baseline.
- XLSX value-only multi-sheet text/number/boolean import-export baseline.
- XLSX inlineStr export + inlineStr/sharedStrings import.
- XLSX ZIP/XML resource limitleri, root relationship validation ve strict formula reject.
- CSV/XLSX regression test matrisi.
- Basic numeric formula parser/evaluator baseline.
- Arithmetic precedence, same-sheet A1 reference, SUM range ve typed formula error modeli.
- Cycle/div0/type/range/dependency/expression limit bariyerleri.
- Raw formula source ile evaluated result ayrimi.

### Siradaki M2 parcasi

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
