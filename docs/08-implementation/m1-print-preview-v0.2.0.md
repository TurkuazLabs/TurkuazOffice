# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-print-preview-v0.2.0.md
# 📌 Amac: M1 Writer Print Preview + Print Minimum implementation kapsam ve sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Session-only preview, PrintTool system dialog adapteri ve canonical page geometry entegrasyonunu tanimlar

Bagimli Oldugu Katman: Documentation

# M1 Print Preview + Print Minimum

## Hedef

Writer belgesini ayri bir document modeline kopyalamadan, mevcut canonical read-model ve page settings ile print preview gostermek ve sistem yazdirma dialoguna guvenli gecis saglamak.

## Katman akisi

View -> Controller -> WriterSessionService -> WriterSessionRepository / WriterLayoutService / PrintTool.

Controller yalniz request aktarir. Preview state repository icinde session-only tutulur. Fiziksel print cagrisi PrintTool dis dunya adaptorundedir.

## Preview

- Focused paragraph once flush edilir.
- Pending mutation queue tamamlanir.
- Yuzde 100 preview layout WriterLayoutService ile canonical page settings uzerinden hesaplanir.
- Preview read-only WriterPage renderidir.
- Preview ilk print action'a keyboard focus verir; kapanista mevcut logical editor selection geri yuklenir.
- Canonical document, revision ve file dirty baseline mutate edilmez.

## Print minimum

PrintTool canonical width/height twip degerlerini inch tabanli dinamik @page rule'a cevirir ve WebView system print capability'sini cagirir.

M1 minimumunda printer discovery ve kullanici secimleri sistem dialoguna delege edilir:

- printer
- page range
- copies
- orientation
- paper size
- margin
- scale

Bu karar Core veya Writer domain katmanini printer driver API'lerine baglamaz.

## Keyboard

- Ctrl+P normal editor durumunda preview acar.
- Ctrl+P preview acikken sistem print dialogunu acar.
- Escape preview'i kapatir.

## Bilinen sinir

Programmatic printer inventory ve direct spool API M1 minimumuna eklenmez. Sistem dialogunun sundugu capability kullanilir. PDF export ayri roadmap adimidir.
