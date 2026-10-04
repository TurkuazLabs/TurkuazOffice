// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/web-wasm-runtime.tool.ts
// # 📌 Amac: wasm-pack generated Rust binding'ini browser runtime'da initialize edip WebCoreTool'a adapte eder
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.4.0
// # Aciklama: Generated module lifecycle'ini View/Service disinda tutar ve basarili init sonrasinda WasmCoreTool uretir
// Bagimli Oldugu Katman: Tool

import initWasm, * as generatedWasmModule from "../generated/wasm-core/turkuaz_office_web_wasm";
import {
  type WasmCoreModule,
  WasmCoreTool,
  type WebCoreTool,
} from "./web-core-tool";

export type WasmRuntimeInitializer = () => Promise<unknown>;

export class WebWasmRuntimeTool {
  public constructor(
    private readonly initialize: WasmRuntimeInitializer,
    private readonly wasmModule: WasmCoreModule,
  ) {}

  public async load(): Promise<WebCoreTool> {
    await this.initialize();
    return new WasmCoreTool(this.wasmModule);
  }
}

export function createBrowserWasmRuntimeTool(): WebWasmRuntimeTool {
  return new WebWasmRuntimeTool(
    async () => {
      await initWasm();
    },
    generatedWasmModule,
  );
}
