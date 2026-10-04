# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-wasm-runtime-v0.4.0.md
# 📌 Amac: M3 Web generated Rust/WASM binding ve browser runtime loader diliminin kapsam ve sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.1.0
# Aciklama: Ayrik binding crate, wasm-pack build, Web runtime Tool composition ve fail-closed bootstrap davranisini dokumante eder

Bagimli Oldugu Katman: Documentation

# M3 Web WASM Runtime v0.4.0

## Durum

Compile-verified Rust Core capability yuzeyi generated browser binding ile runtime'a baglanir.

Pure `turkuaz-office-core` crate'i wasm-bindgen dependency almaz. Browser export katmani ayri `turkuaz-office-web-wasm` crate'inde tutulur.

## Binding crate

`turkuaz-office-web-wasm`:

- workspace uyesidir.
- `cdylib` + `rlib` uretir.
- `turkuaz-office-core` uzerinden capability fonksiyonlarini delegate eder.
- `wasm-bindgen = 0.2.129` ile browser exportlari uretir.
- Business logic veya platform storage kurali tasimaz.

Export edilen browser fonksiyonlari:

- `web_core_abi_version`
- `web_core_document_schema_version`
- `web_core_bridge_kind`
- `web_core_native_file_system_access`

## Generated build

Web package `wasm-pack 0.15.0` ile binding uretir.

`npm run wasm:build` generated JS/TypeScript/WASM dosyalarini `apps/web/src/generated/wasm-core/` altina yazar.

Bu klasor build artifactidir ve Git tarafindan izlenmez.

## Runtime Tool akisi

`main.tsx` bootstrap sirasinda:

1. `createBrowserWasmRuntimeTool()` olusturur.
2. wasm-bindgen generated module'u initialize eder.
3. Generated module'u mevcut `WasmCoreTool` kontratina adapte eder.
4. `WebBootstrapService` gercek Rust/WASM capability degerleri ile kurulur.
5. Init hata verirse silent fallback yapilmaz; kullaniciya bootstrap hatasi gosterilir.

`BrowserCoreContractTool` yalniz foundation/test fallback kontrati olarak kalir; production composition default'u degildir.

## CI

Web quality job'u:

- Node 24 kurar.
- Rust 1.98.1 + `wasm32-unknown-unknown` target kurar.
- npm dependency'lerini kurar.
- `npm run wasm:build` ile generated binding uretir.
- `npm run build:web` ile TypeScript + Vite production build yapar.
- `npm run test:web` ile regression testlerini calistirir.

Rust quality ayrica workspace native check/clippy/test ve pure Core wasm32 compile gate'ini korur.

## Bilerek kapsam disi

- IndexedDB canonical document persistence.
- Browser import/export.
- Offline Service Worker/Cache Storage.
- Writer/Sheet Web editor entegrasyonu.
- Cloud/API ve sync.

## Sonraki dilim

Siradaki M3 odagi canonical Web document persistence icin IndexedDB Repository adapteridir. localStorage metadata index olarak kalir.
