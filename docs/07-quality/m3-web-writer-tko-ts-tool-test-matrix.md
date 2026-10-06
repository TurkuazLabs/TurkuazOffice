# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/m3-web-writer-tko-ts-tool-test-matrix.md
# 📌 Amac: M3 Web Writer TKO typed TypeScript Tool kalite ve regression kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.4.0
# Aciklama: Inspect mapping, fail-closed JSON validation, byte izolasyonu ve aggregate runtime loader kontratini listeler

Bagimli Oldugu Katman: Tool -> View -> Config

# M3 Web Writer TKO TypeScript Tool Test Matrix

| Alan | Otomatik kontrol | Beklenen |
| --- | --- | --- |
| Inspect mapping | Vitest | Rust snake_case summary typed camelCase Web modeline kayipsiz map edilir |
| Invalid JSON | Vitest | Parse edilemeyen cevap stable config hatasi ile reddedilir |
| Numeric safety | Vitest | Unsafe veya negatif sayisal metadata reddedilir |
| Re-encode delegation | Vitest | Byte payload ve app version generated exporta aynen iletilir |
| Byte isolation | Vitest | Generated `Uint8Array` ust katmana ayni mutable referansla sizmaz |
| Aggregate loader | Vitest | Core + Writer TKO exportlari birlikte dogrulanir |
| Browser fallback | Vitest | Binding yoksa Core fallback calisir ve Writer TKO Tool null kalir |
| Missing Writer export | Vitest | Eksik TKO exportu fail-closed module rejection uretir |
| Web build | CI | TypeScript typecheck ve Vite build yesil kalir |
| Web unit tests | CI | Yeni Tool ve mevcut Web regression suite yesil kalir |
| Static contract | verify-project.sh | Tool, docs, runtime state ve roadmap kontrati zorunlu kalir |

## Basari kriteri

- TypeScript tarafinda TKO codec yeniden yazilmaz.
- Generated WASM exportlari Service/View tarafina raw olarak sizmaz.
- Browser fallback native TKO codec capability iddia etmez.
- Import/export View aksiyonlari bu dilimde acilmaz.
