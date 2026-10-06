# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/desktop-ui-refresh-test-matrix.md
# 📌 Amac: Desktop UI refresh icin regression ve urun kabul kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.12.0
# Aciklama: Start Center, ayri ikon kimligi, Writer layout, Sheet layout ve direct launch davranislarini test matrisi olarak sabitler
Bagimli Oldugu Katman: Quality -> View -> Tool

# Desktop UI Refresh Test Matrix

| Alan | Kontrol | Beklenen |
| --- | --- | --- |
| Default launch | Argumansiz desktop acilisi | Start Center acilir |
| Direct Writer | `--module writer` | Writer dogrudan acilir |
| Direct Sheet | `--module sheet` | Sheet dogrudan acilir |
| Safe fallback | bilinmeyen module | Start Center acilir |
| Start Center | Writer karti | Writer View'a gecer |
| Start Center | Sheet karti | Sheet View'a gecer |
| Navigation | Writer/Sheet Home butonu | Start Center'a doner |
| Product identity | Writer/Sheet ikonlari | Farkli renk ve farkli belge/grid sembolu |
| Writer menu | ust menu sirasi | Dosya, Duzenle, Gorunum, Ekle, Bicim, Tablo, Araclar, Yardim |
| Writer layout | genis ekran | Sayfalar + belge + Ozellikler |
| Writer properties | B/I/U ve alignment | Mevcut Controller komutlarina delegasyon |
| Writer responsive | dar ekran | sag ve sonra sol panel gizlenir; belge kullanilabilir kalir |
| Sheet menu | ust menu sirasi | Dosya, Giris, Ekle, Bicim, Veri, Formuller, Gorunum, Yardim |
| Sheet identity | uygulama accent | Writer mavisinden ayri yesil tema |
| Sheet regression | formula/grid/sidebar | mevcut cell/formula/query/chart akislari calisir |
| CI | Frontend quality | build + unit tests PASS |
| CI | Rust quality | verifier + check + wasm + fmt + clippy PASS |
| CI | Workspace tests | Windows + Ubuntu PASS |
| Preview | Community Preview | Windows NSIS + Linux DEB/AppImage PASS |
