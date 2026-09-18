# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0006-schema-versioning.md
# 📌 Amac: Document Schema Versioning mimari kararini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Document Schema Versioning icin kalici karar, gerekce ve sonuclari ADR olarak tutar

Bagimli Oldugu Katman: Documentation

# ADR 0006 - Document Schema Versioning

## Status

Accepted

## Context

Application version ile belge veri semantigi ayni hizda degismez. Eski belgelerin acilabilmesi migration ister.

## Decision

Her canonical belge schema_version tasir. Current schema Core Config ile tanimlanir. Migration Service zinciri eski schema verilerini current modele tasir.

## Consequences

Schema degisikligi fixture ve migration testi gerektirir. Future schema sessizce overwrite edilmez.
