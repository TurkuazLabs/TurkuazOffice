# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/autosave-recovery-test-matrix.md
# 📌 Amac: Writer autosave ve recovery regression senaryolarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Snapshot, startup scan, restore, compare, discard, cleanup ve retention kalite bariyerlerini listeler

Bagimli Oldugu Katman: Test

# Autosave Recovery Test Matrix

## Rust Service

- Dirty document TKO recovery snapshot uretebilir.
- Snapshot listesinde document ve revision metadata korunur.
- Snapshot yeni WriterDesktopService instance'i ile restore edilebilir.
- Compare source TKO mevcutsa disk ve recovery metnini ayri verir.
- Discard package ve YAML metadata sidecar dosyalarini temizler.
- Explicit Save ayni document_id recovery snapshotlarini temizler.
- Invalid snapshot id path traversal olarak kullanilamaz.
- Retention document basina snapshot sayisini ve yas limitini uygular.

## Frontend

- Startup recovery adayi varsa bos document otomatik olusturulmaz.
- Recover recovered document'i dirty baseline ile acar.
- Compare read-only comparison yuzeyi acar.
- Discard son aday silinince yeni belge akisini baslatir.
- Autosave yalniz dirty document icin ve ayni revision tekrar yazilmayacak sekilde calisir.
- Autosave failure aktif belgeyi kapatmaz; recovery error status olarak kalir.
- Explicit New/Open discard karari sonrasi mevcut document recovery temizligi istenir.

## Runtime

- Windows LOCALAPPDATA recovery rootu.
- Linux XDG_STATE_HOME veya HOME fallback recovery rootu.
- Crash sonrasi gercek process restart ile startup recovery paneli.
- Recovery snapshot privacy ve filesystem permission smoke testi.
