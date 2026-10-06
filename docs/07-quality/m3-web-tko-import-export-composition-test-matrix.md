# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/m3-web-tko-import-export-composition-test-matrix.md
# 📌 Amac: M3 Web TKO import/export Service composition kalite ve regression kontrollerini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.4.0
# Aciklama: Codec availability, fail-closed fallback, inspect import, canonical re-encode export ve Controller delegation testlerini listeler

Bagimli Oldugu Katman: Controller -> Service -> Tool -> Config

# M3 Web TKO Import Export Composition Test Matrix

| Alan | Otomatik kontrol | Beklenen |
| --- | --- | --- |
| Codec capability | Vitest | Writer TKO Tool varsa true, fallback modunda false |
| Missing codec import | Vitest | Picker acilmadan stable hata |
| Native import | Vitest | TKO picker -> limit/extension -> inspect -> typed import sonucu |
| Picker cancel | Vitest | null doner, codec cagrilmaz |
| Oversize defense | Vitest | Service ikinci limit kontrolunde codec oncesi reddeder |
| Extension defense | Vitest | TKO disi uzanti codec oncesi reddedilir |
| Native export | Vitest | source bytes -> reencode(app version) -> normalized TKO download |
| Missing codec export | Vitest | Download cagrilmadan stable hata |
| Controller | Vitest | Import/export requestleri yalniz Service'e delege edilir |
| Composition root | Static verifier | Runtime loader Writer TKO Tool'u Service'e enjekte eder |
| View gate | Static verifier | import_export_view_actions false kalir |
| Web build/tests | CI | Typecheck, build ve tum Web unit testleri yesil |

## Basari kriteri

- Raw byte Service API codec'i bypass etmez.
- Browser fallback native TKO capability iddia etmez.
- TKO validation ve canonicalization Rust codec uzerinden calisir.
- Controller logic eklenmez.
- View aksiyonlari Writer Web session entegrasyonu olmadan acilmaz.
