# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/accessibility-keyboard.md
# 📌 Amac: Accessibility ve keyboard-first kalite barini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Mouse zorunlulugunu kaldiran, screen reader ve focus davranisini standardize eden kalite kontratidir

Bagimli Oldugu Katman: Documentation

# Accessibility ve Keyboard-First

## Minimum kalite bari

- Tum ana komutlar keyboard ile ulasilabilir.
- Visible focus zorunludur.
- Focus order mantiksaldir.
- Toolbar ve editor semantic role tasir.
- Screen reader icin accessible name/description vardir.
- Zoom belge verisini bozmaz.
- High contrast modunda kritik durum sadece renkle anlatilmaz.
- Reduced motion tercihi desteklenir.

## Shortcut

Shortcut action'a baglanir; UI component icinde magic key string tutulmaz. Platform mapping config/command registry uzerinden gelir.

## Test

M1 Definition of Done icine keyboard-only smoke test ve minimum screen-reader semantic kontrolu girer.
