# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/suite-code-reuse-audit.md
# 📌 Amac: Turkuaz Office mevcut ortak altyapi, tekrar ve olasi olu kod risklerini somut kanit ve kalite aksiyonlariyla izler
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.1.0
# Aciklama: Kod tabanindaki tekrarlari, dogrulanmamis silme adaylarini, uygulanacak testleri ve devreye alma kriterlerini belirler

Bagimli Oldugu Katman: Controller | Service | Repo | Tool | View | Language | Config

# Suite Shared-Code Audit - 2026-10-09

Referans baseline: TurkuazLabs/TurkuazOffice `main` 46e5aa70f0c9fe5363e63118b53b3cc73801e2bc.
Kapsam: GitHub uzerinden secili source, config, docs ve PR metadata incelemesi.
Kisit: Tam repository dependency graph, canli Windows/Linux startup profili ve Knip/cargo-machete taramasi bu denetimde calistirilmadi.
Karar: Buradaki riskler, otomatik olarak "silinebilir olu kod" sonucunu vermez.

## Gozlemler

| Kod / kontrat | Kanit | Risk turu | Karar |
| --- | --- | --- | --- |
| `apps/desktop/src/config/keyboard.ts` | `WRITER_SHORTCUT_ACTIONS`, `WRITER_ARIA_SHORTCUTS`, `WRITER_SHORTCUT_HINTS`, Sheet esdegerleri ayri tablolar | Metadata tekrar/drift riski | Etkin komutlar icin tek typed descriptor kaynagi |
| `apps/desktop/src/services/keyboard-shortcut.service.ts` ve `sheet-keyboard-shortcut.service.ts` | Iki resolver ortak modifier/kucuk harf normalizasyonu kullaniyor | Sinirli davranis tekrari | Yalniz normalization ortak; command policy modul bazli |
| `apps/desktop/src/views/writer-shell.tsx` ve `sheet-shell.tsx` | Ikisi de `window.addEventListener("keydown")` kurup temizliyor | Olay baglama tekrari | Once parity testi, sonra gercek gereksinim varsa ortak adapter |
| `apps/desktop/src/views/suite-titlebar.tsx` ve `suite-icon.tsx` | Writer/Sheet icin mevcut ortak baslik ve ikon View'u var | Mevcut dogru reuse | Ikinci shared header/icon kutuphanesi ekleme |
| `apps/desktop/src/config/ipc-commands.ts` | Tauri IPC adlari tek merkezi tabloya alinmis | Mevcut dogru reuse | Ayrica ikinci IPC registry olusturma |
| `apps/desktop/src/config/app-container.ts` | Iki modulun repository, service ve Tool instance'lari tek yerde olusturuluyor | Startup/memory aday maliyeti | Gercek sure/memory olcmeden lazy composition yok |
| `apps/desktop/src/services/writer-session.service.ts` | 1321 satir ve file/edit/print/selection/IME akislari | Sorumluluk yogunlasmasi | Odakli Service'lere sadece testli ve gercek ihtiyac varsa bol |
| `apps/desktop/src/services/sheet-session.service.ts` | 978 satir ve cell/table/query/chart/format akislari | Sorumluluk yogunlasmasi | Siklikla degisen akislar icin parcali refactor oncesi test |
| `apps/desktop/src/views/sheet-functions-sidebar.tsx` | Local `search` state yalniz function catalog'u filtreliyor | Yanlis soyutlama riski | Belge Find motoru olarak yeniden kullanma |
| `apps/desktop/tsconfig.json` ve `apps/web/tsconfig.json` | `strict` var; `noUnusedLocals` ve `noUnusedParameters` yok | Olasi erisilemeyen yerel kod | Ilk compiler raporundan sonra staged gate |
| `Cargo.toml` | Workspace Rust lints ve shared crates var | Bagimlilik/feature birikimi olasigi | Mevcut Cargo lint + onayli periodic audit |
| `docs/05-roadmap/roadmap.md`, `config/project.yml`, `apps/desktop/src/main.tsx`, `docs/08-implementation/desktop-suite-entrypoints-v0.5.0.md`, PR #33 | Runtime in-window, roadmap ve config in-window diyor; mevcut desktop suite entrypoints dokumani ve PR #33 ayri process hedefliyor | Mimari karar ve dokumantasyon uyumsuzlugu | PR #33 oncesinde tek hedef sec, gercek runtime'a gore dokumani duzelt |
| PR #32 | Gercek WASM Chromium smoke eklemeyi hedefliyor; acik | Tamamlanmamis kalite kapisi | Bugun zaten varmis gibi kabul etme |
| PR #57 | AltGr korumasi acik PR'da; main'de degil | Kopya bugfix riski | Ayni korumayi ayri ikinci implementation olarak ekleme |

