// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/repositories/indexeddb-document.repository.ts
// # 📌 Amac: M3 Web canonical belge persistence ve metadata-index senkronizasyonunu yonetir
// # 📌 Modul - FileType: Repository - TypeScript
// Version: 0.4.0
// Aciklama: Canonical payloadlari IndexedDB Tool uzerinden saklar; localStorage metadata indexine payload yazmadan deterministic metadata yansitmasi yapar
// Bagimli Oldugu Katman: Repo -> Tool

import { WEB_INVALID_CANONICAL_DOCUMENT_ERROR } from "../config/runtime-config";
import type {
  WebCanonicalDocumentRecord,
  WebDocumentIndexEntry,
} from "../models/web-models";
import type { WebIndexedDbDocumentTool } from "../tools/indexed-db-document.tool";
import type { WebDocumentIndexRepository } from "./browser-document-index.repository";

export interface WebCanonicalDocumentRepository {
  count(): Promise<number>;
  get(id: string): Promise<WebCanonicalDocumentRecord | null>;
  list(): Promise<readonly WebCanonicalDocumentRecord[]>;
  save(document: WebCanonicalDocumentRecord): Promise<void>;
  remove(id: string): Promise<void>;
}

export class IndexedDbDocumentRepository implements WebCanonicalDocumentRepository {
  public constructor(
    private readonly tool: WebIndexedDbDocumentTool,
    private readonly metadataIndex: WebDocumentIndexRepository,
  ) {}

  public async count(): Promise<number> {
    return (await this.list()).length;
  }

  public async get(id: string): Promise<WebCanonicalDocumentRecord | null> {
    const document = await this.tool.get(id);
    return document !== null && isWebCanonicalDocumentRecord(document)
      ? cloneDocument(document)
      : null;
  }

  public async list(): Promise<readonly WebCanonicalDocumentRecord[]> {
    const documents = (await this.tool.list())
      .filter(isWebCanonicalDocumentRecord)
      .map(cloneDocument)
      .sort((left, right) => left.id.localeCompare(right.id));

    this.syncMetadataIndex(documents);
    return documents;
  }

  public async save(document: WebCanonicalDocumentRecord): Promise<void> {
    if (!isWebCanonicalDocumentRecord(document)) {
      throw new TypeError(WEB_INVALID_CANONICAL_DOCUMENT_ERROR);
    }

    const canonical = cloneDocument(document);
    await this.tool.put(canonical);
    this.metadataIndex.save(toIndexEntry(canonical));
  }

  public async remove(id: string): Promise<void> {
    await this.tool.remove(id);
    this.metadataIndex.remove(id);
  }

  private syncMetadataIndex(documents: readonly WebCanonicalDocumentRecord[]): void {
    const ids = new Set(documents.map((document) => document.id));
    for (const entry of this.metadataIndex.list()) {
      if (!ids.has(entry.id)) {
        this.metadataIndex.remove(entry.id);
      }
    }
    for (const document of documents) {
      this.metadataIndex.save(toIndexEntry(document));
    }
  }
}

function cloneDocument(document: WebCanonicalDocumentRecord): WebCanonicalDocumentRecord {
  return {
    id: document.id,
    title: document.title,
    text: document.text,
    schemaVersion: document.schemaVersion,
    revision: document.revision,
  };
}

function toIndexEntry(document: WebCanonicalDocumentRecord): WebDocumentIndexEntry {
  return {
    id: document.id,
    title: document.title,
    revision: document.revision,
  };
}

function isWebCanonicalDocumentRecord(value: unknown): value is WebCanonicalDocumentRecord {
  if (typeof value !== "object" || value === null) {
    return false;
  }

  const candidate = value as Partial<WebCanonicalDocumentRecord>;
  return (
    typeof candidate.id === "string" &&
    typeof candidate.title === "string" &&
    typeof candidate.text === "string" &&
    typeof candidate.schemaVersion === "number" &&
    Number.isSafeInteger(candidate.schemaVersion) &&
    candidate.schemaVersion > 0 &&
    typeof candidate.revision === "number" &&
    Number.isSafeInteger(candidate.revision) &&
    candidate.revision >= 0
  );
}
