// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/writer-find.tool.test.ts
// # 📌 Amac: Writer canonical find eslesmelerinin Unicode offset ve belge sinirlarini dogrular
// # 📌 Modul - FileType: Tool Test - TypeScript
// Version: 0.1.0
// Aciklama: Turkiye locale, emoji, overlapping, paragraph boundary, capped results ve mutasyonsuz tarama
// Bagimli Oldugu Katman: Tool -> View Model

import { describe, expect, it } from "vitest";

import type { WriterDocumentView } from "../views/writer-types";
import {
  findWriterMatches,
  WRITER_FIND_MAX_QUERY_SCALARS,
  WRITER_FIND_MAX_RESULTS,
} from "./writer-find.tool";

function documentWithParagraphs(...paragraphs: readonly string[]): WriterDocumentView {
  return {
    paragraphs: paragraphs.map((plainText, index) => ({
      id: `p${index + 1}`,
      plainText,
    })),
  } as unknown as WriterDocumentView;
}

describe("findWriterMatches", () => {
  it("finds multiple occurrences and preserves paragraph-local offsets", () => {
    const document = documentWithParagraphs("alpha alpha", "alpha", "unrelated");
    expect(findWriterMatches(document, "alpha", "en-US")).toEqual([
      { paragraphId: "p1", startOffset: 0, endOffset: 5 },
      { paragraphId: "p1", startOffset: 6, endOffset: 11 },
      { paragraphId: "p2", startOffset: 0, endOffset: 5 },
    ]);
    expect(document.paragraphs[0]?.plainText).toBe("alpha alpha");
  });

  it("uses Unicode scalar offsets after emoji instead of UTF-16 positions", () => {
    const document = documentWithParagraphs("💠a💠a");
    expect(findWriterMatches(document, "a", "en-US")).toEqual([
      { paragraphId: "p1", startOffset: 1, endOffset: 2 },
      { paragraphId: "p1", startOffset: 3, endOffset: 4 },
    ]);
    expect(findWriterMatches(document, "💠a", "en-US")).toEqual([
      { paragraphId: "p1", startOffset: 0, endOffset: 2 },
      { paragraphId: "p1", startOffset: 2, endOffset: 4 },
    ]);
  });

  it("handles Turkish I and dotted I according to selected locale", () => {
    const document = documentWithParagraphs("I ı İ i");
    expect(findWriterMatches(document, "I", "tr-TR")).toEqual([
      { paragraphId: "p1", startOffset: 0, endOffset: 1 },
      { paragraphId: "p1", startOffset: 2, endOffset: 3 },
    ]);
    expect(findWriterMatches(document, "i", "tr-TR")).toEqual([
      { paragraphId: "p1", startOffset: 4, endOffset: 5 },
      { paragraphId: "p1", startOffset: 6, endOffset: 7 },
    ]);
  });

  it("supports overlapping matches but never joins distinct paragraphs", () => {
    expect(findWriterMatches(documentWithParagraphs("aaaa", "bc"), "aa", "en-US")).toEqual([
      { paragraphId: "p1", startOffset: 0, endOffset: 2 },
      { paragraphId: "p1", startOffset: 1, endOffset: 3 },
      { paragraphId: "p1", startOffset: 2, endOffset: 4 },
    ]);
    expect(findWriterMatches(documentWithParagraphs("ab", "cd"), "bc", "en-US")).toEqual([]);
  });

  it("ignores empty or oversized queries and caps results", () => {
    const document = documentWithParagraphs("a".repeat(WRITER_FIND_MAX_RESULTS + 30));
    expect(findWriterMatches(document, "", "en-US")).toEqual([]);
    expect(findWriterMatches(document, "a".repeat(WRITER_FIND_MAX_QUERY_SCALARS + 1), "en-US")).toEqual([]);
    expect(findWriterMatches(document, "a", "en-US")).toHaveLength(WRITER_FIND_MAX_RESULTS);
  });
});
