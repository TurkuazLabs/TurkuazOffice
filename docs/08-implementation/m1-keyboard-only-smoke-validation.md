# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-keyboard-only-smoke-validation.md
# 📌 Amac: M1 Keyboard-Only Smoke static, frontend regression ve manual validation durumunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Shortcut resolver, focus metadata, smoke checklist ve hosted runner sinirini ayri raporlar

Bagimli Oldugu Katman: Documentation

# M1 Keyboard-Only Smoke Validation

## Static contract

- Shortcut key constants config katmanindadir.
- Shortcut action constants config katmanindadir.
- Resolver KeyboardShortcutService icindedir.
- WriterController yalniz resolver Service cagrisi aktarir.
- WriterShell resolved typed action'i dispatch eder.
- Visible file/history/format/zoom controls aria-keyshortcuts metadata kullanir.
- focus-visible CSS toolbar/select/statusbar controls icin mevcuttur.

## Regression

keyboard-shortcut.service.test.ts:

- file shortcuts.
- history shortcuts.
- formatting shortcuts.
- zoom shortcuts.
- Ctrl/Meta parity.
- IME blocking.
- recovery blocking.
- print-preview Escape/Print isolation.
- unknown/unmodified key ignore.

## Manual smoke

docs/07-quality/keyboard-only-smoke-test.md Tab/focus/dialog/editor/locale/preview/recovery adimlarini tanimlar.

## Compiler-backed durum

GitHub hosted runner gercek frontend-quality step baslatmadan steps=null failure verirse keyboard-only frontend test sonucu onaylanmis sayilmaz.

Static + source-level regression coverage tamamlanir; compiler-backed sonuc pending kalir.
