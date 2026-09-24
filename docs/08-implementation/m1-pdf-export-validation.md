# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-pdf-export-validation.md
# 📌 Amac: M1 PDF export static, regression ve compiler-backed validation durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: PDF adapter katmanlari, system font boundary ve hosted runner sinirini ayri raporlar

Bagimli Oldugu Katman: Documentation

# M1 PDF Export Validation

## Static contract

- PDF Model, Service ve Tool format-adapters crate icindedir.
- System font lookup Desktop Tool katmanindadir.
- PDF file safe-write Desktop Service -> LocalFileTool zincirindedir.
- Controller yalniz request aktarir.
- Frontend export akisi Service uzerinden native save dialog + Tauri Tool cagirir.
- PDF export native TKO file session/path bilgisini degistirmez.

## Regression

- Font request family/bold/italic varyantlarini korur.
- Asset registry strict reject edilir.
- Desktop PDF output PDF signature kontrolu yapar.
- Desktop export native session isolation kontrolu yapar.

## External API verification

- printpdf 0.12.8 external ParsedFont, PdfFontHandle, PdfPage ve subset save API'si kullanilir.
- fontdb 0.24.0 load_system_fonts, Query ve with_face_data API'si kullanilir.

## Compiler-backed durum

GitHub hosted runner gercek step baslatmadan steps=null failure verirse PDF compile/test sonucu onaylanmis sayilmaz. Basarili iddia icin gercek cargo/frontend step logu gerekir.
