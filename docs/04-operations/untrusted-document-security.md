# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/04-operations/untrusted-document-security.md
# 📌 Amac: Guvenilmeyen office ve TKO belge girdileri icin parse ve resource limitlerini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Zip bomb, path traversal, external resource ve macro risklerini sinirlayan belge guvenligi politikasidir

Bagimli Oldugu Katman: Documentation

# Untrusted Document Security

## Varsayim

Kullanicinin actigi her DOCX, XLSX, PPTX, ODF veya TKO dosyasi guvenilmeyen input'tur.

## Zorunlu kontroller

- Archive path traversal reddedilir.
- Symlink/hardlink benzeri beklenmeyen package entry reddedilir.
- Entry count limitlidir.
- Compressed ve expanded size limitlidir.
- Compression ratio limiti vardir.
- XML/YAML/structured parser recursion ve node limitleri vardir.
- External URL fetch varsayilan olarak yapilmaz.
- Embedded executable content calistirilmaz.
- Macro calistirma varsayilan olarak kapali tutulur.

## Macro

M1-M6 boyunca VBA/macro execution kapsam disidir. Macro iceren belge acilabilir; macro preserved-or-dropped compatibility report ile belirtilir fakat calistirilmaz.

## Resource

Image decode ve font parse islemleri de input budget'a tabidir. Tek bir belge sinirsiz RAM veya CPU tuketemez.

## Telemetry

Guvenlik logu document text tasimaz. Hata raporu minimum metadata ile uretilir.

## TKO v1 byte profile

- Yalniz `manifest.yml` ve `content/writer.yml` file entry kabul edilir.
- Directory entry reddedilir.
- ZIP compression yalniz `Stored` olabilir.
- Backslash, absolute path, `.` ve `..` path segmentleri reddedilir.
- Duplicate entry reddedilir.
- Package, manifest, content ve total uncompressed boyut limitleri uygulanir.
- YAML parse canonical domain'e gecmeden once DTO katmaninda yapilir.
- Future schema ve manifest/content mismatch acik hata ile reddedilir.
