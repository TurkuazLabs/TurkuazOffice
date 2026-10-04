# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/desktop-suite-entrypoints-v0.5.0.md
# 📌 Amac: Turkuaz Office Writer ve Sheet uygulamalarinin Windows suite giris modelini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.5.0
# Aciklama: Tek uygulama ici modul secimi yerine ayri Start Menu kisayollari ve ortak native motor kontratini kaydeder
# Bagimli Oldugu Katman: View | Tool | Controller | Config | Distribution

# Desktop Suite Entrypoints v0.5.0

## Karar

Turkuaz Office masaustu paketi tek pencere icinde Writer/Sheet gecisi sunmaz.

Windows kullanicisi suite uygulamalarini ayri girislerden acar:

- Turkuaz Office Writer
- Turkuaz Office Sheet

Ileride Slides, Draw, Base veya baska Community modulleri ayni suite kalibina eklenebilir.

## Ortak motor

Writer ve Sheet ayri kod kopyalari degildir. Iki kisayol ayni guvenli native Desktop motorunu farkli launch hedefiyle acar:

- `--module writer`
- `--module sheet`

Bu model domain, IPC ve format adaptorlerinin tekrar edilmesini engeller.

## Native launch kontrati

Rust Tool katmani komut satiri hedefini parse eder. Controller yalnizca typed launch hedefini frontend composition rootuna aktarir.

Arguman verilmezse geriye uyumluluk ve `.tko` dosya acma akisi icin varsayilan uygulama Writer'dir.

## Windows Start Menu

NSIS installer hook generic Turkuaz Office kisayolunu kaldirir ve `Turkuaz Office` klasoru altinda Writer ve Sheet kisayollarini olusturur.

## Console

Release Windows binary `windows_subsystem = "windows"` ile derlenir. Normal GUI acilisinda siyah konsol penceresi gosterilmez.
