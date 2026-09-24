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


## M1 DOCX minimum

DOCX parser/writer bagimliliklari `turkuaz-office-format-adapters` crate'inde kalir. Writer domain ve UI WordprocessingML veya ZIP kutuphanesi bilmez.

Minimum import/export profili paragraph/run text, B/I/U, font family/size, paragraph alignment ve primary page geometry ile sinirlidir.

Importta canonical modelde temsil edilemeyen yapilar compatibility report ile typed olarak gorunur hale gelir. Exportta minimum profil disindaki canonical yapilar sessizce atilmaz; typed hata ile reddedilir.

DOCX native calisma formati degildir. Desktop akisi Import DOCX / Export DOCX olarak ayrilir; native file session, lock ve recovery semantigi `.tko` uzerinde kalir.


## M1 PDF export minimum

PDF native calisma formati degildir; yalniz export adapteridir.

Canonical WriterDocument, PdfService tarafindan format-specific PdfDocumentModel'e map edilir. PdfWriterTool platform font path veya Desktop state bilmez.

Desktop SystemFontTool requested family + bold/italic face'i cozer ve yalniz font byte + face index bilgisini adaptere verir. Requested family canonical belgede korunur; fallback yalniz render substitution'dir.

M1 PDF profile paragraph/run text, B/I/U, font family/size, paragraph alignment, primary page geometry ve multi-page text flow ile sinirlidir. Table, image, asset registry ve multiple-section structure typed hata ile reddedilir.

Common canonical pagination engine henuz bulunmadigi icin PDF pagination adapter-local baseline'dir. Print Preview ile ayni physical PageSettings kullanilir fakat pixel-perfect pagination esitligi iddia edilmez.
