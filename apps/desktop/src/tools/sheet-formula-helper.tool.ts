// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/sheet-formula-helper.tool.ts
// # 📌 Amac: Destekli Sheet fonksiyon kimligini formul cubugu icin guvenli taslak metnine cevirir
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.10.0
// Aciklama: Function katalogunu tek kaynaktan okuyup View katmanina magic string siz formul taslagi saglar
// Bagimli Oldugu Katman: Tool -> Config

import {
  SHEET_FUNCTION_CATALOG,
  type SheetFunctionId,
} from "../config/sheet-functions";

export class SheetFormulaHelperTool {
  public draft(functionId: SheetFunctionId): string {
    const definition = SHEET_FUNCTION_CATALOG.find(
      (candidate) => candidate.id === functionId,
    );
    if (definition === undefined) {
      return "";
    }
    return definition.draft;
  }
}
