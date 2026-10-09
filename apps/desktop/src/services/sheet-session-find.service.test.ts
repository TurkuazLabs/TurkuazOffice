// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/sheet-session-find.service.test.ts
// # 📌 Amac: SheetSessionService Find isleminde dirty, document revision ve secim degismedigini dogrular
// # 📌 Modul - FileType: Service Test - TypeScript
// Version: 0.1.0
// Aciklama: Gercek service read-model akisi gorunen satirlari kullanir ve belgeyi degistirmez
// Bagimli Oldugu Katman: Service -> Repo -> Tool

import { createRoot } from "solid-js";
import { describe, expect, it } from "vitest";
import { LanguageService } from "../language/language-service";
import { SheetSessionRepository } from "../repositories/sheet-session.repository";
import type { SheetDocumentView } from "../views/sheet-types";
import { SheetSessionService } from "./sheet-session.service";

describe("SheetSessionService read-only Find", () => {
  it("returns sorted visible matches without touching selection, revision or dirty", () => {
    createRoot((dispose) => {
      try {
        const repository = new SheetSessionRepository();
        const model = {
          id: "sheet1",
          revision: 9,
          worksheets: [{
            id: "w1", name: "Sheet1", cellCount: 2,
            cells: [
              { row: 1, column: 0, value: { kind: "text", value: "Istanbul" } },
              { row: 0, column: 2, value: { kind: "text", value: "Istanbul" } },
            ],
          }],
        } as unknown as SheetDocumentView;
        repository.setDocument(model);
        repository.markClean();

        const service = new SheetSessionService(
          repository,
          null as never,
          null as never,
          null as never,
          new LanguageService("tr-TR"),
        );

        expect(service.findMatches("ıstan", "tr-TR", [0, 1])).toEqual([
          { row: 0, column: 2 },
          { row: 1, column: 0 },
        ]);
        expect(service.findMatches("ıstan", "tr-TR", [1])).toEqual([
          { row: 1, column: 0 },
        ]);
        expect(repository.selection()).toBeNull();
        expect(repository.document()?.revision).toBe(9);
        expect(repository.dirty()).toBe(false);
      } finally {
        dispose();
      }
    });
  });
});
