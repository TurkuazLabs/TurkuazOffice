# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-local-open-save-v0.2.0.md
# 📌 Amac: M1 Writer local Open/Save ve TKO v1 byte serializer implementation sonucunu belgeler
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Rust package adapteri, Desktop storage, native dialog, dirty baseline ve guvenlik sinirlarini kaydeder

Bagimli Oldugu Katman: Documentation

# M1 Local Open Save v0.2.0

## Tamamlanan kapsam

### Writer crate

- TKO v1 disk DTO katmani.
- `manifest.yml` serializer/deserializer.
- `content/writer.yml` serializer/deserializer.
- ZIP `Stored` package Tool.
- Entry/path/size/compression allowlist kontrolleri.
- TKO manifest/content profile validation.
- Current-schema-only save guard.
- Domain modelde serde bagimliligi olmadan round-trip.

### Desktop Rust

- `WriterStorageService`.
- `LocalFileTool`.
- Safe-replace local save ve relative-path parent normalization.
- TKO extension validation/normalization.
- Open/Save Tauri commands.
- Stabil storage error DTO mapping.

### Desktop frontend

- Native Tauri Open/Save dialog Tool.
- Unsaved-changes confirm dialog.
- File path + saved revision session baseline.
- Dirty state.
- New/Open discard guard.
- Ctrl/Cmd+O, Ctrl/Cmd+S, Ctrl/Cmd+N file operation flow.
- Ribbon Open/Save commandlari.
- Statusbar saved/unsaved durumu ve aktif file path.

## TKO v1 package

```text
sample.tko
|-- manifest.yml
`-- content/
    `-- writer.yml
```

Bu profile asset veya recovery entry eklenmez. Bunlar implementation hazir oldugunda ayri ADR ile allowlist'e girer.

## Dependency karari

Rust workspace MSRV 1.85 korunur. ZIP icin MSRV uyumlu `zip 6.0.0` baseline kullanilir. YAML compatibility adapteri icin `noyalib 0.0.5` `compat-serde-yaml` feature ile pinlenir. Tauri native dialogs icin Rust ve frontend dialog plugin 2.7.3 kullanilir.

## Bilinen sinirlar

- Autosave/recovery henuz yoktur.
- External file change detection henuz yoktur.
- Recent files henuz yoktur.
- File association henuz yoktur.
- Asset payload package icinde henuz desteklenmez.
- M1 stable cikisina kadar TKO v1 public long-term compatibility garantisi provisional'dir.
