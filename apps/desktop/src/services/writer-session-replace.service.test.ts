// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/writer-session-replace.service.test.ts
// # 📌 Amac: Writer tek eslesme Replace'inde read-only, revision, stale DOM, backend run-stili ve undo sinirlarini korur
// # 📌 Modul - FileType: Service Test - TypeScript
// Version: 0.1.0
// Aciklama: Gercek Service/Repo uzerinden tek Rust command cagrisi ve mutasyon ret davranislarini test eder
// Bagimli Oldugu Katman: Service -> Repo -> Tool

import { createRoot } from "solid-js";
import { afterEach, describe, expect, it, vi } from "vitest";
import { LanguageService } from "../language/language-service";
import { WriterSessionRepository } from "../repositories/writer-session.repository";
import { TextOffsetTool } from "../tools/text-offset.tool";
import type { WriterDocumentView, WriterFileSessionView } from "../views/writer-types";
import { WriterSessionService } from "./writer-session.service";

const STYLE = {
  bold: true, italic: false, underline: false,
  fontFamily: "Arial", fontSizeHalfPoints: 24,
};
const match = { paragraphId: "p1", startOffset: 1, endOffset: 6 };

function documentModel(revision: number, text: string): WriterDocumentView {
  return {
    id: "doc1", title: "Test", revision, plainText: text, sectionCount: 1,
    pageSettings: {
      widthTwips: 12000, heightTwips: 17000,
      marginTopTwips: 1000, marginRightTwips: 1000,
      marginBottomTwips: 1000, marginLeftTwips: 1000,
    },
    paragraphs: [{
      id: "p1", plainText: text, style: { alignment: "left" },
      runs: [{ id: "r1", text, style: STYLE }],
    }],
    images: [],
  };
}

let disposeRoot: (() => void) | null = null;
afterEach(() => {
  disposeRoot?.();
  disposeRoot = null;
  vi.restoreAllMocks();
});

function fixture() {
  return createRoot((dispose) => {
    disposeRoot = dispose;
    const repository = new WriterSessionRepository();
    repository.setNewDocument(documentModel(1, "💠alpha"));
    const replaceRangeWithStyledRuns = vi.fn().mockResolvedValue(documentModel(2, "💠beta"));
    const undo = vi.fn().mockResolvedValue(documentModel(3, "💠alpha"));
    const activeWriterParagraph = vi.fn().mockReturnValue(null);
    const plainText = vi.fn().mockReturnValue("💠alpha");
    const service = new WriterSessionService(
      repository,
      { replaceRangeWithStyledRuns, undo } as never,
      new TextOffsetTool(),
      { activeWriterParagraph, plainText } as never,
      {} as never,
      {} as never,
      {} as never,
      {
        pageLayout: vi.fn().mockReturnValue(null),
        resolveDocumentFonts: vi.fn().mockReturnValue([]),
      } as never,
      new LanguageService("en-US"),
    );
    return { service, repository, replaceRangeWithStyledRuns, undo, activeWriterParagraph, plainText };
  });
}

describe("WriterSessionService Replace one safety", () => {
  it("executes one styled Rust command and restores the source through existing undo", async () => {
    const { service, repository, replaceRangeWithStyledRuns, undo } = fixture();
    expect(service.canReplaceFoundMatch()).toBe(true);
    expect(await service.replaceFoundMatch(match, "alpha", "beta", "en-US")).toBe(true);
    expect(replaceRangeWithStyledRuns).toHaveBeenCalledExactlyOnceWith(
      "doc1", "p1", 1, 6, [{ text: "beta", style: STYLE }],
    );
    expect(repository.document()?.revision).toBe(2);
    expect(repository.dirty()).toBe(true);
    expect(repository.selection()).toEqual({ paragraphId: "p1", startOffset: 5, endOffset: 5 });
    await service.undo();
    expect(undo).toHaveBeenCalledExactlyOnceWith("doc1");
    expect(repository.document()?.paragraphs[0]?.plainText).toBe("💠alpha");
  });

  it("blocks read-only files, wrong matches, no-op replacements and stale revision", async () => {
    const { service, repository, replaceRangeWithStyledRuns } = fixture();
    repository.setFileSession({ readOnly: true } as WriterFileSessionView);
    expect(service.canReplaceFoundMatch()).toBe(false);
    expect(await service.replaceFoundMatch(match, "alpha", "beta", "en-US")).toBe(false);

    repository.setFileSession(null);
    expect(await service.replaceFoundMatch(match, "gamma", "beta", "en-US")).toBe(false);
    expect(await service.replaceFoundMatch(match, "alpha", "alpha", "en-US")).toBe(false);

    const pending = service.replaceFoundMatch(match, "alpha", "beta", "en-US");
    repository.setDocument(documentModel(2, "💠changed"));
    expect(await pending).toBe(false);
    expect(replaceRangeWithStyledRuns).not.toHaveBeenCalled();
  });

  it("blocks unsaved DOM typing to avoid overwriting a more recent live editor draft", async () => {
    const { service, replaceRangeWithStyledRuns, activeWriterParagraph, plainText } = fixture();
    activeWriterParagraph.mockReturnValue({
      dataset: { writerParagraphId: "p1" },
    });
    plainText.mockReturnValue("💠alpha plus live typing");
    expect(await service.replaceFoundMatch(match, "alpha", "beta", "en-US")).toBe(false);
    expect(replaceRangeWithStyledRuns).not.toHaveBeenCalled();
  });
});
