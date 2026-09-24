# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/keyboard-only-smoke-test.md
# 📌 Amac: M1 Desktop Writer keyboard-only smoke test adimlarini ve kabul kriterlerini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Shortcut routing, Tab focus, IME/recovery/print-preview context ve focus-visible davranisini kalite kapisi yapar

Bagimli Oldugu Katman: Documentation

# Keyboard-Only Smoke Test

## Otomatik shortcut smoke

keyboard-shortcut.service.test.ts asagidaki typed shortcutlari dogrular:

- Ctrl/Cmd+N: yeni belge.
- Ctrl/Cmd+O: ac.
- Ctrl/Cmd+S: kaydet.
- Ctrl/Cmd+Shift+S: farkli kaydet.
- Ctrl/Cmd+P: print preview.
- Ctrl/Cmd+Z: undo.
- Ctrl/Cmd+Y ve Ctrl/Cmd+Shift+Z: redo.
- Ctrl/Cmd+B: bold.
- Ctrl/Cmd+I: italic.
- Ctrl/Cmd+U: underline.
- Ctrl/Cmd++ veya Ctrl/Cmd+=: zoom in.
- Ctrl/Cmd+-: zoom out.
- Ctrl/Cmd+0: zoom reset.

## Context smoke

- IME composition aktifken edit shortcut route edilmez.
- Startup recovery karari aktifken document shortcut route edilmez.
- Print Preview acikken Escape preview'i kapatir.
- Print Preview acikken Ctrl/Cmd+P system print action'ina gider.
- Print Preview acikken Save/Edit shortcutlari route edilmez.
- Unknown veya modifier olmayan key event Writer global shortcut olarak tuketilmez.

## Manual keyboard-only smoke

Mouse kullanmadan:

1. Uygulamayi ac.
2. Tab / Shift+Tab ile ribbon button, select ve statusbar locale/zoom kontrollerinde ilerle.
3. Focus-visible halkasinin toolbar/select/statusbar kontrolunde gorundugunu dogrula.
4. Ctrl/Cmd+N ile yeni belge ac.
5. Contenteditable paragraph'a Tab navigation ile odaklan ve metin yaz.
6. Ctrl/Cmd+B, I ve U ile formatting shortcutlarini kullan.
7. Ctrl/Cmd+S ile save dialog'u ac; Escape ile dialog'u kapat.
8. Ctrl/Cmd+O ile open dialog'u ac; Escape ile kapat.
9. Ctrl/Cmd+P ile Print Preview ac; Escape ile kapat.
10. Ctrl/Cmd++, Ctrl/Cmd+- ve Ctrl/Cmd+0 ile zoom davranisini kontrol et.
11. Statusbar locale select'e Tab ile ulas; keyboard ile Turkish/English sec.
12. Recovery panel gorunuyorsa Tab ile Recover / Compare / Delete / Close aksiyonlarina ulasilabildigini dogrula.

## Focusability contract

- Action controls native button kullanir.
- Locale/font selectors native select kullanir.
- Editor paragraph contenteditable ile keyboard focus alir.
- Shortcutli visible controls aria-keyshortcuts metadata tasir.
- Keyboard focus CSS focus-visible ile ayirt edilir.

## M1 acceptance

M1 keyboard-only smoke basarili sayilmasi icin:

- typed shortcut regression suite green olmali.
- static verifier shortcut Service/Controller/View zincirini bulmali.
- manual smoke checklist kritik blok olmadan tamamlanabilmeli.
- hosted runner gercek frontend-quality step'i calismadiysa compiler-backed sonuc pending olarak raporlanmali.
