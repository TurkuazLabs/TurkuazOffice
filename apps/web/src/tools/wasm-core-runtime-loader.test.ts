// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/wasm-core-runtime-loader.test.ts
// # 📌 Amac: WASM runtime loader fallback, init ve fail-closed kontratini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Generated binding yuklenmesi, import hatasi fallback'i ve gecersiz capability reddini test eder
// Bagimli Oldugu Katman: Tool

import { describe, expect, it, vi } from "vitest";

import { WEB_CORE_NATIVE_FS_CONTRACT_ERROR } from "../config/runtime-config";
import {
  type WasmBindgenCoreModule,
  loadWebCoreTool,
} from "./wasm-core-runtime-loader";

function moduleFixture(
  overrides: Partial<WasmBindgenCoreModule> = {},
): WasmBindgenCoreModule {
  return {
    default: vi.fn(async () => undefined),
    web_core_abi_version: () => 1,
    web_core_document_schema_version: () => 1,
    web_core_bridge_kind: () => "rust-wasm",
    web_core_native_file_system_access: () => false,
    ...overrides,
  };
}

describe("loadWebCoreTool", () => {
  it("initializes a generated wasm-bindgen module before exposing capabilities", async () => {
    const module = moduleFixture();
    const importer = vi.fn(async () => module);

    const loaded = await loadWebCoreTool(importer);

    expect(loaded.source).toBe("wasm");
    expect(module.default).toHaveBeenCalledOnce();
    expect(loaded.tool.capabilities()).toMatchObject({
      bridgeKind: "rust-wasm",
      abiVersion: 1,
      schemaVersion: 1,
      nativeFileSystemAccess: false,
    });
  });

  it("uses the browser contract fallback when the generated binding is unavailable", async () => {
    const importer = vi.fn(async () => {
      throw new Error("binding unavailable");
    });

    const loaded = await loadWebCoreTool(importer);

    expect(loaded.source).toBe("browser-fallback");
    expect(loaded.tool.capabilities().bridgeKind).toBe("browser-contract");
  });

  it("fails closed when a loaded wasm binding claims native filesystem access", async () => {
    const module = moduleFixture({
      web_core_native_file_system_access: () => true,
    });

    await expect(loadWebCoreTool(async () => module)).rejects.toThrow(
      WEB_CORE_NATIVE_FS_CONTRACT_ERROR,
    );
  });

  it("rejects a loaded module with a missing generated binding function", async () => {
    const module = moduleFixture();
    Reflect.deleteProperty(module, "web_core_bridge_kind");

    await expect(loadWebCoreTool(async () => module)).rejects.toThrow(
      "WASM Core binding function eksik",
    );
  });
});
