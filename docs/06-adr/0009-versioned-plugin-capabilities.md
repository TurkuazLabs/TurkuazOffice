# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0009-versioned-plugin-capabilities.md
# 📌 Amac: Versioned Plugin Capabilities mimari kararini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Versioned Plugin Capabilities icin kalici karar, gerekce ve sonuclari ADR olarak tutar

Bagimli Oldugu Katman: Documentation

# ADR 0009 - Versioned Plugin Capabilities

## Status

Accepted

## Context

Genisletilebilirlik istenirken pluginlerin Core repository veya tum filesystem/network erisimi almasi guvenli degildir.

## Decision

Plugin contract api_version + capability + explicit permission ile calisir. Native public ABI M1 kapsaminda acilmaz.

## Consequences

Plugin gelistirme biraz daha kontrollu olur; buna karsilik platformlar arasi guvenlik ve compatibility daha yonetilebilir kalir.
