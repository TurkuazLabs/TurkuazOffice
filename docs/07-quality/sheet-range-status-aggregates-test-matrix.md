# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/sheet-range-status-aggregates-test-matrix.md
# 📌 Amac: Sheet range selection ve aggregate kalite kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.5.1
# Aciklama: Canonical summary, formula evaluation, Shift selection, stale response ve status UI dogrulamalarini listeler
# Bagimli Oldugu Katman: Service | Controller | Tool | Repo | View

# Sheet Range Status Aggregates Test Matrix

| Alan | Dogrulama | Beklenen |
| --- | --- | --- |
| Domain | Number + formula | Sum/Average'a katilir |
| Domain | Text + Boolean | Count'a katilir |
| Domain | Empty cell | Count'a katilmaz |
| Formula | Formula evaluation | Canonical deger kullanilir |
| Controller | Range summary | Typed View doner |
| Desktop IPC | Range request/summary | camelCase typed contract |
| Service | Shift extension | Anchor'dan rectangular range |
| Async | Eski summary response | Yeni secimi ezmez |
| View | Range cells | Ayrik vurgu |
| Status | Count | Gorunur |
| Status | Sum/Average | Numeric secimde gorunur |
