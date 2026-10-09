// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/keyboard-shortcut.service.test.ts
// # 📌 Amac: Writer keyboard-only shortcut routing, context block ve print-preview davranisini regression testiyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.1
// Aciklama: Ctrl/Meta file-edit-format-zoom shortcutlari ile IME/recovery/preview context kurallarini smoke test eder
// Bagimli Oldugu Katman: Service -> Config

import { describe, expect, it } from "vitest";

import { WRITER_SHORTCUT_ACTIONS } from "../config/keyboard";
import {
  KeyboardShortcutService,
  type KeyboardShortcutInput,
} from "./keyboard-shortcut.service";

function input(
  key: string,
  overrides: Partial<KeyboardShortcutInput> = {},
): KeyboardShortcutInput {
  return {
    key,
    ctrlKey: true,
    metaKey: false,
    altKey: false,
    altGraphKey: false,
    shiftKey: false,
    isComposing: false,
    printPreviewOpen: false,
    recoveryBlocking: false,
    ...overrides,
  };
}

describe("KeyboardShortcutService", () => {
  const service = new KeyboardShortcutService();

  it.each([
    ["n", WRITER_SHORTCUT_ACTIONS.newDocument],
    ["o", WRITER_SHORTCUT_ACTIONS.open],
    ["s", WRITER_SHORTCUT_ACTIONS.save],
    ["p", WRITER_SHORTCUT_ACTIONS.print],
    ["z", WRITER_SHORTCUT_ACTIONS.undo],
    ["y", WRITER_SHORTCUT_ACTIONS.redo],
    ["b", WRITER_SHORTCUT_ACTIONS.bold],
    ["i", WRITER_SHORTCUT_ACTIONS.italic],
    ["u", WRITER_SHORTCUT_ACTIONS.underline],
    ["l", WRITER_SHORTCUT_ACTIONS.alignLeft],
    ["e", WRITER_SHORTCUT_ACTIONS.alignCenter],
    ["r", WRITER_SHORTCUT_ACTIONS.alignRight],
    ["j", WRITER_SHORTCUT_ACTIONS.alignJustify],
    ["+", WRITER_SHORTCUT_ACTIONS.zoomIn],
    ["=", WRITER_SHORTCUT_ACTIONS.zoomIn],
    ["-", WRITER_SHORTCUT_ACTIONS.zoomOut],
    ["0", WRITER_SHORTCUT_ACTIONS.zoomReset],
  ])("maps Ctrl+%s to %s", (key, expected) => {
    expect(service.resolve(input(key))).toBe(expected);
  });

  it("maps shifted save, print preview, undo and font-size alternatives", () => {
    expect(service.resolve(input("s", { shiftKey: true }))).toBe(
      WRITER_SHORTCUT_ACTIONS.saveAs,
    );
    expect(service.resolve(input("p", { shiftKey: true }))).toBe(
      WRITER_SHORTCUT_ACTIONS.printPreview,
    );
    expect(service.resolve(input("z", { shiftKey: true }))).toBe(
      WRITER_SHORTCUT_ACTIONS.redo,
    );
    expect(service.resolve(input(">", { shiftKey: true }))).toBe(
      WRITER_SHORTCUT_ACTIONS.increaseFontSize,
    );
    expect(service.resolve(input("<", { shiftKey: true }))).toBe(
      WRITER_SHORTCUT_ACTIONS.decreaseFontSize,
    );
  });

  it("accepts Meta as the platform modifier", () => {
    expect(
      service.resolve(input("o", { ctrlKey: false, metaKey: true })),
    ).toBe(WRITER_SHORTCUT_ACTIONS.open);
  });

  it("blocks edit shortcuts during IME composition", () => {
    expect(service.resolve(input("b", { isComposing: true }))).toBeNull();
  });

  it("does not intercept AltGr or Ctrl+Alt character entry", () => {
    expect(service.resolve(input("s", { altKey: true }))).toBeNull();
    expect(service.resolve(input("b", { altKey: true, altGraphKey: true }))).toBeNull();
    expect(service.resolve(input("p", { altKey: true, printPreviewOpen: true }))).toBeNull();
    expect(service.resolve(input("s", { altGraphKey: true }))).toBeNull();
  });

  it("blocks document shortcuts while startup recovery decision is active", () => {
    expect(service.resolve(input("n", { recoveryBlocking: true }))).toBeNull();
  });

  it("restricts print preview shortcuts to Escape and print", () => {
    expect(
      service.resolve(
        input("Escape", {
          ctrlKey: false,
          printPreviewOpen: true,
        }),
      ),
    ).toBe(WRITER_SHORTCUT_ACTIONS.closePrintPreview);
    expect(
      service.resolve(input("p", { printPreviewOpen: true })),
    ).toBe(WRITER_SHORTCUT_ACTIONS.print);
    expect(
      service.resolve(input("p", { printPreviewOpen: true, shiftKey: true })),
    ).toBeNull();
    expect(
      service.resolve(input("s", { printPreviewOpen: true })),
    ).toBeNull();
  });

  it("ignores unmodified and unknown keys", () => {
    expect(service.resolve(input("n", { ctrlKey: false }))).toBeNull();
    expect(service.resolve(input("q"))).toBeNull();
  });
});
