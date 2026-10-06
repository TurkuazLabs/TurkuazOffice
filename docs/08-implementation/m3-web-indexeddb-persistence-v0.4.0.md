# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-indexeddb-persistence-v0.4.0.md
# 📌 Amac: M3 Web IndexedDB canonical document persistence diliminin mimari ve davranis kontratini belgeler
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.4.0
# Aciklama: Canonical belge payloadinin IndexedDB'de, hafif metadata indexinin localStorage'da tutulmasini ve fallback sinirini tanimlar
# Bagimli Oldugu Katman: Service -> Repo -> Tool -> View -> Language

# M3 Web IndexedDB Persistence v0.4.0

## Kapsam

Bu dilim browser tarafinda canonical Core document payload persistence icin IndexedDB adapterini aktif eder.

Canonical kayit alanlari:

- id
- title
- text
- schemaVersion
- revision

localStorage canonical payload tutmaz. Mevcut localStorage Repository yalniz id/title/revision metadata indexidir.

## Mimari akis

Browser API erisimi Tool katmaninda izole edilir:

main.tsx -> BrowserIndexedDbDocumentTool -> IndexedDbDocumentRepository -> WebBootstrapService -> WebController -> App

Repository:

- canonical kaydi IndexedDB Tool'a yazar
- get/list/remove islemlerini typed kontratla sunar
- yalniz current Core schemaVersion kayitlarini kabul eder
- list sirasinda corrupt/uyumsuz kayitlari canonical listeye almaz
- metadata indexini IndexedDB gercekligiyle best-effort senkronize eder
- metadata yazma hatasi canonical commit veya silmeyi basarisiz gostermez
- canonical payload text alanini localStorage'a yazmaz

## IndexedDB kontrati

Database adi, version, object-store adi ve keyPath merkezi runtime config icindedir.

- database: turkuaz-office-web
- version: 1
- store: documents
- keyPath: id

Upgrade yalniz eksik object store'u olusturur. Blocked open terminal hata sayilmaz; request success/error sonucuna kadar pending kalir. Senkron open hatasi veya beklenmeyen database close sonrasinda connection cache temizlenir ve sonraki islem yeniden deneyebilir.

Canonical belge silme islemi metadata index kaydini best-effort siler; localStorage hatasi canonical IndexedDB sonucunu geri almaz. Sonraki canonical list akisi metadata indexini yeniden uzlastirir.

## Fallback siniri

IndexedDB acilamazsa veya bootstrap count islemi hata verirse canonical payload localStorage'a sessizce dusurulmez.

Bootstrap ViewModel:

- documentStorageKind = indexed-db
- documentStorageAvailable = false
- storedDocumentCount = null

Bu davranis browser storage sorununun veri mimarisini sessizce degistirmesini engeller.

## Bu dilimde kapsam disi

- TKO/DOCX/XLSX browser import/export
- Service Worker veya Cache Storage offline cache
- Writer editor web session persistence entegrasyonu
- Sheet grid web session persistence entegrasyonu
- Cloud sync

## Sonraki M3 adimi

Roadmap sirasi browser import/export, offline cache boundary ve Writer/Sheet web read-model entegrasyonudur.
