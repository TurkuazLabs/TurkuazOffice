# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/m3-web-wasm-browser-smoke-test-matrix.md
# 📌 Amac: Generated WASM browser runtime smoke kalite kontrollerini listeler
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.1.0
# Aciklama: Production build, Chromium runtime, rust-wasm bridge ve console-error dogrulamalarini tanimlar
# Bagimli Oldugu Katman: Tool | View | CI

# M3 Web WASM Browser Smoke Test Matrix

| Alan | Dogrulama | Beklenen |
| --- | --- | --- |
| Browser | Chromium headless | Basarili |
| Build | Vite production build | Basarili |
| WASM loader | Generated module import/init | Basarili |
| Bridge | Core bridge read-model | rust-wasm |
| ABI | Core ABI | 1 |
| Native FS | Browser capability | Kapali |
| Console | Browser console errors | Yok |
| Fallback | browser-contract | Kullanilmiyor |
