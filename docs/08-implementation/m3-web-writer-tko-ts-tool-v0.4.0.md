# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-writer-tko-ts-tool-v0.4.0.md
# 📌 Amac: M3 Web Writer TKO generated WASM exportlarinin typed TypeScript Tool adaptasyonunu belgeler
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.4.0
# Aciklama: Aggregate WASM modulu ile browser Service katmani arasinda typed inspect/re-encode Tool sinirini ve fail-closed validation kurallarini tanimlar

Bagimli Oldugu Katman: Tool -> View -> Config

# M3 Web Writer TKO TypeScript Tool v0.4.0

## Karar

Generated `web_writer_tko_inspect` ve `web_writer_tko_reencode` exportlari Service veya View tarafindan dogrudan kullanilmaz.

`WasmWriterTkoTool` generated modulu typed bir Tool kontratina adapte eder. Rust TKO codec semantigi yine `TkoPackageService` icinde kalir; TypeScript ZIP/YAML codec kopyasi olusturmaz.

## Typed inspect siniri

Rust bridge inspect cevabi JSON string olarak gelir. Tool bu cevabi parse eder ve su alanlari dogrular:

- id
- title
- schema_version
- revision
- section_count
- asset_count

Sayisal alanlar negatif olamaz ve JavaScript safe integer sinirinda olmalidir. Bu kontrol, Rust `u64` revision degerinin browser tarafinda sessiz precision kaybina ugramasini engeller.

Gecersiz JSON veya gecersiz alan sekli `WEB_INVALID_TKO_SUMMARY_ERROR` ile fail-closed reddedilir.

## Re-encode siniri

`reencode(bytes, appVersion)` generated WASM exportuna delege edilir. Tool donen `Uint8Array` icin kopya olusturarak WASM/generated binding tarafina ait mutable byte view'in ust katmana sizmasini engeller.

## Runtime loader

Aggregate loader artik Core exportlarina ek olarak Writer TKO exportlarini da zorunlu kontrat olarak dogrular.

WASM module basariyla yuklenirse:

- `tool`: typed Core Tool
- `writerTkoTool`: typed Writer TKO Tool
- `source`: `wasm`

Binding yuklenemezse browser Core fallback korunur ve `writerTkoTool` null kalir. Bu nedenle fallback modunda native TKO codec varmis gibi davranilmaz.

## Bu dilimde kapsam disi

- WebImportExportService ile codec composition
- import edilen TKO belgesini browser session/repository icine alma
- export icin canonical Writer document encode akisi
- View import/export aksiyonlari
- offline cache

## Sonraki M3 adimi

Browser file-transfer Service ile `WebWriterTkoTool` compose edilerek gercek TKO import/export urun akisi kurulacaktir. View aksiyonlari ancak bu composition ve hata kontrati tamamlandiktan sonra acilacaktir.
