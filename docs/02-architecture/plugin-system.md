# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/plugin-system.md
# 📌 Amac: Gelecekteki eklenti sisteminin guvenli sinirlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Plugin contract, capability, permission ve public ABI erteleme kararini ozetler

Bagimli Oldugu Katman: Documentation

# Plugin Sistemi

## Hedef

Turkuaz Office cekirdegi degistirilmeden yeni command, importer, exporter, panel veya otomasyon eklenebilmelidir.

## Plugin API ilkeleri

- Versioned capability manifest.
- Explicit permission.
- Core repository'ye dogrudan erisim yok.
- Public command API uzerinden mutation.
- Platform yetenegi capability bazli.
- Web ve Mobile'da desteklenmeyen capability acikca bildirilmeli.
- Network ve filesystem default-deny.

Detay contract: `docs/02-architecture/plugin-api-contract.md`.

## Guvenlik

Native plugin yukleme ilk surumde acilmayacaktir. Once internal plugin contract stabil hale getirilecek. Dis plugin modeli guvenlik incelemesi, sandbox ve imza/paket karari sonrasinda acilir.
