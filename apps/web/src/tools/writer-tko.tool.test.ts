// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/writer-tko.tool.test.ts
// # 📌 Amac: Writer TKO WASM typed Tool mapping ve fail-closed validation davranisini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Inspect JSON mapping, gecersiz cevap reddi ve re-encode byte izolasyonunu test eder
// Bagimli Oldugu Katman: Tool -> View -> Config

import { describe, expect, it, vi } from "vitest";

import { WEB_INVALID_TKO_SUMMARY_ERROR } from "../config/runtime-config";
import {
  type WasmWriterTkoModule,
  WasmWriterTkoTool,
} from "./writer-tko.tool";

function moduleFixture(
  overrides: Partial<WasmWriterTkoModule> = {},
): WasmWriterTkoModule {
  return {
    web_writer_tko_inspect: vi.fn(
      () =>
        '{"id":"writer-1","title":"Belge","schema_version":1,"revision":2,"section_count":3,"asset_count":4}',
    ),
    web_writer_tko_reencode: vi.fn(() => new Uint8Array([7, 8, 9])),
    ...overrides,
  };
}

describe("WasmWriterTkoTool", () => {
  it("maps the Rust inspect JSON into the typed Web summary", () => {
    const module = moduleFixture();
    const tool = new WasmWriterTkoTool(module);
    const bytes = new Uint8Array([1, 2]);

    expect(tool.inspect(bytes)).toEqual({
      id: "writer-1",
      title: "Belge",
      schemaVersion: 1,
      revision: 2,
      sectionCount: 3,
      assetCount: 4,
    });
    expect(module.web_writer_tko_inspect).toHaveBeenCalledWith(bytes);
  });

  it("rejects invalid inspect JSON instead of leaking an untyped value", () => {
    const tool = new WasmWriterTkoTool(
      moduleFixture({
        web_writer_tko_inspect: () => "{invalid",
      }),
    );

    expect(() => tool.inspect(new Uint8Array())).toThrow(
      WEB_INVALID_TKO_SUMMARY_ERROR,
    );
  });

  it("rejects unsafe or malformed numeric summary fields", () => {
    const tool = new WasmWriterTkoTool(
      moduleFixture({
        web_writer_tko_inspect: () =>
          '{"id":"writer-1","title":"Belge","schema_version":1,"revision":9007199254740992,"section_count":3,"asset_count":4}',
      }),
    );

    expect(() => tool.inspect(new Uint8Array())).toThrow(
      WEB_INVALID_TKO_SUMMARY_ERROR,
    );
  });

  it("passes re-encode arguments through and isolates returned bytes", () => {
    const encoded = new Uint8Array([7, 8, 9]);
    const module = moduleFixture({
      web_writer_tko_reencode: vi.fn(() => encoded),
    });
    const tool = new WasmWriterTkoTool(module);
    const input = new Uint8Array([1, 2]);

    const result = tool.reencode(input, "0.4.0");

    expect(module.web_writer_tko_reencode).toHaveBeenCalledWith(input, "0.4.0");
    expect(result).toEqual(encoded);
    expect(result).not.toBe(encoded);
  });
});
