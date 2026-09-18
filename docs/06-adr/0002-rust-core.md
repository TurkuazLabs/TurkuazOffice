# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0002-rust-core.md
# 📌 Amac: Kalici mimari karari ve gerekcesini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Kalici mimari karari ve gerekcesini kaydeder

Bagimli Oldugu Katman: Documentation

# ADR-0002: Rust Core

Status: Accepted

## Context

Turkuaz Office uzun omurlu ve cok platformlu bir urun olacagi icin bu karar ileride degistirilmesi maliyetli bir siniri etkiler.

## Decision

Platformdan bagimsiz document ve service cekirdeginin ana dili Rust olarak secilir. Sebep: native performans, memory safety, desktop/mobile uyumu ve WASM hedefi.

## Consequences

Karara aykiri implementasyon yeni ADR kabul edilmeden merge edilmez.