## Bu turda kanitlanamayanlar

- Kesin unused source/exports/dependencies sayisi.
- Dinamik Tauri command invocation, WASM generated artifact, conditional feature ve test fixture kullanimlari dahil tum entrypoint erisimi.
- Tekrar eden kodun runtime bundle olcekli maliyeti.
- Bir Service'i parcalamanin performans/net kod azaltma kazanimi.
- Native Search/Replace motorunun halihazirda var oldugu: incelenen Writer/Sheet Desktop Service + keyboard kontratinda bir belge Find/Replace komutu tespit edilmedi; bu tum git gecmisinin tarandigi anlamina gelmez.

## Aksiyon oncelikleri

### P0 - Yeni shortcut ozelligi oncesi
1. Mevcut kisa yol descriptor/action/hint/aria tablosunu cikar ve 1:1 bagla.
2. PR #57 ile ortak modifier guard'in gelecegini kontrol et; ayni AltGr logic'ini bir kez daha yazma.
3. Menu/toolbar/key event'in gercek handler'a, handler'in Service'e ve gerekiyorsa Core mutation'a kadar ulasmasini testle.
4. PR #33 icin "tek pencere mi ayri process mi" kontratini onceden sec; roadmap guncellenmeden birlestirme.

### P1 - Olasi olu kodu dogrula
1. Her TS entrypoint'i belirle: Desktop main, Web main, test, config, generated WASM/IPC ve lazy import.
2. Desktop/Web TS compiler'a `noUnusedLocals` ve `noUnusedParameters` acildiginda kac ve hangi hata ciktigini raporla; mevcut CI'yi bozma.
3. Knip icin Desktop ve Web entrypoint, Tauri IPC string bridge ve generated modul/config girislerini acikca tanimla. Ilk calismasi rapor odakli ve non-blocking olsun.
4. Rust icin mevcut Cargo dependency lint sonuclarini kaydet. cargo-machete karsilastirmasini opsiyonel/periodic yap; proc-macro/build-script/feature gated false-positive adaylarini teyit et.
5. Silebilecegin her dosya/export icin "production imports 0 + dynamic entrypoints 0 + ABI/API consumer 0 + tests/CI green" kaniti bulunsun.
6. Public API'yi kullanan Pro/harici kullanicilar icin sadece internal reachability ile silme karari verme.

### P2 - Calisan Find/Replace
1. `Ctrl+F` ve gercek Find UI ayni PR'da, Worker/Service/Core arama semantics ve status/focus regression ile gelir.
2. Writer Unicode scalar/paragraph/mixed style offset'lerini korur; Sheet sparse cell/formula/visible value semantics ayri belirlenir.
3. `Ctrl+H` replacement mutation, undo/redo, dirty revision, file protection, read-only, IME ve type check'ler olmadan acilmaz.
4. Yeni command registry, sadece gercek iki production handler ortak oldugunda shared olur.

## Ilk uygulanan dilim - Strict TypeScript + keyboard metadata parity

- `apps/desktop/src/config/keyboard-contract.test.ts` Writer/Sheet komut kimligi benzersizligini, kisa yol gorunen yazilarini ve ARIA eslemelerini Vitest ile dogrular.
- 2026-10-09 CI advisory ilk olcum: Web 0; Desktop 1 TS6133. Tek bulgu `apps/desktop/src/services/sheet-session.service.test.ts` dosyasindaki kullanilmayan chart mock callback parametresidir; test davranisi degismeden parametre kaldirildi.
- Ayni CI'nin ikinci olcumu: Web 0 ve Desktop 0. Bu sayilar yalniz TS6133/TS6192/TS6196 sembol diagnostikleridir; unused exports/dependencies taramasi degildir.
- `apps/desktop/tsconfig.json` ve `apps/web/tsconfig.json` icin `noUnusedLocals` ve `noUnusedParameters` artik `true`. Mevcut `npm run build` quality gate bu hatalarda dogrudan fail verir.
- Gecici advisory script ve CI adimlari, strict gate'de gereksiz tekrar olusturacagi icin ayni PR icinde kaldirildi. Final PR'da yeni npm paketi veya ayri arka plan tarayici yoktur.
- Knip, Rust Cargo dependency ve dinamik Tauri/WASM/Pro referanslari henuz tam taranmadigi icin, bu alanlarin silinmesi onaylanmamistir.

