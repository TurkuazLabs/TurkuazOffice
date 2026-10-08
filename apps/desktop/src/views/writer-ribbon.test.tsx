// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-ribbon.test.tsx
// # 📌 Amac: Writer klasik menu format komutlarinda editor selection ve focus geri yuklemesini sabitler
// # 📌 Modul - FileType: Test - TSX
// Version: 0.12.0
// Aciklama: Bicim menusunden inline format uygulandiktan sonra kayitli Writer selection'inin yeniden aktive edildigini dogrular
// Bagimli Oldugu Katman: View -> Controller -> Language

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { LanguageService } from "../language/language-service";
import { WriterRibbon } from "./writer-ribbon";

let dispose: (() => void) | null = null;

afterEach(() => {
  dispose?.();
  dispose = null;
  document.body.innerHTML = "";
});

describe("WriterRibbon classic format menu", () => {
  it("restores the Writer editor selection after a menu format command", async () => {
    const flushFocusedParagraph = vi.fn().mockResolvedValue(undefined);
    const restoreSessionSelection = vi.fn().mockReturnValue(true);
    const toggleBold = vi.fn().mockResolvedValue(undefined);

    const controller = {
      formatState: vi.fn().mockReturnValue({
        canFormat: true,
        bold: false,
        italic: false,
        underline: false,
        fontFamily: "Arial",
        fontSizeHalfPoints: 22,
        alignment: "left",
      }),
      templates: vi.fn().mockReturnValue([]),
      recentFiles: vi.fn().mockReturnValue([]),
      flushFocusedParagraph,
      restoreSessionSelection,
      toggleBold,
      toggleItalic: vi.fn().mockResolvedValue(undefined),
      toggleUnderline: vi.fn().mockResolvedValue(undefined),
      setFontFamily: vi.fn().mockResolvedValue(undefined),
      setFontSizeHalfPoints: vi.fn().mockResolvedValue(undefined),
      increaseFontSize: vi.fn().mockResolvedValue(undefined),
      decreaseFontSize: vi.fn().mockResolvedValue(undefined),
      setParagraphAlignment: vi.fn().mockResolvedValue(undefined),
      createDocument: vi.fn().mockResolvedValue(undefined),
      openDocument: vi.fn().mockResolvedValue(undefined),
      saveDocument: vi.fn().mockResolvedValue(undefined),
      saveDocumentAs: vi.fn().mockResolvedValue(undefined),
      importDocx: vi.fn().mockResolvedValue(undefined),
      exportDocx: vi.fn().mockResolvedValue(undefined),
      exportPdf: vi.fn().mockResolvedValue(undefined),
      openPrintPreview: vi.fn().mockResolvedValue(undefined),
      printDocument: vi.fn().mockResolvedValue(undefined),
      createDocumentFromTemplate: vi.fn().mockResolvedValue(undefined),
      openRecentFile: vi.fn().mockResolvedValue(undefined),
      undo: vi.fn().mockResolvedValue(undefined),
      redo: vi.fn().mockResolvedValue(undefined),
    } as never;

    const root = document.createElement("div");
    document.body.append(root);
    dispose = render(
      () => (
        <WriterRibbon
          controller={controller}
          language={new LanguageService("tr-TR")}
        />
      ),
      root,
    );

    const formatSummary = Array.from(root.querySelectorAll("summary")).find(
      (summary) => summary.textContent === "Bicim",
    );
    expect(formatSummary).not.toBeUndefined();
    formatSummary?.dispatchEvent(new MouseEvent("mousedown", { bubbles: true }));

    const boldButton = Array.from(root.querySelectorAll<HTMLButtonElement>(".writer-menu__popup button")).find(
      (button) => button.textContent?.startsWith("Kalin") === true,
    );
    expect(boldButton).not.toBeUndefined();
    boldButton?.click();

    await Promise.resolve();
    await Promise.resolve();

    expect(flushFocusedParagraph).toHaveBeenCalledTimes(1);
    expect(toggleBold).toHaveBeenCalledTimes(1);
    expect(restoreSessionSelection).toHaveBeenCalledTimes(1);
  });  
  it("shows shortcut hints and keeps print separate from print preview", () => {
    const printDocument = vi.fn().mockResolvedValue(undefined);
    const openPrintPreview = vi.fn().mockResolvedValue(undefined);
    const controller = {
      formatState: vi.fn().mockReturnValue({
        canFormat: true,
        bold: false,
        italic: false,
        underline: false,
        fontFamily: "Arial",
        fontSizeHalfPoints: 22,
        alignment: "left",
      }),
      templates: vi.fn().mockReturnValue([]),
      recentFiles: vi.fn().mockReturnValue([]),
      flushFocusedParagraph: vi.fn().mockResolvedValue(undefined),
      restoreSessionSelection: vi.fn().mockReturnValue(true),
      toggleBold: vi.fn().mockResolvedValue(undefined),
      toggleItalic: vi.fn().mockResolvedValue(undefined),
      toggleUnderline: vi.fn().mockResolvedValue(undefined),
      setFontFamily: vi.fn().mockResolvedValue(undefined),
      setFontSizeHalfPoints: vi.fn().mockResolvedValue(undefined),
      increaseFontSize: vi.fn().mockResolvedValue(undefined),
      decreaseFontSize: vi.fn().mockResolvedValue(undefined),
      setParagraphAlignment: vi.fn().mockResolvedValue(undefined),
      createDocument: vi.fn().mockResolvedValue(undefined),
      openDocument: vi.fn().mockResolvedValue(undefined),
      saveDocument: vi.fn().mockResolvedValue(undefined),
      saveDocumentAs: vi.fn().mockResolvedValue(undefined),
      importDocx: vi.fn().mockResolvedValue(undefined),
      exportDocx: vi.fn().mockResolvedValue(undefined),
      exportPdf: vi.fn().mockResolvedValue(undefined),
      printDocument,
      openPrintPreview,
      createDocumentFromTemplate: vi.fn().mockResolvedValue(undefined),
      openRecentFile: vi.fn().mockResolvedValue(undefined),
      undo: vi.fn().mockResolvedValue(undefined),
      redo: vi.fn().mockResolvedValue(undefined),
    } as never;

    const root = document.createElement("div");
    document.body.append(root);
    dispose = render(
      () => (
        <WriterRibbon
          controller={controller}
          language={new LanguageService("tr-TR")}
        />
      ),
      root,
    );

    expect(root.textContent).toContain("Ctrl+N");
    expect(root.textContent).toContain("Ctrl+Shift+S");
    expect(root.textContent).toContain("Ctrl+Shift+P");
    expect(root.textContent).toContain("Ctrl+Shift+>");
    expect(root.textContent).toContain("Ctrl+Shift+<");

    const printButton = Array.from(
      root.querySelectorAll<HTMLButtonElement>(".writer-menu__popup button"),
    ).find((button) => button.textContent?.endsWith("Ctrl+P") === true);
    expect(printButton).not.toBeUndefined();
    printButton?.click();

    expect(printDocument).toHaveBeenCalledTimes(1);
    expect(openPrintPreview).not.toHaveBeenCalled();
  });

});
