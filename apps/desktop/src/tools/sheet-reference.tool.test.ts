// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/sheet-reference.tool.test.ts
// # 📌 Amac: Desktop Sheet A1 referans uretimini regression testiyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
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
});
