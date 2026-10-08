// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/writer-session-navigation.service.test.ts
// # 📌 Amac: Writer yukari asagi yon tuslariyla paragraph siniri gecisini regression testiyle sabitler
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.2.4
// Aciklama: Caret yalniz ilk veya son gorsel satirdayken komsu paragrafa tasinir ve yatay logical offset korunur
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

function paragraph(id: string, text: string): WriterParagraphView {
  return {
    id,
    plainText: text,
    style: { alignment: "left" },
    runs: [{ id: `${id}-run`, text, style: STYLE }],
  };
}

function document(): WriterDocumentView {
  const paragraphs = [
    paragraph("p1", "alpha"),
    paragraph("p2", "bravo line"),
    paragraph("p3", "charlie"),
  ];
  return {
    id: "writer-navigation-test",
    title: "Navigation",
    revision: 0,
    plainText: paragraphs.map((item) => item.plainText).join("\n"),
    sectionCount: 1,
    pageSettings: PAGE_SETTINGS,
    paragraphs,
    images: [],
  };
}

function serviceFixture(boundary: boolean) {
  const repository = new WriterSessionRepository();
  repository.setNewDocument(document());
  const focusAndRestoreParagraphSelection = vi.fn().mockReturnValue(true);
  const domSelectionTool = {
    readParagraphSelection: vi.fn().mockReturnValue({
      paragraphId: "p2",
      startOffset: 4,
      endOffset: 4,
    }),
    isCaretAtVisualBoundary: vi.fn().mockReturnValue(boundary),
    adjacentParagraphSelection: vi.fn().mockReturnValue({
      paragraphId: "p3",
      startOffset: 2,
      endOffset: 2,
    }),
    focusAndRestoreParagraphSelection,
  };

  const service = new WriterSessionService(
    repository,
    {} as never,
    new TextOffsetTool(),
    domSelectionTool as never,
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

  return { repository, service, domSelectionTool, focusAndRestoreParagraphSelection };
}

describe("WriterSessionService vertical caret navigation", () => {
  it("moves down using the visual-column selection resolved by the DOM tool", async () => {
    const fixture = serviceFixture(true);

    const moved = fixture.service.moveCaretVerticallyFromEditor(
      "p2",
      {} as HTMLElement,
      "down",
    );

    expect(moved).toBe(true);
    expect(fixture.domSelectionTool.adjacentParagraphSelection).toHaveBeenCalledWith(
      expect.anything(),
      "p3",
      "down",
    );
    expect(fixture.repository.selection()).toEqual({
      paragraphId: "p3",
      startOffset: 2,
      endOffset: 2,
    });

    await Promise.resolve();

    expect(fixture.focusAndRestoreParagraphSelection).toHaveBeenCalledWith({
      paragraphId: "p3",
      startOffset: 2,
      endOffset: 2,
    });
  });

  it("lets the browser handle wrapped-line movement away from a visual boundary", () => {
    const fixture = serviceFixture(false);

    const moved = fixture.service.moveCaretVerticallyFromEditor(
      "p2",
      {} as HTMLElement,
      "up",
    );

    expect(moved).toBe(false);
    expect(fixture.repository.selection()).toBeNull();
    expect(fixture.focusAndRestoreParagraphSelection).not.toHaveBeenCalled();
  });
});
