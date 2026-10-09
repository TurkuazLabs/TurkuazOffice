// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/keyboard-contract.test.ts
// # 📌 Amac: Writer ve Sheet komut, gorunen kisayol ve ARIA tanimlarinin ayni kaynakla eslesmesini dogrular
// # 📌 Modul - FileType: Config Test - TypeScript
// Version: 0.1.0
// Aciklama: Ortak komut tablosundaki unutulan hint/ARIA kayitlarini ve birbiriyle cakisan eylem kimliklerini yakalar
// Bagimli Oldugu Katman: Config -> Service -> View

import { describe, expect, it } from "vitest";

import {
  SHEET_ARIA_SHORTCUTS,
  SHEET_SHORTCUT_ACTIONS,
  SHEET_SHORTCUT_HINTS,
  WRITER_ARIA_SHORTCUTS,
  WRITER_SHORTCUT_ACTIONS,
  WRITER_SHORTCUT_HINTS,
} from "./keyboard";

function sortedKeys(source: Record<string, unknown>): string[] {
  return Object.keys(source).sort();
}

function assertUniqueActionIds(actions: Record<string, string>): void {
  const identifiers = Object.values(actions);
  expect(new Set(identifiers).size).toBe(identifiers.length);
}

describe("suite keyboard command metadata contract", () => {
  it("keeps Writer action identifiers unique", () => {
    assertUniqueActionIds(WRITER_SHORTCUT_ACTIONS);
  });

  it("has a hint and ARIA binding for every visible Writer shortcut", () => {
    const visibleActions = Object.entries(WRITER_SHORTCUT_ACTIONS)
      .filter(([, action]) => action !== WRITER_SHORTCUT_ACTIONS.closePrintPreview)
      .map(([name]) => name)
      .sort();

    expect(sortedKeys(WRITER_SHORTCUT_HINTS)).toEqual(visibleActions);
    expect(sortedKeys(WRITER_ARIA_SHORTCUTS)).toEqual(visibleActions);

    for (const key of visibleActions) {
      expect(WRITER_SHORTCUT_HINTS[key as keyof typeof WRITER_SHORTCUT_HINTS].trim()).not.toBe("");
      expect(WRITER_ARIA_SHORTCUTS[key as keyof typeof WRITER_ARIA_SHORTCUTS].trim()).not.toBe("");
    }
  });

  it("keeps Sheet action identifiers unique", () => {
    assertUniqueActionIds(SHEET_SHORTCUT_ACTIONS);
  });

  it("keeps Sheet command and grid-navigation metadata paired", () => {
    expect(sortedKeys(SHEET_SHORTCUT_HINTS)).toEqual(sortedKeys(SHEET_ARIA_SHORTCUTS));

    for (const name of Object.keys(SHEET_SHORTCUT_ACTIONS)) {
      expect(SHEET_SHORTCUT_HINTS).toHaveProperty(name);
      expect(SHEET_ARIA_SHORTCUTS).toHaveProperty(name);
    }

    for (const hint of Object.values(SHEET_SHORTCUT_HINTS)) {
      expect(hint.trim()).not.toBe("");
    }
    for (const aria of Object.values(SHEET_ARIA_SHORTCUTS)) {
      expect(aria.trim()).not.toBe("");
    }
  });
});
