// @vitest-environment jsdom
// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/writer-session-replace-all.service.test.ts
// # 📌 Amac: Writer tumunu degistir isleminde tam atomic IPC, stale revision ve readonly kontratini test eder
// # 📌 Modul - FileType: Service Test - TypeScript
// Version: 0.1.0
// Aciklama: Native islem oncesi mutation queue + live DOM + canonical plan dogrulamalari
// Bagimli Oldugu Katman: Service -> Repository -> Tauri Tool

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
function snapshot(revision: number, text: string): WriterDocumentView {
  return {
    id: "doc1", revision, title: "Replace All", plainText: text,
    sectionCount: 1,
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

function fixture(text = "💠alpha alpha") {
  return createRoot((dispose) => {
    disposeRoot = dispose;
    const repository = new WriterSessionRepository();
    repository.setNewDocument(snapshot(1, text));
    const replaceAllRanges = vi.fn().mockResolvedValue(snapshot(2, "💠beta beta"));
    const activeWriterParagraph = vi.fn().mockReturnValue(null);
    const plainText = vi.fn().mockReturnValue(text);
    const service = new WriterSessionService(
      repository,
      { replaceAllRanges } as never,
      new TextOffsetTool(),
      { activeWriterParagraph, plainText } as never,
      {} as never, {} as never, {} as never,
      {
        pageLayout: vi.fn().mockReturnValue(null),
        resolveDocumentFonts: vi.fn().mockReturnValue([]),
      } as never,
      new LanguageService("en-US"),
    );
    return { repository, service, replaceAllRanges, activeWriterParagraph, plainText };
  });
}

describe("WriterSessionService atomic Replace All", () => {
  it("submits descending Unicode spans in ONE revision-guarded native command", async () => {
    const { service, repository, replaceAllRanges } = fixture();
    repository.setSelection({ paragraphId: "p1", startOffset: 1, endOffset: 6 });
    expect(service.canReplaceAllFoundMatches("alpha", "beta", "en-US")).toBe(true);
    expect(await service.replaceAllFoundMatches("alpha", "beta", "en-US")).toBe(true);
    expect(replaceAllRanges).toHaveBeenCalledExactlyOnceWith("doc1", 1, [
      {
        paragraphId: "p1", startOffset: 7, endOffset: 12,
        runs: [{ text: "beta", style: STYLE }],
      },
      {
        paragraphId: "p1", startOffset: 1, endOffset: 6,
        runs: [{ text: "beta", style: STYLE }],
      },
    ]);
    expect(repository.document()?.revision).toBe(2);
    expect(repository.dirty()).toBe(true);
    expect(repository.selection()).toBeNull();
  });

  it("blocks read-only, no-op and over-limit batches before invoking native code", async () => {
    const { service, repository, replaceAllRanges } = fixture();
    repository.setFileSession({ readOnly: true } as WriterFileSessionView);
    expect(service.canReplaceAllFoundMatches("alpha", "beta", "en-US")).toBe(false);
    expect(await service.replaceAllFoundMatches("alpha", "beta", "en-US")).toBe(false);
    repository.setFileSession(null);
    expect(service.canReplaceAllFoundMatches("alpha", "alpha", "en-US")).toBe(false);
    expect(await service.replaceAllFoundMatches("alpha", "alpha", "en-US")).toBe(false);
    repository.setDocument(snapshot(2, "a".repeat(1001)));
    expect(service.canReplaceAllFoundMatches("a", "b", "en-US")).toBe(false);
    expect(await service.replaceAllFoundMatches("a", "b", "en-US")).toBe(false);
    expect(replaceAllRanges).not.toHaveBeenCalled();
  });

  it("rejects a stale revision while queued, even if the requested query was initially valid", async () => {
    const { service, repository, replaceAllRanges } = fixture();
    const pending = service.replaceAllFoundMatches("alpha", "beta", "en-US");
    repository.setDocument(snapshot(2, "💠different text"));
    expect(await pending).toBe(false);
    expect(replaceAllRanges).not.toHaveBeenCalled();
  });

  it("refuses to overwrite uncommitted contenteditable text", async () => {
    const { service, replaceAllRanges, activeWriterParagraph, plainText } = fixture();
    activeWriterParagraph.mockReturnValue({
      dataset: { writerParagraphId: "p1" },
    });
    plainText.mockReturnValue("💠alpha alpha newer typing");
    expect(await service.replaceAllFoundMatches("alpha", "beta", "en-US")).toBe(false);
    expect(replaceAllRanges).not.toHaveBeenCalled();
  });

  it("does not publish a partial snapshot when native atomic batch rejects", async () => {
    const { service, repository, replaceAllRanges } = fixture();
    replaceAllRanges.mockRejectedValue({ code: "writer.document_revision_conflict" });
    expect(await service.replaceAllFoundMatches("alpha", "beta", "en-US")).toBe(false);
    expect(repository.document()?.plainText).toBe("💠alpha alpha");
    expect(repository.document()?.revision).toBe(1);
  });
});
