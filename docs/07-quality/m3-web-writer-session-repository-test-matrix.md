# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/m3-web-writer-session-repository-test-matrix.md
# 📌 Amac: M3 Writer Web session Repository ve Service regression kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.4.0
# Aciklama: Byte isolation, import replacement, cancel, active export, empty fail-closed ve Controller delegation kontrollerini listeler

Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool

# M3 Web Writer Session Repository Test Matrix

| Alan | Otomatik kontrol | Beklenen |
| --- | --- | --- |
| Repository write isolation | Vitest | Kaynak Uint8Array mutation'i session state'i degistirmez |
| Repository read isolation | Vitest | Donen Uint8Array mutation'i saklanan state'i degistirmez |
| Session close | Vitest | Aktif belge null olur |
| Native import | Vitest | Basarili import aktif session olur |
| Picker cancel | Vitest | Mevcut session korunur |
| Active export | Vitest | Session fileName + rich TKO bytes canonical re-encode Service'e gider |
| Empty export | Vitest | Stable fail-closed hata |
| Controller | Vitest | Active/open/export/close yalniz Session Service'e delege edilir |
| Composition root | Build/static contract | Session Repository ve Service Web composition root'ta baglanir |
| Rich payload safety | Static contract | Core text store'a lossy conversion yapilmaz |
| View gate | Static contract | import_export_view_actions false kalir |

## Basari kriteri

- Rich TKO byte payload kayipsiz korunur.
- Session Repository browser API veya codec is kurali bilmez.
- Session Service browser API'ye dogrudan erismez.
- Controller is kurali eklemez.
- User-facing View aksiyonlari bu dilimde acilmaz.
