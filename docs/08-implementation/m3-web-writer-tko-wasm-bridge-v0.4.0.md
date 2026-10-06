# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-writer-tko-wasm-bridge-v0.4.0.md
# 📌 Amac: M3 Web Writer TKO Rust/WASM aggregate bridge diliminin mimari ve codec kontratini belgeler
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.4.0
# Aciklama: Core capability exportlari ile mevcut Writer TKO codec semantigini tek browser WASM artifactinda birlestirir
# Bagimli Oldugu Katman: Tool -> Controller -> Service -> Writer Service -> View -> Config

# M3 Web Writer TKO WASM Bridge v0.4.0

## Karar

Core crate Writer'a baglanmaz. Browser icin yeni turkuaz-office-web-bridge crate'i Core + Writer crate'lerine baglanir ve aggregate cdylib/WASM artifact uretir.

Bu dependency yonu:

Web Bridge -> Core
Web Bridge -> Writer -> Core

seklindedir. Core -> Writer ters bagimliligi olusmaz.

## Aggregate artifact

Generated browser artifact:

- JavaScript: /wasm/turkuaz_office_web_bridge.js
- WASM: /wasm/turkuaz_office_web_bridge_bg.wasm
- wasm-bindgen: 0.2.129 pinned
- target: wasm32-unknown-unknown

Aggregate module mevcut Core runtime loader kontratini bozmamak icin su exportlari korur:

- web_core_abi_version
- web_core_document_schema_version
- web_core_bridge_kind
- web_core_native_file_system_access

Yeni Writer TKO exportlari:

- web_writer_tko_inspect
- web_writer_tko_reencode

## Core standalone export feature'i

turkuaz-office-core icindeki eski wasm-bindgen export Tool'u web-wasm-exports feature'i ile gate edilir.

Aggregate bridge Core'u normal rlib dependency olarak kullandiginda bu feature acilmaz. Boylece ayni JavaScript export adlarinin aggregate cdylib icinde duplicate uretilmesi engellenir.

Standalone Core wasm check CI'de web-wasm-exports feature'i ile korunur.

## Writer TKO bridge Service

WebWriterTkoBridgeService mevcut Writer TkoPackageService'i yeniden kullanir.

inspect:

TKO bytes -> Writer TkoPackageService deserialize -> canonical WriterDocument -> WebWriterTkoSummaryView

reencode:

TKO bytes -> Writer TkoPackageService deserialize -> canonical WriterDocument -> Writer TkoPackageService serialize -> TKO bytes

TypeScript tarafinda ZIP veya YAML parser kopyasi yoktur.

## Inspect View

WebWriterTkoSummaryView binary asset payload tasimaz. Yalniz:

- id
- title
- schema_version
- revision
- section_count
- asset_count

alanlarini acar.

Bu read-model canonical WriterDocument yerine gecmez.

## Stable error boundary

Bridge domain hatalarini stable Web code'larina map eder:

- web_tko_package_too_large
- web_tko_migration_required
- web_tko_future_schema
- web_tko_invalid_package
- web_tko_json_serialization_failed

JavaScript Tool katmani Rust enum/debug stringlerine baglanmaz.

## Bu dilimde kapsam disi

- TypeScript WebWriterTkoTool adapteri
- browser file-transfer Service ile codec composition
- Writer document Web session/state
- import/export View aksiyonlari
- Writer rich editor Web UI
- IndexedDB'de rich Writer TKO/session persistence

## Sonraki M3 adimi

Generated aggregate moduledeki Writer TKO exportlarini typed TypeScript Tool'a baglamak, browser file-transfer Service ile compose etmek ve codec hazirlik kontratini false'dan gercek WASM capability'ye tasimaktir.
