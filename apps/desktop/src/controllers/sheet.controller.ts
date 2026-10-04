// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/controllers/sheet.controller.ts
// # 📌 Amac: Sheet View requestlerini alip yalnizca SheetSessionService cagirir
// # 📌 Modul - FileType: Controller - TypeScript
// Version: 0.4.1
// Aciklama: Session, dirty-safe create, secili hucre, format, filter-sort ve cell commit requestleri icin ince Controller siniridir
// Bagimli Oldugu Katman: Controller -> Service

import type { SheetSessionService } from "../services/sheet-session.service";
import type {
  SheetFilterModeView,
  SheetHorizontalAlignmentView,
  SheetSortDirectionView,
} from "../views/sheet-types";

export class SheetController {
  public constructor(private readonly service: SheetSessionService) {}

  public initializeSession(): Promise<void> {
    return this.service.initializeSession();
  }

  public createDocument(): Promise<boolean> {
    return this.service.createDocument();
  }

  public selectCell(reference: string, row: number, column: number): Promise<void> {
    return this.service.selectCell(reference, row, column);
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

  public commitCell(reference: string, value: string): Promise<void> {
    return this.service.commitCell(reference, value);
  }
}
