# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/docx-minimum-test-matrix.md
# 📌 Amac: M1 DOCX minimum profile import/export compatibility ve guvenlik test kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Supported, degraded ve rejected DOCX semantiklerini regression bariyeri olarak sabitler

Bagimli Oldugu Katman: Documentation

# DOCX Minimum Test Matrix

## Supported

- Paragraph text.
- Run bold.
- Run italic.
- Run underline.
- Run font family.
- Run font size half-points.
- Paragraph left/center/right/justify alignment.
- Primary section page width/height.
- Primary section top/right/bottom/left margins.
- Tab ve line break run contenti.

## Importta degraded ama gorunur

- Table yapisi kaybolur; cell paragraph text'i salvage edilir ve compatibility report Table tasir.
- Hyperlink target kaybolur; gorunen text korunur ve compatibility report Hyperlink tasir.
- Image, numbering, header/footer, comments, tracked changes ve fields compatibility report'a eklenir.

## Exportta strict reject

- Birden fazla section.
- Table veya image block.
- Canonical asset registry.
- Canonical font limitleri disinda import style.

Sessiz veri kaybi ile DOCX export yapilmaz.

## Package security

- Max package bytes.
- Max archive entry count.
- Max single entry bytes.
- ZIP traversal ve backslash path reddi.
- Yalniz Stored ve Deflate compression.
- document.xml byte, XML depth ve node count limitleri.
- DTD/DOCTYPE reddi.

## Round-trip regression

Canonical Writer -> DOCX -> canonical Writer testi text, typography, alignment ve page geometry semantigini dogrular.

## CI siniri

Hosted runner step baslamadan failure olursa compiler-backed DOCX sonucu basarili sayilmaz.
