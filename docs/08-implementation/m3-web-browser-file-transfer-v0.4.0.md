# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-browser-file-transfer-v0.4.0.md
# 📌 Amac: M3 Web browser file-transfer foundation diliminin kapsam ve mimari sinirlarini belgeler
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.4.0
# Aciklama: Browser file picker ve Blob download byte tasimasini Tool katmaninda izole eder; TKO codec sinirini Rust domain tarafinda tutar
# Bagimli Oldugu Katman: Controller -> Service -> Tool -> Config

# M3 Web Browser File Transfer v0.4.0

## Kapsam

Bu dilim browser import/export urun akisinin platforma ozel byte transport temelini kurar.

- Import ingress HTML file input uzerinden Tool katmaninda yapilir.
- Export egress Blob + object URL + download anchor uzerinden Tool katmaninda yapilir.
- Native filesystem capability acilmaz.
- .tko extension ve MIME profili merkezi Web config icindedir.
- Browser ingress limiti 16 MiB'dir ve mevcut Writer TKO package limitiyle ayni tutulur.
- Boyut kontrolu dosya byte'lari okunmadan once uygulanir.
- Controller yalniz Service metodlarina delege eder.

## Mimari akis

Import byte transport:

View -> WebController -> WebImportExportService -> BrowserFileTransferTool -> browser file input

Export byte transport:

View -> WebController -> WebImportExportService -> BrowserFileTransferTool -> Blob/object URL/download

Bu dilimde View aksiyonu bilerek eklenmez. TKO codec hazir olmadan kullaniciya dosyanin basariyla import/export edildigini gosteren sahte akisa izin verilmez.

## Format siniri

BrowserFileTransferTool yalniz byte tasir. ZIP/YAML parse etmez ve canonical model uretmez.

WebImportExportService:

- .tko accept profilini Tool'a verir
- 16 MiB pre-read limitini Tool requestine verir
- secilen dosya adinin .tko uzantisini dogrular
- export adina eksikse .tko uzantisi ekler
- codec capability icin nativeTkoCodecAvailable=false bildirir

TKO v1 encode/decode semantigi mevcut Rust Writer TkoPackageService icinde kalir. Bu mantik TypeScript'e kopyalanmaz.

## Guvenlik

- Browser accept attribute guvenlik siniri degildir; uzanti Service tarafinda tekrar dogrulanir.
- Boyut guardi file bytes okunmadan once uygulanir.
- Temporary anchor click sonrasinda object URL senkron revoke edilmez; WebKit deferred-download uyumu icin merkezi gecikme sonrasinda revoke edilir.
- Temporary input ve anchor DOM'da kalici tutulmaz.
- Native filesystem API veya path erisimi kullanilmaz.

## Sonraki M3 adimi

Siradaki browser import/export dilimi, mevcut Rust TKO domain servisini browser/WASM bridge uzerinden yeniden kullanarak byte payload ile canonical Writer document arasinda gercek codec mapping kurmaktir. Codec tamamlanmadan import/export View aksiyonlari acilmaz.
