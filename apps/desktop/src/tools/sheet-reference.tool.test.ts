// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/sheet-reference.tool.test.ts
// # 📌 Amac: Desktop Sheet A1 referans uretimini regression testiyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.5.0
// Aciklama: One-based UI koordinat -> zero-based domain adresi ve A1 referans donusumlerini dogrular
// Bagimli Oldugu Katman: Tool

import { describe, expect, it } from "vitest";

import { SheetReferenceTool } from "./sheet-reference.tool";

describe("SheetReferenceTool", () => {
  it("maps one-based grid coordinates to zero-based domain addresses", () => {
    const tool = new SheetReferenceTool();

    expect(tool.domainAddress(1, 1)).toEqual({ row: 0, column: 0 });
    expect(tool.domainAddress(12, 27)).toEqual({ row: 11, column: 26 });
  });

  it("builds A1 references across column boundaries", () => {
    const tool = new SheetReferenceTool();

    expect(tool.reference(1, 1)).toBe("A1");
    expect(tool.reference(9, 26)).toBe("Z9");
    expect(tool.reference(12, 27)).toBe("AA12");
    expect(tool.reference(42, 52)).toBe("AZ42");
  });

  it("moves across grid cells with spreadsheet-style row wrapping", () => {
    const tool = new SheetReferenceTool();

    expect(tool.adjacentGridCell(1, 1, "right", 10, 3)).toMatchObject({
      row: 1,
      column: 2,
      reference: "B1",
      address: { row: 0, column: 1 },
    });
    expect(tool.adjacentGridCell(1, 3, "right", 10, 3)).toMatchObject({
      row: 2,
      column: 1,
      reference: "A2",
    });
    expect(tool.adjacentGridCell(2, 1, "left", 10, 3)).toMatchObject({
      row: 1,
      column: 3,
      reference: "C1",
    });
    expect(tool.adjacentGridCell(1, 1, "left", 10, 3).reference).toBe("A1");
    expect(tool.adjacentGridCell(10, 3, "down", 10, 3).reference).toBe("C10");
  });
});
