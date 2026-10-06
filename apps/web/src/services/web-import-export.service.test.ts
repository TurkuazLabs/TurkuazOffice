// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/services/web-import-export.service.test.ts
// # 📌 Amac: Browser import/export Service byte transport kurallarini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: TKO accept/extension, capability ve download adlandirma davranisini Tool fake'i ile test eder
// Bagimli Oldugu Katman: Service -> Tool -> Config

import { describe, expect, it, vi } from "vitest";

import {
  WEB_BROWSER_IMPORT_MAX_BYTES,
  WEB_NATIVE_DOCUMENT_ACCEPT,
  WEB_NATIVE_DOCUMENT_MIME_TYPE,
  WEB_UNSUPPORTED_IMPORT_FILE_ERROR,
} from "../config/runtime-config";
import type { WebDownloadFile, WebPickedFile } from "../models/web-models";
import { WebImportExportService } from "./web-import-export.service";

function toolFixture(picked: WebPickedFile | null) {
  const pickFile = vi.fn(async () => picked);
  const downloadFile = vi.fn((_file: WebDownloadFile): void => undefined);
  return { pickFile, downloadFile };
}

describe("WebImportExportService", () => {
  it("publishes byte transport without claiming a TKO codec", () => {
    const service = new WebImportExportService(toolFixture(null));

    expect(service.capabilities()).toEqual({
      browserFileTransferAvailable: true,
      nativeTkoCodecAvailable: false,
    });
  });

  it("selects TKO bytes through the browser Tool using the central limit", async () => {
    const picked = {
      name: "Belge.TKO",
      mediaType: WEB_NATIVE_DOCUMENT_MIME_TYPE,
      bytes: new Uint8Array([1, 2]),
    };
    const tool = toolFixture(picked);
    const service = new WebImportExportService(tool);

    await expect(service.pickNativeDocumentBytes()).resolves.toEqual(picked);
    expect(tool.pickFile).toHaveBeenCalledWith({
      accept: WEB_NATIVE_DOCUMENT_ACCEPT,
      maxBytes: WEB_BROWSER_IMPORT_MAX_BYTES,
    });
  });

  it("rejects a selected file outside the native TKO extension contract", async () => {
    const service = new WebImportExportService(
      toolFixture({
        name: "belge.zip",
        mediaType: WEB_NATIVE_DOCUMENT_MIME_TYPE,
        bytes: new Uint8Array([1]),
      }),
    );

    await expect(service.pickNativeDocumentBytes()).rejects.toThrow(
      WEB_UNSUPPORTED_IMPORT_FILE_ERROR,
    );
  });

  it("normalizes the native extension before browser download", () => {
    const tool = toolFixture(null);
    const service = new WebImportExportService(tool);
    const bytes = new Uint8Array([7, 8]);

    service.downloadNativeDocumentBytes("rapor", bytes);

    expect(tool.downloadFile).toHaveBeenCalledWith({
      fileName: "rapor.tko",
      mediaType: WEB_NATIVE_DOCUMENT_MIME_TYPE,
      bytes,
    });
  });
});
