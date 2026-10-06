// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/services/web-writer-session.service.test.ts
// # 📌 Amac: Writer Web session import/export lifecycle is kurallarini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Import replacement, picker cancellation, active export ve empty-session fail-closed davranislarini sabitler
// Bagimli Oldugu Katman: Service -> Repo -> Service

import { describe, expect, it, vi } from "vitest";

import { WEB_WRITER_SESSION_EMPTY_ERROR } from "../config/runtime-config";
import type { WebNativeDocumentImport } from "../models/web-models";
import { InMemoryWebWriterSessionRepository } from "../repositories/web-writer-session.repository";
import type { WebImportExportService } from "./web-import-export.service";
import { WebWriterSessionService } from "./web-writer-session.service";

function importFixture(): WebNativeDocumentImport {
  return {
    fileName: "belge.tko",
    bytes: new Uint8Array([4, 5, 6]),
    summary: {
      id: "writer-1",
      title: "Belge",
      schemaVersion: 1,
      revision: 2,
      sectionCount: 1,
      assetCount: 0,
    },
  };
}

function importExportFixture(imported: WebNativeDocumentImport | null) {
  return {
    importNativeDocument: vi.fn(async () => imported),
    exportNativeDocument: vi.fn(),
  } as unknown as WebImportExportService;
}

describe("WebWriterSessionService", () => {
  it("stores a successful TKO import as the active Writer session", async () => {
    const repository = new InMemoryWebWriterSessionRepository();
    const service = new WebWriterSessionService(
      repository,
      importExportFixture(importFixture()),
    );

    await expect(service.openNativeDocument()).resolves.toMatchObject({
      fileName: "belge.tko",
      summary: { id: "writer-1", revision: 2 },
    });
    expect(service.activeDocument()?.bytes).toEqual(new Uint8Array([4, 5, 6]));
  });

  it("keeps the existing session when the picker is cancelled", async () => {
    const repository = new InMemoryWebWriterSessionRepository();
    repository.replace(importFixture());
    const service = new WebWriterSessionService(
      repository,
      importExportFixture(null),
    );

    await expect(service.openNativeDocument()).resolves.toBeNull();

    expect(service.activeDocument()?.summary.id).toBe("writer-1");
  });

  it("exports the active rich TKO bytes through the canonical re-encode Service", () => {
    const repository = new InMemoryWebWriterSessionRepository();
    repository.replace(importFixture());
    const importExportService = importExportFixture(null);
    const service = new WebWriterSessionService(repository, importExportService);

    service.exportActiveDocument();

    expect(importExportService.exportNativeDocument).toHaveBeenCalledWith(
      "belge.tko",
      new Uint8Array([4, 5, 6]),
    );
  });

  it("fails closed when export is requested without an active session", () => {
    const service = new WebWriterSessionService(
      new InMemoryWebWriterSessionRepository(),
      importExportFixture(null),
    );

    expect(() => service.exportActiveDocument()).toThrow(
      WEB_WRITER_SESSION_EMPTY_ERROR,
    );
  });

  it("closes the active session without touching the codec", () => {
    const repository = new InMemoryWebWriterSessionRepository();
    repository.replace(importFixture());
    const importExportService = importExportFixture(null);
    const service = new WebWriterSessionService(repository, importExportService);

    service.closeDocument();

    expect(service.activeDocument()).toBeNull();
    expect(importExportService.exportNativeDocument).not.toHaveBeenCalled();
  });
});
