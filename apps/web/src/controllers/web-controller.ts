// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/controllers/web-controller.ts
// # 📌 Amac: M3 Web View requestlerini Service katmanina iletir
// # 📌 Modul - FileType: Controller - TypeScript
// Version: 0.4.0
// Aciklama: Controller logic tutmadan bootstrap ve Writer Web session lifecycle requestlerini Service katmanina delege eder
// Bagimli Oldugu Katman: Controller -> Service

import type {
  WebBootstrapViewModel,
  WebWriterSessionDocument,
} from "../models/web-models";
import { WebBootstrapService } from "../services/web-bootstrap-service";
import { WebWriterSessionService } from "../services/web-writer-session.service";

export class WebController {
  public constructor(
    private readonly bootstrapService: WebBootstrapService,
    private readonly writerSessionService: WebWriterSessionService,
  ) {}

  public initialize(): Promise<WebBootstrapViewModel> {
    return this.bootstrapService.initialize();
  }

  public activeWriterDocument(): WebWriterSessionDocument | null {
    return this.writerSessionService.activeDocument();
  }

  public openWriterDocument(): Promise<WebWriterSessionDocument | null> {
    return this.writerSessionService.openNativeDocument();
  }

  public exportWriterDocument(): void {
    this.writerSessionService.exportActiveDocument();
  }

  public closeWriterDocument(): void {
    this.writerSessionService.closeDocument();
  }
}
