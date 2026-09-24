# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/pdf-export-test-matrix.md
# 📌 Amac: M1 PDF export layout, font, pagination ve strict compatibility test kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: PDF physical page geometry, embedded system font ve unsupported structure bariyerlerini kalite kontrati yapar

Bagimli Oldugu Katman: Documentation

# PDF Export Test Matrix

## Supported minimum

- Paragraph text.
- B/I/U.
- Font family request + Desktop render fallback.
- Font size half-points.
- Left/center/right/justify paragraph alignment.
- Primary section page size ve margins.
- Tab ve explicit line break.
- Multi-page text flow.
- Embedded external font + subset save.

## Strict reject

- Birden fazla canonical section.
- Table block.
- Image block.
- Canonical asset registry.
- Cozulemeyen system font.
- Font icinde bulunmayan glyph.
- Gecersiz page geometry.
- PDF page/output limit asimi.

Sessiz data kaybi ile PDF export yapilmaz.

## Geometry

- Canonical twip -> point: 20 twip = 1 point.
- PDF page size canonical PageSettings kaynaklidir.
- Browser DOM pixel olcumu PDF canonical kaynagi degildir.
- Line height M1 adapter baseline degeridir.
- Common canonical pagination engine henuz yoktur; PDF pagination adapter-local baseline'dir.

## Fonts

- Requested family canonical belgede degismez.
- Desktop SystemFontTool requested family + bold/italic face arar.
- Merkezi fallback family zinciri yalniz render substitution yapar.
- Font byte + face index PDF adapterine verilir.
- Font PDF'e embed edilir ve save sirasinda subset edilir.

## Regression

- PdfService font request testi family/bold/italic semantigini kilitler.
- Unsupported canonical asset strict reject edilir.
- Desktop PDF export testi gercek system font ile PDF signature ve native session isolation kontrol eder.

## CI siniri

Hosted runner step baslamadan failure olursa compiler-backed PDF sonucu basarili sayilmaz.
