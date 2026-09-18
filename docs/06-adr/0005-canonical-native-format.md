# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0005-canonical-native-format.md
# 📌 Amac: Canonical Native Format TKO mimari kararini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Canonical Native Format TKO icin kalici karar, gerekce ve sonuclari ADR olarak tutar

Bagimli Oldugu Katman: Documentation

# ADR 0005 - Canonical Native Format TKO

## Status

Accepted

## Context

Dis office formatlarina gore domain model kurmak Writer, Sheet ve Web hedeflerini birbirine baglar.

## Decision

Turkuaz Office canonical modelini format bagimsiz tutacak ve native paket siniri icin `.tko` uzantisini ayiracaktir. Public binary encoding M1 sonunda freeze edilecektir.

## Consequences

DOCX/XLSX/PPTX/ODF adapter olarak kalir. Native schema migration sorumlulugu bize aittir.
