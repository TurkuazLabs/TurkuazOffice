# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-tko-import-export-composition-v0.4.0.md
# 📌 Amac: M3 Web browser file-transfer ile Writer TKO typed Tool Service composition kararlarini belgeler
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.4.0
# Aciklama: Native TKO import validation ve canonical re-encode download akisini Service sinirinda birlestirir

Bagimli Oldugu Katman: Controller -> Service -> Tool -> View -> Config

# M3 Web TKO Import Export Composition v0.4.0

## Karar

Browser file-transfer Tool ve typed `WebWriterTkoTool` yalniz `WebImportExportService` icinde compose edilir.

Controller byte veya codec is kurali uygulamaz. View generated WASM exportlarina ve browser file API'lerine dogrudan erismez.

## Import akisi

`importNativeDocument()`:

1. Writer TKO Tool yoksa picker acilmadan fail-closed hata verir.
2. Browser picker merkezi `.tko` accept profili ve 16 MiB limiti ile acilir.
3. Tool fake veya browser davranisi limiti atlarsa Service byte uzunlugunu tekrar kontrol eder.
4. Dosya uzantisi `.tko` degilse codec cagrilmadan reddedilir.
5. Byte payload Rust-backed `inspect` ile dogrulanir.
6. Service dosya adi, izole byte kopyasi ve typed TKO summary dondurur.

Picker iptali hata degildir ve null sonucu uretir.

## Export akisi

`exportNativeDocument(fileName, bytes)`:

1. Writer TKO Tool yoksa fail-closed hata verir.
2. Byte payload `WEB_APP_VERSION` ile Rust-backed `reencode` fonksiyonuna verilir.
3. Canonical re-encode ciktisi browser download Tool'una aktarilir.
4. Dosya adi merkezi `.tko` uzanti profiline normalize edilir.

Raw byte download artik public Service akisi degildir; codec bypass edilmez.

## Capability

`nativeTkoCodecAvailable` artik sabit false degildir. Deger yalniz runtime loader gercek WASM Writer TKO Tool sagladiginda true olur.

Browser Core fallback calisabilir; fakat fallback modunda TKO codec capability false kalir ve import/export Service fail-closed davranir.

## Bu dilimde kapsam disi

- Writer Web session/repository'ye imported canonical document yerlestirme
- mevcut Writer editor state'inden yeni TKO package uretme
- View import/export butonlarini aktif etme
- offline cache

Bu nedenle Service composition tamamlanmis olsa da kullaniciya tam belge ac/kaydet ozelligi hazir diye sunulmaz.

## Sonraki M3 adimi

Writer/Sheet web read-model ve kullanici yuzeyi entegrasyonu icinde Writer session/repository bridge kurulduktan sonra View import/export aksiyonlari acilabilir.
