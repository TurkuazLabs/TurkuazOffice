// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/services/web-import-export.service.test.ts
// # 📌 Amac: Browser file-transfer ve Writer TKO Tool composition kurallarini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Codec availability, TKO inspect import, canonical re-encode export, limit/extension ve fallback davranislarini Tool fake'leri ile test eder
// Bagimli Oldugu Katman: Service -> Tool -> Config

import { describe, expect, it, vi } from "vitest";

import {
  WEB_APP_VERSION,
  WEB_BROWSER_IMPORT_MAX_BYTES,
  WEB_IMPORT_FILE_TOO_LARGE_ERROR,
  WEB_NATIVE_DOCUMENT_ACCEPT,
  WEB_NATIVE_DOCUMENT_MIME_TYPE,
  WEB_TKO_CODEC_UNAVAILABLE_ERROR,
  WEB_UNSUPPORTED_IMPORT_FILE_ERROR,
} from "../config/runtime-config";
import type {
  WebDownloadFile,
  WebPickedFile,
  WebWriterTkoSummary,
} from "../models/web-models";
import type { WebWriterTkoTool } from "../tools/writer-tko.tool";
import { WebImportExportService } from "./web-import-export.service";

function fileToolFixture(picked: WebPickedFile | null) {
  const pickFile = vi.fn(async () => picked);
  const downloadFile = vi.fn((_file: WebDownloadFile): void => undefined);
  return { pickFile, downloadFile };
}

function codecFixture() {
  const inspect = vi.fn(
    (_bytes: Uint8Array): WebWriterTkoSummary => ({
      id: "writer-1",
      title: "Belge",
      schemaVersion: 1,
      revision: 2,
      sectionCount: 1,
      assetCount: 0,
    }),
  );
  const reencode = vi.fn(
    (bytes: Uint8Array, _appVersion: string): Uint8Array => bytes.slice(),
  );
  const codec: WebWriterTkoTool = { inspect, reencode };
  return { ...codec, inspect, reencode };
}

describe("WebImportExportService", () => {
  it("reports codec availability only when the typed Writer TKO Tool exists", () => {
    expect(
      new WebImportExportService(fileToolFixture(null), null).capabilities(),
    ).toEqual({
      browserFileTransferAvailable: true,
      nativeTkoCodecAvailable: false,
    });

    expect(
      new WebImportExportService(
        fileToolFixture(null),
        codecFixture(),
      ).capabilities(),
    ).toEqual({
      browserFileTransferAvailable: true,
      nativeTkoCodecAvailable: true,
    });
  });

  it("fails before opening a file picker when the WASM TKO codec is unavailable", async () => {
    const fileTool = fileToolFixture(null);
    const service = new WebImportExportService(fileTool, null);

    await expect(service.importNativeDocument()).rejects.toThrow(
      WEB_TKO_CODEC_UNAVAILABLE_ERROR,
    );
    expect(fileTool.pickFile).not.toHaveBeenCalled();
  });

  it("selects, validates and inspects a native TKO document", async () => {
    const picked = {
      name: "Belge.TKO",
      mediaType: WEB_NATIVE_DOCUMENT_MIME_TYPE,
      bytes: new Uint8Array([1, 2]),
    };
    const fileTool = fileToolFixture(picked);
    const codec = codecFixture();
    const service = new WebImportExportService(fileTool, codec);

    const imported = await service.importNativeDocument();

    expect(fileTool.pickFile).toHaveBeenCalledWith({
      accept: WEB_NATIVE_DOCUMENT_ACCEPT,
      maxBytes: WEB_BROWSER_IMPORT_MAX_BYTES,
    });
    expect(codec.inspect).toHaveBeenCalledWith(picked.bytes);
    expect(imported).toMatchObject({
      fileName: "Belge.TKO",
      summary: {
        id: "writer-1",
        title: "Belge",
        revision: 2,
      },
    });
    expect(imported?.bytes).toEqual(picked.bytes);
    expect(imported?.bytes).not.toBe(picked.bytes);
  });

  it("returns null on picker cancellation without calling the codec", async () => {
    const codec = codecFixture();
    const service = new WebImportExportService(fileToolFixture(null), codec);

    await expect(service.importNativeDocument()).resolves.toBeNull();
    expect(codec.inspect).not.toHaveBeenCalled();
  });

  it("rechecks returned byte length before calling the codec", async () => {
    const codec = codecFixture();
    const service = new WebImportExportService(
      fileToolFixture({
        name: "large.tko",
        mediaType: WEB_NATIVE_DOCUMENT_MIME_TYPE,
        bytes: new Uint8Array(WEB_BROWSER_IMPORT_MAX_BYTES + 1),
      }),
      codec,
    );

    await expect(service.importNativeDocument()).rejects.toThrow(
      WEB_IMPORT_FILE_TOO_LARGE_ERROR,
    );
    expect(codec.inspect).not.toHaveBeenCalled();
  });

  it("rejects a selected non-TKO extension before calling the codec", async () => {
    const codec = codecFixture();
    const service = new WebImportExportService(
      fileToolFixture({
        name: "belge.zip",
        mediaType: WEB_NATIVE_DOCUMENT_MIME_TYPE,
        bytes: new Uint8Array([1]),
      }),
      codec,
    );

    await expect(service.importNativeDocument()).rejects.toThrow(
      WEB_UNSUPPORTED_IMPORT_FILE_ERROR,
    );
    expect(codec.inspect).not.toHaveBeenCalled();
  });

  it("canonicalizes TKO bytes through Rust-backed re-encode before download", () => {
    const fileTool = fileToolFixture(null);
    const codec = codecFixture();
    const service = new WebImportExportService(fileTool, codec);
    const source = new Uint8Array([7, 8]);
    const canonical = new Uint8Array([9, 10]);
    codec.reencode.mockReturnValue(canonical);

    service.exportNativeDocument("rapor", source);

    expect(codec.reencode).toHaveBeenCalledWith(source, WEB_APP_VERSION);
    expect(fileTool.downloadFile).toHaveBeenCalledWith({
      fileName: "rapor.tko",
      mediaType: WEB_NATIVE_DOCUMENT_MIME_TYPE,
      bytes: canonical,
    });
  });

  it("rejects export before download when the WASM TKO codec is unavailable", () => {
    const fileTool = fileToolFixture(null);
    const service = new WebImportExportService(fileTool, null);

    expect(() =>
      service.exportNativeDocument("rapor", new Uint8Array([1])),
    ).toThrow(WEB_TKO_CODEC_UNAVAILABLE_ERROR);
    expect(fileTool.downloadFile).not.toHaveBeenCalled();
  });
});