## Ikinci dilim - Files / Exports / Dependencies candidate audit

CI workflow: `.github/workflows/code-reachability-audit.yml` (PR + elle tetikleme).

- Desktop ve Web ayri npm projeleri olarak incelenir. Knip 6.39.0 kendi Vite/Vitest konfigunu ve `src/main.tsx` entrypoint'ini otomatik cozer; varsayilan kurallar kullanilir. Ikinci bir entrypoint listesi ve ignore-all config eklenmez.
- `knip-full.json` gelistirme/test baglantilarini; `knip-production.json` gercek uygulama baglantilarini ayri raporlar. Aradaki fark test-only erisimi belirlemek icin kullanilir. `--fix` ve `--allow-remove-files` ASLA calistirilmaz.
- Rust icin sabit `cargo-machete 0.9.2` taramasi workspace `Cargo.toml` bagimliliklarini inceler. Pinned CLI tarafinda `--json` uyumsuzlugu goruldugu icin metin raporu (`cargo-machete.txt`) uretir; CLI hatalari fail-closed kontrol edilir. Kod uretimi, Rust proc-macro, renamed crates, cfg/feature-gated kod ve harici FFI false-positive nedenleridir.
- Tum analizler normal `workspace-ci` build/test gate'lerinden bagimsizdir. Rapor var diye CI basarisiz olmaz; arac gercekten calismazsa CI hata verir. Ilk denemede Tauri JSON5 Knip parse uyumsuzlugu ve Rust `--json` CLI uyumsuzlugu CI loglariyla saptanmistir; taramanin gercekten tamamlanmasi zorunludur.
- Sadece 14 gunluk artifacts ve job summary tutulur; gereksiz kalici package/binary dependency repo uzerine eklenmez.
- `apps/desktop/src/config/ipc-commands.ts` Tauri command string'leri frontend Tool'larindan cagrilir; Rust tarafinda `apps/desktop/src-tauri/src/lib.rs` `tauri::generate_handler!` ile register edilir. Yalniz TS graph'a bakarak Rust IPC command silinmez.
- `apps/web/src/main.tsx` dinamik WASM loader ve generated public module ile calisir. Generated WASM tarama disinda kalabilir; baglantilar manuel teyit edilir.
- Pro repo Community public API'sini dependency olarak kullanir. Community dis kullanici/Pro reference analizi olmadan export silme onaylanmaz.

### 2026-10-09 CI'da olculen ilk gercek sonuclar

Gecerli denetim: `code-reachability-audit` run #37914196700; Knip 6.39.0 ve cargo-machete 0.9.2.

| Kapsam | Full (test dahil) | Production | Yorum |
| --- | --- | --- | --- |
| Desktop unused files | 0 | 0 | Tum dosyalar graph uzerinden ulasilabilir gorunuyor; dinamik giris noktasi istisnalari saklidir |
| Desktop unused npm dependencies | 0 | 0 | Knip candidate yok |
| Desktop unused exports + types | 13 | 15 | 2 ek dev-host/port sabiti sadece Vite config'te kullanildigi icin production taramasinda ayrica isaretlenir |
| Web unused files | 0 | 0 | Graph candidate yok |
| Web unused npm dependencies | 0 | 0 | Knip candidate yok |
| Web unused exports + types | 1 | 4 | 2 ek dev-host/port sabiti Vite config icin gercekten kullanilir |
| Rust unused dependencies | 0 | 0 | `cargo machete .` workspace taramasi "didn't find any unused dependencies" raporu verdi |

Dogrulanan false-positive / must-keep:
- `apps/desktop/src/config/runtime-config.ts`: `DESKTOP_DEV_HOST` ve `DESKTOP_DEV_PORT` `apps/desktop/vite.config.ts` tarafindan kullanilir; production Knip bu build config'i production giris noktasi saymaz. Silinmez.
- `apps/web/src/config/runtime-config.ts`: `WEB_DEV_HOST` ve `WEB_DEV_PORT` `apps/web/vite.config.ts` tarafindan kullanilir. Silinmez.
- `apps/desktop/src/config/file-format.ts`: `TKO_FILE_EXTENSION`, `DOCX_FILE_EXTENSION`, `PDF_FILE_EXTENSION` dogrudan kendi `*_FILE_EXTENSIONS` listelerini kurar. Gerekiyorsa sadece disari export kaldirilir, ic tanim korunur.
- `apps/web/src/tools/wasm-core-runtime-loader.ts`: `WasmBindgenCoreModule` interface'i importer ve runtime WASM kontrolunde kullanilir; silinmez.
- `apps/web/src/models/web-models.ts`: `WebDocumentStorageKind` yerel `WebBootstrapViewModel` tarafindan kullanilir; silinmez.

