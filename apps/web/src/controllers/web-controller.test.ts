// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/controllers/web-controller.test.ts
// # 📌 Amac: WebController bootstrap ve Writer session requestlerini yalniz Service katmanina delege ettigini dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Controller logic eklenmesini engelleyen bootstrap, active/open/export/close delegation regressionlarini sabitler
// Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool

import { describe, expect, it, vi } from "vitest";

import type {
  WebCanonicalDocumentRecord,
  WebNativeDocumentImport,
} from "../models/web-models";
import type { WebCanonicalDocumentRepository } from "../repositories/indexeddb-document.repository";
import { InMemoryWebWriterSessionRepository } from "../repositories/web-writer-session.repository";
import { WebBootstrapService } from "../services/web-bootstrap-service";
import type { WebImportExportService } from "../services/web-import-export.service";
import { WebWriterSessionService } from "../services/web-writer-session.service";
import { BrowserCoreContractTool } from "../tools/web-core-tool";
import { WebController } from "./web-controller";

function canonicalRepositoryFixture(): WebCanonicalDocumentRepository {
  return {
    count: async () => 0,
    get: async () => null,
    list: async () => [],
    save: async (_document: WebCanonicalDocumentRecord) => undefined,
    remove: async () => undefined,
  };
}

function importFixture(): WebNativeDocumentImport {
  return {
    fileName: "belge.tko",
    bytes: new Uint8Array([1, 2]),
    summary: {
      id: "writer-1",
      title: "Belge",
      schemaVersion: 1,
      revision: 1,
      sectionCount: 1,
      assetCount: 0,
    },
  };
}

function sessionServiceFixture() {
  const importExportService = {
    importNativeDocument: vi.fn(async () => importFixture()),
    exportNativeDocument: vi.fn(),
  } as unknown as WebImportExportService;
  return new WebWriterSessionService(
    new InMemoryWebWriterSessionRepository(),
    importExportService,
  );
}

describe("WebController", () => {
  it("delegates bootstrap initialization to WebBootstrapService", async () => {
    const bootstrapService = new WebBootstrapService(
      canonicalRepositoryFixture(),
      new BrowserCoreContractTool(),
    );
    const initialize = vi.spyOn(bootstrapService, "initialize");
    const controller = new WebController(
      bootstrapService,
      sessionServiceFixture(),
    );

    await controller.initialize();

    expect(initialize).toHaveBeenCalledOnce();
  });

  it("delegates Writer open to WebWriterSessionService", async () => {
    const sessionService = sessionServiceFixture();
    const open = vi.spyOn(sessionService, "openNativeDocument");
    const controller = new WebController(
      new WebBootstrapService(
        canonicalRepositoryFixture(),
        new BrowserCoreContractTool(),
      ),
      sessionService,
    );

    await controller.openWriterDocument();

    expect(open).toHaveBeenCalledOnce();
  });

  it("delegates active Writer session lookup", () => {
    const sessionService = sessionServiceFixture();
    const active = vi.spyOn(sessionService, "activeDocument");
    const controller = new WebController(
      new WebBootstrapService(
        canonicalRepositoryFixture(),
        new BrowserCoreContractTool(),
      ),
      sessionService,
    );

    controller.activeWriterDocument();

    expect(active).toHaveBeenCalledOnce();
  });

  it("delegates Writer export without transforming bytes in Controller", async () => {
    const sessionService = sessionServiceFixture();
    await sessionService.openNativeDocument();
    const exportActive = vi.spyOn(sessionService, "exportActiveDocument");
    const controller = new WebController(
      new WebBootstrapService(
        canonicalRepositoryFixture(),
        new BrowserCoreContractTool(),
      ),
      sessionService,
    );

    controller.exportWriterDocument();

    expect(exportActive).toHaveBeenCalledOnce();
  });

  it("delegates Writer close to WebWriterSessionService", () => {
    const sessionService = sessionServiceFixture();
    const close = vi.spyOn(sessionService, "closeDocument");
    const controller = new WebController(
      new WebBootstrapService(
        canonicalRepositoryFixture(),
        new BrowserCoreContractTool(),
      ),
      sessionService,
    );

    controller.closeWriterDocument();

    expect(close).toHaveBeenCalledOnce();
  });
});
