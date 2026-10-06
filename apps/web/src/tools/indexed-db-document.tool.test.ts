// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/indexed-db-document.tool.test.ts
// # 📌 Amac: Browser IndexedDB Tool open lifecycle ve cache recovery davranisini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Blocked open, senkron open hatasi retry ve unexpected close cache eviction davranislarini sabitler
// Bagimli Oldugu Katman: Tool -> Config

import { describe, expect, it, vi } from "vitest";

import { BrowserIndexedDbDocumentTool } from "./indexed-db-document.tool";

interface PrivateDatabaseAccess {
  database(): Promise<IDBDatabase>;
}

function databaseAccess(tool: BrowserIndexedDbDocumentTool): PrivateDatabaseAccess {
  return tool as unknown as PrivateDatabaseAccess;
}

function databaseFixture(): IDBDatabase {
  return {
    objectStoreNames: {
      contains: () => true,
    },
    createObjectStore: vi.fn(),
    close: vi.fn(),
    onversionchange: null,
    onclose: null,
  } as unknown as IDBDatabase;
}

function requestFixture(database: IDBDatabase): IDBOpenDBRequest {
  return {
    result: database,
    error: null,
    onupgradeneeded: null,
    onsuccess: null,
    onerror: null,
    onblocked: null,
  } as unknown as IDBOpenDBRequest;
}

describe("BrowserIndexedDbDocumentTool database lifecycle", () => {
  it("keeps a blocked open pending until the request later succeeds", async () => {
    const database = databaseFixture();
    const request = requestFixture(database);
    const factory = {
      open: vi.fn(() => request),
    } as unknown as IDBFactory;
    const access = databaseAccess(new BrowserIndexedDbDocumentTool(factory));

    const opening = access.database();
    request.onblocked?.call(request, new Event("blocked"));
    request.onsuccess?.call(request, new Event("success"));

    await expect(opening).resolves.toBe(database);
  });

  it("retries after a synchronous IDBFactory open failure", async () => {
    const database = databaseFixture();
    const request = requestFixture(database);
    let attempt = 0;
    const factory = {
      open: vi.fn(() => {
        attempt += 1;
        if (attempt === 1) {
          throw new Error();
        }
        return request;
      }),
    } as unknown as IDBFactory;
    const access = databaseAccess(new BrowserIndexedDbDocumentTool(factory));

    await expect(access.database()).rejects.toThrow();

    const retry = access.database();
    request.onsuccess?.call(request, new Event("success"));

    await expect(retry).resolves.toBe(database);
    expect(attempt).toBe(2);
  });

  it("evicts a resolved database after an unexpected close event", async () => {
    const firstDatabase = databaseFixture();
    const secondDatabase = databaseFixture();
    const firstRequest = requestFixture(firstDatabase);
    const secondRequest = requestFixture(secondDatabase);
    const factory = {
      open: vi
        .fn()
        .mockReturnValueOnce(firstRequest)
        .mockReturnValueOnce(secondRequest),
    } as unknown as IDBFactory;
    const access = databaseAccess(new BrowserIndexedDbDocumentTool(factory));

    const first = access.database();
    firstRequest.onsuccess?.call(firstRequest, new Event("success"));
    await expect(first).resolves.toBe(firstDatabase);

    firstDatabase.onclose?.call(firstDatabase, new Event("close"));

    const second = access.database();
    secondRequest.onsuccess?.call(secondRequest, new Event("success"));

    await expect(second).resolves.toBe(secondDatabase);
    expect(factory.open).toHaveBeenCalledTimes(2);
  });
});
