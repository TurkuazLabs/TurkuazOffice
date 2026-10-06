// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/controllers/web-controller.test.ts
// # 📌 Amac: WebController bootstrap ve typed native TKO import/export requestlerini yalniz Service katmanina delege ettigini dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Controller logic eklenmesini engelleyen bootstrap, import ve export delegation regressionlarini sabitler
// Bagimli Oldugu Katman: Controller -> Service -> Tool

import { describe, expect, it, vi } from "vitest";

import type {
  WebCanonicalDocumentRecord,
  WebDownloadFile,
  WebWriterTkoSummary,
} from "../models/web-models";
import type { WebCanonicalDocumentRepository } from "../repositories/indexeddb-document.repository";
import { WebBootstrapService } from "../services/web-bootstrap-service";
import { WebImportExportService } from "../services/web-import-export.service";
import { BrowserCoreContractTool } from "../tools/web-core-tool";
import type { WebWriterTkoTool } from "../tools/writer-tko.tool";
import { WebController } from "./web-controller";

function repositoryFixture(): WebCanonicalDocumentRepository {
  return {
    count: async () => 0,
    get: async () => null,
    list: async () => [],
    save: async (_document: WebCanonicalDocumentRecord) => undefined,
    remove: async () => undefined,
  };
}

function fileToolFixture() {
  const pickFile = vi.fn(async () => null);
  const downloadFile = vi.fn((_file: WebDownloadFile): void => undefined);
  return { pickFile, downloadFile };
}

function codecFixture(): WebWriterTkoTool {
  return {
    inspect: vi.fn(
      (_bytes: Uint8Array): WebWriterTkoSummary => ({
        id: "writer-1",
        title: "Belge",
        schemaVersion: 1,
        revision: 1,
        sectionCount: 1,
        assetCount: 0,
      }),
    ),
    reencode: vi.fn((bytes: Uint8Array) => bytes.slice()),
  };
}

describe("WebController", () => {
  it("delegates bootstrap initialization to WebBootstrapService", async () => {
    const bootstrapService = new WebBootstrapService(
      repositoryFixture(),
      new BrowserCoreContractTool(),
    );
    const initialize = vi.spyOn(bootstrapService, "initialize");
    const controller = new WebController(
      bootstrapService,
      new WebImportExportService(fileToolFixture(), codecFixture()),
    );

    await controller.initialize();

    expect(initialize).toHaveBeenCalledOnce();
  });

  it("delegates native import to WebImportExportService", async () => {
    const importExportService = new WebImportExportService(
      fileToolFixture(),
      codecFixture(),
    );
    const importDocument = vi.spyOn(importExportService, "importNativeDocument");
    const controller = new WebController(
      new WebBootstrapService(repositoryFixture(), new BrowserCoreContractTool()),
      importExportService,
    );

    await controller.importNativeDocument();

    expect(importDocument).toHaveBeenCalledOnce();
  });

  it("delegates native export without transforming bytes in Controller", () => {
    const importExportService = new WebImportExportService(
      fileToolFixture(),
      codecFixture(),
    );
    const exportDocument = vi.spyOn(importExportService, "exportNativeDocument");
    const controller = new WebController(
      new WebBootstrapService(repositoryFixture(), new BrowserCoreContractTool()),
      importExportService,
    );
    const bytes = new Uint8Array([9, 10]);

    controller.exportNativeDocument("belge", bytes);

    expect(exportDocument).toHaveBeenCalledWith("belge", bytes);
  });
});
