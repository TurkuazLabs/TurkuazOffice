// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/controllers/web-controller.ts
// # 📌 Amac: M3 Web View requestlerini Service katmanina iletir
// # 📌 Modul - FileType: Controller - TypeScript
// Version: 0.4.0
// Aciklama: Controller logic tutmadan bootstrap ve typed native TKO import/export requestlerini Service katmanina delege eder
// Bagimli Oldugu Katman: Controller -> Service

import type {
  WebBootstrapViewModel,
  WebNativeDocumentImport,
} from "../models/web-models";
import { WebBootstrapService } from "../services/web-bootstrap-service";
import { WebImportExportService } from "../services/web-import-export.service";

export class WebController {
  public constructor(
    private readonly bootstrapService: WebBootstrapService,
    private readonly importExportService: WebImportExportService,
  ) {}

  public initialize(): Promise<WebBootstrapViewModel> {
    return this.bootstrapService.initialize();
  }

  public importNativeDocument(): Promise<WebNativeDocumentImport | null> {
    return this.importExportService.importNativeDocument();
  }

  public exportNativeDocument(fileName: string, bytes: Uint8Array): void {
    this.importExportService.exportNativeDocument(fileName, bytes);
  }
}
