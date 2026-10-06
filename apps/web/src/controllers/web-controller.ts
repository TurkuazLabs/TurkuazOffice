// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/controllers/web-controller.ts
// # 📌 Amac: M3 Web View requestlerini Service katmanina iletir
// # 📌 Modul - FileType: Controller - TypeScript
// # Version: 0.4.0
// # Aciklama: Controller logic tutmadan bootstrap request sinirini saglar
// Bagimli Oldugu Katman: Controller

import type {
  WebBootstrapViewModel,
  WebPickedFile,
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

  public selectNativeImportFile(): Promise<WebPickedFile | null> {
    return this.importExportService.pickNativeDocumentBytes();
  }

  public downloadNativeExportFile(fileName: string, bytes: Uint8Array): void {
    this.importExportService.downloadNativeDocumentBytes(fileName, bytes);
  }
}
