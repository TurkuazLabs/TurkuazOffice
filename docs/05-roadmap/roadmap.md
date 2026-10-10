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

## R2.x - Sheet Productivity Extensions v0.5.0-v0.11.0

Durum: Roadmap onayli ardil Sheet dilimleri aktif gelistirme hattidir.

- v0.5.0: Excel/Calc esinli hybrid Sheet UI.
- v0.5.1: rectangular selection range ve canonical status aggregates.
- v0.5.2: session-only freeze panes.
- v0.6.0: canonical table objects ve table header filter entegrasyonu.
- v0.7.0: canonical conditional formatting kurallari.
- v0.8.0: SUM, AVERAGE, MIN, MAX ve IF function library.
- v0.9.0: OOXML table parts, worksheet relationships ve conditional-format metadata round-trip.
- v0.10.0: aranabilir Functions sidebar ve formula bar draft helper.
- v0.11.0: canonical Basic Charts Desktop IPC, create/remove paneli ve dependency-free SVG Bar/Line/Pie renderer.

Bu uzanti R2'nin tamamlanmis baseline'ini degistirmez; yeni Sheet productivity dilimlerinin sirasini ve kapsam sinirini resmi olarak tanimlar. v0.11.0 sonrasi yeni fonksiyon gruplari veya yeni spreadsheet yetenekleri ayri roadmap guncellemesi gerektirir.

## R2.y - Desktop UI Refresh v0.12.0

Durum: Aktif. Onaylanan Turkuaz Office gorsel konseptinin gercek Desktop uygulamasina uygulanma dilimidir.

- Varsayilan suite acilisinda modern Start Center.
- Writer ve Sheet icin ayri fakat ayni aileye ait uygulama ikonlari.
- Start Center -> Writer/Sheet ayni pencere modul gecisi.
- `--module writer` ve `--module sheet` direct-launch kontratinin korunmasi.
- Writer ust menunun kelime islemci siralamasina alinmasi.
- Writer Sayfalar + belge canvas + Ozellikler layout'u.
- Sheet ust menunun spreadsheet siralamasina alinmasi ve yesil urun kimligi.
- Proje tanitiminda onaylanan Start Center, Writer, Sheet ve genel UI konsept gorsellerinin yayinlanmasi.
- Frontend/Rust/Windows/Linux Community Preview gate'leri.

Kapsam disi: canonical document modeli, format/schema degisikligi, Sheet native Open/Save, Sunum/PDF urun implementasyonu ve yeni Writer editing semantigi.

## R2.y Quality Gate - Shared Command Reuse Audit

Durum: Planlandi; runtime ozelligi degildir ve M3 Web milestone sirasini degistirmez.

- Yeni Writer/Sheet kisa yol veya Find/Replace ozelligi gelmeden once mevcut command/action/menu/hint/aria envanteri tamamlanir.
- Ortak kabul edilen tek kontrat keyboard normalization, typed command metadata, capability ve UI status/focus sozlesmesidir; Writer/Sheet canonical arama ve mutation servisleri modulde kalir.
- Her ortak export en az iki gercek production tuketici ile kanitlanir. Gelmesi planlanan modul icin bos abstract class olusturulmaz.
- Unused TS/Rust code raporlanir; dinamik Tauri/WASM/pro plugin entrypoint teyidi olmadan kod silinmez.
- Find ilk gercek UX dilimi; Replace ise read-only, undo, dirty revision ve atomic edit semantigi testlerinden sonra acilir.
- PR #33'un ayri process mimarisi, bu roadmap'deki ayni pencere modulu ile uyumsuzdur; PR #33 merge kararindan once ayrica guncelleme gerekir.
- Referanslar: `docs/06-adr/0021-suite-command-reuse-boundary.md` ve `docs/07-quality/suite-code-reuse-audit.md`.

## R2.z - Writer Find Read-Only v0.1.0

Durum: PR uzerinde gelistirme; CI ve Desktop kabul testleri tamamlanmadan bitmis sayilmaz.

- Writer canonical paragraph read-model (DOM text aramasi degil) uzerinde locale-aware case-insensitive Bul.
- Ctrl+F ve Edit > Bul ayni acilan Find paneline gider; query ve onceki/sonraki eslesme ile secim/odak mevcut DomSelection Tool'dan geri yuklenir.
- Unicode scalar offset; Turkce I/i locale testi; paragraph ve document boundary; 1000 eslesme koruma limiti.
- Metin degisikligi, replace, dirty revision, undo history mutation YOK; read-only .tko belge acilsa dahi Bul calisabilir.
- Sheet Bul, Writer Replace, Sheet Replace, tum-belge aralik/OOXML/tablolar ve shortcut metadata genisletmeleri sonraki ayri acceptance PR'larina birakilir.
- M3 Web'in mevcut baseline'i degistirilmez; yeni app veya kopya Find engine kurulmaz.

