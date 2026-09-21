# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/print-preview-test-matrix.md
# 📌 Amac: Writer Print Preview + Print Minimum regression ve acceptance kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Session-only preview, canonical geometry, system print adapter ve keyboard akisinin kalite barini sabitler

Bagimli Oldugu Katman: Documentation

# Print Preview Test Matrix

## Preview state

- Preview belge modelini mutate etmez.
- Preview zoom degeri session-only ve yuzde 100 canonical render baseline'idir.
- Preview acilmadan once focused paragraph mutation queue flush edilir.
- Preview kapatildiginda editor belgesi, logical selection, typing style ve dirty revision ayni kalir.
- Preview acikken editor mutation shortcutlari devre disi kalir.
- Preview acildiginda ilk print action focus alir; kapanista logical editor selection geri yuklenir.

## Physical page geometry

- Canonical width/height twip degerleri PrintTool tarafindan inch tabanli @page rule'a cevrilir.
- Browser DOM olcumu canonical print geometry kaynagi sayilmaz.
- Belge marginleri WriterPage tarafindan canonical layout servisinden gelir.
- Print CSS shell chrome, ribbon, statusbar ve preview toolbar alanlarini kagida basmaz.

## System dialog ownership

- Printer secimi.
- Page range.
- Copies.
- Orientation.
- Paper size.
- Margin.
- Scale.

Bu secenekler M1 minimumunda sistem yazdirma dialoguna delege edilir. Turkuaz Office canonical belgeyi ve fiziksel page size profilini saglar.

## Keyboard

- Ctrl+P preview acar.
- Preview acikken Ctrl+P sistem print dialogunu acar.
- Preview acikken Escape preview'i kapatir.

## Regression

- PrintTool pageRule Vitest testi physical twip -> inch donusumunu kilitler.
- verify-project print config, Tool, View, Controller, Service, Repo, keyboard ve docs contractlarini arar.
- Hosted CI runner step baslatmadan failure olursa compiler-backed sonucu basarili sayilmaz.
