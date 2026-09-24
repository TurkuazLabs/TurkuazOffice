# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-recent-files-v0.2.0.md
# 📌 Amac: M1 Recent Files implementation kapsam ve katman sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Desktop state root, YAML metadata Repo, business Service, IPC ve ribbon quick-open akisini tanimlar

Bagimli Oldugu Katman: Documentation

# M1 Recent Files

## Mimari

Frontend:
View -> Controller -> WriterSessionService -> WriterSessionRepository / TauriWriterTool.

Backend:
Controller -> WriterDesktopService -> RecentFilesService -> RecentFilesRepository -> RecentFilesMetadataTool / LocalFileTool.

Controller katmaninda dedup, persistence veya path business logic bulunmaz.

## State root

AppStatePathTool Windows/Linux/macOS local state root'unu tek yerde cozer.

RecoveryPathTool bu ortak root'u yeniden kullanir. Recent metadata ayni root altinda recent-files.yml olarak tutulur.

## Native file scope

Recent listesi yalniz .tko native calisma dosyalarini kabul eder.

DOCX import, DOCX export ve PDF export recent native file listesine eklenmez.

## Record semantigi

Basarili native Open/Save ardindan frontend ayri recordRecentFile IPC cagirir.

Bu ayrim bilincli olarak yapildi: recent metadata yazma hatasi basarili document Open/Save islemini rollback etmez.

## Normalize

Service kayit sirasinda LocalFileTool::canonicalize_file kullanir. Liste okunurken eski kayitlar yeniden canonicalize edilir.

Ayni canonical path tek kayittir. Windows'ta path duplicate karsilastirmasi case-insensitive'dir.

## Retention

Liste en son erisilen siradadir ve en fazla 12 kayit tutar.

Diskte artik bulunmayan .tko kayitlari list okumasinda prune edilir ve metadata temiz haliyle yeniden persist edilir.

## Frontend UX

Startup recent metadata recovery panelinden bagimsiz yuklenir.

Home ribbon ilk 3 kaydi quick-open butonu olarak gosterir. Buton text'i dosya title, tooltip tam path'tir.

Recent dosya acma normal dirty/discard guard ve file protection zincirini kullanir.
