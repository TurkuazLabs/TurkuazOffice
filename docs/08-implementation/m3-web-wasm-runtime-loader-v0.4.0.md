# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-wasm-runtime-loader-v0.4.0.md
# 📌 Amac: M3 Web WASM runtime loader foundation kararlarini ve fail-safe davranisini dokumante eder
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.1.2
# Aciklama: Generated wasm-bindgen modulunun Tool katmaninda yuklenmesi, fallback ve fail-closed kontratini tanimlar
# Bagimli Oldugu Katman: Tool | Service | Config

# M3 Web WASM Runtime Loader v0.4.0

## Kapsam

Bu dilim, compile-verified Rust WASM Core yuzeyi ile browser composition root arasina gercek bir runtime yukleme siniri ekler.

- Generated aggregate binding yolu merkezi config ile `/wasm/turkuaz_office_web_bridge.js` olarak tanimlanir.
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

Pinned wasm-bindgen generated aggregate JS/WASM artifact pipeline'i `tools/build-web-wasm.sh` ve `Web WASM artifact` CI job'u ile aktiftir. Runtime loader Core capability exportlarini ayni Tool siniri arkasinda yukler; aggregate modul ayni zamanda Writer TKO inspect/re-encode exportlarini tasir.

## Sonraki Dilim

Canonical Core document payload persistence ve browser byte-transfer foundation tamamlanmistir. Bundan sonraki M3 sirasi aggregate Writer TKO exportlarini typed TypeScript Tool'a baglama, gercek browser import/export, offline cache boundary ve Writer/Sheet web read-model entegrasyonudur.
