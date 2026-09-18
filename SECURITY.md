# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/SECURITY.md
# 📌 Amac: Turkuaz Office guvenlik bildirim ve guvenli gelistirme politikasini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Security issue disclosure, belge input guvenligi ve secret handling kurallarini toplar

Bagimli Oldugu Katman: Documentation

# Security Policy

## Vulnerability disclosure

Public issue icinde exploit detayi veya kullanici belgesi paylasilmamalidir. TurkuazLabs public security contact kanali stable public release oncesi repository metadata icinde ilan edilecektir.

## Scope

- Malicious document parsing.
- Path traversal/archive extraction.
- Plugin permission bypass.
- Arbitrary code execution.
- Cloud auth ve tenant isolation gelecekte scope'a girer.

## Secrets

Secret, token, certificate veya signing key repository'ye commit edilmez. CI secret store kullanilir.

## Macro

Macro execution desteklenmez ve varsayilan olarak kapali tutulur.
