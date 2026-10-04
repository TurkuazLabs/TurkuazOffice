# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/r2-sheet-desktop-integration.md
# 📌 Amac: R2 Sheet Desktop Integration baseline kapsam, mimari sinir ve dogrulama durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.1.0
# Aciklama: M2 Sheet engine uzerine eklenen Desktop kullanici yuzeyi, session safety ve roadmap sinirlarini dokumante eder

Bagimli Oldugu Katman: Documentation

# R2 Sheet Desktop Integration Baseline

## Durum

R2 baseline tamamlandi ve mevcut Desktop calisma agacinda aktiftir.

Bu faz yeni bir Sheet engine yazmaz. M2 icinde tamamlanan canonical SheetDocument, Formula Engine, CellFormat ve non-mutating row query kabiliyetlerini Tauri 2 + SolidJS Desktop yuzeyine baglar.

## Mimari akis

Sheet Desktop akisinda temel sinir korunur:

View -> Controller -> Service -> Repo/Tool -> Rust backend

- View sadece kullanici girdisini toplar ve Controller cagirir.
- Controller yalnizca Service request siniridir.
- Service session, dirty-state, typed mutation ve async race kurallarini yonetir.
- Repository reactive Desktop read-model state'ini tutar.
- Tool Tauri IPC ve UI/domain koordinat adaptasyonunu kapsuller.
- Formula hesaplama, canonical mutation, format ve row query kurallari Rust Sheet domain/service tarafinda kalir.
- Language metinleri merkezi typed language pack uzerinden gelir.

## Tamamlanan yuzey

- Writer <-> Sheet modul switcher.
- 30x12 gorunur editable grid baseline'i.
- Text, finite number, boolean, formula ve clear mutationlari.
- UI A1 referansi ile zero-based domain koordinati adaptasyonu.
- Secili hucre state'i.
- Formula bari raw canonical degeri gosterir.
- Evaluated value backend Formula Engine sonucundan gelir.
- Bold, italic ve underline.
- General, left, center ve right horizontal alignment.
- Automatic veya 0..12 decimal places.
- Sparse format cache ile hucreler arasi format gorunurlugu.
- Secili kolon icin non-mutating filter/sort.
- Non-empty, text contains, numeric comparison ve boolean filter modlari.
- Ascending/descending deterministic row siralama.

## Session safety

R2 baseline yalnizca gorunur UI eklemez; Desktop session race ve veri kaybi risklerini de kapatir.

- Aktif grid edit draft'i async format/read-model guncellemesinden ayridir.
- Formula bar draft'i async selection refresh sirasinda korunur.
- Dirty Sheet uzerinde Yeni Sheet istegi native discard confirmation ister.
- Yeni Sheet iptal edilirse mevcut query UI state'i korunur.
- Yeni belge kuruldugunda secim, format cache ve query state temizlenir.
- Row query request'leri generation + document identity ile stale response'a karsi korunur.
- Geciken format mutation'i daha yeni hucre secimini geri alamaz.
- Cell mutation tamamlaninca o anda secili hucre yeniden evaluate edilir.
- Kuyrukta bekleyen cell commit kaynak document kimligine baglidir; eski Sheet edit'i yeni belgeye uygulanamaz.
- Numeric query girdisi canonical decimal pattern disindaki hex benzeri formlari kabul etmez.

## Dogrulama

PR #26 ile session safety hardening sonrasi asagidaki release gate'leri basarili tamamlandi:

- Frontend TypeScript typecheck.
- Vite production build.
- Frontend unit/regression testleri.
- Rust project contract.
- cargo check.
- cargo fmt --check.
- cargo clippy.
- Windows workspace test.
- Ubuntu workspace test.
- Windows NSIS bundle.
- Linux DEB bundle.
- Linux AppImage bundle.
- SHA-256 checksum.
- Preview artifact upload.

## R2 kapsam disi

Asagidaki ozellikler mevcut baseline'in parcasi degildir:

- Sheet native file Open/Save/persistence.
- Chart render/editor UI.
- Multi-range selection.
- Spreadsheet clipboard urun akisi.
- Autofill/drag-fill.
- Gelismis formula kutuphanesi.
- Pivot table.
- Gelismis conditional formatting.

Bu maddelerden biri gelistirilecekse once roadmap guncellenir. Kalici mimari karar gerekiyorsa ilgili ADR ayni degisiklik setine eklenir.

## Sonraki milestone

Mevcut roadmap sirasinda siradaki planli urun milestone'u M3 Web v0.4.0'dir.

R2 kapsam disi yeni Sheet ozelligi M3 oncesinde ele alinacaksa roadmap once explicit olarak degistirilir.
