# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/README.md
# 📌 Amac: Turkuaz Office monorepo giris dokumani ve gelistirme yonlendirmesi
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.1
# Aciklama: Tamamlanan Writer, Sheet engine ve Sheet Desktop baseline durumunu monorepo girisinde ozetler

Bagimli Oldugu Katman: Documentation

# Turkuaz Office

Turkuaz Office; Windows ve Linux ile baslayan, ileride macOS, Web, Android ve iOS istemcilerini ayni urun ailesinde birlestirmeyi hedefleyen modul tabanli bir ofis platformudur.

## Urun hedefi

Microsoft Office ile ozellik sayisi yarisi yapmak hedef degildir. Hedef; gunluk belge, tablo ve sunum islerini hizli, sade, guvenilir ve genisletilebilir bir urunle karsilamaktir.

## Writer M1 v0.2.0 durumu

Writer M1 feature kapsami tamamlanmistir. Tauri 2 + SolidJS shell, rich-text/IME, typography/ribbon, native Open/Save, autosave/recovery, file protection, font/layout, Clipboard Minimum, Print Preview + Print, DOCX Minimum, PDF Export Minimum, Recent Files, File Associations, Template Foundation, tr-TR/en-US runtime UI ve Keyboard-Only Smoke calisma agacinda aktiftir. v0.3.1 Community Preview hardening, gercek Windows/Linux CI ve bundle artifactlari ile bu kapsami release seviyesinde dogrular.

Foundation Hardening ve tamamlanan Writer M1 kontratlari kod ile temsil edilir:

- Canonical Document Model dosya formatindan bagimsizdir.
- Turkuaz Office native paket uzantisi icin `.tko` calisma karari vardir.
- Her belge `schema_version` ve `revision` tasir.
- Schema migration zinciri Core Service sorumlulugudur.
- DOCX/XLSX/PPTX/ODF import-export formatidir; canonical model degildir.
- Autosave, recovery, file locking ve external-change davranisi platform kontrati olarak tanimlidir.
- Font/layout, clipboard, print, accessibility ve localization ayri kontratlardir.
- Plugin API versioned capability + permission modeli ile sinirlandirilmistir.
- Makrolar varsayilan olarak kapali ve guvenilmeyen belge girdileri limitlidir.
- Cloud gelmeden once offline-first revision/sync sinirlari tanimlanmistir.
- Performans hedefleri olculebilir budget olarak tutulur.
- Writer canonical tree Section -> Block -> Paragraph/TextRun yapisinda calisir.
- Writer mutation yalnizca typed command uzerinden yapilir.
- Selection NodeId + logical Unicode scalar offset tabanlidir.
- Undo/redo ilk baseline olarak snapshot history kullanir ve revision geri sarmaz.
- TKO v1 logical manifest/content profile ve ZIP+YAML byte encoding kod seviyesinde vardir.
- Desktop paragraph editor IME-aware contenteditable surface kullanir; DOM canonical state degildir.
- B/I/U style mutation typed range command ile Rust Writer Core tarafinda uygulanir.
- Paragraph typing mutationlari run stillerini korumak icin minimal Unicode diff uretir.
- Caret B/I/U/font secimi canonical belgeyi degistirmeden Repository typing-style state olarak tutulur.
- Yeni metin girildiginde typing-style `InsertStyledText` commandina donusur.
- Font family ve size CharacterStyle icinde merkezi domain limitleriyle saklanir.
- Paragraph hizalama left/center/right/justify olarak typed paragraph command ile uygulanir.
- Desktop Writer iki katmanli ribbon baseline kullanir; Giris tabinda belge, history, font ve paragraph gruplari vardir.
- Native Open/Save dialoglari Tauri dialog Tool arkasindadir.
- Local `.tko` save hedefi dogrudan truncate edilmez; platforma gore safe-replace uygulanir.
- File path ve saved revision baseline ile dirty document guard uygulanir.
- Dirty document icin source dosyayi overwrite etmeyen TKO recovery snapshotlari uretilir.
- Startup recovery adaylari Recover / Compare / Discard karariyla kullaniciya sunulur.
- Content fingerprint ile diskteki `.tko` degisikligi timestamp bagimsiz algilanir.
- Cooperative sidecar lock alinamayan belge backend seviyesinde read-only acilir.
- Same-path Save dis degisiklik varken explicit kullanici karari olmadan overwrite etmez.
- Reload From Disk / Keep Local Version / Save As file protection akisi aktiftir.
- Primary section PageSettings Tauri read-modeline twip olarak tasinir.
- Desktop page width/height/margin canonical twip degerlerinden 96 CSS px/inch render referansiyla uretilir.
- Zoom %50-%200 session state'tir; belge revision, dirty state ve undo history'yi degistirmez.
- Font fallback render katmaninda cozulur; requested font family canonical belgede korunur.
- Windows/Linux icin Arial/Calibri/Times New Roman/Georgia/Verdana/Courier New fallback profilleri merkezi configte tutulur.

Desktop + Writer M1 feature kapsami tamamlanmistir. M2 Sheet engine ve R2 Sheet Desktop integration baseline'i tamamlanmistir. M3 Web v0.4.0 gelistirmesi browser foundation ve compile-verified Rust WASM Core dilimleriyle devam etmektedir.

## Sheet M2 v0.3.0 durumu

M2 Cell Model, CSV/XLSX minimum, Basic Formula Engine, Format/Filter/Sort ve Basic Charts parcalari aktiftir:

