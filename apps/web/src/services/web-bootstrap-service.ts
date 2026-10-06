// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/services/web-bootstrap-service.ts
// # 📌 Amac: M3 Web uygulamasinin acilis is kurallarini yonetir
// # 📌 Modul - FileType: Service - TypeScript
// # Version: 0.4.0
// # Aciklama: Core capability ve browser metadata-index durumunu tek typed ViewModel olarak uretir
// Bagimli Oldugu Katman: Service

import {
  WEB_APP_VERSION,
  WEB_DOCUMENT_STORAGE_KIND,
} from "../config/runtime-config";
import type { WebBootstrapViewModel } from "../models/web-models";
import type { WebCanonicalDocumentRepository } from "../repositories/indexeddb-document.repository";
import type { WebCoreTool } from "../tools/web-core-tool";

export class WebBootstrapService {
  public constructor(
    private readonly repository: WebCanonicalDocumentRepository,
    private readonly coreTool: WebCoreTool,
  ) {}

  public async initialize(): Promise<WebBootstrapViewModel> {
    try {
      return {
        version: WEB_APP_VERSION,
        capabilities: this.coreTool.capabilities(),
        documentStorageKind: WEB_DOCUMENT_STORAGE_KIND,
        documentStorageAvailable: true,
        storedDocumentCount: await this.repository.count(),
      };
    } catch {
      return {
        version: WEB_APP_VERSION,
        capabilities: this.coreTool.capabilities(),
        documentStorageKind: WEB_DOCUMENT_STORAGE_KIND,
        documentStorageAvailable: false,
        storedDocumentCount: null,
      };
    }
  }
}
