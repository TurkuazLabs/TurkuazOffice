// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/wasm-core-runtime-loader.test.ts
// # 📌 Amac: Aggregate WASM runtime loader Core/Writer TKO adapter, fallback, init ve fail-closed kontratini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Generated binding yuklenmesi, typed TKO Tool availability, import hatasi fallback'i ve eksik export reddini test eder
// Bagimli Oldugu Katman: Tool -> Config

import { describe, expect, it, vi } from "vitest";

import {
  WEB_CORE_NATIVE_FS_CONTRACT_ERROR,
  WEB_WASM_BINDING_FUNCTION_MISSING_ERROR,
} from "../config/runtime-config";
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
    web_writer_tko_inspect: () =>
      '{"id":"writer-1","title":"Belge","schema_version":1,"revision":2,"section_count":1,"asset_count":0}',
    web_writer_tko_reencode: () => new Uint8Array([4, 5]),
    ...overrides,
  };
}

describe("loadWebCoreTool", () => {
  it("initializes the aggregate wasm-bindgen module before exposing Core and Writer TKO tools", async () => {
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
    expect(loaded.writerTkoTool?.inspect(new Uint8Array([1]))).toMatchObject({
      id: "writer-1",
      title: "Belge",
      revision: 2,
    });
  });

  it("uses the browser contract fallback without claiming a native TKO Tool", async () => {
    const importer = vi.fn(async () => {
      throw new Error("binding unavailable");
    });

    const loaded = await loadWebCoreTool(importer);

    expect(loaded.source).toBe("browser-fallback");
    expect(loaded.tool.capabilities().bridgeKind).toBe("browser-contract");
    expect(loaded.writerTkoTool).toBeNull();
  });

  it("fails closed when a loaded wasm binding claims native filesystem access", async () => {
    const module = moduleFixture({
      web_core_native_file_system_access: () => true,
    });

    await expect(loadWebCoreTool(async () => module)).rejects.toThrow(
      WEB_CORE_NATIVE_FS_CONTRACT_ERROR,
    );
  });

  it("rejects a loaded module with a missing Core binding function", async () => {
    const module = moduleFixture();
    Reflect.deleteProperty(module, "web_core_bridge_kind");

    await expect(loadWebCoreTool(async () => module)).rejects.toThrow(
      WEB_WASM_BINDING_FUNCTION_MISSING_ERROR,
    );
  });

  it("rejects a loaded module with a missing Writer TKO binding function", async () => {
    const module = moduleFixture();
    Reflect.deleteProperty(module, "web_writer_tko_inspect");

    await expect(loadWebCoreTool(async () => module)).rejects.toThrow(
      WEB_WASM_BINDING_FUNCTION_MISSING_ERROR,
    );
  });
});
