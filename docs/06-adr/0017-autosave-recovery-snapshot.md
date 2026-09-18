# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0017-autosave-recovery-snapshot.md
# 📌 Amac: Writer autosave ve crash recovery snapshot mimarisi kararini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Explicit Save'den bagimsiz TKO snapshot + YAML metadata, retention ve startup recovery kararini sabitler

Bagimli Oldugu Katman: Service -> Tool -> View

# ADR 0017 - Autosave Recovery Snapshot

## Durum

Kabul edildi.

## Karar

Writer autosave, kullanicinin hedef `.tko` dosyasini sessizce overwrite etmez. Dirty canonical document belirli araliklarla local recovery dizinine snapshot olarak yazilir.

Snapshot content normal TKO v1 serializer kullanir. Recovery icin ikinci bir document schema veya ikinci bir serializer olusturulmaz.

Her snapshot icin ayri YAML metadata tutulur:

- snapshot_id
- document_id
- title
- source_path
- persisted_revision
- recovery_revision
- schema_version
- created_at_unix_ms
- created_by_app_version

## Startup davranisi

Uygulama bos belge olusturmadan once recovery index tarar. Aday varsa kullanici Recover, Compare veya Discard karari verir.

Recover snapshot'i canonical Writer Controller'a yukler. Source path varsa korunur; dirty baseline persisted_revision olarak kalir.

Compare recovery metni ile diskteki source TKO metnini read-only olarak yan yana gosterir. Source acilamiyorsa bu durum acikca belirtilir.

Discard snapshot package ve metadata sidecar dosyasini siler.

## Retention

Retention degerleri config'te tutulur. M1 baseline her document icin en fazla 5 snapshot ve en fazla 7 gun yas siniridir.

## Explicit Save

Basarili explicit Save sonrasi ayni document_id icin recovery snapshotlari best-effort temizlenir. Save basarisi recovery cleanup hatasina baglanmaz.

## Guvenlik

Snapshot package normal TKO package limitleri ve schema validation'dan tekrar gecer. Snapshot id yalniz ASCII digit ve tire kabul eder; metadata path olarak kullanici girdisi kullanilmaz.
