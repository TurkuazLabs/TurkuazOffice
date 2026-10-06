// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/repositories/indexeddb-document.repository.test.ts
// # 📌 Amac: IndexedDB canonical document Repository davranisini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Canonical save/get/list/remove, deterministic order, metadata-index sync ve invalid payload guardlarini test eder
// Bagimli Oldugu Katman: Repo -> Tool

import { beforeEach, describe, expect, it } from "vitest";

import {
  WEB_INVALID_CANONICAL_DOCUMENT_ERROR,
  WEB_STORAGE_NAMESPACE,
} from "../config/runtime-config";
import type { WebCanonicalDocumentRecord } from "../models/web-models";
import type { WebIndexedDbDocumentTool } from "../tools/indexed-db-document.tool";
import { BrowserDocumentIndexRepository } from "./browser-document-index.repository";
import { IndexedDbDocumentRepository } from "./indexeddb-document.repository";

class MemoryIndexedDbDocumentTool implements WebIndexedDbDocumentTool {
  private readonly documents = new Map<string, WebCanonicalDocumentRecord>();

  public async get(id: string): Promise<WebCanonicalDocumentRecord | null> {
    return this.documents.get(id) ?? null;
  }

  public async list(): Promise<readonly WebCanonicalDocumentRecord[]> {
    return [...this.documents.values()];
  }

  public async put(document: WebCanonicalDocumentRecord): Promise<void> {
    this.documents.set(document.id, { ...document });
  }

  public async remove(id: string): Promise<void> {
    this.documents.delete(id);
  }

  public seed(document: WebCanonicalDocumentRecord): void {
    this.documents.set(document.id, document);
  }
}

const DOCUMENT_A: WebCanonicalDocumentRecord = {
  id: "a",
  title: "Belge A",
  text: "Merhaba",
  schemaVersion: 1,
  revision: 2,
};

describe("IndexedDbDocumentRepository", () => {
  beforeEach(() => {
    window.localStorage.clear();
  });

  it("stores canonical payload in IndexedDB and mirrors only metadata to localStorage", async () => {
    const tool = new MemoryIndexedDbDocumentTool();
    const index = new BrowserDocumentIndexRepository(window.localStorage);
    const repository = new IndexedDbDocumentRepository(tool, index);

    await repository.save(DOCUMENT_A);

    expect(await repository.get("a")).toEqual(DOCUMENT_A);
    expect(index.list()).toEqual([{ id: "a", title: "Belge A", revision: 2 }]);
    expect(window.localStorage.getItem(WEB_STORAGE_NAMESPACE)).not.toContain(
      DOCUMENT_A.text,
    );
  });

  it("lists canonical documents deterministically and repairs stale metadata", async () => {
    const tool = new MemoryIndexedDbDocumentTool();
    tool.seed({ ...DOCUMENT_A, id: "b", title: "Belge B" });
    tool.seed(DOCUMENT_A);
    const index = new BrowserDocumentIndexRepository(window.localStorage);
    index.save({ id: "stale", title: "Eski", revision: 1 });
    const repository = new IndexedDbDocumentRepository(tool, index);

    expect((await repository.list()).map((document) => document.id)).toEqual(["a", "b"]);
    expect(index.list()).toEqual([
      { id: "a", title: "Belge A", revision: 2 },
      { id: "b", title: "Belge B", revision: 2 },
    ]);
  });

  it("removes canonical payload and its metadata index entry together", async () => {
    const tool = new MemoryIndexedDbDocumentTool();
    const index = new BrowserDocumentIndexRepository(window.localStorage);
    const repository = new IndexedDbDocumentRepository(tool, index);
    await repository.save(DOCUMENT_A);

    await repository.remove(DOCUMENT_A.id);

    expect(await repository.get(DOCUMENT_A.id)).toBeNull();
    expect(index.list()).toEqual([]);
  });

  it("rejects invalid canonical records before writing browser storage", async () => {
    const tool = new MemoryIndexedDbDocumentTool();
    const repository = new IndexedDbDocumentRepository(
      tool,
      new BrowserDocumentIndexRepository(window.localStorage),
    );

    await expect(
      repository.save({
        ...DOCUMENT_A,
        schemaVersion: 0,
      }),
    ).rejects.toThrow(WEB_INVALID_CANONICAL_DOCUMENT_ERROR);
  });
});
