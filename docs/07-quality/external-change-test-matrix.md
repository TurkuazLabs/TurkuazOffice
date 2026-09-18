# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/external-change-test-matrix.md
# 📌 Amac: External-change, file-lock ve read-only regression senaryolarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Fingerprint, save-conflict, reload, keep-local, read-only ve Save As kalite bariyerlerini listeler

Bagimli Oldugu Katman: Test

# External Change Test Matrix

## Rust Service

- Ilk save sonrasinda file session lock sahipligini raporlar.
- Ayni dosyayi ikinci service acarsa read-only state alir.
- Read-only session text mutation yapamaz.
- Disk byte icerigi degisirse state `modified` olur.
- Dosya silinirse state `missing` olur.
- Same-path Save external change varken `writer.external_change_conflict` ile durur.
- Explicit acknowledge sonrasi same-path Save yapilabilir.
- Save As yeni path icin yeni lock alir ve session writable olur.
- Normal session drop yalniz kendi token'ina ait sidecar lock'i temizler.

## Frontend

- 2 saniyelik poll canonical document'i mutate etmez.
- Lock varsa contenteditable kapanir.
- B/I/U, paragraph format, undo/redo read-only durumda mutation uretmez.
- Modified state Reload / Keep Local / Save As aksiyonlarini gosterir.
- Missing state Keep Local / Save As aksiyonlarini gosterir; reload gizlenir.
- Read-only lock state Save As sunar.
- Ctrl/Cmd+Shift+S Save As akisini acar.

## Runtime

- Iki Turkuaz Office process'i ayni `.tko` dosyasini acma smoke testi.
- Harici editor/process ile dosya byte degisikligi smoke testi.
- Harici delete/rename smoke testi.
- Windows ve Linux sidecar path/permission smoke testi.
- Hard-crash stale lock davranisi belgelenmis read-only fallback ile dogrulanir.
