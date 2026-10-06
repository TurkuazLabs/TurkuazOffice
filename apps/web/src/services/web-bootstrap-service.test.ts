// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/services/web-bootstrap-service.test.ts
// # 📌 Amac: Web bootstrap Service'in Core capability ve IndexedDB storage durumunu dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: IndexedDB success ve unavailable akislarinda typed bootstrap ViewModel uretimini regression testleriyle sabitler
// Bagimli Oldugu Katman: Service -> Repo -> Tool

import { describe, expect, it } from "vitest";

import type { WebCanonicalDocumentRecord } from "../models/web-models";
import type { WebCanonicalDocumentRepository } from "../repositories/indexeddb-document.repository";
import { BrowserCoreContractTool } from "../tools/web-core-tool";
import { WebBootstrapService } from "./web-bootstrap-service";

function repositoryWithCount(
  count: () => Promise<number>,
): WebCanonicalDocumentRepository {
  return {
    count,
    get: async () => null,
    list: async () => [],
    save: async (_document: WebCanonicalDocumentRecord) => undefined,
    remove: async () => undefined,
  };
}

describe("WebBootstrapService", () => {
  it("reports IndexedDB canonical storage when repository bootstrap succeeds", async () => {
    const service = new WebBootstrapService(
      repositoryWithCount(async () => 3),
      new BrowserCoreContractTool(),
    );

    await expect(service.initialize()).resolves.toMatchObject({
      documentStorageKind: "indexed-db",
      documentStorageAvailable: true,
      storedDocumentCount: 3,
    });
  });

  it("keeps canonical storage unavailable instead of falling back to localStorage payloads", async () => {
    const service = new WebBootstrapService(
      repositoryWithCount(async () => {
        throw new Error("indexed-db-unavailable");
      }),
      new BrowserCoreContractTool(),
    );

    await expect(service.initialize()).resolves.toMatchObject({
      documentStorageKind: "indexed-db",
      documentStorageAvailable: false,
      storedDocumentCount: null,
    });
  });
});
