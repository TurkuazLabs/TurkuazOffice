// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/indexed-db-document.tool.ts
// # 📌 Amac: Browser IndexedDB document object store erisimini typed Tool adaptorunde izole eder
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.4.0
// Aciklama: Canonical web belge kayitlari icin open/upgrade/get/list/put/remove islemlerini browser IndexedDB API'sine delege eder
// Bagimli Oldugu Katman: Tool -> Config

import {
  WEB_DOCUMENT_DB_NAME,
  WEB_DOCUMENT_DB_VERSION,
  WEB_DOCUMENT_STORE_KEY_PATH,
  WEB_DOCUMENT_STORE_NAME,
  WEB_INDEXED_DB_OPERATION_ERROR,
} from "../config/runtime-config";
import type { WebCanonicalDocumentRecord } from "../models/web-models";

export interface WebIndexedDbDocumentTool {
  get(id: string): Promise<WebCanonicalDocumentRecord | null>;
  list(): Promise<readonly WebCanonicalDocumentRecord[]>;
  put(document: WebCanonicalDocumentRecord): Promise<void>;
  remove(id: string): Promise<void>;
}

export class BrowserIndexedDbDocumentTool implements WebIndexedDbDocumentTool {
  private databasePromise: Promise<IDBDatabase> | null = null;

  public constructor(private readonly factory: IDBFactory) {}

  public get(id: string): Promise<WebCanonicalDocumentRecord | null> {
    return this.request("readonly", (store) => store.get(id)).then(
      (value) => (value as WebCanonicalDocumentRecord | undefined) ?? null,
    );
  }

  public list(): Promise<readonly WebCanonicalDocumentRecord[]> {
    return this.request("readonly", (store) => store.getAll()).then(
      (value) => value as WebCanonicalDocumentRecord[],
    );
  }

  public async put(document: WebCanonicalDocumentRecord): Promise<void> {
    await this.request("readwrite", (store) => store.put(document));
  }

  public async remove(id: string): Promise<void> {
    await this.request("readwrite", (store) => store.delete(id));
  }

  private database(): Promise<IDBDatabase> {
    if (this.databasePromise !== null) {
      return this.databasePromise;
    }

    const opening = this.openDatabase();
    this.databasePromise = opening;

    void opening.then(
      (database) => {
        database.onversionchange = () => {
          database.close();
          this.clearDatabasePromise(opening);
        };
        database.onclose = () => {
          this.clearDatabasePromise(opening);
        };
      },
      () => {
        this.clearDatabasePromise(opening);
      },
    );

    return opening;
  }

  private openDatabase(): Promise<IDBDatabase> {
    return new Promise<IDBDatabase>((resolve, reject) => {
      let request: IDBOpenDBRequest;
      try {
        request = this.factory.open(WEB_DOCUMENT_DB_NAME, WEB_DOCUMENT_DB_VERSION);
      } catch (error: unknown) {
        reject(error);
        return;
      }

      request.onupgradeneeded = () => {
        const database = request.result;
        if (!database.objectStoreNames.contains(WEB_DOCUMENT_STORE_NAME)) {
          database.createObjectStore(WEB_DOCUMENT_STORE_NAME, {
            keyPath: WEB_DOCUMENT_STORE_KEY_PATH,
          });
        }
      };
      request.onsuccess = () => resolve(request.result);
      request.onerror = () =>
        reject(request.error ?? new Error(WEB_INDEXED_DB_OPERATION_ERROR));
      request.onblocked = () => {
        // Blocked bildirimi terminal hata degildir; diger baglanti kapaninca istek tamamlanabilir.
      };
    });
  }

  private clearDatabasePromise(opening: Promise<IDBDatabase>): void {
    if (this.databasePromise === opening) {
      this.databasePromise = null;
    }
  }

  private async request(
    mode: IDBTransactionMode,
    createRequest: (store: IDBObjectStore) => IDBRequest,
  ): Promise<unknown> {
    const database = await this.database();
    return new Promise<unknown>((resolve, reject) => {
      const transaction = database.transaction(WEB_DOCUMENT_STORE_NAME, mode);
      const store = transaction.objectStore(WEB_DOCUMENT_STORE_NAME);
      let result: unknown;

      let request: IDBRequest;
      try {
        request = createRequest(store);
      } catch (error: unknown) {
        reject(error);
        return;
      }

      request.onsuccess = () => {
        result = request.result;
      };
      request.onerror = () =>
        reject(request.error ?? new Error(WEB_INDEXED_DB_OPERATION_ERROR));
      transaction.oncomplete = () => resolve(result);
      transaction.onerror = () =>
        reject(transaction.error ?? new Error(WEB_INDEXED_DB_OPERATION_ERROR));
      transaction.onabort = () =>
        reject(transaction.error ?? new Error(WEB_INDEXED_DB_OPERATION_ERROR));
    });
  }
}
