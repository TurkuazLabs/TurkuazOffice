// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/suite-find-bar.test.tsx
// # 📌 Amac: Writer/Sheet ortak Find klavye, wrap ve async focus akisini tek regression kontratiyla korur
// # 📌 Modul - FileType: View Test - TSX
// Version: 0.1.0
// Aciklama: Sync focus reject, async gezinme ve stale query engelini ayri ayri test eder
// Bagimli Oldugu Katman: View -> Language

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { LanguageService } from "../language/language-service";
import { SuiteFindBar, type SuiteFindBarLabels } from "./suite-find-bar";

const labels: SuiteFindBarLabels = {
  find: "writerFind",
  previous: "writerFindPrevious",
  next: "writerFindNext",
  close: "writerFindClose",
  noMatches: "writerFindNoMatches",
};

let dispose: (() => void) | null = null;
afterEach(() => {
  dispose?.();
  dispose = null;
  document.body.innerHTML = "";
});

function mountFindBar(
  findMatches: (query: string) => readonly string[],
  onNavigate: (match: string) => boolean | void | Promise<boolean | void>,
) {
  const root = document.createElement("div");
  document.body.append(root);
  dispose = render(() => (
    <SuiteFindBar
      className="writer-find-bar"
      labels={labels}
      language={new LanguageService("en-US")}
      maxQueryScalars={128}
      findMatches={findMatches}
      onNavigate={onNavigate}
      onClose={vi.fn()}
      inputRef={() => undefined}
    />
  ), root);
  const input = root.querySelector<HTMLInputElement>('input[type="search"]')!;
  const next = root.querySelector<HTMLButtonElement>('[aria-label="Next"]')!;
  return { root, input, next };
}

describe("SuiteFindBar shared navigation", () => {
  it("does not advance the counter when Writer rejects an unavailable paragraph", () => {
    const onNavigate = vi.fn().mockReturnValue(false);
    const { root, input, next } = mountFindBar(
      (query) => query === "test" ? ["paragraph-1"] : [],
      onNavigate,
    );

    input.value = "test";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    next.click();
    expect(onNavigate).toHaveBeenCalledWith("paragraph-1");
    expect(root.textContent).toContain("0 / 1");
  });

  it("advances after Sheet navigation finishes but not while it is pending", async () => {
    let release!: (value: void) => void;
    const pending = new Promise<void>((resolve) => { release = resolve; });
    const onNavigate = vi.fn().mockReturnValueOnce(pending).mockResolvedValue(undefined);
    const { root, input, next } = mountFindBar(
      (query) => query === "sale" ? ["A1", "B2"] : [],
      onNavigate,
    );

    input.value = "sale";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    next.click();
    next.click();
    expect(onNavigate).toHaveBeenCalledTimes(1);
    expect(root.textContent).toContain("0 / 2");

    release();
    await vi.waitFor(() => expect(root.textContent).toContain("1 / 2"));
    next.click();
    await vi.waitFor(() => expect(root.textContent).toContain("2 / 2"));
    expect(onNavigate).toHaveBeenLastCalledWith("B2");
  });

  it("ignores a stale navigation result after the search query changes", async () => {
    let release!: (value: void) => void;
    const onNavigate = vi.fn().mockImplementation(() =>
      new Promise<void>((resolve) => { release = resolve; }),
    );
    const { root, input, next } = mountFindBar(
      (query) => query === "first" ? ["A1"] : query === "second" ? ["C3"] : [],
      onNavigate,
    );

    input.value = "first";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    next.click();

    input.value = "second";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    release();
    await Promise.resolve();
    await Promise.resolve();
    expect(root.textContent).toContain("0 / 1");
  });
});