## R2.aa - Sheet Find Visible Grid v0.1.0

Durum: PR uzerinde gelistirme; tum platform CI ve kabul testleri olmadan tamamlandi sayilmaz.

- Sheet ilk aktif worksheet'in sparse canonical cell read-model degerlerinde, raw formula dahil, Turkce locale duyarliligiyla buyuk/kucuk harf duyarsiz Bul.
- Kapsam acik: Desktop tarafinda gorunen/erisebilen A-Z sutunlari ve ilk 100 satir, aktif filtre/siralama sonucu gorunen satirlar. Gizlenmis satirlar, 101+ satir, AA+ sutun ve diger worksheet'ler sonuca dahil DEGILDIR.
- Ctrl+F ile Edit > Bul ayni gercek Find panelini acar. Onceki/sonraki, sonuc sayaci, bos eslesme durumu ve mevcut SheetController.selectCell uzerinden secim/odak calisir.
- Edit/Find sadece read-model sorgusudur; dirty state, belge revision'i ve cell mutation degismez. Hesaplanmis/formullu degerin sonucunu degil, canonical cell raw formula metnini arar.
- Ilerideki tum-sheet arama ve Replace icin sadece plan kaydi vardir; kullanilmayan backend abstraction veya sahte menu yoktur.
- Writer/Sheet Find panel CSS'i ayni stil kontratini paylasir; canonical arama uygulamalari veri tipleri farkli oldugundan modul Tools'larda ayri kalir.

## R2.ab - Writer Replace One (tek eslesme) v0.1.0

Durum: PR uzerinde gelistirme; tam CI ve kullanici kabulunden once tamamlandi sayilmaz.

- Mevcut Writer Bul panelinde yalniz SECILI TEK eslesmeyi degistirme komutu; Ctrl+H ve tumunu degistir YOK.
- Gercek mutasyon writer_replace_range_with_styled_runs native Rust command ile yapilir. Bu command bir undo snapshot olusturur ve dosya kilidinde ensure_writable calistirir.
- Frontend WriterReplace planlama Tool'u canonical UTF-32 scalar karakter araligini, kaynak query ve locale eslesmesini, run toplamini, ilk eslesen karakter stilini, bos/uzun replacement ve paragraph disi aralik risklerini dogrular.
- Service mutation queue'sunda beklenen belge ID/revision, read-only file session, degismis paragraf metni ve henuz commit edilmemis aktif DOM text yeniden kontrol edilir. Backend kendi file write lock kontrolunu uygular.
- Degisen araligin disindaki rich text run'lari ve paragraf stilleri Rust command tarafindan korunur. Tek operation sonrasinda undo/redo, tekstil mutasyonu ve read-only regresyonlari istenir.
- Yetkinin UI'da pasif gorunmesi backend write lock'un yerine GECMEZ. Native `expectedRevision` precondition'i Rust state lock icinde yanlis revizyonlu istegi typed `writer.document_revision_conflict` ile reddeder. Harici dis sureclerin disk degisiklikleri hala ayri lock/external file change protokolune tabidir.
- Sheet Replace, cross-paragraph Replace, regex, toplu Replace ve Ctrl+H sonraki ayri kabul/guvenlik asamalarina birakilir.

## R2.ac - Writer Replace All: read-only batch preflight v0.1.0

Durum: sadece Tool/test; islev kullaniciya ACILMADI.

- Canonical Writer Find Tool'u icin mevcut 1000 sonuc limiti varsayilan kalir; toplu degistirme plani icin bir adet ek sonuc probe edilir, boylece kirpilmis kismi sessizce degistirmek yasaktir.
- Eslesmeler soldan saga ve paragraf bazinda cakismayan bloklara ayrilir. Komut adayi, ilk eslesme karakterinin run stilini, Unicode scalar offsetlerini, orijinal belge kimlik ve revision'ini korur.
- Hatalı/eksik rich runs, gecersiz query/replacement, no-op, max limit fazlasi ya da cross-paragraph alana cikan aralikta plan URETILMEZ.
- Islem listesi ters belge sirasinda uretilir, ancak bu tek basina atomik mutation saglamaz. Rust tarafinda tum eslesmeleri tek canonical komutta / tek undo snapshot'inda ve expected revision ile uygulayan backend olmadan servis, IPC veya UI'da Replace All acilmaz.
- API ve CI kabul testi tamamlanincaya kadar mevcut Writer tek eslesme Replace ve Sheet salt-okunur Find kontratlari korunur.

