# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-wasm-runtime-loader-v0.4.0.md
# 📌 Amac: M3 Web WASM runtime loader foundation kararlarini ve fail-safe davranisini dokumante eder
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.1.0
# Aciklama: Generated wasm-bindgen modulunun Tool katmaninda yuklenmesi, fallback ve fail-closed kontratini tanimlar
# Bagimli Oldugu Katman: Tool | Service | Config

# M3 Web WASM Runtime Loader v0.4.0

## Kapsam

Bu dilim, compile-verified Rust WASM Core yuzeyi ile browser composition root arasina gercek bir runtime yukleme siniri ekler.

- Generated binding yolu merkezi config ile `/wasm/turkuaz_office_core.js` olarak tanimlanir.
- Dynamic import yalniz Tool katmaninda yapilir.
- wasm-bindgen default initializer, capability fonksiyonlari okunmadan once calistirilir.
- Binding dosyasi henuz deploy edilmemisse mevcut browser-contract Tool fallback'i kullanilir.
- Binding yuklenmis fakat kontrati eksik veya native filesystem yetkisi talep ediyorsa sistem fail-closed davranir.
- View ve Service katmanlari generated module ayrintilarini bilmez.

## Mimari Akis

`main.tsx -> loadWebCoreTool -> WasmCoreTool -> WebBootstrapService -> WebController -> App`

Fallback akisi:

`main.tsx -> loadWebCoreTool -> BrowserCoreContractTool -> WebBootstrapService`

## Guvenlik Siniri

Import/yukleme hatasi deploy-time eksik artefakt olarak ele alinir ve browser fallback'e izin verilir. Buna karsilik basariyla yuklenen bir modulun kontrat hatasi sessizce fallback'e dusurulmez. Eksik export veya native filesystem capability iddiasi uygulama acilisini durdurur.

## Sonraki Dilim

Bu foundation generated binding'i uretmez. Siradaki is:

1. Rust exportlarini wasm-bindgen ile generated JS/WASM artefaktina baglamak.
2. CI icinde deterministik binding build adimi eklemek.
3. `apps/web/public/wasm` veya build pipeline tarafinda uretilen artefakti runtime URL'ine koymak.
4. Gercek generated module ile browser smoke testi eklemek.
