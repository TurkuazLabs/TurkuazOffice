# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0010-writer-logical-offset.md
# 📌 Amac: Writer selection ve command offset semantigini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: UI pixel veya DOM offset yerine canonical logical text offset kullanma kararini belgeler

Bagimli Oldugu Katman: Documentation

# ADR 0010 - Writer Logical Text Offset

## Durum

Accepted for v0.2.0 domain; IME prototype sonrasi review required.

## Karar

Writer domain `TextPosition` icinde paragraph NodeId, run NodeId ve logical offset tasir. v0.2.0 baseline offset birimi Unicode scalar sayisidir.

## Neden

- UTF-8 byte index public editor kontrati olmamali.
- DOM/JavaScript UTF-16 index'i Core'a sizmamali.
- Desktop, Web, Mobile ve Collaboration ayni logical contract'i kullanabilmeli.

## Adapter sorumlulugu

View adapter DOM/IME platform offsetini Writer logical offsetine cevirir. Core pixel koordinati veya DOM Range saklamaz.

## Review kosulu

Combining character, emoji ZWJ ve IME composition fixturelari UI prototype asamasinda test edilir. Gerekirse grapheme-aware internal representation yeni ADR ile kabul edilir.
