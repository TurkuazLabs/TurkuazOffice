// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/keyboard-shortcut.service.ts
// # 📌 Amac: Desktop Writer keyboard shortcut girdilerini context'e gore typed action'a cozer
// # 📌 Modul - FileType: Service - TypeScript
// Version: 0.4.0
// Aciklama: IME, recovery ve print-preview context kurallarini View if-zincirinden ayirip test edilebilir hale getirir
// Bagimli Oldugu Katman: Service -> Config

import {
  KEYBOARD_KEYS,
  WRITER_SHORTCUT_ACTIONS,
  type WriterShortcutAction,
} from "../config/keyboard";

export interface KeyboardShortcutInput {
  readonly key: string;
  readonly ctrlKey: boolean;
  readonly metaKey: boolean;
  readonly shiftKey: boolean;
  readonly isComposing: boolean;
  readonly printPreviewOpen: boolean;
  readonly recoveryBlocking: boolean;
}

export class KeyboardShortcutService {
  public resolve(input: KeyboardShortcutInput): WriterShortcutAction | null {
    const modifier = input.ctrlKey || input.metaKey;
    const key = input.key.toLowerCase();

    if (input.printPreviewOpen) {
      if (key === KEYBOARD_KEYS.escape) {
        return WRITER_SHORTCUT_ACTIONS.closePrintPreview;
      }
      if (modifier && key === KEYBOARD_KEYS.p && !input.shiftKey) {
        return WRITER_SHORTCUT_ACTIONS.print;
      }
      return null;
    }

    if (!modifier || input.isComposing || input.recoveryBlocking) {
      return null;
    }

    if (key === KEYBOARD_KEYS.plus || key === KEYBOARD_KEYS.equal) {
      return WRITER_SHORTCUT_ACTIONS.zoomIn;
    }
    if (key === KEYBOARD_KEYS.minus) {
      return WRITER_SHORTCUT_ACTIONS.zoomOut;
    }
    if (key === KEYBOARD_KEYS.zero) {
      return WRITER_SHORTCUT_ACTIONS.zoomReset;
    }
    if (key === KEYBOARD_KEYS.z && !input.shiftKey) {
      return WRITER_SHORTCUT_ACTIONS.undo;
    }
    if (key === KEYBOARD_KEYS.y || (key === KEYBOARD_KEYS.z && input.shiftKey)) {
      return WRITER_SHORTCUT_ACTIONS.redo;
    }
    if (key === KEYBOARD_KEYS.n) {
      return WRITER_SHORTCUT_ACTIONS.newDocument;
    }
    if (key === KEYBOARD_KEYS.o) {
      return WRITER_SHORTCUT_ACTIONS.open;
    }
    if (key === KEYBOARD_KEYS.p && input.shiftKey) {
      return WRITER_SHORTCUT_ACTIONS.printPreview;
    }
    if (key === KEYBOARD_KEYS.p) {
      return WRITER_SHORTCUT_ACTIONS.print;
    }
    if (key === KEYBOARD_KEYS.s && input.shiftKey) {
      return WRITER_SHORTCUT_ACTIONS.saveAs;
    }
    if (key === KEYBOARD_KEYS.s) {
      return WRITER_SHORTCUT_ACTIONS.save;
    }
    if (key === KEYBOARD_KEYS.b) {
      return WRITER_SHORTCUT_ACTIONS.bold;
    }
    if (key === KEYBOARD_KEYS.i) {
      return WRITER_SHORTCUT_ACTIONS.italic;
    }
    if (key === KEYBOARD_KEYS.u) {
      return WRITER_SHORTCUT_ACTIONS.underline;
    }
    if (key === KEYBOARD_KEYS.l) {
      return WRITER_SHORTCUT_ACTIONS.alignLeft;
    }
    if (key === KEYBOARD_KEYS.e) {
      return WRITER_SHORTCUT_ACTIONS.alignCenter;
    }
    if (key === KEYBOARD_KEYS.r) {
      return WRITER_SHORTCUT_ACTIONS.alignRight;
    }
    if (key === KEYBOARD_KEYS.j) {
      return WRITER_SHORTCUT_ACTIONS.alignJustify;
    }
    if (input.shiftKey && key === KEYBOARD_KEYS.greaterThan) {
      return WRITER_SHORTCUT_ACTIONS.increaseFontSize;
    }
    if (input.shiftKey && key === KEYBOARD_KEYS.lessThan) {
      return WRITER_SHORTCUT_ACTIONS.decreaseFontSize;
    }

    return null;
  }
}
