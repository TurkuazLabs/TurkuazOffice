# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/README.md
# 📌 Amac: M3 Web browser istemcisinin aktif foundation kapsam ve sinirlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.4.0
# Aciklama: SolidJS browser shell, metadata index ve Core Tool boundary baseline'ini dokumante eder

Bagimli Oldugu Katman: Documentation

# Turkuaz Office Web

M3 Web v0.4.0 gelistirmesi baslamistir.

## Aktif foundation

- SolidJS + TypeScript + Vite browser shell.
- Controller -> Service -> Repo/Tool -> View -> Language katman akisi.
- localStorage tabanli hafif document metadata/index Repository.
- Index yalniz `id`, `title` ve `revision` tutar; canonical belge payload'i localStorage'a yazilmaz.
- Corrupt metadata girdisinde fail-safe empty-list davranisi.
- Rust/WASM entegrasyonunun gelecekte baglanacagi typed Core Tool siniri.
- Native filesystem erisimi kapali.
- Frontend build ve Vitest regression testi CI gate'ine baglidir.

## M3 icinde siradaki adimlar

- Gercek WASM-compatible Rust Core slice.
- IndexedDB tabanli canonical document persistence adapteri.
- Browser import/export adapteri.
- Offline cache boundary.
- Writer/Sheet web read-model entegrasyonu.

Web istemcisi native Tauri API detayina veya local filesystem path semantigine dogrudan baglanmaz.
