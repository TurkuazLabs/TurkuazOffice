# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/m3-web-indexeddb-persistence-test-matrix.md
# 📌 Amac: M3 Web IndexedDB canonical persistence kalite ve regression kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.4.0
# Aciklama: Canonical payload, metadata index, fallback siniri, bootstrap durumu ve browser build gate'lerini listeler
# Bagimli Oldugu Katman: Repo -> Tool -> Service -> View

# M3 Web IndexedDB Persistence Test Matrix

| Alan | Otomatik kontrol | Beklenen |
| --- | --- | --- |
| Canonical save/get | IndexedDbDocumentRepository unit testi | id/title/text/schemaVersion/revision kaybi olmadan round-trip |
| Metadata index | Repository unit testi | localStorage yalniz id/title/revision tasir |
| Payload isolation | Repository unit testi | document text localStorage metadata icinde bulunmaz |
| Deterministic list | Repository unit testi | canonical kayitlar id bazli deterministic siralanir |
| Metadata repair | Repository unit testi | stale metadata silinir, IndexedDB kayitlari indexe yansir |
| Remove | Repository unit testi | canonical payload ve metadata kaydi birlikte silinir |
| Invalid record | Repository unit testi | current schema disindaki kaydi save/get/list akislarinda reddeder |
| Bootstrap success | WebBootstrapService unit testi | IndexedDB available ve canonical belge sayisi raporlanir |
| Bootstrap failure | WebBootstrapService unit testi | localStorage payload fallback yapmadan unavailable raporlanir |
| Metadata partial failure | Repository unit testi | canonical commit/silme basarili kalir; metadata daha sonra list ile onarilir |
| IndexedDB blocked open | Tool unit testi | blocked terminal hata sayilmaz; sonraki success kabul edilir |
| IndexedDB sync open error | Tool unit testi | rejected open cache temizlenir ve sonraki islem retry eder |
| IndexedDB unexpected close | Tool unit testi | cached connection temizlenir ve sonraki islem yeniden acar |
| Browser API compile | npm run build | DOM IndexedDB tipleri strict TypeScript ile derlenir |
| Web regression | npm test | tum Web unit testleri yesil |
| Static contract | tools/verify-project.sh | aktif IndexedDB ve runtime-loader kontrati dogrulanir |

## Manuel smoke

1. Web uygulamasini browser'da ac.
2. DevTools Application > IndexedDB altinda turkuaz-office-web / documents store'unu dogrula.
3. Canonical kayit eklendiginde localStorage metadata indexinde text payload bulunmadigini dogrula.
4. IndexedDB engellendiginde uygulamanin canonical storage icin Kullanilamiyor gosterdigini ve localStorage'a payload yazmadigini dogrula.
