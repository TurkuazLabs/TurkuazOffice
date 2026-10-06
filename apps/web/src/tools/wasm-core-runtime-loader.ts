// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/wasm-core-runtime-loader.ts
// # 📌 Amac: Generated wasm-bindgen aggregate modulunu browser runtime'inda yukleyip typed Core ve Writer TKO Tool adapterlerine donusturur
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.4.0
// Aciklama: WASM binding yukleme, init ve Core/Writer TKO kontrat dogrulamasini View/Service katmanlarindan izole eder
// Bagimli Oldugu Katman: Tool -> Config

import {
  WEB_CORE_WASM_MODULE_URL,
  WEB_WASM_BINDING_FUNCTION_MISSING_ERROR,
} from "../config/runtime-config";
import {
  BrowserCoreContractTool,
  type WasmCoreModule,
  type WebCoreTool,
  WasmCoreTool,
} from "./web-core-tool";
import {
  type WasmWriterTkoModule,
  type WebWriterTkoTool,
  WasmWriterTkoTool,
} from "./writer-tko.tool";

export interface WasmBindgenCoreModule
  extends WasmCoreModule,
    WasmWriterTkoModule {
  default(input?: unknown): Promise<unknown>;
}

export type WasmCoreModuleImporter = (
  moduleUrl: string,
) => Promise<WasmBindgenCoreModule>;

export interface LoadedWebCoreTool {
  readonly tool: WebCoreTool;
  readonly writerTkoTool: WebWriterTkoTool | null;
  readonly source: "wasm" | "browser-fallback";
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
    "web_writer_tko_inspect",
    "web_writer_tko_reencode",
  ];

  for (const name of requiredFunctions) {
    if (typeof module[name] !== "function") {
      throw new TypeError(
        `${WEB_WASM_BINDING_FUNCTION_MISSING_ERROR}: ${String(name)}`,
      );
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
      writerTkoTool: null,
      source: "browser-fallback",
    };
  }

  assertWasmModule(module);
  await module.default();

  const tool = new WasmCoreTool(module);
  tool.capabilities();

  return {
    tool,
    writerTkoTool: new WasmWriterTkoTool(module),
    source: "wasm",
  };
}
