# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/autosave-recovery.md
# 📌 Amac: Autosave, crash recovery ve session restore davranislarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Kayip veri riskini azaltan recovery state machine ve retention sinirini tanimlar

Bagimli Oldugu Katman: Documentation

# Autosave ve Recovery

## Ilke

Autosave kullanicinin explicit save islemiyle ayni sey degildir. Kullanici dosyasinin ustune sessizce yazmak yerine recovery snapshot uretilir.

## State

Belge icin en az su bilgiler takip edilir:

- document_id
- source_location
- persisted_revision
- working_revision
- recovery_revision
- last_successful_save
- last_recovery_snapshot

## Crash recovery

Uygulama normal kapanmadiysa sonraki acilista recovery index taranir. Kullanici Recover, Compare veya Discard karari verebilir.

## Retention

Snapshot sayisi ve yas limiti config uzerinden yonetilir. Inline sabitlenmez.

## Privacy

Recovery data belge kadar hassas kabul edilir. Cloud sync acik degilse local recovery dosyasi cloud'a tasinmaz.

## Session restore

Session restore, acik tab listesi ve unsaved state referanslarini tutabilir; belge icerigini ikinci kez farkli bir formatta saklamaz.


## M1 implementation baseline

- Autosave interval: config uzerinden 30000 ms.
- Autosave explicit source `.tko` dosyasini overwrite etmez.
- Snapshot content normal TKO v1 serializer kullanir.
- Metadata ayri YAML sidecar olarak tutulur.
- Her document icin en fazla 5 snapshot saklanir.
- Snapshot maksimum yasi 604800 saniyedir.
- Startup scan bos document yaratmadan once calisir.
- Kullanici Recover, Compare veya Discard karari verir.
- Explicit Save ayni document recovery snapshotlarini temizler.
- IME composition'i bozmayacak sekilde autosave DOM'u zorla flush etmez; canonical Rust snapshot alir.
