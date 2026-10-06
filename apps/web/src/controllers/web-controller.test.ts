// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/controllers/web-controller.test.ts
// # 📌 Amac: WebController bootstrap ve browser import/export requestlerini yalniz Service katmanina delege ettigini dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Controller logic eklenmesini engelleyen bootstrap, file-select ve download delegation regressionlarini sabitler
// Bagimli Oldugu Katman: Controller -> Service -> Tool

import { describe, expect, it, vi } from "vitest";

import type {
  WebCanonicalDocumentRecord,
  WebDownloadFile,
} from "../models/web-models";
import type { WebCanonicalDocumentRepository } from "../repositories/indexeddb-document.repository";
import { WebBootstrapService } from "../services/web-bootstrap-service";
import { WebImportExportService } from "../services/web-import-export.service";
import { BrowserCoreContractTool } from "../tools/web-core-tool";
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

describe("WebController", () => {
  it("delegates bootstrap initialization to WebBootstrapService", async () => {
    const bootstrapService = new WebBootstrapService(
      repositoryFixture(),
      new BrowserCoreContractTool(),
    );
    const initialize = vi.spyOn(bootstrapService, "initialize");
    const controller = new WebController(
      bootstrapService,
      new WebImportExportService(fileToolFixture()),
    );

    await controller.initialize();

    expect(initialize).toHaveBeenCalledOnce();
  });

  it("delegates native file selection to WebImportExportService", async () => {
    const fileTool = fileToolFixture();
    const importExportService = new WebImportExportService(fileTool);
    const select = vi.spyOn(importExportService, "pickNativeDocumentBytes");
    const controller = new WebController(
      new WebBootstrapService(repositoryFixture(), new BrowserCoreContractTool()),
      importExportService,
    );

    await controller.selectNativeImportFile();

    expect(select).toHaveBeenCalledOnce();
  });

  it("delegates native download without transforming bytes in Controller", () => {
    const fileTool = fileToolFixture();
    const importExportService = new WebImportExportService(fileTool);
    const download = vi.spyOn(importExportService, "downloadNativeDocumentBytes");
    const controller = new WebController(
      new WebBootstrapService(repositoryFixture(), new BrowserCoreContractTool()),
      importExportService,
    );
    const bytes = new Uint8Array([9, 10]);

    controller.downloadNativeExportFile("belge", bytes);

    expect(download).toHaveBeenCalledWith("belge", bytes);
  });
});
