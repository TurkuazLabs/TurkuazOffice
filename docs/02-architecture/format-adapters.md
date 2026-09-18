# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/format-adapters.md
# 📌 Amac: DOCX XLSX PPTX ODT CSV ve PDF format adaptoru sinirlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: DOCX XLSX PPTX ODT CSV ve PDF format adaptoru sinirlarini tanimlar

Bagimli Oldugu Katman: Documentation

# Format Adapter Mimarisi

## Import

External format -> parser Tool -> mapping Service -> canonical model.

## Export

Canonical model -> mapping Service -> writer Tool -> external format.

## Neden adapter

Writer UI DOCX kutuphanesini bilmez. Sheet UI XLSX kutuphanesini bilmez. Format kutuphanesi degistirildiginde editor mantigi korunur.

## Uyumluluk profili

Her format icin destek seviyesi dokumante edilir:

- Supported
- Partial
- Preserved but not editable
- Unsupported

## Round-trip

Fixture dosya ac -> modele map et -> tekrar export et -> structural comparison yap. Sadece dosyanin acilmasi uyumluluk kabul edilmez.
