// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-find-bar.test.tsx
// # 📌 Amac: Sheet Find panelinde query, hucre gezinmesi, sayac, wraparound ve Escape davranisi
// # 📌 Modul - FileType: View Test - TSX
// Version: 0.1.0
// Aciklama: Gercek panel eventlerini controller sonucu ve navigation callback uzerinden test eder
// Bagimli Oldugu Katman: View -> Controller -> Language

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";
import { LanguageService } from "../language/language-service";
import { SheetFindBar } from "./sheet-find-bar";

let dispose: (() => void) | null = null;
afterEach(() => {
  dispose?.();
  dispose = null;
  document.body.innerHTML = "";
});

describe("SheetFindBar", () => {
  it("shows match count and navigates first/next/previous with wrapping", async () => {
    const matches = [{ row: 0, column: 0 }, { row: 2, column: 1 }];
    const findMatches = vi.fn((query: string) => query === "sale" ? matches : []);
    const onNavigate = vi.fn().mockResolvedValue(undefined);
    const onClose = vi.fn();
    const root = document.createElement("div");
    document.body.append(root);
    dispose = render(() => (
      <SheetFindBar
        controller={{ findMatches } as never}
        language={new LanguageService("en-US")}
        visibleRows={[0, 1, 2]}
        onNavigate={onNavigate}
        onClose={onClose}
        inputRef={() => undefined}
      />
    ), root);

    const input = root.querySelector<HTMLInputElement>('input[type="search"]');
    expect(input).not.toBeNull();
    input!.value = "sale";
    input!.dispatchEvent(new Event("input", { bubbles: true }));
    expect(root.textContent).toContain("0 / 2");

    const next = root.querySelector<HTMLButtonElement>('[aria-label="Next"]');
    const previous = root.querySelector<HTMLButtonElement>('[aria-label="Previous"]');
    next!.click();
    await vi.waitFor(() => expect(root.textContent).toContain("1 / 2"));
    expect(onNavigate).toHaveBeenLastCalledWith(matches[0]);
    next!.click();
    await vi.waitFor(() => expect(root.textContent).toContain("2 / 2"));
    next!.click();
    await vi.waitFor(() => expect(root.textContent).toContain("1 / 2"));
    previous!.click();
    await vi.waitFor(() => expect(root.textContent).toContain("2 / 2"));
    expect(onNavigate).toHaveBeenCalledTimes(4);

    input!.dispatchEvent(new KeyboardEvent("keydown", {
      key: "Escape", bubbles: true, cancelable: true,
    }));
    expect(onClose).toHaveBeenCalledTimes(1);
    expect(findMatches).toHaveBeenCalledWith("sale", "en-US", [0, 1, 2]);
  });

  it("disables navigation for missing cells", () => {
    const root = document.createElement("div");
    document.body.append(root);
    dispose = render(() => (
      <SheetFindBar
        controller={{ findMatches: () => [] } as never}
        language={new LanguageService("en-US")}
        visibleRows={[0]}
        onNavigate={vi.fn()}
        onClose={vi.fn()}
        inputRef={() => undefined}
      />
    ), root);
    const input = root.querySelector<HTMLInputElement>("input");
    input!.value = "missing";
    input!.dispatchEvent(new Event("input", { bubbles: true }));
    expect(root.textContent).toContain("No matches");
    expect(root.querySelector<HTMLButtonElement>('[aria-label="Next"]')?.disabled).toBe(true);
  });
});
