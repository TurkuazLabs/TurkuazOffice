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
        controller={{ findMatches, focusFindMatch, canReplaceFoundMatch: () => true, replaceFoundMatch: vi.fn(), canReplaceAllFoundMatches: () => false, replaceAllFoundMatches: vi.fn() } as never}
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
        controller={{ findMatches, focusFindMatch: vi.fn(), canReplaceFoundMatch: () => false, replaceFoundMatch: vi.fn(), canReplaceAllFoundMatches: () => false, replaceAllFoundMatches: vi.fn() } as never}
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
  it("replaces only the selected match and resets the counter through the real Writer adapter", async () => {
    const match = { paragraphId: "p1", startOffset: 0, endOffset: 3 };
    const findMatches = vi.fn((query: string) => query === "abc" ? [match] : []);
    const focusFindMatch = vi.fn().mockReturnValue(true);
    const replaceFoundMatch = vi.fn().mockResolvedValue(true);
    const root = document.createElement("div");
    document.body.append(root);
    dispose = render(() => (
      <WriterFindBar
        controller={{ findMatches, focusFindMatch, canReplaceFoundMatch: () => true, replaceFoundMatch, canReplaceAllFoundMatches: () => false, replaceAllFoundMatches: vi.fn() } as never}
        language={new LanguageService("en-US")}
        onClose={vi.fn()}
        inputRef={() => undefined}
      />
    ), root);

    const search = root.querySelector<HTMLInputElement>('input[type="search"]')!;
    search.value = "abc";
    search.dispatchEvent(new Event("input", { bubbles: true }));
    const replace = root.querySelector<HTMLButtonElement>('[aria-label="Replace Match"]')!;
    expect(replace.disabled).toBe(true);
    root.querySelector<HTMLButtonElement>('[aria-label="Next"]')!.click();
    expect(replace.disabled).toBe(false);

    const replacement = root.querySelector<HTMLInputElement>('[aria-label="Replacement text"]')!;
    replacement.value = "xyz";
    replacement.dispatchEvent(new Event("input", { bubbles: true }));
    replace.click();
    await vi.waitFor(() => expect(replaceFoundMatch).toHaveBeenCalledWith(
      match, "abc", "xyz", "en-US",
    ));
    await vi.waitFor(() => expect(root.textContent).toContain("0 / 1"));
  });

  it("does not offer Replace on a read-only Writer document", () => {
    const match = { paragraphId: "p1", startOffset: 0, endOffset: 3 };
    const root = document.createElement("div");
    document.body.append(root);
    const replaceFoundMatch = vi.fn();
    dispose = render(() => (
      <WriterFindBar
        controller={{
          findMatches: () => [match],
          focusFindMatch: () => true,
          canReplaceFoundMatch: () => false,
          replaceFoundMatch,
          canReplaceAllFoundMatches: () => false,
          replaceAllFoundMatches: vi.fn(),
        } as never}
        language={new LanguageService("en-US")}
        onClose={vi.fn()}
        inputRef={() => undefined}
      />
    ), root);
    const search = root.querySelector<HTMLInputElement>('input[type="search"]')!;
    search.value = "abc";
    search.dispatchEvent(new Event("input", { bubbles: true }));
    root.querySelector<HTMLButtonElement>('[aria-label="Next"]')!.click();
    const replace = root.querySelector<HTMLButtonElement>('[aria-label="Replace Match"]')!;
    expect(replace.disabled).toBe(true);
    replace.click();
    expect(replaceFoundMatch).not.toHaveBeenCalled();
  });

  it("offers atomic Replace All without requiring a selected match", async () => {
    const match = { paragraphId: "p1", startOffset: 0, endOffset: 3 };
    const findMatches = vi.fn((query: string) => query === "abc" ? [match] : []);
    const replaceAllFoundMatches = vi.fn().mockResolvedValue(true);
    const canReplaceAllFoundMatches = vi.fn(
      (query: string, replacement: string) => query === "abc" && replacement === "xyz",
    );
    const root = document.createElement("div");
    document.body.append(root);
    dispose = render(() => (
      <WriterFindBar
        controller={{
          findMatches, focusFindMatch: () => true,
          canReplaceFoundMatch: () => true, replaceFoundMatch: vi.fn(),
          canReplaceAllFoundMatches, replaceAllFoundMatches,
        } as never}
        language={new LanguageService("en-US")}
        onClose={vi.fn()}
        inputRef={() => undefined}
      />
    ), root);

    const query = root.querySelector<HTMLInputElement>('input[type="search"]')!;
    query.value = "abc";
    query.dispatchEvent(new Event("input", { bubbles: true }));
    const all = root.querySelector<HTMLButtonElement>('[aria-label="Replace All"]')!;
    expect(all.disabled).toBe(true);

    const replacement = root.querySelector<HTMLInputElement>('[aria-label="Replacement text"]')!;
    replacement.value = "xyz";
    replacement.dispatchEvent(new Event("input", { bubbles: true }));
    expect(all.disabled).toBe(false);
    expect(root.textContent).toContain("0 / 1");
    all.click();
    await vi.waitFor(() =>
      expect(replaceAllFoundMatches).toHaveBeenCalledExactlyOnceWith("abc", "xyz", "en-US"),
    );
    expect(canReplaceAllFoundMatches).toHaveBeenCalledWith("abc", "xyz", "en-US");
  });

  it("disables Replace All for read-only and incomplete plans", () => {
    const root = document.createElement("div");
    document.body.append(root);
    const replaceAllFoundMatches = vi.fn();
    dispose = render(() => (
      <WriterFindBar
        controller={{
          findMatches: () => [{ paragraphId: "p1", startOffset: 0, endOffset: 2 }],
          focusFindMatch: () => true,
          canReplaceFoundMatch: () => false, replaceFoundMatch: vi.fn(),
          canReplaceAllFoundMatches: () => false, replaceAllFoundMatches,
        } as never}
        language={new LanguageService("en-US")}
        onClose={vi.fn()}
        inputRef={() => undefined}
      />
    ), root);
    const input = root.querySelector<HTMLInputElement>('input[type="search"]')!;
    input.value = "aa";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    const all = root.querySelector<HTMLButtonElement>('[aria-label="Replace All"]')!;
    expect(all.disabled).toBe(true);
    all.click();
    expect(replaceAllFoundMatches).not.toHaveBeenCalled();
  });

});
