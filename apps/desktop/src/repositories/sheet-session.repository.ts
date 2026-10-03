// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/repositories/sheet-session.repository.ts
// # 📌 Amac: Desktop Sheet oturum read-model ve durum bilgisini bellekte tutar
// # 📌 Modul - FileType: Repo - TypeScript
// Version: 0.4.0
// Aciklama: Canonical Sheet belgesini degil backend snapshotini, loading ve error durumunu reactive saklar
// Bagimli Oldugu Katman: Repo

import { createSignal, type Accessor } from "solid-js";

import type { SheetDocumentView } from "../views/sheet-types";

export type SheetSessionStatus = "idle" | "loading" | "ready" | "error";

export class SheetSessionRepository {
  private readonly documentSignal = createSignal<SheetDocumentView | null>(null);
  private readonly statusSignal = createSignal<SheetSessionStatus>("idle");
  private readonly errorCodeSignal = createSignal<string | null>(null);

  public readonly document: Accessor<SheetDocumentView | null> = this.documentSignal[0];
  public readonly status: Accessor<SheetSessionStatus> = this.statusSignal[0];
  public readonly errorCode: Accessor<string | null> = this.errorCodeSignal[0];

  public setLoading(): void {
    this.statusSignal[1]("loading");
    this.errorCodeSignal[1](null);
  }

  public setDocument(document: SheetDocumentView): void {
    this.documentSignal[1](document);
    this.statusSignal[1]("ready");
    this.errorCodeSignal[1](null);
  }

  public setError(errorCode: string): void {
    this.statusSignal[1]("error");
    this.errorCodeSignal[1](errorCode);
  }
}
