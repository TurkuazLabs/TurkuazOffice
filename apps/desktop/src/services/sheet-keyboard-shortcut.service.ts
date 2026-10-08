// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/sheet-keyboard-shortcut.service.ts
// # 📌 Amac: Desktop Sheet keyboard shortcut girdilerini context'e gore typed action'a cozer
// # 📌 Modul - FileType: Service - TypeScript
// Version: 0.1.0
// Aciklama: Spreadsheet kisa yollarini View if-zincirinden ayirir ve ortak keyboard config kontratini kullanir
// Bagimli Oldugu Katman: Service -> Config

import {
  KEYBOARD_KEYS,
  SHEET_SHORTCUT_ACTIONS,
  type SheetShortcutAction,
} from "../config/keyboard";

export interface SheetKeyboardShortcutInput {
  readonly key: string;
  readonly ctrlKey: boolean;
  readonly metaKey: boolean;
  readonly shiftKey: boolean;
  readonly altKey: boolean;
  readonly isComposing: boolean;
  readonly editingActive: boolean;
}

export class SheetKeyboardShortcutService {
  public resolve(input: SheetKeyboardShortcutInput): SheetShortcutAction | null {
    if (input.isComposing) {
      return null;
    }

    const key = input.key.toLowerCase();
    const modifier = input.ctrlKey || input.metaKey;

    if (!modifier && !input.altKey && key === KEYBOARD_KEYS.f2) {
      return SHEET_SHORTCUT_ACTIONS.editCell;
    }
    if (
      !modifier &&
      !input.altKey &&
      input.editingActive &&
      key === KEYBOARD_KEYS.escape
    ) {
      return SHEET_SHORTCUT_ACTIONS.cancelEdit;
    }
    if (input.altKey && !modifier && key === KEYBOARD_KEYS.equal) {
      return SHEET_SHORTCUT_ACTIONS.insertSum;
    }
    if (!modifier) {
      return null;
    }

    if (key === KEYBOARD_KEYS.home.toLowerCase()) {
      return SHEET_SHORTCUT_ACTIONS.firstCell;
    }
    if (key === KEYBOARD_KEYS.n && !input.shiftKey) {
      return SHEET_SHORTCUT_ACTIONS.newDocument;
    }
    if (key === KEYBOARD_KEYS.b && !input.shiftKey) {
      return SHEET_SHORTCUT_ACTIONS.bold;
    }
    if (key === KEYBOARD_KEYS.i && !input.shiftKey) {
      return SHEET_SHORTCUT_ACTIONS.italic;
    }
    if (key === KEYBOARD_KEYS.u && !input.shiftKey) {
      return SHEET_SHORTCUT_ACTIONS.underline;
    }
    if (key === KEYBOARD_KEYS.one && !input.shiftKey) {
      return SHEET_SHORTCUT_ACTIONS.toggleProperties;
    }
    if (key === KEYBOARD_KEYS.l && input.shiftKey) {
      return SHEET_SHORTCUT_ACTIONS.toggleQuery;
    }

    return null;
  }
}
