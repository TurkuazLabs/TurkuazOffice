# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/storage.md
# 📌 Amac: Local ve cloud storage abstraction stratejisini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.1
# Aciklama: Local safe-replace implementation ile cloud/browser storage abstraction stratejisini tanimlar

Bagimli Oldugu Katman: Documentation

# Storage Mimarisi

## Ilke

Core hicbir zaman `C:\\` veya `/home` varsayimi yapmaz. Belge storage bir Repository/Tool kontrati arkasinda kalir.

## Adaptor durumu

- Desktop `LocalFileTool` + `WriterStorageService`: M1 aktif.
- BrowserDocumentIndexRepository: M3 foundation'da localStorage metadata indexi olarak aktif; canonical document payload saklamaz.
- BrowserIndexedDbRepository: canonical Web document persistence icin planli.
- MobileAppStorageRepository: planli.
- CloudDocumentRepository: planli.

## Local-first

Cloud hesabi olmayan kullanici tam local belge akisina sahip olmalidir.

## Atomic save

Yerel kayitta hedef dosyanin ustune dogrudan yazilmaz. Temp file ayni klasorde olusturulur ve sync edilir. Unix rename replace kullanir. Windows mevcut hedefi backup sidecar'a tasiyip yeni temp file'i hedefe yerlestirir; hata durumunda rollback dener. Relative file path current directory parent olarak normalize edilir. Platform farklari `LocalFileTool` icinde kalir.

## Recovery

Autosave ana dosyanin yerine gecmez. Recovery journal ayri tutulur ve basarili explicit save sonrasinda temizlenir.


## Recent Files

Recent Files canonical document storage degildir; Desktop local state metadata'sidir.

Platform state root AppStatePathTool tarafindan cozulur. recent-files.yml persistence Repo/Tool katmanlarinda kalir.

Liste yalniz native .tko calisma dosyalarini kabul eder. DOCX/PDF foreign-format import/export hedefleri recent native document listesine girmez.

Recent path kaydi canonicalize edilir, duplicate tek kayit olur, en son erisilen basa tasinir ve missing files list okumasinda prune edilir.

Recent metadata yazma hatasi basarili native document Open/Save islemini rollback etmez.