Sadece odakli removal/reduction PR'sinde incelenecek adaylar:
- Desktop `DEFAULT_LOCALE` export'u: eski `tr` sabiti ile yeni `tr-TR` localization sozlesmesi uyumu teyit edilmeli.
- Desktop `WRITER_RIBBON_TABS` ve `WriterRibbonTabConfig`: artik gercek `writer-ribbon.tsx` klasik menu kontrol listesi yerine pasif placeholder olabilir.
- Desktop `WRITER_PRINT_SYSTEM_DIALOG_CAPABILITIES`: merkez `config/project.yml` print ownership kontratiyla tekrar etme ihtimali var.
- Desktop `ClipboardDomTextNodeModel`, `WriterParagraphStyleView`, `WriterDocxCompatibilityView`, `WriterExternalChangeStateView`, `SheetWorksheetView`, `SheetChartView`: dahili type union ve public API kullanimlari tek tek denetlenmeden kaldirilmaz.

Arac/sonuc farki:
- Ilk cargo-machete denemesinde desteklenmeyen `--json` parametresi stdout'u bos birakip CI'yi yanlis PASS gosterebildi. Nihai calisma `cargo machete .` ve status/output dogrulamasi ile gecerli tarama yapti.
- Knip Desktop ilk denemesinde geceli Tauri JSON5 konfigunu JSON parser ile okuma hatasi vardi. `apps/desktop/knip.jsonc` yalnizca Tauri config parsing'i bosaltir; entrypoint, Vite/Vitest ve kaynak kod taramasi korunur.

### Odakli temizleme - PR #61

Uygulanan degisiklikler:
- `WRITER_RIBBON_TABS` ve bundan tureyen `WriterRibbonTabConfig` gercek `writer-ribbon.tsx` menusunun kullanmadigi tumu disabled placeholder kayitlariydi; kaldirildi.
- `WRITER_PRINT_SYSTEM_DIALOG_CAPABILITIES` ikinci listeydi ve print dialog'u zaten platforma delege ediliyordu; kaldirildi.
- `DEFAULT_LOCALE = "tr"` mevcut `tr-TR` / `en-US` kontratina bagli degildi ve kullanilmiyordu; kaldirildi.
- `tools/verify-project.sh` eski disabled listeyi dogrulamak yerine canli `WRITER_ALIGNMENT_COMMANDS` metadata'sini dogrular.

Korunan kontratlar: Vite development host/port, Writer active alignments, native file dialog extension arrays, Web WASM importer, internal type unions, Rust IPC ve Pro public API.
Temizlik sirasinda dokuman schema ve ekran davranisi degistirilmez; ayri PR'da tam CI gerekir.

### Internal export daraltma - sonraki dilim

Knip full taramasinda kullanilmayan export olarak raporlanan fakat dosya icinde hala gereken tanimlar **silinmez**; yalnizca disariya `export` edilmesi kaldirilir.

- Desktop `file-format.ts`: `TKO_FILE_EXTENSION`, `DOCX_FILE_EXTENSION`, `PDF_FILE_EXTENSION` lokal kalir; gercekten kullanilan `*_FILE_EXTENSIONS` listeleri export edilmeye devam eder.
- Desktop `clipboard-model.ts`: `ClipboardDomTextNodeModel` lokal kalir; `ClipboardDomNodeModel` union type disari aciktir.
- Desktop `writer-types.ts`: `WriterParagraphStyleView`, `WriterDocxCompatibilityView`, `WriterExternalChangeStateView` daha ust seviyede export edilen DTO'larin ic alanlarinda kullanilmaya devam eder.
- Desktop `sheet-types.ts`: `SheetWorksheetView` ve `SheetChartView` ust belge DTO'larinin icinde korunur.
- Web `web-models.ts`: `WebDocumentStorageKind` `WebBootstrapViewModel` icinde kullanilan lokal type olur.

