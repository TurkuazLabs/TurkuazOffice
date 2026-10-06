# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-wasm-runtime-loader-v0.4.0.md
# 📌 Amac: M3 Web WASM runtime loader foundation kararlarini ve fail-safe davranisini dokumante eder
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.1.1
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

## Guncel durum

Pinned wasm-bindgen generated JS/WASM artifact pipeline'i `tools/build-web-wasm.sh` ve `Web WASM artifact` CI job'u ile aktiftir. Runtime loader generated modulu ayni Tool siniri arkasinda yukler.

## Sonraki Dilim

Canonical Core document payload persistence IndexedDB adapteri ile tamamlanmistir. Bundan sonraki M3 sirasi browser import/export, offline cache boundary ve Writer/Sheet web read-model entegrasyonudur.
