// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/web-core-tool.ts
// # 📌 Amac: Browser UI ile Rust WASM Core arasindaki typed Tool adapter sinirini tanimlar
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.4.0
// # Aciklama: Foundation fallback capability kontrati ile gelecekteki generated WASM module adapterini ayni Tool yuzeyinde toplar
// Bagimli Oldugu Katman: Tool

import {
  WEB_CORE_ABI_VERSION,
  WEB_CORE_BRIDGE_KIND,
  WEB_CORE_NATIVE_FS_CONTRACT_ERROR,
  WEB_CORE_SCHEMA_VERSION,
} from "../config/runtime-config";
import type { WebCoreCapabilities } from "../models/web-models";

export interface WebCoreTool {
  capabilities(): WebCoreCapabilities;
}

export interface WasmCoreModule {
  web_core_abi_version(): number;
  web_core_document_schema_version(): number;
  web_core_bridge_kind(): string;
  web_core_native_file_system_access(): boolean;
}

export class BrowserCoreContractTool implements WebCoreTool {
  public capabilities(): WebCoreCapabilities {
    return {
      bridgeKind: WEB_CORE_BRIDGE_KIND,
      abiVersion: WEB_CORE_ABI_VERSION,
      schemaVersion: WEB_CORE_SCHEMA_VERSION,
      browserMetadataStorage: true,
      nativeFileSystemAccess: false,
    };
  }
}

export class WasmCoreTool implements WebCoreTool {
  public constructor(private readonly wasmModule: WasmCoreModule) {}

  public capabilities(): WebCoreCapabilities {
    const nativeFileSystemAccess =
      this.wasmModule.web_core_native_file_system_access();
    if (nativeFileSystemAccess) {
      throw new Error(WEB_CORE_NATIVE_FS_CONTRACT_ERROR);
    }

    return {
      bridgeKind: this.wasmModule.web_core_bridge_kind(),
      abiVersion: this.wasmModule.web_core_abi_version(),
      schemaVersion: this.wasmModule.web_core_document_schema_version(),
      browserMetadataStorage: true,
      nativeFileSystemAccess: false,
    };
  }
}
