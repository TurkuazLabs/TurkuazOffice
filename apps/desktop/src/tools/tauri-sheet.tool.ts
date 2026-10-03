// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/tauri-sheet.tool.ts
// # 📌 Amac: Desktop frontend ile Tauri Rust backend arasindaki Sheet IPC adaptasyonunu yapar
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.4.0
// Aciklama: Create/get/text/number/boolean/formula/clear commandlarini typed olarak tasir
// Bagimli Oldugu Katman: Tool

import { invoke } from "@tauri-apps/api/core";

import { IPC_COMMANDS } from "../config/ipc-commands";
import type { SheetDocumentView } from "../views/sheet-types";

interface SheetCellCommandInput {
  readonly documentId: string;
  readonly worksheetId: string;
  readonly reference: string;
}

export class TauriSheetTool {
  public createDocument(): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetCreateDocument);
  }

  public getDocument(documentId: string): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetGetDocument, { documentId });
  }

  public setText(input: SheetCellCommandInput, value: string): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetSetText, { ...input, value });
  }

  public setNumber(input: SheetCellCommandInput, value: number): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetSetNumber, { ...input, value });
  }

  public setBoolean(input: SheetCellCommandInput, value: boolean): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetSetBoolean, { ...input, value });
  }

  public setFormula(input: SheetCellCommandInput, expression: string): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetSetFormula, { ...input, expression });
  }

  public clearCell(input: SheetCellCommandInput): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetClearCell, input);
  }
}
