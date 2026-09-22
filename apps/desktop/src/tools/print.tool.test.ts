// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/print.tool.test.ts
// # 📌 Amac: PrintTool fiziksel @page geometry donusumunu regression testi ile kilitler
// # 📌 Modul - FileType: Tool Test - TypeScript
// # Version: 0.2.0
// # Aciklama: Twip page boyutunun inch tabanli print rule'a canonical olarak aktarildigini dogrular
// Bagimli Oldugu Katman: Tool -> Config -> View

import { describe, expect, it } from "vitest";

import { PrintTool } from "./print.tool";

describe("PrintTool", () => {
  it("maps canonical twip page size to physical print inches", () => {
    const rule = new PrintTool().pageRule({
      widthTwips: 1440,
      heightTwips: 2880,
      marginTopTwips: 144,
      marginRightTwips: 144,
      marginBottomTwips: 144,
      marginLeftTwips: 144,
    });

    expect(rule).toBe("@page { size: 1in 2in; margin: 0; }");
  });
});
