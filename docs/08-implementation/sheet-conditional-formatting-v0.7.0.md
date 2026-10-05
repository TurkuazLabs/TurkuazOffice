# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/sheet-conditional-formatting-v0.7.0.md
# 📌 Amac: Turkuaz Sheet canonical conditional formatting mimarisini ve masaustu akislarini dokumante eder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.7.0
# Aciklama: Range tabanli kurallar, formula-aware match, priority ve semantic highlight davranisini tanimlar
# Bagimli Oldugu Katman: Controller | Service | Repo | Tool | View | Language | Config | Adapter

# Sheet Conditional Formatting v0.7.0

## Kapsam

Conditional Formatting yalnizca CSS veya frontend state degildir. Kurallar canonical SheetDocument metadata'sinda saklanir.

Bir kural:

- stabil ConditionalFormatRuleId,
- worksheet kimligi,
- rectangular range,
- condition,
- semantic style,
- deterministic priority

tasir.

## Ilk condition seti

- Number Greater Than
- Number Less Than
- Number Equals
- Text Contains

Numeric condition girdileri finite number olmak zorundadir. Text Contains bos veya yalniz whitespace degerini kabul etmez.

## Stiller

Ilk masaustu baseline'i uc semantic highlight kullanir:

- warning
- success
- accent

Canonical model ham RGB degeri tasimaz. View semantic style'i platform theme'i ile render eder.

## Formula davranisi

Match hesabi canonical Sheet formula evaluator kullanir. Formula sonucu numeric ise numeric rule'a katilir.

Tek bir formula hucrede evaluation error olmasi tum conditional-format projection'ini bozmaz. O hucre no-match sayilir ve diger hucreler degerlendirilmeye devam eder.

## Priority

Daha dusuk priority numarasi daha yuksek onceliklidir.

Ayni hucrede birden fazla rule true ise ilk priority kazanir. Rule silinirse siradaki matching rule gorunur hale gelir.

## Desktop akis

View -> Controller -> SheetSessionService -> TauriSheetTool -> Tauri Controller -> Desktop Service -> SheetController -> SheetService -> Repo

Frontend condition match business logic'i hesaplamaz. Gorunur 100 x 26 grid icin backend match projection'i alinir ve Session Repo'da cache edilir.

## XLSX guvenligi

Mevcut XLSX adapter value-only'dir ve OOXML conditionalFormatting parcalarini yazmaz.

Canonical conditional-format metadata bulunan belge export edilirken metadata sessizce kaybedilmez. Export, OOXML rule destegi eklenene kadar UnsupportedConditionalFormat ile strict reject olur.
