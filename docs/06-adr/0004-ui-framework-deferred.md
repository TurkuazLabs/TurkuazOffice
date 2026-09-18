# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0004-ui-framework-deferred.md
# 📌 Amac: Kalici mimari karari ve gerekcesini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Kalici mimari karari ve gerekcesini kaydeder

Bagimli Oldugu Katman: Documentation

# ADR-0004: UI Framework Karari Ertelendi

Status: Accepted

## Context

Turkuaz Office uzun omurlu ve cok platformlu bir urun olacagi icin bu karar ileride degistirilmesi maliyetli bir siniri etkiler.

## Decision

Foundation asamasinda React/Solid gibi framework secimi kilitlenmez. Once Writer editing prototype, accessibility, performance ve Tauri/Web kod paylasimi benchmark edilir. TypeScript UI yonu korunur.

## Consequences

Karara aykiri implementasyon yeni ADR kabul edilmeden merge edilmez.
