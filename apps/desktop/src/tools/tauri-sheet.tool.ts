// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/tauri-sheet.tool.ts
// # 📌 Amac: Desktop frontend ile Tauri Rust backend arasindaki Sheet IPC adaptasyonunu yapar
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.11.0
// Aciklama: Create/get/format/table/conditional-format/chart/range-summary/query/evaluated-cell/text/number/boolean/formula/clear commandlarini typed olarak tasir
// Bagimli Oldugu Katman: Tool

import { invoke } from "@tauri-apps/api/core";

import { IPC_COMMANDS } from "../config/ipc-commands";
import type {
  SheetCellFormatView,
  SheetCellView,
  SheetChartCreateRequestView,
  SheetChartDataRequestView,
  SheetChartDataView,
  SheetChartRemoveRequestView,
  SheetConditionalFormatCreateRequestView,
  SheetConditionalFormatMatchView,
  SheetConditionalFormatMatchesRequestView,
  SheetConditionalFormatRemoveRequestView,
  SheetDocumentView,
  SheetRangeSummaryRequestView,
  SheetRangeSummaryView,
  SheetRowQueryRequestView,
  SheetRowQueryResultView,
  SheetTableCreateRequestView,
  SheetTableRemoveRequestView,
} from "../views/sheet-types";

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

  public getCellFormat(input: SheetCellCommandInput): Promise<SheetCellFormatView> {
    return invoke<SheetCellFormatView>(IPC_COMMANDS.sheetGetCellFormat, { ...input });
  }

  public setCellFormat(
    input: SheetCellCommandInput,
    format: SheetCellFormatView,
  ): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetSetCellFormat, { ...input, format });
  }

  public createTable(request: SheetTableCreateRequestView): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetCreateTable, { request });
  }

  public removeTable(request: SheetTableRemoveRequestView): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetRemoveTable, { request });
  }

  public createConditionalFormat(
    request: SheetConditionalFormatCreateRequestView,
  ): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetCreateConditionalFormat, { request });
  }

  public removeConditionalFormat(
    request: SheetConditionalFormatRemoveRequestView,
  ): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetRemoveConditionalFormat, { request });
  }

  public getConditionalFormatMatches(
    request: SheetConditionalFormatMatchesRequestView,
  ): Promise<readonly SheetConditionalFormatMatchView[]> {
    return invoke<readonly SheetConditionalFormatMatchView[]>(
      IPC_COMMANDS.sheetGetConditionalFormatMatches,
      { request },
    );
  }

  public createChart(request: SheetChartCreateRequestView): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetCreateChart, { request });
  }

  public removeChart(request: SheetChartRemoveRequestView): Promise<SheetDocumentView> {
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetRemoveChart, { request });
  }

  public getChartData(request: SheetChartDataRequestView): Promise<SheetChartDataView> {
    return invoke<SheetChartDataView>(IPC_COMMANDS.sheetGetChartData, { request });
  }

  public getRangeSummary(request: SheetRangeSummaryRequestView): Promise<SheetRangeSummaryView> {
    return invoke<SheetRangeSummaryView>(IPC_COMMANDS.sheetGetRangeSummary, { request });
  }

  public queryRows(request: SheetRowQueryRequestView): Promise<SheetRowQueryResultView> {
    return invoke<SheetRowQueryResultView>(IPC_COMMANDS.sheetQueryRows, { request });
  }

  public getEvaluatedCell(input: SheetCellCommandInput): Promise<SheetCellView | null> {
    return invoke<SheetCellView | null>(IPC_COMMANDS.sheetGetEvaluatedCell, { ...input });
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
    return invoke<SheetDocumentView>(IPC_COMMANDS.sheetClearCell, { ...input });
  }
}
