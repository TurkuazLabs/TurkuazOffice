# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-clipboard-styled-fragment-v0.2.0.md
# 📌 Amac: M1 Clipboard Minimum icin atomic styled-fragment mutation dilimini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Typed styled run replace komutunu Rust domain, Tauri IPC ve Desktop frontend zincirinde belgeler

Bagimli Oldugu Katman: Documentation

# M1 Clipboard Styled Fragment

## Kapsam

Bu dilim Clipboard Minimum'un canonical Writer mutation temelidir. Tam clipboard runtime entegrasyonu degildir.

Hedef, paste ve cut islemlerinin canonical Writer belgesini DOM uzerinden dolayli mutate etmek yerine typed command ile degistirmesidir.

## Typed fragment

Fragment her run icin iki alan tasir:

- `text`
- `CharacterStyle`

Run stili bold, italic, underline, requested font family ve font size half-point alanlarini canonical Writer kontratindan kullanir. Clipboard verisi repository veya service nesnesi tasimaz.

## Atomic command

`ReplaceRangeWithStyledRuns` tek command olarak calisir.

- Styled paste: secili range yerine bir veya daha fazla styled run koyar.
- Caret paste: collapsed range yerine styled run ekler.
- Cut: secili range yerine bos fragment verir.
- Bos fragment + collapsed range: `EmptyCommand` ile reddedilir.
- M1 bu dilimde yalniz ayni paragraf icindeki replace islemini kabul eder.
- Cross-paragraph replace acik `CrossParagraphFragmentReplaceNotSupported` hatasi verir.

Tek command `WriterEditorService::execute` uzerinden history'ye girdigi icin paste veya cut tek undo adimidir.

## Style koruma

Replace range disinda kalan prefix ve suffix run'larinin `CharacterStyle` degerleri korunur. Fragment run'lari kendi stilleriyle yeni canonical `TextRun` nesnelerine donusturulur. Ayni stile sahip bitisik run'lar command sonunda birlestirilir.

Tum paragraf cut ile bosalirsa Writer invariant'i korunur ve paragraf tek bir bos editable run ile birakilir.

## Katman akisi

Desktop akisi su siniri izler:

`WriterController -> WriterSessionService -> TauriWriterTool -> Tauri Writer Controller -> WriterDesktopService -> WriterCommandService`

Controller clipboard is kurali tasimaz. Canonical mutation Rust Service katmaninda kalir.

## Runtime entegrasyon durumu

Desktop paragraph View artik copy/cut/paste eventlerini ince Controller sinirina aktarir. Clipboard Service:

- Copy icin canonical secimi styled run'lara dilimler ve internal MIME + HTML + plain text representationlarini birlikte yazar.
- Paste icin Internal Turkuaz MIME'i schema ve limit kontroluyle ilk tercih olarak okur.
- Internal payload yoksa inert DOM parse sonucu basic bold/italic/underline/font family/font size whitelist'i ile sanitize edilmis HTML run'larini kullanir.
- Rich representation kullanilamazsa plain text'i caret/selection baslangic stiline map eder.
- Cut icin ayni representationlari yazdiktan sonra bos fragment ile atomic replace yapar.
- Paste/cut basarili oldugunda caret atomic fragment sonuna tasinir.

## Bu dilimin disinda

- Cross-paragraph rich fragment replace.
- Image clipboard payloadlari.
- Table-aware Writer/Sheet clipboard adapterlari.
- OS seviyesinde clipboard history entegrasyonu.
