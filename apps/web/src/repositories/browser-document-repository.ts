// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/repositories/browser-document-repository.ts
// # 📌 Amac: M3 Web belge snapshotlarini browser Storage uzerinde saklar
// # 📌 Modul - FileType: Repository - TypeScript
// # Version: 0.4.0
// # Aciklama: Native filesystem kullanmadan local-first browser persistence baseline'i saglar
// Bagimli Oldugu Katman: Repo

import { WEB_STORAGE_NAMESPACE } from "../config/runtime-config";
import type { WebDocumentSnapshot } from "../models/web-models";

export interface WebDocumentRepository {
  list(): readonly WebDocumentSnapshot[];
  save(document: WebDocumentSnapshot): void;
  remove(id: string): void;
}

export class BrowserDocumentRepository implements WebDocumentRepository {
  public constructor(private readonly storage: Storage) {}

  public list(): readonly WebDocumentSnapshot[] {
    const raw = this.storage.getItem(WEB_STORAGE_NAMESPACE);
    if (raw === null) {
      return [];
    }

    try {
      const parsed: unknown = JSON.parse(raw);
      if (!Array.isArray(parsed)) {
        return [];
      }

      return parsed.filter(isWebDocumentSnapshot);
    } catch {
      return [];
    }
  }

  public save(document: WebDocumentSnapshot): void {
    const documents = this.list().filter((item) => item.id !== document.id);
    const next = [...documents, document].sort((left, right) =>
      left.id.localeCompare(right.id),
    );
    this.storage.setItem(WEB_STORAGE_NAMESPACE, JSON.stringify(next));
  }

  public remove(id: string): void {
    const next = this.list().filter((item) => item.id !== id);
    this.storage.setItem(WEB_STORAGE_NAMESPACE, JSON.stringify(next));
  }
}

function isWebDocumentSnapshot(value: unknown): value is WebDocumentSnapshot {
  if (typeof value !== "object" || value === null) {
    return false;
  }

  const candidate = value as Partial<WebDocumentSnapshot>;
  return (
    typeof candidate.id === "string" &&
    typeof candidate.title === "string" &&
    typeof candidate.content === "string" &&
    typeof candidate.revision === "number" &&
    Number.isSafeInteger(candidate.revision) &&
    candidate.revision >= 0
  );
}
