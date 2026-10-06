# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-ooxml-metadata-test-matrix.md
# 📌 Amac: XLSX table ve conditional-format metadata kalite kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.9.0
# Aciklama: Package parts, relationships, style mapping, round-trip ve strict reject kontrollerini listeler
# Bagimli Oldugu Katman: Service | Model | Tool | Adapter

# Sheet OOXML Metadata Test Matrix

| Alan | Dogrulama | Beklenen |
| --- | --- | --- |
| Export | Canonical table | tableN.xml + worksheet rel + tableParts |
| Export | Conditional rule | conditionalFormatting/cfRule |
| Export | Conditional style | styles.xml dxfs |
| Export | Content types | table + styles overrides |
| Import | Table relationship | Canonical SheetTable |
| Import | Table range/name | Kayipsiz |
| Import | Number Greater Than | Canonical condition |
| Import | Number Less Than | Canonical condition |
| Import | Number Equals | Canonical condition |
| Import | Text Contains | Canonical condition |
| Import | warning/success/accent fill | Semantic style mapping |
| Import | Unknown dxf fill | Accent fallback |
| Safety | Unsafe table relationship path | InvalidPackage |
| Safety | Unknown conditional rule | UnsupportedConditionalFormat |
| Safety | Multi-range sqref | UnsupportedConditionalFormat |
| Compatibility | Formula cell | Existing strict reject korunur |
| Round trip | Table + conditional metadata | Export -> Import metadata korunur |
