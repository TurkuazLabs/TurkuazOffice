// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/suite-titlebar.test.tsx
// # 📌 Amac: Ortak suite baslik cubugunda Baslangic Merkezi terminolojisini sabitler
// # 📌 Modul - FileType: Test - TSX
// Version: 0.12.0
// Aciklama: Writer ve Sheet icin ortak geri donus dugmesinin ayni Baslangic Merkezi adini kullandigini dogrular
// Bagimli Oldugu Katman: View -> Language

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { LanguageService } from "../language/language-service";
import { SuiteTitlebar } from "./suite-titlebar";

let dispose: (() => void) | null = null;

afterEach(() => {
  dispose?.();
  dispose = null;
  document.body.innerHTML = "";
});

describe("SuiteTitlebar", () => {
  it.each(["writer", "sheet"] as const)(
    "uses Baslangic Merkezi as the shared %s return destination",
    (moduleIcon) => {
      const root = document.createElement("div");
      document.body.append(root);
      const onHome = vi.fn();

      dispose = render(
        () => (
          <SuiteTitlebar
            moduleIcon={moduleIcon}
            moduleName={moduleIcon === "writer" ? "Writer" : "Sheet"}
            documentName="Adsiz Belge"
            language={new LanguageService("tr-TR")}
            onHome={onHome}
          />
        ),
        root,
      );

      const button = root.querySelector<HTMLButtonElement>(".office-titlebar__home");
      expect(button).not.toBeNull();
      expect(button?.textContent).toContain("Baslangic Merkezi");
      expect(button?.title).toBe("Baslangic Merkezine don");

      button?.click();
      expect(onHome).toHaveBeenCalledTimes(1);
    },
  );
});
