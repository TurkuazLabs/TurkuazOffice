# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-foundation-v0.4.0.md
# 📌 Amac: M3 Web v0.4.0 ilk foundation diliminin kapsam, mimari sinir ve dogrulama kurallarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.1.0
# Aciklama: Browser shell, metadata index, Core Tool boundary ve CI gate baseline'ini dokumante eder

Bagimli Oldugu Katman: Documentation

# M3 Web Foundation v0.4.0

## Durum

M3 Web milestone gelistirmesi baslamistir. Bu dokuman ilk foundation dilimini tanimlar; M3 tamamlandi anlamina gelmez.

## Mimari akis

Web istemcisi ayni katman kuralini korur:

View -> Controller -> Service -> Repo/Tool -> Core boundary

- View storage veya native API kullanmaz.
- Controller yalnizca Service cagirir.
- Service bootstrap ve ileride belge session is kurallarini yonetir.
- Repository bu dilimde yalniz browser document metadata index adapteridir.
- Tool Rust/WASM veya API core erisim siniridir.
- Language gorunur metinleri merkezi olarak tasir.
- Native filesystem web istemcisinin kontrati degildir.

## Bu dilimde tamamlananlar

- SolidJS + TypeScript + Vite web application shell.
- Merkezi runtime config.
- Typed WebDocumentIndexEntry ve WebBootstrapViewModel.
- localStorage tabanli metadata/index Repository.
- Index yalniz `id`, `title` ve `revision` saklar; canonical document payload saklamaz.
- Deterministic replace/list/remove davranisi.
- Corrupt metadata girdisinde fail-safe davranis.
- Typed Core capability Tool boundary.
- Thin WebController ve WebBootstrapService.
- Turkce Language paketi.
- Responsive foundation durum View'i.
- Repository regression testleri.
- Ayrik Web frontend CI build/test gate'i.

## Bilerek kapsam disi

Bu ilk dilimde asagidakiler tamamlanmis sayilmaz:

- Rust Core'un gercek WASM export/binding katmani.
- IndexedDB tabanli canonical document persistence adapteri.
- TKO/DOCX/XLSX browser import-export.
- Service Worker veya Cache Storage offline cache.
- Writer editor web yuzeyi.
- Sheet grid web yuzeyi.
- Cloud/API, hesap veya sync.

## Takip eden durum

Foundation sonrasinda compile-verified Rust WASM Core, pinned wasm-bindgen generated artifact pipeline'i ve browser runtime loader tamamlanmistir.

Canonical Core document payload persistence'i `m3-web-indexeddb-persistence-v0.4.0.md` diliminde IndexedDB adapterine tasinmistir. localStorage yalniz metadata indexidir.

M3 kalan urun kapsami browser import/export, offline cache boundary ve Writer/Sheet web read-model entegrasyonudur.
