// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/controllers/sheet.controller.ts
// # 📌 Amac: Sheet View requestlerini alip yalnizca SheetSessionService cagirir
// # 📌 Modul - FileType: Controller - TypeScript
// Version: 0.4.0
// Aciklama: Session baslatma, secili hucre, yeni Sheet ve cell commit requestleri icin ince Controller siniridir
// Bagimli Oldugu Katman: Controller -> Service

import type { SheetSessionService } from "../services/sheet-session.service";

export class SheetController {
  public constructor(private readonly service: SheetSessionService) {}

  public initializeSession(): Promise<void> {
    return this.service.initializeSession();
  }

  public createDocument(): Promise<void> {
    return this.service.createDocument();
  }

  public selectCell(reference: string, row: number, column: number): Promise<void> {
    return this.service.selectCell(reference, row, column);
  }

  public commitCell(reference: string, value: string): Promise<void> {
    return this.service.commitCell(reference, value);
  }
}
