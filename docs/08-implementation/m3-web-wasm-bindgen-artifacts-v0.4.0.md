# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-wasm-bindgen-artifacts-v0.4.0.md
# 📌 Amac: M3 Web icin wasm-bindgen generated JS/WASM artifact pipeline kapsam ve sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.1.0
# Aciklama: Rust wasm32 cdylib exportlari, pinned wasm-bindgen CLI, generated browser artifact ve CI upload kontratini dokumante eder
# Bagimli Oldugu Katman: Tool | Service | Config | CI

# M3 Web wasm-bindgen Artifacts v0.4.0

## Durum

Bu dilim compile-verified Core ve runtime loader arasindaki eksik generated artifact halkasini kurar.

## Rust export siniri

`turkuaz-office-core` hem `rlib` hem `cdylib` uretir. `wasm-bindgen` bagimliligi yalnizca `wasm32` hedefinde etkinlesir.

Generated JavaScript exportlari Tool katmanindaki `web_wasm_exports.rs` adaptorunden gelir:

- `web_core_abi_version`
- `web_core_document_schema_version`
- `web_core_bridge_kind`
- `web_core_native_file_system_access`

Business capability degerleri export katmaninda tekrar uretilmez; mevcut `web_core_service` fonksiyonlarina delege edilir.

## Deterministik build

`tools/build-web-wasm.sh`:

1. `wasm-bindgen 0.2.129` CLI surumunu dogrular.
2. Core crate'i release `wasm32-unknown-unknown` hedefinde derler.
3. `--target web` generated JS/WASM dosyalarini `apps/web/public/wasm` altinda uretir.
4. JS ve WASM dosyalarinin bos olmadigini dogrular.
5. Dört zorunlu capability exportunun generated JS glue icinde oldugunu dogrular.

Generated dosyalar source control'e alinmaz.

## CI

`workspace-ci` icinde `Web WASM artifact` job'u pinned CLI kurar, build scriptini calistirir ve `turkuaz-office-web-wasm` adiyla GitHub Actions artifact'i yukler.

## Sonraki dilim

Sonraki adim generated artifact'i Web build/smoke test akisi icinde gercek runtime loader ile yuklemek ve fallback yerine `rust-wasm` capability sonucunu browser seviyesinde dogrulamaktir.
