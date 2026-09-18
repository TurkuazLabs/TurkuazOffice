# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0015-caret-typing-style-ribbon-typography.md
# 📌 Amac: Writer caret typing-style, font controls ve paragraph alignment ribbon kararini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Collapsed caret format state gecici UI state olarak tutulur; canonical mutation yalniz metin girdiginde Rust command olur

Bagimli Oldugu Katman: Documentation

# ADR 0015 - Caret Typing Style ve Typography Ribbon

## Durum

Accepted.

## Problem

ADR 0014 ile secili metin icin rich-text komutlari tamamlandi. Ancak gunluk Writer davranisi icin collapsed caret uzerinde B/I/U, font family ve font size secildiginde henuz belgeye metin eklenmeden canonical document mutation uretilmemelidir. Ayrica paragraph alignment karakter stili degil paragraph stili olarak saklanmalidir.

## Karar

Caret uzerindeki gelecekteki yazi stili `WriterSessionRepository` icinde gecici `typingStyle` state olarak tutulur.

Bu state:

- canonical document degildir,
- `.tko` verisi degildir,
- undo history entry uretmez,
- caret ile gercek metin girildiginde `InsertStyledText` commandina donusur.

Secili range varsa font/B/I/U degisikligi `ApplyCharacterStyle` commandi ile canonical belgeye hemen uygulanir.

Paragraph alignment `ApplyParagraphStyle` commandi ile paragraph seviyesinde uygulanir.

## Typography kontrati

Character style baseline:

- bold,
- italic,
- underline,
- font family,
- font size half-points.

Font size canonical domain icinde half-point biriminde tutulur. UI punto degerini half-point degerine cevirir. Domain font size ve font family icin merkezi limit uygular.

## Ribbon kontrati

Desktop Writer iki katmanli ribbon baseline kullanir:

1. menu/tab satiri,
2. aktif Giris command bandi.

Giris bandinda:

- belge,
- history,
- font,
- paragraph

gruplari bulunur.

Dosya, Ekle ve Gorunum tablari bu milestone'da shell olarak gorunur fakat implementation roadmap sirasi gelene kadar aktif command acmaz.

## Selection ve focus

B/I/U ve paragraph alignment butonlari `mousedown` sirasinda editor selection'ini korur.

Native `select` kontrolleri fokus almak zorunda oldugu icin font family/size seciminden hemen once aktif paragraph pending text mutation queue'ya flush edilir. Bu sayede stale backend snapshot uzerine font mutation uygulanmaz. Blur commit, browser selection artik editor icinde degilse Repository icindeki son gecerli logical selection'i silmez. Secim tamamlandiginda Tool bu logical selection ile paragraph editor fokusunu ve caret/range selection'i geri yukler; boylece typing-style editore tekrar tiklamadan kullanilabilir.

## Typing-style reset

Gecici typing style:

- pointer ile yeni caret seciminde,
- caret navigation keylerinde,
- new document,
- undo,
- redo,
- paragraph merge

sinirlarinda temizlenir.

Normal karakter inputu ve IME composition boyunca korunur.

## Sonuclar

Olumlu:

- Word benzeri "Bold'a bas ve yaz" davranisi canonical state'i kirletmeden elde edilir.
- Font ve size secimi mevcut TextRun stillerini kaybetmez.
- Paragraph alignment character run'larina sizmaz.
- Desktop/Web/Mobile icin typing-style semantigi tekrar kullanilabilir.

Bedeller:

- UI session state ile canonical document state arasinda acik sinir korunmalidir.
- Imported bilinmeyen fontlar icin ileride font discovery/fallback UI gerekir.
- Cross-paragraph selection halen ayri selection-engine milestone'udur.
