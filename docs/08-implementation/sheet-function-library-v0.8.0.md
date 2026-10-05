# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/sheet-function-library-v0.8.0.md
# 📌 Amac: Turkuaz Sheet temel function library mimarisini ve evaluation semantigini dokumante eder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.8.0
# Aciklama: SUM/AVERAGE/MIN/MAX/IF, A1 range, comparison ve guvenlik limitlerini tanimlar
# Bagimli Oldugu Katman: Service | Tool | Config

# Sheet Function Library v0.8.0

## Kapsam

Bu dilim mevcut same-sheet formula motorunu Excel/Calc tarzinda temel fonksiyonlarla genisletir:

- SUM
- AVERAGE
- MIN
- MAX
- IF

Parser ayrica rectangular A1 range ve comparison operatorlerini destekler.

## Range

Ornek:

- `SUM(A1:A10)`
- `AVERAGE(B2:D20)`
- `MIN(B3:A1)`

Ters yazilan range endpoint'leri normalize edilir.

Tek bir formula evaluation sirasinda aggregate fonksiyonlar tarafindan genisletilen toplam range hucre sayisi 100000 ile sinirlidir.

## Aggregate semantigi

Range icinde:

- number degerler kullanilir,
- formula hucreleri canonical evaluator ile hesaplanir,
- empty hucreler yok sayilir,
- text hucreler yok sayilir,
- boolean hucreler yok sayilir.

Scalar expression argumanlari numeric olmak zorundadir.

`AVERAGE` icin numeric deger yoksa division-by-zero typed error uretilir.

`MIN` ve `MAX` icin numeric deger yoksa mevcut numeric-baseline geregi 0 doner.

## Comparison

Desteklenen operatorler:

- =
- <>
- >
- >=
- <
- <=

Comparison sonucu canonical numeric boolean olarak 1 veya 0'dur.

## IF

`IF(condition, true_expression, false_expression)` tam uc arguman ister.

IF lazy branch evaluation kullanir. Secilmeyen branch evaluate edilmez. Bu nedenle:

`IF(A1>0, 42, 1/0)`

A1 pozitifse division-by-zero uretmez.

Hem comma hem semicolon function-argument separator kabul edilir.

## Guvenlik

- max formula length: 4096
- max parse depth: 64
- max operations: 128
- max function arguments: 64
- max expanded range cells: 100000
- existing dependency depth ve cycle detection korunur

Raw range expression aggregate/function context disinda numeric scalar olarak kullanilamaz ve `FormulaRangeNotAllowed` verir.
