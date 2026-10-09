// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-find-bar.test.tsx
// # 📌 Amac: Gercek Find panelinin query, onceki/sonraki ve Escape arayuz davranisini sabitler
// # 📌 Modul - FileType: View Test - TSX
// Version: 0.1.0
// Aciklama: Controller uzerinden selection odaklamasi, no-match ve wraparound semantigini test eder
// Bagimli Oldugu Katman: View -> Controller -> Language

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { LanguageService } from "../language/language-service";
import { WriterFindBar } from "./writer-find-bar";

let dispose: (() => void) | null = null;
afterEach(() => {
  dispose?.();
  dispose = null;
  document.body.innerHTML = "";
});

describe("WriterFindBar", () => {
  it("searches the document and navigates matches, wrapping around without edits", async () => {
    const matchA = { paragraphId: "p1", startOffset: 0, endOffset: 3 };
    const matchB = { paragraphId: "p2", startOffset: 2, endOffset: 5 };
    const findMatches = vi.fn((query: string) => query === "abc" ? [matchA, matchB] : []);
    const focusFindMatch = vi.fn().mockReturnValue(true);
    const onClose = vi.fn();
    const root = document.createElement("div");
    document.body.append(root);
    dispose = render(() => (
      <WriterFindBar
        controller={{ findMatches, focusFindMatch } as never}
        language={new LanguageService("en-US")}
        onClose={onClose}
        inputRef={() => undefined}
      />
    ), root);

    const input = root.querySelector<HTMLInputElement>('input[type="search"]');
    expect(input).not.toBeNull();
    input!.value = "abc";
    input!.dispatchEvent(new Event("input", { bubbles: true }));
    expect(root.textContent).toContain("0 / 2");

    const next = root.querySelector<HTMLButtonElement>('[aria-label="Next"]');
    const previous = root.querySelector<HTMLButtonElement>('[aria-label="Previous"]');
    expect(next).not.toBeNull();
    expect(previous).not.toBeNull();

    next!.click();
    expect(focusFindMatch).toHaveBeenLastCalledWith(matchA);
    expect(root.textContent).toContain("1 / 2");
    next!.click();
    expect(focusFindMatch).toHaveBeenLastCalledWith(matchB);
    next!.click();
    expect(focusFindMatch).toHaveBeenLastCalledWith(matchA);
    previous!.click();
    expect(focusFindMatch).toHaveBeenLastCalledWith(matchB);

    input!.dispatchEvent(new KeyboardEvent("keydown", {
      key: "Escape", bubbles: true, cancelable: true,
    }));
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("shows empty results and disables navigation until query matches", () => {
    const findMatches = vi.fn().mockReturnValue([]);
    const root = document.createElement("div");
    document.body.append(root);
    dispose = render(() => (
      <WriterFindBar
        controller={{ findMatches, focusFindMatch: vi.fn() } as never}
        language={new LanguageService("en-US")}
        onClose={vi.fn()}
        inputRef={() => undefined}
      />
    ), root);

    const input = root.querySelector<HTMLInputElement>("input");
    input!.value = "missing";
    input!.dispatchEvent(new Event("input", { bubbles: true }));
    expect(root.textContent).toContain("No matches");
    expect(root.querySelector<HTMLButtonElement>('[aria-label="Next"]')?.disabled).toBe(true);
    expect(root.querySelector<HTMLButtonElement>('[aria-label="Previous"]')?.disabled).toBe(true);
  });
});
