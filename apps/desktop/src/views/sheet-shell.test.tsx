// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-shell.test.tsx
// # 📌 Amac: Sheet grid aktif editor draft'inin async read-model guncellemelerinde korunmasini dogrular
// # 📌 Modul - FileType: Test - TSX
// Version: 0.4.1
// Aciklama: Format cache'i focus sonrasinda guncellense bile kullanicinin commit edilmemis input degerinin ezilmedigini regression testiyle sabitler
// Bagimli Oldugu Katman: View -> Controller -> Repo

import { render } from "solid-js/web";
import { afterEach, describe, expect, it } from "vitest";

import type { SheetController } from "../controllers/sheet.controller";
import { LanguageService } from "../language/language-service";
import { SheetSessionRepository } from "../repositories/sheet-session.repository";
import type { SheetDocumentView } from "./sheet-types";
import { SheetShell } from "./sheet-shell";

const DOCUMENT: SheetDocumentView = {
  id: "sheet-document-1",
  title: "Sheet",
  revision: 1,
  worksheets: [
    {
      id: "worksheet-1",
      name: "Sheet1",
      cellCount: 1,
      cells: [
        {
          row: 0,
          column: 0,
          value: { kind: "number", value: 12.34 },
        },
      ],
    },
  ],
  chartCount: 0,
};

let dispose: (() => void) | null = null;

afterEach(() => {
  dispose?.();
  dispose = null;
  document.body.innerHTML = "";
});

describe("SheetShell active cell draft", () => {
  it("keeps uncommitted input when async format cache updates", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);

    const controller = {
      initializeSession: async () => undefined,
      createDocument: async () => undefined,
      selectCell: async () => undefined,
      toggleBold: async () => undefined,
      toggleItalic: async () => undefined,
      toggleUnderline: async () => undefined,
      setHorizontalAlignment: async () => undefined,
      setDecimalPlaces: async () => undefined,
      commitCell: async () => undefined,
    } as unknown as SheetController;

    const root = document.createElement("div");
    document.body.append(root);
    dispose = render(
      () => (
        <SheetShell
          controller={controller}
          repository={repository}
          language={new LanguageService("en-US")}
          onSelectModule={() => undefined}
        />
      ),
      root,
    );

    const input = root.querySelector<HTMLInputElement>('input[aria-label="A1"]');
    expect(input).not.toBeNull();
    input!.focus();
    input!.value = "123";
    input!.dispatchEvent(new Event("input", { bubbles: true }));

    repository.setCellFormat("worksheet-1", "A1", {
      bold: false,
      italic: false,
      underline: false,
      horizontalAlignment: "general",
      decimalPlaces: 2,
    });
    await Promise.resolve();

    expect(input!.value).toBe("123");
  });
});
