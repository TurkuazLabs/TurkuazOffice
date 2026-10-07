// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/writer-session-enter.service.test.ts
// # 📌 Amac: Writer Enter akisi sonrasi yeni paragraf secimini ve caret gecisini regression testiyle sabitler
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.2.2
// Aciklama: DOM metni commit edildikten sonra split sonucundaki yeni paragrafin basina selection/focus tasindigini dogrular
// Bagimli Oldugu Katman: Service -> Repo -> Tool

import { describe, expect, it, vi } from "vitest";

import { WriterSessionRepository } from "../repositories/writer-session.repository";
import { TextOffsetTool } from "../tools/text-offset.tool";
import type { WriterDocumentView, WriterParagraphView } from "../views/writer-types";
import { WriterSessionService } from "./writer-session.service";

const STYLE = {
  bold: false,
  italic: false,
  underline: false,
  fontFamily: "Arial",
  fontSizeHalfPoints: 22,
} as const;

const PAGE_SETTINGS = {
  widthTwips: 11906,
  heightTwips: 16838,
  marginTopTwips: 1440,
  marginRightTwips: 1440,
  marginBottomTwips: 1440,
  marginLeftTwips: 1440,
} as const;

function paragraph(id: string, runId: string, text: string): WriterParagraphView {
  return {
    id,
    plainText: text,
    style: { alignment: "left" },
    runs: [{ id: runId, text, style: STYLE }],
  };
}

function document(
  revision: number,
  paragraphs: readonly WriterParagraphView[],
): WriterDocumentView {
  return {
    id: "writer-document-1",
    title: "Enter Test",
    revision,
    plainText: paragraphs.map((item) => item.plainText).join("\n"),
    sectionCount: 1,
    pageSettings: PAGE_SETTINGS,
    paragraphs,
    images: [],
  };
}

describe("WriterSessionService Enter paragraph split", () => {
  it("moves the session selection and browser caret to the new paragraph", async () => {
    const repository = new WriterSessionRepository();
    repository.setNewDocument(document(0, [paragraph("p1", "r1", "")]));

    const committed = document(1, [paragraph("p1", "r1", "abc")]);
    const split = document(2, [
      paragraph("p1", "r1", "abc"),
      paragraph("p2", "r2", ""),
    ]);

    const replaceParagraphText = vi.fn().mockResolvedValue(committed);
    const splitParagraph = vi.fn().mockResolvedValue(split);
    const focusAndRestoreParagraphSelection = vi.fn().mockReturnValue(true);

    const service = new WriterSessionService(
      repository,
      { replaceParagraphText, splitParagraph } as never,
      new TextOffsetTool(),
      {
        readParagraphSelection: vi.fn().mockReturnValue({
          paragraphId: "p1",
          startOffset: 3,
          endOffset: 3,
        }),
        plainText: vi.fn().mockReturnValue("abc"),
        focusAndRestoreParagraphSelection,
      } as never,
      {} as never,
      {} as never,
      {} as never,
      {
        pageLayout: vi.fn().mockReturnValue({
          pageWidthPx: 794,
          pageHeightPx: 1123,
          marginTopPx: 96,
          marginRightPx: 96,
          marginBottomPx: 96,
          marginLeftPx: 96,
          scale: 1,
          zoomPercent: 100,
        }),
        resolveDocumentFonts: vi.fn().mockReturnValue([]),
      } as never,
      {} as never,
    );

    await service.splitParagraphFromEditor("p1", {} as HTMLElement);

    expect(replaceParagraphText).toHaveBeenCalledTimes(1);
    expect(replaceParagraphText).toHaveBeenCalledWith(
      "writer-document-1",
      "p1",
      "abc",
      null,
    );
    expect(splitParagraph).toHaveBeenCalledTimes(1);
    expect(splitParagraph).toHaveBeenCalledWith("writer-document-1", "p1", 3);
    expect(repository.document()).toBe(split);
    expect(repository.selection()).toEqual({
      paragraphId: "p2",
      startOffset: 0,
      endOffset: 0,
    });
    expect(focusAndRestoreParagraphSelection).toHaveBeenCalledTimes(1);
    expect(focusAndRestoreParagraphSelection).toHaveBeenCalledWith({
      paragraphId: "p2",
      startOffset: 0,
      endOffset: 0,
    });
  });
});
