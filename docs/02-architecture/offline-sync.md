# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/offline-sync.md
# 📌 Amac: Cloud gelmeden once offline-first revision ve sync protocol sinirlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Device, revision, operation id ve conflict stratejisinin gelecekteki collaboration ile uyumunu korur

Bagimli Oldugu Katman: Documentation

# Offline-First Sync Contract

## Bugun neden tanimliyoruz

Cloud M4'te gelecektir. Ancak Document ID ve revision semantigi M1'de olusacagi icin senkronizasyonu imkansiz hale getirecek kararlar bugun alinmamalidir.

## Kimlikler

- Document ID global olarak stabil kalir.
- Device ID platform adapterindan gelir.
- Revision monoton local document state'i tanimlar.
- Sync operation ileride ayri operation_id tasir.

## Conflict

M1 local save conflict'i external-change policy ile cozer. M4 cloud conflict'i ayni revision contract uzerine kurulur. M7 collaboration geldiginde CRDT veya OT secimi ayri ADR ile yapilir; bugun bir algoritma zorla secilmez.

## Offline

Kullanici cloud hesabi olmadan tum local editor fonksiyonlarini kullanabilmelidir. Cloud feature core editing icin zorunlu dependency degildir.

## Tombstone

Cloud delete ileride hard delete yerine versioned tombstone gerektirebilir. Bu nedenle repository contract permanent delete varsayimi yapmaz.
