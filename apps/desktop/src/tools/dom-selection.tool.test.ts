// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/dom-selection.tool.test.ts
// # 📌 Amac: Writer paragraphlar arasi dikey caret hedefini gorsel X sutunundan cozer
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.2.2
// Aciklama: DOM caret-from-point sonucunun logical offset ve paragraph selection modeline donusturulmesini dogrular
// Bagimli Oldugu Katman: Tool

import { afterEach, describe, expect, it, vi } from "vitest";

import { WRITER_PARAGRAPH_MARKER_VALUE } from "../config/dom-contract";
import { DomSelectionTool } from "./dom-selection.tool";
import { TextOffsetTool } from "./text-offset.tool";

const originalCaretRangeFromPoint = (
  document as Document & { caretRangeFromPoint?: (x: number, y: number) => Range | null }
).caretRangeFromPoint;

afterEach(() => {
  document.body.innerHTML = "";
  const browserDocument = document as Document & {
    caretRangeFromPoint?: (x: number, y: number) => Range | null;
  };
  if (originalCaretRangeFromPoint === undefined) {
    delete browserDocument.caretRangeFromPoint;
  } else {
    browserDocument.caretRangeFromPoint = originalCaretRangeFromPoint;
  }
  vi.restoreAllMocks();
});

describe("DomSelectionTool adjacent paragraph selection", () => {
  it("projects the source caret X coordinate onto the target boundary line", () => {
    const source = document.createElement("div");
    source.dataset.writerParagraph = WRITER_PARAGRAPH_MARKER_VALUE;
    source.dataset.writerParagraphId = "p1";
    source.textContent = "abcdef";

    const target = document.createElement("div");
    target.dataset.writerParagraph = WRITER_PARAGRAPH_MARKER_VALUE;
    target.dataset.writerParagraphId = "p2";
    target.textContent = "uvwxyz";

    document.body.append(source, target);

    Object.defineProperty(source, "getBoundingClientRect", {
      value: () => ({
        x: 100,
        y: 100,
        left: 100,
        top: 100,
        right: 300,
        bottom: 130,
        width: 200,
        height: 30,
        toJSON: () => ({}),
      }),
    });
    Object.defineProperty(target, "getBoundingClientRect", {
      value: () => ({
        x: 100,
        y: 200,
        left: 100,
        top: 200,
        right: 300,
        bottom: 230,
        width: 200,
        height: 30,
        toJSON: () => ({}),
      }),
    });

    const sourceText = source.firstChild!;
    const selection = window.getSelection()!;
    const sourceRange = document.createRange();
    sourceRange.setStart(sourceText, 4);
    sourceRange.collapse(true);
    Object.defineProperty(sourceRange, "getBoundingClientRect", {
      value: () => ({
        x: 160,
        y: 104,
        left: 160,
        top: 104,
        right: 160,
        bottom: 124,
        width: 0,
        height: 20,
        toJSON: () => ({}),
      }),
    });
    selection.removeAllRanges();
    selection.addRange(sourceRange);

    const browserDocument = document as Document & {
      caretRangeFromPoint?: (x: number, y: number) => Range | null;
    };
    const caretRangeFromPoint = vi.fn((x: number, y: number) => {
      expect(x).toBe(160);
      expect(y).toBeGreaterThan(200);
      expect(y).toBeLessThan(230);
      const range = document.createRange();
      range.setStart(target.firstChild!, 2);
      range.collapse(true);
      return range;
    });
    browserDocument.caretRangeFromPoint = caretRangeFromPoint;

    const tool = new DomSelectionTool(new TextOffsetTool());
    const resolved = tool.adjacentParagraphSelection(source, "p2", "down");

    expect(caretRangeFromPoint).toHaveBeenCalledTimes(1);
    expect(resolved).toEqual({
      paragraphId: "p2",
      startOffset: 2,
      endOffset: 2,
    });
  });
});
