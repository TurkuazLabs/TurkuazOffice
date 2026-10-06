# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/m3-web-browser-file-transfer-test-matrix.md
# 📌 Amac: M3 Web browser file-transfer foundation kalite ve regression kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.4.0
# Aciklama: File picker, boyut guardi, cancel, Blob download, object URL cleanup ve Service/Controller sinirlarini listeler
# Bagimli Oldugu Katman: Controller -> Service -> Tool -> Config

# M3 Web Browser File Transfer Test Matrix

| Alan | Otomatik kontrol | Beklenen |
| --- | --- | --- |
| File picker | BrowserFileTransferTool unit testi | Secilen dosya byte olarak doner, temporary input silinir |
| Picker cancel | BrowserFileTransferTool unit testi | null doner ve temporary input silinir |
| Pre-read limit | BrowserFileTransferTool unit testi | 16 MiB ustu dosyada arrayBuffer cagrilmaz |
| Download | BrowserFileTransferTool unit testi | Blob object URL ile anchor click yapilir |
| URL cleanup | BrowserFileTransferTool unit testi | download sonunda object URL revoke edilir |
| TKO accept | WebImportExportService unit testi | merkezi .tko accept ve max-byte requesti Tool'a iletilir |
| Extension validation | WebImportExportService unit testi | .tko disi secim reddedilir |
| Export filename | WebImportExportService unit testi | eksik .tko uzantisi deterministic eklenir |
| Codec honesty | WebImportExportService unit testi | byte transfer true, native TKO codec false raporlanir |
| Controller layering | WebController unit testi | import/export requestlerinde yalniz Service delege edilir |
| Web compile | npm run build | strict TypeScript browser API kontrati derlenir |
| Web regression | npm test | tum Web unit testleri yesil |
| Static contract | tools/verify-project.sh | file-transfer ve no-codec siniri zorunlu kalir |

## Basari kriteri

- Browser byte ingress/egress native filesystem yetkisi olmadan calisir.
- TKO domain parse/serialize TypeScript katmanina kopyalanmaz.
- View codec olmadan gercek import/export aksiyonu sunmaz.
- Browser ingress limiti mevcut Rust TKO package limitiyle uyumludur.