## R2.ad - Writer Replace All native atomic batch v0.1.0

Durum: PR icinde backend + Rust acceptance; UI AKTIF DEGIL.

- Yeni typed `writer_replace_all_ranges` Tauri komutu `documentId`, zorunlu `expectedRevision` ve en fazla 1000 `replacements` alir. Native `WriterDesktopState` mutex'i altinda file lock ve document revision yeniden dogrulanir.
- Her aralik Unicode scalar offset, paragraf, bounds, ters belge sirasi, ayni paragrafta non-overlap ve her replacement icin run/metin boyutu denetiminden gecer. Hatalarda typed `writer.invalid_replace_batch` ile mutation ONCESI durur.
- Validated araliklar mevcut domain `execute_batch` uzerinden tek transaction ve tek undo/redo snapshot'i olarak islenir. Domain ikinci command'de hata verirse clone disina hicbir belge/history kaydi yazilmaz.
- Rust acceptance: ayni paragrafta iki eslesme, Unicode, tek Undo/Redo; stale revision reddi, overlap reddi ve basarili ilk komuttan sonra ikinci komut hatasinda tam rollback.
- Replace All gorunur UI, Ctrl+H, Sheet Replace ve harici disk senkronizasyonu hala kapsam disi. Browser kaynakli canli DOM typing ve busy queue siniri denetlenmeden yeni buton acilmayacak.

## R2.ae - Writer Replace All visible workflow v0.1.0

Durum: PR icinde; Windows/Linux test + paket CI ve merge bitmeden release degil.

- Writer Bul panelindeki mevcut yeni metin girdisi icin gercek `Tumunu Degistir` eylemi; secili tek Find sonucu gerektirmez.
- Eylem ancak canonical preflight tam, gecersiz olmayan, en fazla 1000 eslesmeli ve salt-okunur olmayan Writer belgesinde etkinlesir. Overlap ve Turkish locale/Unicode scalar offsetleri korunur.
- Writer Controller -> WriterSessionService -> TauriWriterTool -> `writer_replace_all_ranges` native IPC -> Rust atomic `execute_batch` gercek üretim yoludur.
- Mutation queue icinde doc id/revision, stale query, salt-okunur kilit ve commit edilmemis aktif DOM text yeniden kontrol edilir; native tarafta zorunlu expected-revision ve dosya kilidi tekrar kontrol edilir.
- Tek IPC/tek belge revizyonu/tek Undo+Redo. Hata durumunda belgeye kismi edit veya history commit yazilmaz.
- Sheet hala salt-okunur Find; Ctrl+H, regex, cross-paragraph replacement ve 1000+ eslesme destegi ayri gelecektir.

## M3 - Web v0.4.0

Durum: Basladi. Ilk browser foundation dilimi aktiftir; milestone tamamlanmamistir.

### Tamamlanan M3 parcalari

- SolidJS + TypeScript + Vite browser client shell.
- Controller -> Service -> Repo/Tool -> View -> Language web katman iskeleti.
- localStorage tabanli metadata/index Repository baseline'i; canonical document payload persistence'i degildir.
- Typed Core Tool boundary.
- Native filesystem erisiminin web kontrati disinda tutulmasi.
- Web frontend build/test CI gate'i.
- `turkuaz-office-core` icin `wasm32-unknown-unknown` compile gate'i.
- Typed Rust Web Core ABI/schema capability yuzeyi.
- Web tarafinda generated module icin `WasmCoreTool` adapter kontrati.
- Pinned wasm-bindgen generated JavaScript/WASM artifact CI pipeline'i.
- Browser runtime loader ve fail-closed WASM capability dogrulamasi.
- IndexedDB canonical Core document persistence adapteri.
- localStorage metadata index ile IndexedDB canonical payload ayrimi.
- Browser file picker + Blob download byte-transfer Tool foundation'i.
- .tko accept/download profili ve 16 MiB pre-read browser ingress guardi.
- TKO codec hazir olmadan View aksiyonu acmayan import/export Service siniri.
- Core + Writer dependency yonunu koruyan aggregate Web WASM bridge crate'i.
- Mevcut Writer TkoPackageService'i kullanan TKO inspect/re-encode WASM exportlari.
- Generated Writer TKO WASM exportlarini fail-closed typed TypeScript Tool ile acan runtime adapter.
- Browser file-transfer ile typed TKO Tool'u compose eden inspect import + canonical re-encode export Service hatti.
- Zengin TKO byte payloadini kayipsiz koruyan session-only Writer Web Repository ve Service lifecycle hatti.

### M3 kalan kapsam

- Kullaniciya acilan TKO import/export View akisi.
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
