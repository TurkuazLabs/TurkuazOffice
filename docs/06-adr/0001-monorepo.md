# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0001-monorepo.md
# 📌 Amac: Kalici mimari karari ve gerekcesini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Kalici mimari karari ve gerekcesini kaydeder

Bagimli Oldugu Katman: Documentation

# ADR-0001: Monorepo

Status: Accepted

## Context

Turkuaz Office uzun omurlu ve cok platformlu bir urun olacagi icin bu karar ileride degistirilmesi maliyetli bir siniri etkiler.

## Decision

Turkuaz Office Desktop Web Mobile services ve shared core ayni repo icinde tutulur. Sebep: ortak document kontratlarini atomik degistirmek, tek CI ve kolay refactor. Deploy birimleri yine bagimsizdir.

## Consequences

Karara aykiri implementasyon yeni ADR kabul edilmeden merge edilmez.
