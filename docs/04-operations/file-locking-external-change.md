# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/04-operations/file-locking-external-change.md
# 📌 Amac: Ayni dosyanin coklu pencere veya dis uygulama tarafindan degistirilmesi durumunu tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: File lock, fingerprint, read-only ve conflict save davranislarini tarif eder

Bagimli Oldugu Katman: Documentation

# File Locking ve External Change

## Hedef

Kullanici verisini sessiz overwrite etmemek.

## Acilista

Desktop storage adapter kaynak dosya metadata fingerprint'i alir. Platform destekliyorsa advisory lock kullanilir; lock tek guvenlik mekanizmasi sayilmaz.

## Kaydetmeden once

Disk fingerprint acilis/son save fingerprint'i ile karsilastirilir. Degismisse normal overwrite yapilmaz.

## Kullanici secenekleri

- Reload external version.
- Save As.
- Compare.
- Force overwrite: sadece acik kullanici onayi ile.

## Coklu pencere

Ayni Turkuaz Office instance'i ayni canonical path'i iki editor tabinda writable acmamaya calisir. Ikinci gorunum read-only veya mevcut taba gecis sunar.

## Network filesystem

Lock semantics guvenilir olmayabilir. Fingerprint + revision kontrolu her durumda korunur.
