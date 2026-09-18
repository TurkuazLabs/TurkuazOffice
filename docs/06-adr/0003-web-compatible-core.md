# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0003-web-compatible-core.md
# 📌 Amac: Kalici mimari karari ve gerekcesini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Kalici mimari karari ve gerekcesini kaydeder

Bagimli Oldugu Katman: Documentation

# ADR-0003: Web Uyumlu Core

Status: Accepted

## Context

Turkuaz Office uzun omurlu ve cok platformlu bir urun olacagi icin bu karar ileride degistirilmesi maliyetli bir siniri etkiler.

## Decision

Core native filesystem, window veya OS API import etmez. Browser uyumu icin dis dunya Tool/Repository kontratlarina itilir. Uygun pure core bolumleri WASM hedefleyebilir.

## Consequences

Karara aykiri implementasyon yeni ADR kabul edilmeden merge edilmez.
