// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/web-core-tool.ts
// # 📌 Amac: Browser UI ile gelecekteki Rust WASM core arasindaki Tool sinirini tanimlar
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.4.0
// # Aciklama: Native API bagimliligi olmadan typed core ve metadata-storage capability handshake baseline'i saglar
// Bagimli Oldugu Katman: Tool

import {
  WEB_CORE_BRIDGE_KIND,
  WEB_CORE_SCHEMA_VERSION,
} from "../config/runtime-config";
import type { WebCoreCapabilities } from "../models/web-models";

export interface WebCoreTool {
  capabilities(): WebCoreCapabilities;
}

export class BrowserCoreContractTool implements WebCoreTool {
  public capabilities(): WebCoreCapabilities {
    return {
      bridgeKind: WEB_CORE_BRIDGE_KIND,
      schemaVersion: WEB_CORE_SCHEMA_VERSION,
      browserMetadataStorage: true,
      nativeFileSystemAccess: false,
    };
  }
}
