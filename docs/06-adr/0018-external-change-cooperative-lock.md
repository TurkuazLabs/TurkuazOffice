# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0018-external-change-cooperative-lock.md
# 📌 Amac: Desktop Writer external-change ve cooperative file-lock kararini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Content fingerprint, read-only fallback, backend mutation guard ve explicit overwrite kararini sabitler

Bagimli Oldugu Katman: Service -> Tool -> View -> Language

# ADR 0018 - External Change + Cooperative Lock

## Durum

Kabul edildi.

## Karar

Desktop Writer acik `.tko` dosyasini yalniz timestamp ile izlemeyecek. Baseline, dosya byte length + content fingerprint ile tutulacak.

Ayni `.tko` icin Turkuaz Office surecleri cooperative sidecar lock kullanacak. Lock alinamazsa belge acilacak ancak backend seviyesinde read-only olacak.

Read-only yalniz View davranisi degildir. Text mutation, style mutation, paragraph mutation ve undo/redo Service katmaninda da reddedilir.

## External change state

Typed state:

- `untracked`
- `unchanged`
- `modified`
- `missing`

Frontend belirli araliklarla status sorgular. Save ayni path'e yapilmadan once backend fingerprint'i tekrar kontrol eder; polling gecikse bile silent overwrite yapilmaz.

## Kullanici kararlari

External state `modified` veya `missing` ise:

- Reload From Disk
- Keep Local Version
- Save As

`Keep Local Version` explicit kullanici karari sayilir. Backend mevcut disk baseline'ini kabul eder ve ardindan local canonical belgeyi ayni path'e explicit Save ile yazar.

Read-only lock durumunda same-path overwrite yasaktir; Save As serbesttir.

## Lock sahipligi

Sidecar lock `create_new` ile alinir. Lock release sirasinda token tekrar dogrulanir; process kendisine ait olmayan lock dosyasini silmez.

Normal process kapanisinda lock Drop ile temizlenir.

## Bilinen sinir

M1 cooperative sidecar lock, OS kernel advisory lock degildir. Hard-crash sonrasi stale sidecar otomatik process-liveness cozumu bu fazda yoktur. Stale lock varsa belge read-only acilir ve Save As kullanilabilir. Kernel/native lock adaptorleri ayri ADR ile genisletilecektir.
