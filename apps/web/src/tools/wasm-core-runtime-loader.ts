// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/wasm-core-runtime-loader.ts
// # 📌 Amac: Generated wasm-bindgen modulunu browser runtime'inda yukleyip typed Core Tool adapterine donusturur
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.4.0
// Aciklama: WASM binding yukleme, init ve kontrat dogrulamasini View/Service katmanlarindan izole eder
// Bagimli Oldugu Katman: Tool

import { WEB_CORE_WASM_MODULE_URL } from "../config/runtime-config";
import {
  BrowserCoreContractTool,
  type WasmCoreModule,
  type WebCoreTool,
  WasmCoreTool,
} from "./web-core-tool";

export interface WasmBindgenCoreModule extends WasmCoreModule {
  default(input?: unknown): Promise<unknown>;
}

export type WasmCoreModuleImporter = (
  moduleUrl: string,
) => Promise<WasmBindgenCoreModule>;

export interface LoadedWebCoreTool {
  tool: WebCoreTool;
  source: "wasm" | "browser-fallback";
}

const defaultImporter: WasmCoreModuleImporter = async (moduleUrl) =>
  (await import(/* @vite-ignore */ moduleUrl)) as WasmBindgenCoreModule;

function assertWasmModule(module: WasmBindgenCoreModule): void {
  const requiredFunctions: Array<keyof WasmBindgenCoreModule> = [
    "default",
    "web_core_abi_version",
    "web_core_document_schema_version",
    "web_core_bridge_kind",
    "web_core_native_file_system_access",
  ];

  for (const name of requiredFunctions) {
    if (typeof module[name] !== "function") {
      throw new TypeError(`WASM Core binding function eksik: ${String(name)}`);
    }
  }
}

export async function loadWebCoreTool(
  importer: WasmCoreModuleImporter = defaultImporter,
  fallback: WebCoreTool = new BrowserCoreContractTool(),
): Promise<LoadedWebCoreTool> {
  let module: WasmBindgenCoreModule;
  try {
    module = await importer(WEB_CORE_WASM_MODULE_URL);
  } catch {
    return {
      tool: fallback,
      source: "browser-fallback",
    };
  }

  assertWasmModule(module);
  await module.default();

  const tool = new WasmCoreTool(module);
  tool.capabilities();

  return {
    tool,
    source: "wasm",
  };
}
