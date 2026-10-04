# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/m3-web-wasm-runtime-test-matrix.md
# 📌 Amac: M3 generated WASM binding ve runtime loader kalite gate'lerini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.1.0
# Aciklama: Binding crate, wasm-pack generation, runtime Tool, fail-closed bootstrap ve CI kapsamlarini listeler

Bagimli Oldugu Katman: Documentation

# M3 Web WASM Runtime Test Matrix

| Alan | Otomatik kontrol |
| --- | --- |
| Binding crate native compile | workspace `cargo check --all-targets` |
| Binding delegation | `turkuaz-office-web-wasm` Rust unit testi |
| Pure Core wasm32 compile | `WASM Core Check` |
| Generated binding | `npm run wasm:build` |
| Generated TS import | `npm run build:web` |
| Runtime initializer ordering | `web-wasm-runtime.tool.test.ts` |
| Runtime init failure | silent fallback olmadigini dogrulayan Tool testi |
| Core ABI mapping | `web-core-tool.test.ts` |
| Native filesystem boundary | fail-closed `WasmCoreTool` testi |
| Web production bundle | Vite production build |
| Static contract | `tools/verify-project.sh` |

## Basari kriteri

- Generated `.wasm` ve JS glue build sirasinda sifirdan uretilir.
- Production composition generated Rust/WASM runtime'i kullanir.
- Generated artifact Git'e commit edilmez.
- Runtime init hatasi browser-contract fallback ile gizlenmez.
- Core capability degerleri TypeScript tarafinda yeniden business logic olarak tanimlanmaz.
