# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/05-roadmap/roadmap.md
# 📌 Amac: Turkuaz Office milestone sirasini ve roadmap deviation yasagini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.1
# Aciklama: Foundation, Writer, Sheet engine, Sheet Desktop baseline ve takip eden platform milestone sirasini sabitler

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

Durum: Feature kapsami tamamlandi. v0.3.1 Community Preview hardening ile compiler-backed Windows/Linux CI ve gercek desktop bundle dogrulamasi yeniden acilmistir.

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

v0.3.1 Community Preview release gate; frontend build/test, Rust check/fmt/clippy/test ve Windows/Linux bundle artifactlari ile kapanir.

## M2 - Sheet v0.3.0

Durum: Engine feature kapsami tamamlandi. Sheet Desktop entegrasyon baseline'i R2 kapsaminda tamamlandi; yeni Sheet urun ozellikleri roadmap guncellenmeden eklenmez.

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
- Canonical FormulaCell value modeli.
- Same-worksheet basic formula parser/evaluator.
- Numeric literal, simple/absolute/mixed A1, + - * /, parentheses ve unary +/-.
- Recursive dependency evaluation, empty-reference=0 ve typed cycle/depth/division/non-numeric guardlari.
- Formula Engine regression test matrisi.
- Canonical sparse cell format metadata ve revision-aware format mutation.
- Formula-aware non-mutating filter/sort row query baseline.
- Format/filter/sort regression test matrisi.
- Canonical Bar/Line/Pie chart definition modeli ve deterministic ChartId.
- Formula-aware chart data projection ve typed chart validation.
- Basic Charts regression test matrisi.
- 100000-cell deterministic benchmark profile ve explicit ignored harness.
- Sparse build, deterministic View projection ve 100000-row sorted query workloadlari.

### M2 kapanis durumu

M2 feature kapsaminda planlanan alt fazlar tamamlandi.

M2 engine CI ile dogrulanir. Sheet Desktop kullanici yuzeyi mevcut calisma agacinda R2 baseline'i ile aktiftir.

## R1 - Community Preview Hardening v0.3.1

- Stable GitHub Actions action surumleri.
- Frontend build + unit test.
- Rust workspace check/fmt/clippy.
- Windows + Linux workspace test.
- Windows NSIS gercek bundle.
- Linux DEB + AppImage gercek bundle.
- SHA-256 artifact checksum.
- Writer desktop preview release gate.
- Sheet engine compiler validation.
- R1 baslangic gate'inde Sheet desktop UI kapsam disiydi.

Bu release-hardening fazi yeni urun ozelligi degildir; R1'in tarihsel release gate kapsamidir. R1 sonrasinda eklenen Sheet Desktop entegrasyonu R2 altinda izlenir.

## R2 - Sheet Desktop Integration Baseline

Durum: Tamamlandi ve mevcut Desktop calisma agacinda aktiftir.

### Tamamlanan R2 parcasi

- Writer <-> Sheet modul gecisi.
- Tauri 2 + SolidJS icinde 30x12 gorunur Sheet grid baseline'i.
- Text / number / boolean / formula / clear cell mutation hatti.
- Desktop View -> Controller -> Service -> Repo/Tool -> Rust backend katman siniri.
- Secili hucre state'i ve formula bari.
- Raw hucre degeri ile backend Formula Engine hesaplanan degerinin ayri gosterimi.
- Bold / italic / underline ve general/left/center/right hizalama kontrolleri.
- Canonical CellFormat decimalPlaces tabanli number display baseline'i.
- Sparse frontend format cache.
- Secili kolon icin non-mutating filter/sort UI.
- Aktif grid/formula draft'larini async read-model guncellemelerinden koruma.
- Dirty Sheet icin Yeni Sheet discard onayi.
- Stale row-query response generation + document identity korumasi.
- Geciken format mutation'inin yeni secimi geri almamasi.
- Kuyruktaki cell mutation'larini kaynak document kimligine baglama.
- Frontend build/test, Rust check/fmt/clippy/test ve Windows/Linux bundle CI dogrulamasi.

### R2 kapsam disi

- Sheet native dosya persistence / Open / Save.
- Sheet chart render/editor UI.
- Gelismis multi-range selection.
- Sheet clipboard/copy-paste urun akisi.
- Gelismis formula/autofill/pivot ozellikleri.

Bu kapsam disi maddeler sessizce R2'ye eklenmez. Yeni Sheet urun ozelligi gerekiyorsa once roadmap ve gerekirse ADR guncellenir.

## M3 - Web v0.4.0

Durum: Basladi. Ilk browser foundation dilimi aktiftir; milestone tamamlanmamistir.

### Tamamlanan ilk foundation parcasi

- SolidJS + TypeScript + Vite browser client shell.
- Controller -> Service -> Repo/Tool -> View -> Language web katman iskeleti.
- Browser Storage tabanli local-first Repository baseline'i.
- Typed Core Tool boundary.
- Native filesystem erisiminin web kontrati disinda tutulmasi.
- Web frontend build/test CI gate'i.
- `turkuaz-office-core` icin `wasm32-unknown-unknown` compile gate'i.
- Typed Rust Web Core ABI/schema capability yuzeyi.
- Web tarafinda generated module icin `WasmCoreTool` adapter kontrati.

### M3 kalan kapsam

- Generated Rust/WASM JavaScript binding ve runtime loader.
- IndexedDB canonical document persistence.
- Browser import/export.
- Offline cache boundary.
- Writer/Sheet web read-model ve kullanici yuzeyi entegrasyonu.

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
