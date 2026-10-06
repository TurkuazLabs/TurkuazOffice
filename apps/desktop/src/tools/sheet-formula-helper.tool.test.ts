// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/sheet-formula-helper.tool.test.ts
// # 📌 Amac: Sheet function katalogunun formul taslaklarina deterministic map edildigini dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.10.0
// Aciklama: Destekli SUM, AVERAGE, MIN, MAX ve IF taslaklarini regression testiyle kilitler
// Bagimli Oldugu Katman: Tool -> Config

import { describe, expect, it } from "vitest";

import { SHEET_FUNCTION_CATALOG } from "../config/sheet-functions";
import { SheetFormulaHelperTool } from "./sheet-formula-helper.tool";

describe("SheetFormulaHelperTool", () => {
  it("returns the configured draft for every supported function", () => {
    const tool = new SheetFormulaHelperTool();

    for (const definition of SHEET_FUNCTION_CATALOG) {
      expect(tool.draft(definition.id)).toBe(definition.draft);
    }
  });
});
