// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/repositories/browser-document-index.repository.ts
// # 📌 Amac: M3 Web document index metadata kayitlarini browser Storage uzerinde saklar
// # 📌 Modul - FileType: Repository - TypeScript
// # Version: 0.4.0
// # Aciklama: Canonical document payload tutmadan localStorage tabanli hafif metadata index baseline'i saglar
// Bagimli Oldugu Katman: Repo

import { WEB_STORAGE_NAMESPACE } from "../config/runtime-config";
import type { WebDocumentIndexEntry } from "../models/web-models";

export interface WebDocumentIndexRepository {
  list(): readonly WebDocumentIndexEntry[];
  save(entry: WebDocumentIndexEntry): void;
  remove(id: string): void;
}

export class BrowserDocumentIndexRepository implements WebDocumentIndexRepository {
  public constructor(private readonly storage: Storage) {}

  public list(): readonly WebDocumentIndexEntry[] {
    const raw = this.storage.getItem(WEB_STORAGE_NAMESPACE);
    if (raw === null) {
      return [];
    }

    try {
      const parsed: unknown = JSON.parse(raw);
      if (!Array.isArray(parsed)) {
        return [];
      }

      return parsed
        .filter(isWebDocumentIndexEntry)
        .map(({ id, title, revision }) => ({ id, title, revision }));
    } catch {
      return [];
    }
  }

  public save(entry: WebDocumentIndexEntry): void {
    const entries = this.list().filter((item) => item.id !== entry.id);
    const next = [...entries, entry].sort((left, right) =>
      left.id.localeCompare(right.id),
    );
    this.storage.setItem(WEB_STORAGE_NAMESPACE, JSON.stringify(next));
  }

  public remove(id: string): void {
    const next = this.list().filter((item) => item.id !== id);
    this.storage.setItem(WEB_STORAGE_NAMESPACE, JSON.stringify(next));
  }
}

function isWebDocumentIndexEntry(value: unknown): value is WebDocumentIndexEntry {
  if (typeof value !== "object" || value === null) {
    return false;
  }

  const candidate = value as Partial<WebDocumentIndexEntry>;
  return (
    typeof candidate.id === "string" &&
    typeof candidate.title === "string" &&
    typeof candidate.revision === "number" &&
    Number.isSafeInteger(candidate.revision) &&
    candidate.revision >= 0
  );
}
