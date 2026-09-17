# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/README.md
# 📌 Amac: Turkuaz Office monorepo giris dokumani ve gelistirme yonlendirmesi
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Writer v0.2.0 domain, Desktop editor, storage, recovery, file protection ve font/layout gelisim durumunu tanimlar

Bagimli Oldugu Katman: Documentation

# Turkuaz Office

Turkuaz Office; Windows ve Linux ile baslayan, ileride macOS, Web, Android ve iOS istemcilerini ayni urun ailesinde birlestirmeyi hedefleyen modul tabanli bir ofis platformudur.

## Urun hedefi

Microsoft Office ile ozellik sayisi yarisi yapmak hedef degildir. Hedef; gunluk belge, tablo ve sunum islerini hizli, sade, guvenilir ve genisletilebilir bir urunle karsilamaktir.

## Writer Domain v0.2.0 durumu

Bu paket `v0.2.0` Writer modulunun aktif M1 gelistirme snapshot'idir; son kullaniciya yonelik stable Writer release degildir. Tauri 2 + SolidJS shell, rich-text/IME motoru, logical DOM selection, B/I/U, font family/size, caret typing-style, paragraph alignment, native Open/Save, `.tko` ZIP+YAML serializer, autosave/recovery, external-change/file-lock protection ve twip tabanli font/layout baseline calisma agacinda aktiftir. Clipboard, DOCX, PDF ve print M1 icinde siradaki fazlardir.

Foundation Hardening korunurken ilk urun modulu olan headless Writer Domain aktif hale getirilmistir. Asagidaki mimari kararlar ve Writer kontratlari artik kod ile temsil edilir:

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

Ilk kullanilabilir urun hedefi halen `Desktop + Writer` olarak tanimlidir. Web, Mobile, API ve Collaboration mimari sinirlari korunur; roadmap sirasi disinda implementation acilmaz.

## Monorepo

```text
TurkuazOffice/
|-- apps/
|   |-- desktop/        # Tauri 2 + SolidJS aktif M1 shell
|   |-- web/
|   `-- mobile/
|-- services/
|   |-- api/
|   `-- collaboration/
|-- crates/
|   |-- turkuaz-office-core/
|   `-- turkuaz-office-writer/
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
