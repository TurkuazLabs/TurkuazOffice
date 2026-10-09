// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/writer-replace.tool.test.ts
// # 📌 Amac: Single-match Replace planinin Unicode, run stili, revision ve guvenlik sinirlarini korur
// # 📌 Modul - FileType: Tool Test - TypeScript
// Version: 0.1.0
// Aciklama: Turkce locale, surrogate, old/invalid selection, formati koruma, bos silme, tutarsiz DTO
// Bagimli Oldugu Katman: Tool -> View Model

import { describe, expect, it } from "vitest";
import type { WriterDocumentView, WriterRunView } from "../views/writer-types";
import { planWriterReplaceOne } from "./writer-replace.tool";

const normal = { bold: false, italic: false, underline: false, fontFamily: "Arial", fontSizeHalfPoints: 24 };
const bold = { ...normal, bold: true };

function doc(...runs: readonly WriterRunView[]): WriterDocumentView {
  return {
    id: "doc1",
    revision: 17,
    paragraphs: [{
      id: "p1",
      plainText: runs.map((run) => run.text).join(""),
      runs,
      style: { alignment: "left" },
    }],
  } as unknown as WriterDocumentView;
}

describe("planWriterReplaceOne", () => {
  it("uses Unicode-scalar offsets and the style of the first matching character", () => {
    const model = doc(
      { id: "r1", text: "💠Hi ", style: normal },
      { id: "r2", text: "İstanbul", style: bold },
      { id: "r3", text: "!", style: normal },
    );
    const plan = planWriterReplaceOne(
      model, { paragraphId: "p1", startOffset: 4, endOffset: 12 },
      "istanbul", "Ankara", "tr-TR",
    );
    expect(plan).toEqual({
      documentId: "doc1",
      revision: 17,
      paragraphId: "p1",
      startOffset: 4,
      endOffset: 12,
      runs: [{ text: "Ankara", style: bold }],
    });
    expect(model.paragraphs[0]?.plainText).toBe("💠Hi İstanbul!");
  });

  it("supports deletion and a case-only replacement but not an unchanged no-op", () => {
    const model = doc({ id: "r1", text: "Hello", style: normal });
    const selection = { paragraphId: "p1", startOffset: 0, endOffset: 5 };
    expect(planWriterReplaceOne(model, selection, "hello", "", "en-US")?.runs).toEqual([]);
    expect(planWriterReplaceOne(model, selection, "hello", "hello", "en-US")?.runs)
      .toEqual([{ text: "hello", style: normal }]);
    expect(planWriterReplaceOne(model, selection, "hello", "Hello", "en-US")).toBeNull();
  });

  it("rejects stale text, out-of-range offsets, paragraph mismatch and newlines", () => {
    const model = doc({ id: "r1", text: "alpha", style: normal });
    const match = { paragraphId: "p1", startOffset: 0, endOffset: 5 };
    expect(planWriterReplaceOne(model, match, "beta", "z", "en-US")).toBeNull();
    expect(planWriterReplaceOne(model, { ...match, endOffset: 7 }, "alpha", "z", "en-US")).toBeNull();
    expect(planWriterReplaceOne(model, { ...match, paragraphId: "missing" }, "alpha", "z", "en-US")).toBeNull();
    expect(planWriterReplaceOne(model, { ...match, startOffset: -1 }, "alpha", "z", "en-US")).toBeNull();
    expect(planWriterReplaceOne(model, match, "alpha", "line\nbreak", "en-US")).toBeNull();
    expect(planWriterReplaceOne(model, match, "alpha", "a".repeat(4097), "en-US")).toBeNull();
  });

  it("rejects rich-text DTOs whose run boundaries no longer match canonical text", () => {
    const model = doc({ id: "r1", text: "alpha", style: normal });
    const stale = {
      ...model,
      paragraphs: [{ ...model.paragraphs[0]!, plainText: "alphabet" }],
    } as WriterDocumentView;
    expect(planWriterReplaceOne(
      stale, { paragraphId: "p1", startOffset: 0, endOffset: 5 },
      "alpha", "beta", "en-US",
    )).toBeNull();
  });
});
