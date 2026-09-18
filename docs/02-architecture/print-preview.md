# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/print-preview.md
# 📌 Amac: Print, print preview ve PDF export arasindaki layout kontratini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Yazdirma akisinin platform adaptorleri ile ayrilmasini ve preview tutarliligini tanimlar

Bagimli Oldugu Katman: Documentation

# Print ve Print Preview

## Ilke

Print Preview ayri bir belge modeli yaratmaz. Writer layout engine'in print profile render sonucunu kullanir.

## M1 minimumu

- Printer discovery Tool adapteri.
- Page range.
- Copies.
- Orientation.
- Paper size.
- Margin.
- Scale.
- Print preview.
- PDF export ile yakin layout sonucu.

## Platform siniri

Core printer driver API bilmez. Desktop Tool adapteri Windows/Linux native print sistemleri ile iletisim kurar. Web istemcisi browser print capability'si ile sinirli bir adapter kullanir.

## Hata davranisi

Printer unavailable, invalid range veya spool error Service seviyesinde typed error'a cevrilir; View kullaniciya language label ile aciklar.
