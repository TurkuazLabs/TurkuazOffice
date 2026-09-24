# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-file-associations-v0.2.0.md
# 📌 Amac: M1 native TKO file association implementation kapsam ve platform sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Tauri bundle association, startup argument Tool/Service ve frontend cold-start open akisini tanimlar

Bagimli Oldugu Katman: Documentation

# M1 File Associations

## Bundle

Tauri bundle.fileAssociations native .tko extension'ini Turkuaz Office ile esler.

MIME:

application/x-turkuaz-office

Windows release target NSIS'tir.

Linux release targets AppImage + DEB olarak korunur. File association installer entegrasyonu DEB hedefinde beklenir; portable AppImage icin association kaydi garanti edilmez.

## Startup architecture

Operating system -> process argv -> StartupArgumentsTool -> StartupFileService -> WriterDesktopService -> Tauri Controller -> TauriWriterTool -> WriterSessionService -> normal native Open pipeline.

Controller business logic tasimaz.

StartupFileService yalniz existing .tko path kabul eder ve LocalFileTool canonicalization kullanir.

## Recovery precedence

Cold start sirasinda Recovery listesi once kontrol edilir.

Recovery snapshot varsa mevcut Recover / Compare / Discard UX korunur ve startup associated file hemen acilmaz.

Recovery yoksa pending startup file one-shot IPC ile alinir ve normal Open pipeline kullanilir.

## Native Open reuse

Association acilisi ayri storage/parser yolu yaratmaz.

WriterStorageService, TKO package validation, file lock, external-change baseline, Recent Files record ve read-only fallback mevcut native Open davranisini yeniden kullanir.

## M1 siniri

Bu faz cold-start association profilidir.

Calisan uygulamaya ikinci OS file-open istegini tek instance uzerinden tasimak icin single-instance event handoff gerekir. Bu davranis M1 minimumunda uygulanmaz.
