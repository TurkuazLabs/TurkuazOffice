# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-pdf-export-v0.2.0.md
# 📌 Amac: M1 PDF export implementation kapsam ve mimari sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Format adapter PDF writer, Desktop system font resolver ve multi-page text export akisini tanimlar

Bagimli Oldugu Katman: Documentation

# M1 PDF Export

## Mimari

Canonical WriterDocument -> PdfService -> PdfDocumentModel -> PdfWriterTool -> PDF bytes.

Desktop tarafinda:

WriterSessionService -> Tauri Tool -> WriterDesktopService -> WriterPdfService -> SystemFontTool + PdfService -> LocalFileTool.

Controller business logic tasimaz.

## Font siniri

SystemFontTool platform font dizinlerini fontdb ile tarar. Requested family, bold ve italic face sorgulanir. Requested family bulunamazsa merkezi fallback family zinciri denenir.

Format adapter platform font path bilmez. Yalniz PdfFontData icindeki bytes + face_index kullanir.

Requested family canonical WriterDocument icinde degistirilmez.

## PDF writer

printpdf external font embedding kullanilir. Her font ParsedFont olarak yuklenir ve document resource'a eklenir. Save sirasinda font subsetting aciktir.

M1 layout baseline:

- canonical physical page geometry.
- canonical margins.
- font metric tabanli character width.
- multi-page text flow.
- left/center/right/justify.
- underline graphics line.
- tab expansion.
- paragraph gap.

## Strict profile

Table, image, asset registry ve multiple section export edilmez. Bu yapilar typed error ile reddedilir.

## Bilinen sinir

Writer icin ortak canonical shaping/pagination engine henuz yoktur. Bu nedenle M1 PDF pagination adapter-local deterministik baseline'dir; Print Preview ile ayni physical page settings kullanir fakat pixel-perfect pagination esitligi iddia edilmez.
