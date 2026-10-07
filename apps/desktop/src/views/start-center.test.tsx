// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/start-center.test.tsx
// # 📌 Amac: Desktop Start Center modul gecisi ve ayri Writer/Sheet ikon kimligini dogrular
// # 📌 Modul - FileType: Test - TSX
// Version: 0.12.0
// Aciklama: Writer/Sheet kart click delegasyonu, menu metni ve ayri suite ikon class'larini jsdom ile sabitler
// Bagimli Oldugu Katman: View -> Language

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { LanguageService } from "../language/language-service";
import { StartCenter } from "./start-center";

let dispose: (() => void) | null = null;

function mount(onOpenWriter = vi.fn(), onOpenSheet = vi.fn()) {
  const root = document.createElement("div");
  document.body.append(root);
  dispose = render(
    () => (
      <StartCenter
        language={new LanguageService("tr-TR")}
        onOpenWriter={onOpenWriter}
        onOpenSheet={onOpenSheet}
      />
    ),
    root,
  );
  return { root, onOpenWriter, onOpenSheet };
}

afterEach(() => {
  dispose?.();
  dispose = null;
  document.body.innerHTML = "";
});

describe("StartCenter", () => {
  it("opens Writer and Sheet through their own launch cards", () => {
    const { root, onOpenWriter, onOpenSheet } = mount();
    const writer = root.querySelector<HTMLButtonElement>(".start-center__module-card--writer");
    const sheet = root.querySelector<HTMLButtonElement>(".start-center__module-card--sheet");

    expect(writer).not.toBeNull();
    expect(sheet).not.toBeNull();

    writer?.click();
    sheet?.click();

    expect(onOpenWriter).toHaveBeenCalledTimes(1);
    expect(onOpenSheet).toHaveBeenCalledTimes(1);
  });

  it("renders distinct Writer and Sheet product icon identities", () => {
    const { root } = mount();

    expect(root.querySelector(".suite-icon--writer")).not.toBeNull();
    expect(root.querySelector(".suite-icon--sheet")).not.toBeNull();
    expect(root.textContent).toContain("Baslangic Merkezi");
    expect(root.textContent).toContain("Son Belgeler");
    expect(root.textContent).toContain("Sablonlar");
  });
});
