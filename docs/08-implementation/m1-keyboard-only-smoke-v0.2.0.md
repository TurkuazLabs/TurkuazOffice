# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-keyboard-only-smoke-v0.2.0.md
# 📌 Amac: M1 Keyboard-Only Smoke implementation kapsam ve mimari sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Typed shortcut resolver, WriterShell dispatch, aria-keyshortcuts ve smoke test akisini tanimlar

Bagimli Oldugu Katman: Documentation

# M1 Keyboard-Only Smoke

## Mimari

Keyboard event:

WriterShell View -> WriterController -> KeyboardShortcutService -> typed WriterShortcutAction.

Controller shortcut karari uretmez; yalniz Service cagrisi aktarir.

View DOM'a ozel davranislari yonetir:

- preventDefault.
- focused contenteditable blur/flush.
- resolved action icin mevcut Controller command cagrisi.

## Typed shortcut actions

Action stringleri config/keyboard.ts icindeki WRITER_SHORTCUT_ACTIONS kaynagindan gelir.

Visible shortcut controls WRITER_ARIA_SHORTCUTS metadata'sini kullanir.

## Context rules

KeyboardShortcutService:

- Ctrl ve Meta modifier'larini ayni semantic modifier olarak kabul eder.
- IME composition sirasinda normal Writer shortcutlarini bloklar.
- Recovery karar ekrani document shortcutlarini bloklar.
- Print Preview acikken yalniz Escape ve Print shortcutlarini kabul eder.

## Existing command reuse

Keyboard shortcutlari ayri business workflow yaratmaz.

New/Open/Save/Print/Undo/Redo/Format/Zoom mevcut WriterController komutlarini yeniden kullanir.

## Focus

Ribbon/statusbar action controls native button/select kullanir.

Mevcut focus-visible CSS kurallari:

- toolbar button.
- ribbon select.
- statusbar button.
- language select.

Editor contenteditable surface keyboard focus alir.

## Validation

Vitest keyboard-shortcut.service.test.ts resolver davranisini test eder.

Manual keyboard-only checklist docs/07-quality/keyboard-only-smoke-test.md icindedir.

## M1 siniri

Bu faz screen-reader certification veya tum platform accessibility audit'i degildir.

Ama keyboard ile temel Writer document lifecycle ve editing workflow'larinin ulasilabilir oldugunu smoke seviyesinde kapatir.
