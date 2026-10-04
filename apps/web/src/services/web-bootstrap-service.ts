// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/services/web-bootstrap-service.ts
// # 📌 Amac: M3 Web uygulamasinin acilis is kurallarini yonetir
// # 📌 Modul - FileType: Service - TypeScript
// # Version: 0.4.0
// # Aciklama: Core capability ve browser metadata-index durumunu tek typed ViewModel olarak uretir
// Bagimli Oldugu Katman: Service

import { WEB_APP_VERSION } from "../config/runtime-config";
import type { WebBootstrapViewModel } from "../models/web-models";
import type { WebDocumentIndexRepository } from "../repositories/browser-document-index.repository";
import type { WebCoreTool } from "../tools/web-core-tool";

export class WebBootstrapService {
  public constructor(
    private readonly repository: WebDocumentIndexRepository,
    private readonly coreTool: WebCoreTool,
  ) {}

  public initialize(): WebBootstrapViewModel {
    return {
      version: WEB_APP_VERSION,
      capabilities: this.coreTool.capabilities(),
      storedDocumentCount: this.repository.list().length,
    };
  }
}
