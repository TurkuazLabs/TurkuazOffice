# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/typography-ribbon-test-matrix.md
# 📌 Amac: Writer typography, typing-style, alignment ve ribbon kalite matrisini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Domain, Desktop backend, frontend static ve manual typography kontrollerini listeler

Bagimli Oldugu Katman: Documentation

# Typography + Ribbon Test Matrix

## Rust domain automatic

| Alan | Fixture | Beklenti |
| --- | --- | --- |
| Styled insert | Plain `AB`, caret 1, bold `X` | `A` plain + `X` styled + `B` plain |
| Font patch | Italic run + family/size patch | Italic korunur, family/size degisir |
| Validation | Bos family veya limit disi size | `InvalidCharacterStyle` |
| Paragraph | Center alignment patch | Sadece paragraph alignment degisir |
| History | Alignment/style command + undo | Onceki snapshot geri gelir |

## Desktop backend automatic

| Alan | Fixture | Beklenti |
| --- | --- | --- |
| Typing style | `AB` -> styled `AXB` minimal diff | Sadece `X` typing style alir |
| Read model | Font patch | DTO family/half-point degeri guncel |
| Alignment | Right command | Paragraph View `Right` tasir |
| Unicode | Emoji yaninda styled insert | Scalar offset bozulmaz |

## Frontend static

- Strict TypeScript typecheck.
- Font family config View disinda merkezi tutulur.
- Font size listesi config View disinda merkezi tutulur.
- Canonical document frontend tarafinda mutate edilmez.
- `WriterRibbon` yalniz Controller cagirir.
- Font select pending paragraph flush eder.
- Blur commit gecerli DOM selection yoksa Repository selection degerini silmez.
- Font/size select sonrasinda editor focus + logical selection restore kontrati vardir.
- Paragraph renderer alignment read-model kullanir.
- Caret typing style Repository state'idir.

## Windows manual

- Caret -> Bold -> yaz -> yalniz yeni metin bold.
- Caret -> Georgia -> yaz -> yalniz yeni metin Georgia.
- Caret -> 18pt -> yaz -> yalniz yeni metin 18pt.
- Range -> font degistir -> selection disi run korunur.
- Left/Center/Right/Justify gorunumu.
- Font select sonrasi yazi kaybi olmamasi.
- Font select sonrasi editore tekrar tiklamadan typing-style ile yazmaya devam edilebilmesi.
- Turkish IME composition + typing-style.
- Undo styled inputu tek anlamli adimda geri alir.

## Linux manual

- WebKitGTK select/editor focus gecisi.
- Font family fallback gorunumu.
- Alignment parity.
- IBus/Fcitx composition + styled input.

## Exit criteria

Typography/ribbon alt fazi tamamlandi sayilabilmesi icin static validation temiz olmali. Production-ready Writer icin gercek Rust compile/test, dependency-resolved Vite build ve Windows/Linux manual smoke bariyerleri halen zorunludur.
