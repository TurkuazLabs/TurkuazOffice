# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/recent-files-test-matrix.md
# 📌 Amac: M1 Recent Files persistence, dedup, retention ve prune test kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Native TKO recent listesi icin canonical path, maksimum kayit, siralama ve missing-file cleanup bariyerlerini sabitler

Bagimli Oldugu Katman: Documentation

# Recent Files Test Matrix

## Scope

Recent Files yalniz native .tko calisma dosyalarini izler.

DOCX import ve DOCX/PDF export hedefleri recent native document listesine girmez.

## Persistence

- Metadata local Desktop state root altinda recent-files.yml dosyasinda tutulur.
- Metadata safe-replace ile yazilir.
- YAML serialize/deserialize Tool katmanindadir.
- Disk erisimi Repo katmanindadir.
- Dedup, retention ve prune is kurallari Service katmanindadir.

## Path rules

- Record oncesi dosya gercekten var olmalidir.
- Uzanti .tko olmalidir.
- Path fs::canonicalize ile normalize edilir.
- Windows duplicate karsilastirmasi case-insensitive'dir.
- Diger platformlarda canonical path exact karsilastirilir.

## Ordering and retention

- En son erisilen dosya listenin basindadir.
- Ayni dosya yeniden kaydedilirse duplicate olusmaz; basa tasinir.
- Maksimum kayit sayisi 12'dir.
- Limit asan en eski kayitlar atilir.

## Missing file cleanup

Liste okunurken artik diskte bulunmayan kayitlar prune edilir ve temizlenmis metadata tekrar yazilir.

## Session isolation

Recent metadata kaydi aktif Writer document veya native file-session state'ini mutate etmez.

## Frontend

- Startup recent listeyi recovery kararindan bagimsiz yukler.
- Basarili native Open/Save sonrasi record IPC cagrilir.
- Recent metadata hatasi basarili document Open/Save sonucunu geri almaz.
- Home ribbon ilk 3 recent dosyayi tam path tooltip ile quick-open butonu olarak gosterir.
- Recent quick-open normal unsaved-changes guard'ini kullanir.

## Regression

- Canonical duplicate tek kayit olur.
- Limit 12'de kalir.
- Latest-first siralama korunur.
- Missing file prune edilir.
- Explicit recent record native file session'i degistirmez.
