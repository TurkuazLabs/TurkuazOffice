// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/controllers/web-controller.ts
// # 📌 Amac: M3 Web View requestlerini Service katmanina iletir
// # 📌 Modul - FileType: Controller - TypeScript
// # Version: 0.4.0
// # Aciklama: Controller logic tutmadan bootstrap request sinirini saglar
// Bagimli Oldugu Katman: Controller

import type { WebBootstrapViewModel } from "../models/web-models";
import { WebBootstrapService } from "../services/web-bootstrap-service";

export class WebController {
  public constructor(private readonly service: WebBootstrapService) {}

  public initialize(): WebBootstrapViewModel {
    return this.service.initialize();
  }
}
