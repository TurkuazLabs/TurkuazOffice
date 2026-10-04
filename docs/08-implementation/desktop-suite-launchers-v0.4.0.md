# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/desktop-suite-launchers-v0.4.0.md
# 📌 Amac: Turkuaz Office Desktop suite launcher, ayri Writer/Sheet process ve Windows Start Menu kisayol mimarisini dokumante eder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.1.0
# Aciklama: Tek pencere modul switcher yerine Start Center ve process launch kontratini kaydeder
# Bagimli Oldugu Katman: Controller | Service | Tool | View | Config | Distribution

# Desktop Suite Launchers v0.4.0

## Karar

Writer ve Sheet ayni pencere icinde modul degistiren tek uygulama gibi davranmaz.

Turkuaz Office artik suite launcher modelini kullanir:

- Turkuaz Office: Start Center
- Turkuaz Writer: ayri process
- Turkuaz Sheet: ayri process
- Slides ve Draw: Start Center icinde planlanan modul kartlari

## Windows Start Menu

NSIS installer `Turkuaz Office` klasoru altinda su kisayollari olusturur:

- Turkuaz Office
- Turkuaz Writer
- Turkuaz Sheet

Writer ve Sheet kisayollari ayni guvenilir desktop binarysini `--module=writer` ve `--module=sheet` parametreleriyle baslatir. Ortak Core/Service kodu kopyalanmaz.

## File association

Bir `.tko` dosyasi process argumenti olarak gelirse acilis modulu otomatik Writer olur. Bu sayede native belge cift tiklama akisi Start Centera dusmez.

## Windows console

Release binary `windows_subsystem = "windows"` ile derlenir. Normal kullanici acilisinda console penceresi gosterilmez.

## Mimari sinir

Akis:

Launcher shortcut -> process argument -> StartupArgumentsTool -> DesktopLaunchService -> DesktopLaunchController -> TauriDesktopLaunchTool -> main composition -> Start Center / Writer / Sheet

Frontend canonical state tutmaz; yalnizca hangi uygulama yuzeyinin compose edilecegini belirler.
