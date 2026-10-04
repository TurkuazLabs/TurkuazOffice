# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/desktop-suite-launcher-test-matrix.md
# 📌 Amac: Desktop suite launcher ve Windows kisayol davranisinin kalite kapilarini tanimlar
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.1.0
# Aciklama: Start Center, process argumanlari, native belge acilisi, console ve NSIS kisayollarini listeler
# Bagimli Oldugu Katman: Service | View | Distribution | CI

# Desktop Suite Launcher Test Matrix

| Alan | Dogrulama | Beklenen |
| --- | --- | --- |
| Default launch | Parametresiz acilis | Start Center |
| Writer shortcut | --module=writer | Turkuaz Writer |
| Sheet shortcut | --module=sheet | Turkuaz Sheet |
| Native .tko | Dosya argumenti | Writer |
| In-app switcher | Writer/Sheet shell | Yok |
| Start Center | Writer/Sheet launch | Ayri process |
| Planned apps | Slides/Draw | Pasif kart |
| Windows release | Console penceresi | Yok |
| Start Menu folder | Turkuaz Office | Mevcut |
| Start Menu shortcut | Turkuaz Writer | Mevcut |
| Start Menu shortcut | Turkuaz Sheet | Mevcut |
| Uninstall | Ek kisayollar | Silinir |
