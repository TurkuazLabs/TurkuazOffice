// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/controllers/sheet.controller.ts
// # 📌 Amac: Sheet View requestlerini alip yalnizca SheetSessionService cagirir
// # 📌 Modul - FileType: Controller - TypeScript
// Version: 0.11.0
// Aciklama: Session, chart, function draft, secili hucre/range, table, conditional format, format, filter-sort ve cell commit requestleri icin ince Controller siniridir
// Bagimli Oldugu Katman: Controller -> Service

import type { SheetChartType } from "../config/sheet-charts";
import type { SheetFunctionId } from "../config/sheet-functions";
import type { SheetFindMatch } from "../tools/sheet-find.tool";
import type { SheetSessionService } from "../services/sheet-session.service";
import type {
  SheetKeyboardShortcutInput,
  SheetKeyboardShortcutService,
} from "../services/sheet-keyboard-shortcut.service";
import type { SheetShortcutAction } from "../config/keyboard";
import type {
  SheetConditionalFormatModeView,
  SheetConditionalFormatStyleView,
  SheetFilterModeView,
  SheetHorizontalAlignmentView,
  SheetSortDirectionView,
} from "../views/sheet-types";

export class SheetController {
  public constructor(
    private readonly service: SheetSessionService,
    private readonly keyboardShortcutService: SheetKeyboardShortcutService,
  ) {}

  public resolveKeyboardShortcut(
    input: SheetKeyboardShortcutInput,
  ): SheetShortcutAction | null {
    return this.keyboardShortcutService.resolve(input);
  }

  public initializeSession(): Promise<void> {
    return this.service.initializeSession();
  }

  public createDocument(): Promise<boolean> {
    return this.service.createDocument();
  }

  public findMatches(
    query: string,
    locale: string,
    visibleRows: readonly number[],
  ): readonly SheetFindMatch[] {
    return this.service.findMatches(query, locale, visibleRows);
  }

  public selectCell(reference: string, row: number, column: number): Promise<void> {
    return this.service.selectCell(reference, row, column);
  }

  public extendSelection(row: number, column: number): Promise<void> {
    return this.service.extendSelection(row, column);
  }

  public createTableFromSelection(): Promise<void> {
    return this.service.createTableFromSelection();
  }

  public removeTableAtSelection(): Promise<void> {
    return this.service.removeTableAtSelection();
  }

  public applyConditionalFormat(
    mode: SheetConditionalFormatModeView,
    value: string,
    style: SheetConditionalFormatStyleView,
  ): Promise<void> {
    return this.service.applyConditionalFormat(mode, value, style);
  }

  public removeConditionalFormatAtSelection(): Promise<void> {
    return this.service.removeConditionalFormatAtSelection();
  }

  public createChartFromSelection(chartType: SheetChartType, title: string): Promise<void> {
    return this.service.createChartFromSelection(chartType, title);
  }

  public removeChart(chartId: string): Promise<void> {
    return this.service.removeChart(chartId);
  }

  public selectChart(chartId: string): Promise<void> {
    return this.service.selectChart(chartId);
  }

  public freezeAtSelection(): void {
    this.service.freezeAtSelection();
  }

  public freezeTopRow(): void {
    this.service.freezeTopRow();
  }

  public freezeFirstColumn(): void {
    this.service.freezeFirstColumn();
  }

  public unfreezePanes(): void {
    this.service.unfreezePanes();
  }

  public toggleBold(): Promise<void> {
    return this.service.toggleBold();
  }

  public toggleItalic(): Promise<void> {
    return this.service.toggleItalic();
  }

  public toggleUnderline(): Promise<void> {
    return this.service.toggleUnderline();
  }

  public setHorizontalAlignment(alignment: SheetHorizontalAlignmentView): Promise<void> {
    return this.service.setHorizontalAlignment(alignment);
  }

  public setDecimalPlaces(decimalPlaces: number | null): Promise<void> {
    return this.service.setDecimalPlaces(decimalPlaces);
  }

  public applyRowQuery(
    filterMode: SheetFilterModeView,
    filterValue: string,
    sortDirection: SheetSortDirectionView,
  ): Promise<void> {
    return this.service.applyRowQuery(filterMode, filterValue, sortDirection);
  }

  public clearRowQuery(): void {
    this.service.clearRowQuery();
  }

  public functionFormulaDraft(functionId: SheetFunctionId): string | null {
    return this.service.functionFormulaDraft(functionId);
  }

  public commitCell(reference: string, value: string): Promise<void> {
    return this.service.commitCell(reference, value);
  }
}