- `turkuaz-office-sheet` workspace crate.
- Sparse `SheetDocument -> Worksheet -> CellAddress/CellValue` model.
- Deterministik `BTreeMap` cell storage.
- Text / finite number / boolean / formula value baseline.
- Simple A1 reference Tool.
- XLSX grid sinirlari: 1048576 satir x 16384 kolon.
- Sparse set/get/clear ve revision no-op kurallari.
- Thin Controller, Repo, Tool, View ve Language katmanlari.
- Basic same-worksheet formula parser/evaluator: numeric literal, A1/absolute-mixed A1, + - * /, parentheses ve unary +/-.
- Formula dependency recursion, cycle/depth/division/non-finite guardlari.
- Empty referenced cell numeric 0 kabul edilir; text/boolean numeric coercion yapilmaz.
- CSV/XLSX minimum profile canonical formulayi sessiz downgrade etmez ve exportta typed UnsupportedFormula ile reddeder.
- Sparse canonical CellFormat metadata: bold/italic/underline/horizontal alignment/decimal places.
- Format mutation revision-aware'dir ve default format sparse mapte tutulmaz.
- Filter/sort query'leri Service katmaninda formula-aware ve non-mutating calisir.
- Tek kolon ascending/descending sort deterministic row-index tie-break kullanir.
- Query range 100000 satir ile sinirlidir.
- Canonical Bar/Line/Pie chart definition modeli ve stable ChartId baseline'i aktiftir.
- Chart kategori kaynagi text, deger kaynagi finite number veya numeric formula olabilir.
- Chart data projection mevcut Formula Engine'i kullanir; render kutuphanesi canonical katmana gomulmez.
- Chart source range maksimum 1000 point ile sinirlidir.

- 100000-cell benchmark profile aktiftir: sparse build, View projection ve descending numeric row query explicit ignored workload olarak olculur.
- Hosted CI varyansi nedeniyle sabit millisecond threshold yoktur; timing ayni donanim/toolchain uzerinde karsilastirilir.

M2 engine feature kapsami tamamlanmistir. R2 Sheet Desktop baseline'i de mevcut calisma agacinda aktiftir:

- Writer <-> Sheet modul gecisi ve 30x12 editable grid.
- Typed text/number/boolean/formula/clear mutation hatti.
- Secili hucre + formula bari + backend evaluated value.
- Bold/italic/underline, horizontal alignment ve decimal places.
- Secili kolon non-mutating filter/sort.
- Aktif edit draft korumasi, dirty New Sheet discard guard ve async stale-response korumalari.

Sheet native Open/Save, chart editor/render UI ve gelismis spreadsheet ozellikleri R2 kapsamina dahil degildir. Bunlar roadmap guncellenmeden eklenmez.

## Community Preview v0.3.1

R1 v0.3.1 tarihsel preview release kapsami:

- Windows: NSIS installer.
- Linux: DEB + AppImage.
- Writer desktop: release candidate.
- Sheet engine: CI ile dogrulanan headless/core katman.
- Sheet desktop UI R1 release gate kapsaminda degildi.
- Preview artifactlari unsigned'dir; stable release etiketi icin platform signing gerekir.
- CI artifactlari SHA-256 checksum ile birlikte uretilir.

R1 sonrasinda tamamlanan R2 ile mevcut `main` dalinda Sheet desktop UI aktiftir: typed edit, formula bar, format, filter/sort ve session-safety yuzeyi vardir. Bu R2 durumu, tarihsel v0.3.1 artifact kapsamiyla karistirilmaz.

## Monorepo

```text
TurkuazOffice/
|-- apps/
|   |-- desktop/        # Tauri 2 + SolidJS aktif Writer + Sheet shell
|   |-- web/            # SolidJS M3 browser foundation
|   `-- mobile/
|-- services/
|   |-- api/
|   `-- collaboration/
|-- crates/
|   |-- turkuaz-office-core/
|   |-- turkuaz-office-writer/
|   |-- turkuaz-office-sheet/
|   `-- turkuaz-office-format-adapters/
|-- config/
|-- docs/
|-- tools/
`-- .github/
```

## Core kontrolu

```powershell
cd E:\Projects\TurkuazOffice
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
```

## Dokumantasyon kurali

Mimariyi etkileyen her degisiklik kod ile ayni pull request icinde ilgili `docs/` dosyasini guncellemelidir. Yeni kalici mimari kararlar `docs/06-adr/` altinda ADR olarak kaydedilir. Bir format, schema, permission veya platform contract degisikligi dokumansiz merge edilmez.

Baslangic noktasi: `docs/README.md`.


## Desktop Shell kontrolu

```powershell
cd E:\Projects\TurkuazOffice\apps\desktop
npm install --no-audit --no-fund
npm run build
npm run tauri:dev
```

Frontend canonical document state tutmaz; Rust Writer Domain tek dogruluk kaynagidir.


## Web Foundation kontrolu

```powershell
cd E:\\Projects\\TurkuazOffice\\apps\\web
npm install --no-audit --no-fund
npm run build
npm test
```

Web View native filesystem kullanmaz. localStorage yalniz document metadata indexidir; canonical Web document payload persistence'i IndexedDB adapteri icin planlidir. `turkuaz-office-core` wasm32 hedefinde CI ile derlenir; runtime generated binding henuz planli asamadadir ve Core erisimi Tool siniri arkasindadir.
