# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/04-operations/security.md
# 📌 Amac: Desktop Web Mobile Cloud icin temel guvenlik prensiplerini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Belge gizliligi, plugin, web, cloud, supply-chain ve untrusted input sinirlarini toplar

Bagimli Oldugu Katman: Documentation

# Guvenlik Temeli

## Belge verisi

Belge icerigi hassas kabul edilir. Telemetry acik izin olmadan belge metni tasiyamaz.

## Untrusted input

Tum belge importlari guvenilmeyen input kabul edilir. Ayrintili limitler `untrusted-document-security.md` dosyasindadir.

## Plugin

Dis plugin native code calistirma yetkisi varsayilan olarak kapali tutulur. Permission/capability modeli default-deny'dir.

## Web

Browser istemcisi API token ve belge erisim yetkisini minimum kapsamla kullanir. CSP, origin ve browser storage ayarlari Web milestone'unda explicit konfigure edilir.

## Cloud

Cloud eklendiginde auth, storage encryption, tenant isolation, audit ve token rotation ayri threat model ile ele alinacaktir.

## Supply chain

Dependency sayisi kontrolsuz artirilmaz. Yeni dependency amaci, bakim durumu ve lisansi review edilir. Lockfile ve release metadata korunur.

## Macro

Macro execution varsayilan ve mevcut roadmap kapsaminda kapali tutulur.
