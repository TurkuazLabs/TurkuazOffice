# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0008-macros-disabled-by-default.md
# 📌 Amac: Macros Disabled By Default mimari kararini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Macros Disabled By Default icin kalici karar, gerekce ve sonuclari ADR olarak tutar

Bagimli Oldugu Katman: Documentation

# ADR 0008 - Macros Disabled By Default

## Status

Accepted

## Context

Office dosyalari macro ve embedded executable content tasiyabilir. Basit gunluk office hedefi macro runtime gerektirmez.

## Decision

Macro execution varsayilan olarak ve mevcut roadmap boyunca kapali kalir. Import macro metadata gorebilir fakat calistirmaz.

## Consequences

Guvenlik yuzeyi kuculur. Macro isteyen ileri uyumluluk ayri threat model ve ADR gerektirir.
