// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-paragraph.test.tsx
// # 📌 Amac: Writer Enter focus gecisinde stale blur commit tekrarini engeller
// # 📌 Modul - FileType: Test - TSX
// Version: 0.2.2
// Aciklama: Enter ile split basladiginda eski contenteditable paragraf blur olsa bile ikinci kez commit edilmedigini dogrular
// Bagimli Oldugu Katman: View -> Controller

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { LanguageService } from "../language/language-service";
import type { WriterParagraphView } from "./writer-types";
import { WriterParagraphEditor } from "./writer-paragraph";

const PARAGRAPH: WriterParagraphView = {
  id: "p1",
  plainText: "abc",
  style: { alignment: "left" },
  runs: [
    {
      id: "r1",
      text: "abc",
      style: {
        bold: false,
        italic: false,
        underline: false,
        fontFamily: "Arial",
        fontSizeHalfPoints: 22,
      },
    },
  ],
};

let dispose: (() => void) | null = null;

afterEach(() => {
  dispose?.();
  dispose = null;
  document.body.innerHTML = "";
});

describe("WriterParagraphEditor Enter", () => {
  it("does not re-commit stale paragraph DOM when split focus causes blur", async () => {
    let resolveSplit!: () => void;
    const splitPromise = new Promise<void>((resolve) => {
      resolveSplit = resolve;
    });
    const splitParagraphFromEditor = vi.fn().mockReturnValue(splitPromise);
    const commitParagraphFromEditor = vi.fn().mockResolvedValue(undefined);

    const controller = {
      restoreSelection: vi.fn().mockReturnValue(false),
      captureSelection: vi.fn().mockReturnValue(null),
      clearTypingStyle: vi.fn(),
      splitParagraphFromEditor,
      commitParagraphFromEditor,
      isCaretAtParagraphStart: vi.fn().mockReturnValue(false),
      mergeWithPreviousFromEditor: vi.fn().mockResolvedValue(undefined),
      copySelection: vi.fn(),
      cutSelection: vi.fn().mockResolvedValue(undefined),
      pasteSelection: vi.fn().mockResolvedValue(undefined),
    } as never;

    const root = document.createElement("div");
    document.body.append(root);
    dispose = render(
      () => (
        <WriterParagraphEditor
          paragraph={PARAGRAPH}
          controller={controller}
          language={new LanguageService("tr-TR")}
          canMergeWithPrevious={false}
          readOnly={false}
          renderScale={1}
          fontResolutions={[]}
        />
      ),
      root,
    );

    const editor = root.querySelector<HTMLDivElement>(".writer-paragraph");
    expect(editor).not.toBeNull();

    const event = new KeyboardEvent("keydown", {
      key: "Enter",
      bubbles: true,
      cancelable: true,
    });
    editor?.dispatchEvent(event);

    expect(event.defaultPrevented).toBe(true);
    expect(splitParagraphFromEditor).toHaveBeenCalledTimes(1);

    editor?.dispatchEvent(new FocusEvent("blur"));

    expect(commitParagraphFromEditor).not.toHaveBeenCalled();

    resolveSplit();
    await splitPromise;
    await Promise.resolve();
  });
});
