# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-wasm-browser-smoke-v0.4.0.md
# 📌 Amac: Generated Rust WASM Core modulunun gercek browser runtime smoke dogrulamasini dokumante eder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.1.0
# Aciklama: Playwright Chromium uzerinden production Web build, runtime loader ve rust-wasm capability sonucunu dogrular
# Bagimli Oldugu Katman: Tool | Service | Controller | View | CI

# M3 Web WASM Browser Smoke v0.4.0

## Kapsam

Bu dilim generated wasm-bindgen artifactinin yalnizca uretilmesini degil, production Web build icinde gercek Chromium runtime'inda yuklenmesini dogrular.

## Akis

1. Rust Core `wasm32-unknown-unknown` release build edilir.
2. wasm-bindgen generated JS/WASM dosyalari `apps/web/public/wasm` altinda uretilir.
3. Vite production build generated dosyalari Web cikisina dahil eder.
4. Playwright Chromium ile production preview acilir.
5. Uygulama runtime loader uzerinden generated modulu initialize eder.
6. View read-model sonucu browser fallback yerine `rust-wasm` bridge gosterir.

## Smoke kontrati

Browser smoke testi:

- Core bridge: `rust-wasm`
- Core ABI: `1`
- Native filesystem: `Kapali`
- Browser console error: yok

Bu test generated module importu, WASM fetch/instantiate, Tool adapter, Service, Controller ve View zincirini birlikte kapsar.

## CI

`Web WASM artifact` job'u artifact build sonrasinda Web dependency'lerini ve Chromium'u kurar, production build alir ve `npm run smoke:wasm` komutunu calistirir.

## Sonraki dilim

WASM runtime zinciri browser seviyesinde dogrulandiktan sonra M3'te siradaki ana veri katmani isi IndexedDB-backed canonical document persistence'tir.
