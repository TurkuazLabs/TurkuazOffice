// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/shortcut-focus.tool.test.ts
// # 📌 Amac: Writer/Sheet editor kisayollarinin diger metin kontrollerine uygulanmamasini dogrular
// # 📌 Modul - FileType: Tool Test - TypeScript
// Version: 0.1.0
// Aciklama: Giris alani, formula bari, nested editor ve diger contenteditable odak davranislarini regression ile korur
// Bagimli Oldugu Katman: Tool -> Config

import { describe, expect, it } from "vitest";

import {
  SHEET_GRID_EDITOR_CLASS,
  SHEET_GRID_EDITOR_SELECTOR,
  WRITER_PARAGRAPH_MARKER_VALUE,
  WRITER_PARAGRAPH_SELECTOR,
} from "../config/dom-contract";
import { shortcutBelongsToOtherTextControl } from "./shortcut-focus.tool";

describe("suite shortcut focus ownership", () => {
  it("keeps Writer paragraph actions but yields to unrelated editors", () => {
    const editor = document.createElement("div");
    editor.contentEditable = "true";
    editor.dataset.writerParagraph = WRITER_PARAGRAPH_MARKER_VALUE;
    const run = document.createElement("span");
    editor.append(run);

    expect(shortcutBelongsToOtherTextControl(run, WRITER_PARAGRAPH_SELECTOR)).toBe(false);

    const search = document.createElement("input");
    const notes = document.createElement("textarea");
    const otherEditor = document.createElement("div");
    otherEditor.setAttribute("contenteditable", "true");
    expect(shortcutBelongsToOtherTextControl(search, WRITER_PARAGRAPH_SELECTOR)).toBe(true);
    expect(shortcutBelongsToOtherTextControl(notes, WRITER_PARAGRAPH_SELECTOR)).toBe(true);
    expect(shortcutBelongsToOtherTextControl(otherEditor, WRITER_PARAGRAPH_SELECTOR)).toBe(true);
  });

  it("preserves Sheet grid shortcuts while formula input keeps native editing", () => {
    const grid = document.createElement("input");
    grid.className = SHEET_GRID_EDITOR_CLASS;
    const formula = document.createElement("input");
    formula.className = "sheet-formula-bar__input";
    const cellTools = document.createElement("select");

    expect(shortcutBelongsToOtherTextControl(grid, SHEET_GRID_EDITOR_SELECTOR)).toBe(false);
    expect(shortcutBelongsToOtherTextControl(formula, SHEET_GRID_EDITOR_SELECTOR)).toBe(true);
    expect(shortcutBelongsToOtherTextControl(cellTools, SHEET_GRID_EDITOR_SELECTOR)).toBe(true);
  });

  it("does not hijack window-level shortcuts when no text control owns the event", () => {
    const button = document.createElement("button");
    expect(shortcutBelongsToOtherTextControl(button, WRITER_PARAGRAPH_SELECTOR)).toBe(false);
    expect(shortcutBelongsToOtherTextControl(window, SHEET_GRID_EDITOR_SELECTOR)).toBe(false);
    expect(shortcutBelongsToOtherTextControl(null, SHEET_GRID_EDITOR_SELECTOR)).toBe(false);
  });
});
