# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/sheet-range-status-aggregates-v0.5.1.md
# 📌 Amac: Turkuaz Sheet coklu range secimi ve durum cubugu aggregate davranisini dokumante eder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.5.1
# Aciklama: Shift rectangular selection ile canonical Rust Count/Sum/Average ozetini Desktop UI'ya baglar
# Bagimli Oldugu Katman: Controller | Service | Repo | Tool | View | Language

# Sheet Range Status Aggregates v0.5.1

## Entegrasyon tabani

Excel/Calc hybrid Sheet UI (#35) main dalindadir. Bu dilim dogrudan main uzerinde yalnizca range selection ve canonical status aggregate davranisini ekler.

## Kapsam

Turkuaz Sheet, Excel ve Calc'taki hizli secim ozeti davranisini canonical Sheet motoruna baglar.

- Normal hucre secimi aktif hucreyi belirler.
- Shift + hucre secimi aktif hucreyi anchor kabul ederek rectangular range olusturur.
- Secili range grid uzerinde ayri vurgu ile gosterilir.
- Durum cubugu Count, Sum ve Average degerlerini gosterir.

## Aggregate semantigi

- Count: secili range icindeki bos olmayan hucre sayisi.
- Numeric Count: sayisal degerlendirmeye katilan hucre sayisi.
- Sum: yalnizca sayisal degerlerin toplami.
- Average: yalnizca sayisal degerlerin ortalamasi.
- Formula hucreleri canonical formula motorunda evaluate edilerek aggregate'e katilir.
- Text ve Boolean hucreler Count'a katilir; Sum/Average'a katilmaz.

## Mimari

Aggregate hesabi frontend View veya Service'te yapilmaz.

Akis:

SheetService -> SheetController -> DesktopService -> Tauri Controller/DTO -> TauriSheetTool -> SheetSessionService -> Repo -> View

Async range summary cevaplari generation guard ile korunur; eski response yeni secimi ezemez.

## UX

Aktif hucre accent border ile, rectangular range ise hafif accent background ile ayrilir.

Normal hucre secimi range state'ini temizler. Yeni belge acildiginda selection range ve aggregate state sifirlanir.
