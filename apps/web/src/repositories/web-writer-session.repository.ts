// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/repositories/web-writer-session.repository.ts
// # 📌 Amac: Browser Writer oturumunda aktif TKO belge kaydini veri kaybi olmadan saklar
// # 📌 Modul - FileType: Repository - TypeScript
// Version: 0.4.0
// Aciklama: Zengin Writer TKO byte payloadini ve inspect summary bilgisini session-only Repository sinirinda izole eder
// Bagimli Oldugu Katman: Repo -> Model

import type { WebWriterSessionDocument } from "../models/web-models";

export interface WebWriterSessionRepository {
  active(): WebWriterSessionDocument | null;
  replace(document: WebWriterSessionDocument): void;
  clear(): void;
}

export class InMemoryWebWriterSessionRepository implements WebWriterSessionRepository {
  private current: WebWriterSessionDocument | null = null;

  public active(): WebWriterSessionDocument | null {
    return this.current === null ? null : cloneSessionDocument(this.current);
  }

  public replace(document: WebWriterSessionDocument): void {
    this.current = cloneSessionDocument(document);
  }

  public clear(): void {
    this.current = null;
  }
}

function cloneSessionDocument(
  document: WebWriterSessionDocument,
): WebWriterSessionDocument {
  return {
    fileName: document.fileName,
    bytes: document.bytes.slice(),
    summary: {
      id: document.summary.id,
      title: document.summary.title,
      schemaVersion: document.summary.schemaVersion,
      revision: document.summary.revision,
      sectionCount: document.summary.sectionCount,
      assetCount: document.summary.assetCount,
    },
  };
}
