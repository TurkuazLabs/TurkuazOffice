// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/writer-replace-all.tool.test.ts
// # 📌 Amac: Writer Replace All preflight Unicode, overlap ve eksiksizlik garantilerini dogrular
// # 📌 Modul - FileType: Tool Test - TypeScript
// Version: 0.1.0
// Aciklama: Planin scalar offset, rich style, reverse order, max result ve read-only bagimsizligini denetler
// Bagimli Oldugu Katman: Tool -> canonical DTO

import { describe, expect, it } from "vitest";
import type {
  WriterCharacterStyleView,
  WriterDocumentView,
  WriterParagraphView,
  WriterRunView,
} from "../views/writer-types";
import {
  findWriterMatches,
  WRITER_FIND_MAX_RESULTS,
} from "./writer-find.tool";
import { planWriterReplaceAll } from "./writer-replace-all.tool";

const normal: WriterCharacterStyleView = {
  bold: false, italic: false, underline: false,
  fontFamily: "Arial", fontSizeHalfPoints: 24,
};
const bold = { ...normal, bold: true };

function paragraph(id: string, ...runs: readonly WriterRunView[]): WriterParagraphView {
  return {
    id,
    plainText: runs.map((run) => run.text).join(""),
    runs,
    style: { alignment: "left" },
  };
}

function documentModel(...paragraphs: readonly WriterParagraphView[]): WriterDocumentView {
  return {
    id: "doc1",
    revision: 7,
    title: "Test",
    plainText: paragraphs.map((p) => p.plainText).join("\n"),
    paragraphs,
  } as WriterDocumentView;
}

describe("planWriterReplaceAll (read-only)", () => {
  it("uses scalar offsets, preserves first matched run style and orders edits descending", () => {
    const model = documentModel(
      paragraph("p1",
        { id: "r1", text: "💠A ", style: normal },
        { id: "r2", text: "İSTANBUL", style: bold },
        { id: "r3", text: " and İstanbul", style: normal },
      ),
      paragraph("p2", { id: "r4", text: "İstanbul!", style: normal }),
    );
    const snapshot = JSON.stringify(model);
    const plan = planWriterReplaceAll(model, "istanbul", "Ankara", "tr-TR");
    expect(plan?.replacements.map((m) => ({
      paragraphId: m.paragraphId, start: m.startOffset, end: m.endOffset,
      style: m.runs[0]?.style,
    }))).toEqual([
      { paragraphId: "p2", start: 0, end: 8, style: normal },
      { paragraphId: "p1", start: 16, end: 24, style: normal },
      { paragraphId: "p1", start: 3, end: 11, style: bold },
    ]);
    expect(plan?.revision).toBe(7);
    expect(JSON.stringify(model)).toBe(snapshot);
  });

  it("chooses leftmost non-overlapping matches instead of applying conflicting spans", () => {
    const model = documentModel(paragraph(
      "p1", { id: "r1", text: "aaaaa", style: normal },
    ));
    const plan = planWriterReplaceAll(model, "aa", "b", "en-US");
    expect(plan?.replacements.map((m) => [m.startOffset, m.endOffset]))
      .toEqual([[2, 4], [0, 2]]);
    expect(findWriterMatches(model, "aa", "en-US")).toHaveLength(4);
  });

  it("never silently replaces only a truncated portion of a large document", () => {
    const model = documentModel(paragraph(
      "p1", { id: "r1", text: "a".repeat(WRITER_FIND_MAX_RESULTS + 1), style: normal },
    ));
    expect(findWriterMatches(model, "a", "en-US")).toHaveLength(WRITER_FIND_MAX_RESULTS);
    expect(findWriterMatches(model, "a", "en-US", WRITER_FIND_MAX_RESULTS + 1))
      .toHaveLength(WRITER_FIND_MAX_RESULTS + 1);
    expect(planWriterReplaceAll(model, "a", "b", "en-US")).toBeNull();
  });

  it("accepts the exact cap, and rejects no-op, invalid or inconsistent plans", () => {
    const model = documentModel(paragraph(
      "p1", { id: "r1", text: "a".repeat(WRITER_FIND_MAX_RESULTS), style: normal },
    ));
    expect(planWriterReplaceAll(model, "a", "b", "en-US")?.replacements)
      .toHaveLength(WRITER_FIND_MAX_RESULTS);
    expect(planWriterReplaceAll(model, "a", "a", "en-US")).toBeNull();
    expect(planWriterReplaceAll(model, "a", "a\nb", "en-US")).toBeNull();
    expect(planWriterReplaceAll(model, "", "b", "en-US")).toBeNull();

    const corrupted = {
      ...model,
      paragraphs: [{ ...model.paragraphs[0]!, plainText: "ab" }],
    } as WriterDocumentView;
    expect(planWriterReplaceAll(corrupted, "a", "b", "en-US")).toBeNull();
  });
});
