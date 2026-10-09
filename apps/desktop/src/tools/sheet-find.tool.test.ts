// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/sheet-find.tool.test.ts
// # 📌 Amac: Sheet sparse hucrelerde read-only arama, filtre ve A-Z/100 satir sinirlarini dogrular
// # 📌 Modul - FileType: Tool Test - TypeScript
// Version: 0.1.0
// Aciklama: Turkce locale, formula/number/bool, gorunen satirlar, stabil siralama, mutasyonsuz bul
// Bagimli Oldugu Katman: Tool -> View Model

import { describe, expect, it } from "vitest";
import type { SheetCellView, SheetDocumentView } from "../views/sheet-types";
import { findSheetMatches, SHEET_FIND_MAX_QUERY_SCALARS } from "./sheet-find.tool";

function doc(...cells: readonly SheetCellView[]): SheetDocumentView {
  return {
    worksheets: [{ id: "w1", name: "Sheet1", cells, cellCount: cells.length }],
  } as unknown as SheetDocumentView;
}

describe("findSheetMatches", () => {
  it("finds and sorts sparse text, numeric, boolean and raw formula cells", () => {
    const model = doc(
      { row: 2, column: 1, value: { kind: "text", value: "sale 2026" } },
      { row: 0, column: 3, value: { kind: "formula", value: "=SUM(A1:A2026)" } },
      { row: 0, column: 0, value: { kind: "number", value: 2026 } },
      { row: 1, column: 1, value: { kind: "boolean", value: true } },
    );
    expect(findSheetMatches(model, "2026", "en-US", [0, 1, 2])).toEqual([
      { row: 0, column: 0 },
      { row: 0, column: 3 },
      { row: 2, column: 1 },
    ]);
    expect(findSheetMatches(model, "true", "en-US", [0, 1, 2])).toEqual([
      { row: 1, column: 1 },
    ]);
    expect(model.worksheets[0]?.cells).toHaveLength(4);
  });

  it("applies Turkish case matching and excludes filtered or off-grid rows/columns", () => {
    const model = doc(
      { row: 0, column: 0, value: { kind: "text", value: "ISPARTA" } },
      { row: 1, column: 0, value: { kind: "text", value: "İZMİR" } },
      { row: 99, column: 25, value: { kind: "text", value: "İZMİR" } },
      { row: 100, column: 0, value: { kind: "text", value: "İZMİR" } },
      { row: 0, column: 26, value: { kind: "text", value: "İZMİR" } },
    );
    expect(findSheetMatches(model, "izmir", "tr-TR", [0, 1, 99, 100])).toEqual([
      { row: 1, column: 0 },
      { row: 99, column: 25 },
    ]);
    expect(findSheetMatches(model, "isparta", "tr-TR", [0, 1])).toEqual([]);
    expect(findSheetMatches(model, "ısparta", "tr-TR", [0, 1])).toEqual([
      { row: 0, column: 0 },
    ]);
    expect(findSheetMatches(model, "izmir", "tr-TR", [0, 99])).toEqual([
      { row: 99, column: 25 },
    ]);
  });

  it("ignores empty and oversized queries and does not span cells", () => {
    const model = doc(
      { row: 0, column: 0, value: { kind: "text", value: "ab" } },
      { row: 0, column: 1, value: { kind: "text", value: "cd" } },
    );
    expect(findSheetMatches(model, "", "en-US", [0])).toEqual([]);
    expect(findSheetMatches(model, "a".repeat(SHEET_FIND_MAX_QUERY_SCALARS + 1), "en-US", [0])).toEqual([]);
    expect(findSheetMatches(model, "bc", "en-US", [0])).toEqual([]);
  });
});
