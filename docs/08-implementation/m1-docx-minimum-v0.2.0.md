# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-docx-minimum-v0.2.0.md
# 📌 Amac: M1 DOCX minimum profile implementation kapsam ve mimari sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Format adapter crate, safe ZIP/XML Tool ve canonical Writer mapping Service kararlarini tanimlar

Bagimli Oldugu Katman: Documentation

# M1 DOCX Minimum Profile

## Mimari

External DOCX -> DocxArchiveTool -> DocxXmlTool -> DocxService -> canonical WriterDocument.

Export ters yonde canonical WriterDocument -> DocxService -> format-specific model -> DocxXmlTool -> DocxArchiveTool.

Writer domain ve UI DOCX kutuphanesini bilmez.

## Ayrik crate

crates/turkuaz-office-format-adapters format-specific parser/writer bagimliliklarini Writer domain disinda tutar.

## Supported minimum

- Paragraph text.
- B/I/U.
- Font family.
- Font size half-points.
- Paragraph alignment.
- Primary section page size ve margins.
- Tab ve line break.
- Stored/Deflate DOCX package.

## Compatibility report

Import, canonical modelde temsil edilemeyen yapilari sessizce basarili saymaz. Table, image, numbering, hyperlink, header/footer, comments, tracked changes ve fields typed compatibility feature olarak raporlanir.

M1 import salvage politikasi table cell text ve hyperlink gorunen text'ini paragraph/run olarak koruyabilir; structural semantics report'ta kayip olarak kalir.

## Strict export

Canonical belgede table/image, asset registry veya birden fazla section varsa minimum DOCX export typed hata verir. Minimum profil disi veriyi sessizce atmaz.

## Security

DOCX untrusted ZIP/XML olarak ele alinir. Package size, entry count, entry size, path traversal, compression allowlist, XML size/depth/node count ve DOCTYPE bariyerleri uygulanir.

## Dependency

- zip = 6.0.0, deflate feature.
- quick-xml = 0.42.0.

Bu bagimliliklar yalniz format adapter crate'indedir.
