// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/web-wasm-runtime.tool.test.ts
// # 📌 Amac: Web WASM runtime loader Tool init ve adapter akislarini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// # Version: 0.4.0
// # Aciklama: Init tamamlanmadan capability okunmadigini ve module degerlerinin WasmCoreTool'a aktarildigini test eder
// Bagimli Oldugu Katman: Tool

import { describe, expect, it } from "vitest";

import type { WasmCoreModule } from "./web-core-tool";
import { WebWasmRuntimeTool } from "./web-wasm-runtime.tool";

describe("WebWasmRuntimeTool", () => {
  it("initializes generated binding before exposing the Core Tool", async () => {
    const calls: string[] = [];
    const wasmModule: WasmCoreModule = {
      web_core_abi_version: () => {
        calls.push("abi");
        return 1;
      },
      web_core_document_schema_version: () => 1,
      web_core_bridge_kind: () => "rust-wasm",
      web_core_native_file_system_access: () => false,
    };
    const runtime = new WebWasmRuntimeTool(async () => {
      calls.push("init");
    }, wasmModule);

    const coreTool = await runtime.load();
    expect(calls).toEqual(["init"]);

    expect(coreTool.capabilities()).toEqual({
      bridgeKind: "rust-wasm",
      abiVersion: 1,
      schemaVersion: 1,
      browserMetadataStorage: true,
      nativeFileSystemAccess: false,
    });
    expect(calls).toEqual(["init", "abi"]);
  });

  it("propagates initialization failure instead of silently falling back", async () => {
    const wasmModule: WasmCoreModule = {
      web_core_abi_version: () => 1,
      web_core_document_schema_version: () => 1,
      web_core_bridge_kind: () => "rust-wasm",
      web_core_native_file_system_access: () => false,
    };
    const runtime = new WebWasmRuntimeTool(
      async () => {
        throw new Error("wasm-init-failed");
      },
      wasmModule,
    );

    await expect(runtime.load()).rejects.toThrow("wasm-init-failed");
  });
});
