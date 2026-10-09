// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/writer-session-find.service.test.ts
// # 📌 Amac: Canonical Writer Find sonucu secimi gercek DomSelectionTool ile geri yukler
// # 📌 Modul - FileType: Service Test - TypeScript
// Version: 0.1.0
// Aciklama: Unicode scalar secim, DOM range/focus, scroll ve read-only revision korunmasini dogrular
// Bagimli Oldugu Katman: Service -> Repo -> Tool

import { afterEach, describe, expect, it, vi } from "vitest";

import { WRITER_PARAGRAPH_MARKER_VALUE } from "../config/dom-contract";
import { LanguageService } from "../language/language-service";
import { WriterSessionRepository } from "../repositories/writer-session.repository";
import { DomSelectionTool } from "../tools/dom-selection.tool";
import { TextOffsetTool } from "../tools/text-offset.tool";
import type { WriterDocumentView } from "../views/writer-types";
import { WriterSessionService } from "./writer-session.service";

afterEach(() => {
  document.body.innerHTML = "";
  window.getSelection()?.removeAllRanges();
});

describe("WriterSessionService read-only Find", () => {
  it("uses the real DOM selection tool for Unicode text without changing revision", () => {
    const repository = new WriterSessionRepository();
    const documentModel = {
      id: "doc-1",
      revision: 7,
      paragraphs: [{ id: "p1", plainText: "💠alpha 💠alpha" }],
    } as unknown as WriterDocumentView;
    repository.setNewDocument(documentModel);

    const textOffsetTool = new TextOffsetTool();
    const domSelectionTool = new DomSelectionTool(textOffsetTool);
    const service = new WriterSessionService(
      repository,
      null as never,
      textOffsetTool,
      domSelectionTool,
      null as never,
      null as never,
      null as never,
      null as never,
      new LanguageService("en-US"),
    );

    const editor = document.createElement("div");
    editor.contentEditable = "true";
    editor.tabIndex = 0;
    editor.dataset.writerParagraph = WRITER_PARAGRAPH_MARKER_VALUE;
    editor.dataset.writerParagraphId = "p1";
    editor.textContent = "💠alpha 💠alpha";
    editor.scrollIntoView = vi.fn();
    document.body.append(editor);

    const results = service.findMatches("alpha", "en-US");
    expect(results).toEqual([
      { paragraphId: "p1", startOffset: 1, endOffset: 6 },
      { paragraphId: "p1", startOffset: 8, endOffset: 13 },
    ]);
    expect(service.focusFindMatch(results[1]!)).toBe(true);
    expect(repository.selection()).toEqual(results[1]);
    expect(window.getSelection()?.toString()).toBe("alpha");
    expect(editor.scrollIntoView).toHaveBeenCalledWith({ block: "nearest" });
    expect(repository.document()?.revision).toBe(7);
    expect(repository.dirty()).toBe(false);
    expect(service.focusFindMatch({ paragraphId: "p1", startOffset: 99, endOffset: 100 })).toBe(false);
  });
});
