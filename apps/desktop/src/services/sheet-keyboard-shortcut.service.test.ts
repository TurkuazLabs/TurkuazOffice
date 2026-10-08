// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/sheet-keyboard-shortcut.service.test.ts
// # 📌 Amac: Sheet keyboard shortcut routing ve context kurallarini regression testiyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.1.0
// Aciklama: Spreadsheet kisa yollarinin Ctrl/Meta, Alt, Shift ve IME durumlarinda dogru typed action'a cozuldugunu test eder
// Bagimli Oldugu Katman: Service -> Config

import { describe, expect, it } from "vitest";

import { SHEET_SHORTCUT_ACTIONS } from "../config/keyboard";
import {
  SheetKeyboardShortcutService,
  type SheetKeyboardShortcutInput,
} from "./sheet-keyboard-shortcut.service";

function input(
  key: string,
  overrides: Partial<SheetKeyboardShortcutInput> = {},
): SheetKeyboardShortcutInput {
  return {
    key,
    ctrlKey: true,
    metaKey: false,
    shiftKey: false,
    altKey: false,
    isComposing: false,
    ...overrides,
  };
}

describe("SheetKeyboardShortcutService", () => {
  const service = new SheetKeyboardShortcutService();

  it.each([
    ["n", SHEET_SHORTCUT_ACTIONS.newDocument],
    ["b", SHEET_SHORTCUT_ACTIONS.bold],
    ["i", SHEET_SHORTCUT_ACTIONS.italic],
    ["u", SHEET_SHORTCUT_ACTIONS.underline],
    ["Home", SHEET_SHORTCUT_ACTIONS.firstCell],
    ["1", SHEET_SHORTCUT_ACTIONS.toggleProperties],
  ])("maps Ctrl+%s to %s", (key, expected) => {
    expect(service.resolve(input(key))).toBe(expected);
  });

  it("maps Ctrl+Shift+L to the filter and sort panel", () => {
    expect(service.resolve(input("l", { shiftKey: true }))).toBe(
      SHEET_SHORTCUT_ACTIONS.toggleQuery,
    );
  });

  it("maps F2, Escape and Alt+= without Ctrl", () => {
    expect(
      service.resolve(
        input("F2", {
          ctrlKey: false,
        }),
      ),
    ).toBe(SHEET_SHORTCUT_ACTIONS.editCell);
    expect(
      service.resolve(
        input("Escape", {
          ctrlKey: false,
        }),
      ),
    ).toBe(SHEET_SHORTCUT_ACTIONS.cancelEdit);
    expect(
      service.resolve(
        input("=", {
          ctrlKey: false,
          altKey: true,
        }),
      ),
    ).toBe(SHEET_SHORTCUT_ACTIONS.insertSum);
  });

  it("accepts Meta as the platform modifier", () => {
    expect(
      service.resolve(input("b", { ctrlKey: false, metaKey: true })),
    ).toBe(SHEET_SHORTCUT_ACTIONS.bold);
  });

  it("blocks shortcuts during IME composition and unknown combinations", () => {
    expect(service.resolve(input("b", { isComposing: true }))).toBeNull();
    expect(service.resolve(input("q"))).toBeNull();
    expect(
      service.resolve(input("=", { ctrlKey: false, altKey: false })),
    ).toBeNull();
  });
});