Community Desktop ve Web `package.json` kayitlari `private: true` olarak isaretlidir. 2026-10-09 tarihinde Pro repo tree'si iki Rust crate'i ve ilgili operasyon/dokuman dosyalarindan olusur; bu tipler icin herhangi bir TypeScript importu bulunmaz. Pro canonical Core/Rust contract'ini degistirmiyoruz.

Bu adim **runtime veri veya belge schema temizligi degildir**. Internal type baglantilari ve tam CI korunur. Hedef Knip full-mode unused export/type adaylarinin sifira inmesi; sonuc gercek CI'da ayrica dogrulanmadan tamamlandi denmez. Production modunda Vite host/port adaylari build config bagimliligidir ve silinmez.

### Sadece aday olarak isaretleme kurali

Bir dosyanin silinebilir oldugu ancak su dort kanit birlikte varsa kabul edilir:
1. Production entrypoint/import/IPC/WASM/Pro consumer yok.
2. Dinamik import, test fixture, build script, cfg flag veya reflection referansi yok.
3. Public API uyumluluk riski incelendi, gerekiyorsa deprecation yapildi.
4. Silme degisikliginin Windows/Linux Rust, frontend/web, WASM ve desktop package CI sonucu PASS.

## Mevcut komut yuzeylerinin somut eslesmesi ve odak sahipligi

2026-10-09 Writer/Sheet View denetiminde:
- Writer Ribbon (menu + toolbar) `createDocument/openDocument/saveDocument/printDocument/undo/redo`, `toggleBold/Italic/Underline` ve `setParagraphAlignment` icin ayni WriterController'i cagirir. WriterShell klavye action'lari da ayni Controller'a gider; tekrar document engine yoktur.
- SheetMenubar menu callback'lerini SheetShell'den alir. `newDocument`, `toggleProperties`, `toggleQuery` ve `insertFunctionDraft` klavye ve UI tarafinda ayni gercek mutation/panel state fonksiyonlarini kullanir.
- Writer/Sheet pencere seviyesi `keydown` handler'larinin baska metin alanlarinda local undo/format tuslarini ele gecirmesini engellemek icin tek gercek ortak consumer Tool'u eklendi: `shortcut-focus.tool.ts`.
- Writer icin `WRITER_PARAGRAPH_SELECTOR`; Sheet icin `SHEET_GRID_EDITOR_SELECTOR` izinli gercek editor yuzeyleridir. Sheet formula bar ve diger input/textarea/select/contenteditable yuzeyleri browser input klavyesini korur. Global pencere tuslari editor disi normal buton odaginda aynen calisir.
- Hem pure jsdom tool regression'i hem gercek SheetShell formulu/hucresi `Ctrl+B` event routing testi bulunur. AltGr, IME ve native Editor action-service mantigi degistirilmemistir.
- Find/Replace veya baska olmayan komut icin kisa yol/menu placeholder'i eklenmedi. Bu ozellik canonical Writer/Sheet Service mutation, undo, read-only, IME ve focus acceptance testleri tamamlandiginda ayri gelistirilecektir.

## Denetim raporu kontrol listesi

- [ ] `entrypoint-graph` olusturuldu ve dinamik/generator/Tauri/WASM referanslari manuel teyit edildi.
- [ ] Var olan komutlarin UI action -> Controller -> Service -> Core/Tool path haritasi yazildi.
- [ ] Shortcut/hint/aria cakismalari icin unit test ve TR klavye regression var.
- [ ] `noUnusedLocals`, `noUnusedParameters` hatalari raporlandi.
- [ ] Knip raporu false-positive siniflarina ayrildi.
- [ ] Rust Cargo unused-dependency raporu ve macro/feature istisnalari kaydedildi.
- [ ] Kullanilmayan her kod silme PR'si runtime ve CI ile ayri onaylandi.
- [ ] PR #33 roadmap uyumu netlestirildi.
- [ ] Find/Replace baska yerde yalniz kisa yol olarak degil tam calisan feature olarak test edildi.

## Referanslar

- Architecture decision: `docs/06-adr/0021-suite-command-reuse-boundary.md`
- Roadmap: `docs/05-roadmap/roadmap.md`
- TypeScript compiler: https://www.typescriptlang.org/tsconfig/noUnusedLocals.html
- Knip production analysis: https://knip.dev/typescript/unused-dependencies
- Knip issue validation: https://knip.dev/guides/handling-issues
- Cargo dependency lints: https://doc.rust-lang.org/stable/cargo/reference/lints.html
- cargo-machete limitations: https://github.com/bnjbvr/cargo-machete
