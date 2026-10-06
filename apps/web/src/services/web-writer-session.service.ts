// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/services/web-writer-session.service.ts
// # 📌 Amac: Browser Writer TKO oturum lifecycle ve import/export is kurallarini yonetir
// # 📌 Modul - FileType: Service - TypeScript
// Version: 0.4.0
// Aciklama: Native TKO importunu session Repository'ye alir, aktif belgeyi canonical re-encode export hattina delege eder
// Bagimli Oldugu Katman: Service -> Repo -> Service

import { WEB_WRITER_SESSION_EMPTY_ERROR } from "../config/runtime-config";
import type { WebWriterSessionDocument } from "../models/web-models";
import type { WebWriterSessionRepository } from "../repositories/web-writer-session.repository";
import type { WebImportExportService } from "./web-import-export.service";

export class WebWriterSessionService {
  public constructor(
    private readonly repository: WebWriterSessionRepository,
    private readonly importExportService: WebImportExportService,
  ) {}

  public activeDocument(): WebWriterSessionDocument | null {
    return this.repository.active();
  }

  public async openNativeDocument(): Promise<WebWriterSessionDocument | null> {
    const imported = await this.importExportService.importNativeDocument();
    if (imported === null) {
      return null;
    }

    this.repository.replace(imported);
    return this.repository.active();
  }

  public exportActiveDocument(): void {
    const active = this.repository.active();
    if (active === null) {
      throw new Error(WEB_WRITER_SESSION_EMPTY_ERROR);
    }

    this.importExportService.exportNativeDocument(active.fileName, active.bytes);
  }

  public closeDocument(): void {
    this.repository.clear();
  }
}
