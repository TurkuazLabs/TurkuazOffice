# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/system-context.md
# 📌 Amac: Platform bilesenlerinin sistem seviyesinde sorumluluklarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Platform bilesenlerinin sistem seviyesinde sorumluluklarini tanimlar

Bagimli Oldugu Katman: Documentation

# Sistem Baglami

## Ana bilesenler

`Core` belge semantics, command semantics ve is kurallarini tasir.

`Desktop` Windows/Linux/macOS platform yeteneklerini adapte eder.

`Web` browser sandbox icinde calisir; local native varsayim yapmaz.

`Mobile` Android/iOS dokunmatik UX ve native izinlerini adapte eder.

`API` hesap, cloud metadata ve senkronizasyon kontratlarini sunar.

`Collaboration` ortak duzenleme operation akisini yonetir.

## Bagimlilik yonu

```text
View -> Controller -> Service -> Repository / Tool
                          |
                          `-> Domain-like document types
```

Platform kodu Core is kurallarina sahip olamaz. Core platform SDK import edemez.
