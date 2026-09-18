# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0007-offline-first-sync-boundary.md
# 📌 Amac: Offline-First Sync Boundary mimari kararini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Offline-First Sync Boundary icin kalici karar, gerekce ve sonuclari ADR olarak tutar

Bagimli Oldugu Katman: Documentation

# ADR 0007 - Offline-First Sync Boundary

## Status

Accepted

## Context

Cloud sonraki milestone olsa da document id ve revision M1 kararlaridir.

## Decision

Local editing cloud olmadan tam calisir. Document ID stabil, revision monotondur. CRDT/OT algoritmasi M7 oncesi secilmez fakat bugunku model bu secimi engellemez.

## Consequences

Core cloud dependency tasimaz. Sync eklenirken local belge modelini yeniden yazma riski azalir.
