// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/writer-tko.tool.ts
// # 📌 Amac: Generated Writer TKO WASM exportlarini typed TypeScript Tool sinirina adapte eder
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.4.0
// Aciklama: TKO inspect JSON cevabini dogrular ve re-encode byte ciktisini browser Service katmani icin typed hale getirir
// Bagimli Oldugu Katman: Tool -> View -> Config

import { WEB_INVALID_TKO_SUMMARY_ERROR } from "../config/runtime-config";
import type { WebWriterTkoSummary } from "../models/web-models";

export interface WebWriterTkoTool {
  inspect(bytes: Uint8Array): WebWriterTkoSummary;
  reencode(bytes: Uint8Array, appVersion: string): Uint8Array;
}

export interface WasmWriterTkoModule {
  web_writer_tko_inspect(bytes: Uint8Array): string;
  web_writer_tko_reencode(bytes: Uint8Array, appVersion: string): Uint8Array;
}

interface WasmWriterTkoSummary {
  readonly id: unknown;
  readonly title: unknown;
  readonly schema_version: unknown;
  readonly revision: unknown;
  readonly section_count: unknown;
  readonly asset_count: unknown;
}

function isNonNegativeSafeInteger(value: unknown): value is number {
  return (
    typeof value === "number" &&
    Number.isSafeInteger(value) &&
    value >= 0
  );
}

function parseSummary(raw: string): WebWriterTkoSummary {
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch {
    throw new Error(WEB_INVALID_TKO_SUMMARY_ERROR);
  }

  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
    throw new Error(WEB_INVALID_TKO_SUMMARY_ERROR);
  }

  const summary = parsed as WasmWriterTkoSummary;
  if (
    typeof summary.id !== "string" ||
    typeof summary.title !== "string" ||
    !isNonNegativeSafeInteger(summary.schema_version) ||
    !isNonNegativeSafeInteger(summary.revision) ||
    !isNonNegativeSafeInteger(summary.section_count) ||
    !isNonNegativeSafeInteger(summary.asset_count)
  ) {
    throw new Error(WEB_INVALID_TKO_SUMMARY_ERROR);
  }

  return {
    id: summary.id,
    title: summary.title,
    schemaVersion: summary.schema_version,
    revision: summary.revision,
    sectionCount: summary.section_count,
    assetCount: summary.asset_count,
  };
}

export class WasmWriterTkoTool implements WebWriterTkoTool {
  public constructor(private readonly wasmModule: WasmWriterTkoModule) {}

  public inspect(bytes: Uint8Array): WebWriterTkoSummary {
    return parseSummary(this.wasmModule.web_writer_tko_inspect(bytes));
  }

  public reencode(bytes: Uint8Array, appVersion: string): Uint8Array {
    return this.wasmModule.web_writer_tko_reencode(bytes, appVersion).slice();
  }
}
