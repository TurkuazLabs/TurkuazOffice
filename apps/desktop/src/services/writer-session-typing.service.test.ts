// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/writer-session-typing.service.test.ts
// # 📌 Amac: Writer canli yazim sirasinda gec gelen stale backend snapshot'inin UI modelini geri sarmasini engeller
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.2.5
// Aciklama: Ardisik typing commitlerinde yalniz aktif DOM ile uyumlu son snapshot'in Repository'ye publish edilmesini dogrular
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

function paragraph(text: string): WriterParagraphView {
  return {
    id: "p1",
    plainText: text,
    style: { alignment: "left" },
    runs: [{ id: "r1", text, style: STYLE }],
  };
}

function document(revision: number, text: string): WriterDocumentView {
  return {
    id: "writer-typing-test",
    title: "Typing",
    revision,
    plainText: text,
    sectionCount: 1,
    pageSettings: PAGE_SETTINGS,
    paragraphs: [paragraph(text)],
    images: [],
  };
}

describe("WriterSessionService live typing synchronization", () => {
  it("does not publish an older typing snapshot over newer active DOM text", async () => {
    const repository = new WriterSessionRepository();
    const initial = document(0, "a");
    repository.setNewDocument(initial);

    let liveText = "ab";
    const editor = {
      dataset: { writerParagraphId: "p1" },
    } as unknown as HTMLElement;

    let resolveFirst!: (value: WriterDocumentView) => void;
    const firstPromise = new Promise<WriterDocumentView>((resolve) => {
      resolveFirst = resolve;
    });

    let resolveSecond!: (value: WriterDocumentView) => void;
    const secondPromise = new Promise<WriterDocumentView>((resolve) => {
      resolveSecond = resolve;
    });

    const replaceParagraphText = vi
      .fn()
      .mockReturnValueOnce(firstPromise)
      .mockReturnValueOnce(secondPromise);

    const service = new WriterSessionService(
      repository,
      { replaceParagraphText } as never,
      new TextOffsetTool(),
      {
        plainText: vi.fn().mockImplementation(() => liveText),
        readParagraphSelection: vi.fn().mockImplementation(() => ({
          paragraphId: "p1",
          startOffset: liveText.length,
          endOffset: liveText.length,
        })),
        activeWriterParagraph: vi.fn().mockReturnValue(editor),
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

    const firstCommit = service.commitParagraphFromEditor("p1", editor);
    await vi.waitFor(() => {
      expect(replaceParagraphText).toHaveBeenCalledTimes(1);
    });

    liveText = "abc";
    const secondCommit = service.commitParagraphFromEditor("p1", editor);

    resolveFirst(document(1, "ab"));
    await firstCommit;

    expect(repository.document()).toBe(initial);

    await vi.waitFor(() => {
      expect(replaceParagraphText).toHaveBeenCalledTimes(2);
    });
    resolveSecond(document(2, "abc"));
    await secondCommit;

    expect(replaceParagraphText).toHaveBeenNthCalledWith(
      1,
      "writer-typing-test",
      "p1",
      "ab",
      null,
    );
    expect(replaceParagraphText).toHaveBeenNthCalledWith(
      2,
      "writer-typing-test",
      "p1",
      "abc",
      null,
    );
    expect(repository.document()?.revision).toBe(2);
    expect(repository.document()?.paragraphs[0]?.plainText).toBe("abc");
  });
});
