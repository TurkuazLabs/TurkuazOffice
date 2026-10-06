# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-wasm-bindgen-artifacts-v0.4.0.md
# 📌 Amac: M3 Web icin wasm-bindgen generated JS/WASM artifact pipeline kapsam ve sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.1.2
# Aciklama: Rust wasm32 cdylib exportlari, pinned wasm-bindgen CLI, generated browser artifact ve CI upload kontratini dokumante eder
# Bagimli Oldugu Katman: Tool | Service | Config | CI

# M3 Web wasm-bindgen Artifacts v0.4.0

## Durum

Bu dilim compile-verified Rust Web bridge ile runtime loader arasindaki generated artifact halkasini kurar. Ilk Core-only artifact daha sonra aggregate Core + Writer TKO bridge artifactina genisletilmistir.

## Rust export siniri

`turkuaz-office-web-bridge` hem `rlib` hem `cdylib` uretir ve `turkuaz-office-core` + `turkuaz-office-writer` crate'lerini aggregate eder. Core'un standalone wasm-bindgen Tool'u `web-wasm-exports` feature'i ile gate edilir; aggregate bridge Core dependency'sinde bu feature acilmaz.

Generated JavaScript exportlari aggregate bridge Tool katmanindaki `web_wasm_exports.rs` adaptorunden gelir:

- `web_core_abi_version`
- `web_core_document_schema_version`
- `web_core_bridge_kind`
- `web_core_native_file_system_access`
- `web_writer_tko_inspect`
- `web_writer_tko_reencode`

Core capability degerleri export katmaninda tekrar uretilmez; mevcut `web_core_service` fonksiyonlarina delege edilir. Writer TKO exportlari mevcut `TkoPackageService` codec semantigini yeniden kullanir.

## Deterministik build

`tools/build-web-wasm.sh`:

1. `wasm-bindgen 0.2.129` CLI surumunu dogrular.
2. `turkuaz-office-web-bridge` crate'ini release `wasm32-unknown-unknown` hedefinde derler.
3. `--target web` generated JS/WASM dosyalarini `apps/web/public/wasm` altinda `turkuaz_office_web_bridge` adi ile uretir.
4. JS ve WASM dosyalarinin bos olmadigini dogrular.
5. Dört Core capability + iki Writer TKO exportunun generated JS glue icinde oldugunu dogrular.

Generated dosyalar source control'e alinmaz.

## CI

`workspace-ci` icinde `Web WASM artifact` job'u pinned CLI kurar, build scriptini calistirir ve `turkuaz-office-web-wasm` adiyla GitHub Actions artifact'i yukler.

## Takip eden durum

Browser runtime loader generated modulu Tool katmaninda yukleyecek sekilde tamamlanmistir. Canonical Core document payload persistence de IndexedDB adapteri ile aktif hale gelmistir.

M3 kalan kapsam Writer TKO generated exportlarini typed TypeScript Tool'a baglama, gercek browser import/export urun akisi, offline cache boundary ve Writer/Sheet web read-model entegrasyonudur.
