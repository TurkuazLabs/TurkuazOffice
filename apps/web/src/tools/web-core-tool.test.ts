// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/web-core-tool.test.ts
// # 📌 Amac: Browser fallback ve Rust WASM Core Tool capability adapter kontratini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// # Version: 0.4.0
// # Aciklama: ABI/schema/bridge mapping ve native-filesystem fail-closed davranisini test eder
// Bagimli Oldugu Katman: Tool

import { describe, expect, it } from "vitest";

import { WEB_CORE_NATIVE_FS_CONTRACT_ERROR } from "../config/runtime-config";
import {
  BrowserCoreContractTool,
  type WasmCoreModule,
  WasmCoreTool,
} from "./web-core-tool";

describe("WebCoreTool", () => {
  it("keeps the foundation browser contract explicit before generated WASM binding is loaded", () => {
    expect(new BrowserCoreContractTool().capabilities()).toEqual({
      bridgeKind: "browser-contract",
      abiVersion: 1,
      schemaVersion: 1,
      browserMetadataStorage: true,
      nativeFileSystemAccess: false,
    });
  });

  it("maps the Rust WASM capability module without duplicating core values", () => {
    const wasmModule: WasmCoreModule = {
      web_core_abi_version: () => 2,
      web_core_document_schema_version: () => 3,
      web_core_bridge_kind: () => "rust-wasm",
      web_core_native_file_system_access: () => false,
    };

    expect(new WasmCoreTool(wasmModule).capabilities()).toEqual({
      bridgeKind: "rust-wasm",
      abiVersion: 2,
      schemaVersion: 3,
      browserMetadataStorage: true,
      nativeFileSystemAccess: false,
    });
  });

  it("rejects a WASM module that claims native filesystem access", () => {
    const wasmModule: WasmCoreModule = {
      web_core_abi_version: () => 1,
      web_core_document_schema_version: () => 1,
      web_core_bridge_kind: () => "rust-wasm",
      web_core_native_file_system_access: () => true,
    };

    expect(() => new WasmCoreTool(wasmModule).capabilities()).toThrow(
      WEB_CORE_NATIVE_FS_CONTRACT_ERROR,
    );
  });
});
