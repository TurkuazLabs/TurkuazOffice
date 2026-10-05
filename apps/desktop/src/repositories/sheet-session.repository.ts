// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/repositories/sheet-session.repository.ts
// # 📌 Amac: Desktop Sheet oturum read-model ve durum bilgisini bellekte tutar
// # 📌 Modul - FileType: Repo - TypeScript
// Version: 0.5.0
// Aciklama: Backend snapshot, dirty-state, secili hucre/range, range summary, sparse format cache ve row-query state'ini reactive saklar
// Bagimli Oldugu Katman: Repo

import { createSignal, type Accessor } from "solid-js";

import type {
  SheetCellFormatView,
  SheetDocumentView,
  SheetRangeSummaryView,
  SheetRowQueryStateView,
  SheetSelectionRangeView,
  SheetSelectionView,
} from "../views/sheet-types";

export type SheetSessionStatus = "idle" | "loading" | "ready" | "error";

export class SheetSessionRepository {
  private readonly documentSignal = createSignal<SheetDocumentView | null>(null);
  private readonly statusSignal = createSignal<SheetSessionStatus>("idle");
  private readonly dirtySignal = createSignal(false);
  private readonly errorCodeSignal = createSignal<string | null>(null);
  private readonly selectionSignal = createSignal<SheetSelectionView | null>(null);
  private readonly selectionRangeSignal = createSignal<SheetSelectionRangeView | null>(null);
  private readonly rangeSummarySignal = createSignal<SheetRangeSummaryView | null>(null);
  private readonly rangeSummaryErrorCodeSignal = createSignal<string | null>(null);
  private readonly cellFormatsSignal = createSignal<ReadonlyMap<string, SheetCellFormatView>>(
    new Map(),
  );
  private readonly rowQuerySignal = createSignal<SheetRowQueryStateView | null>(null);
  private readonly rowQueryErrorCodeSignal = createSignal<string | null>(null);

  public readonly document: Accessor<SheetDocumentView | null> = this.documentSignal[0];
  public readonly status: Accessor<SheetSessionStatus> = this.statusSignal[0];
  public readonly dirty: Accessor<boolean> = this.dirtySignal[0];
  public readonly errorCode: Accessor<string | null> = this.errorCodeSignal[0];
  public readonly selection: Accessor<SheetSelectionView | null> = this.selectionSignal[0];
  public readonly selectionRange: Accessor<SheetSelectionRangeView | null> =
    this.selectionRangeSignal[0];
  public readonly rangeSummary: Accessor<SheetRangeSummaryView | null> = this.rangeSummarySignal[0];
  public readonly rangeSummaryErrorCode: Accessor<string | null> =
    this.rangeSummaryErrorCodeSignal[0];
  public readonly rowQuery: Accessor<SheetRowQueryStateView | null> = this.rowQuerySignal[0];
  public readonly rowQueryErrorCode: Accessor<string | null> = this.rowQueryErrorCodeSignal[0];

  public cellFormat(worksheetId: string, reference: string): SheetCellFormatView | null {
    return this.cellFormatsSignal[0]().get(this.formatKey(worksheetId, reference)) ?? null;
  }

  public setCellFormat(
    worksheetId: string,
    reference: string,
    format: SheetCellFormatView,
  ): void {
    const next = new Map(this.cellFormatsSignal[0]());
    next.set(this.formatKey(worksheetId, reference), format);
    this.cellFormatsSignal[1](next);
  }

  public clearCellFormats(): void {
    this.cellFormatsSignal[1](new Map());
  }

  public setRowQuery(query: SheetRowQueryStateView): void {
    this.rowQuerySignal[1](query);
    this.rowQueryErrorCodeSignal[1](null);
  }

  public setRowQueryError(errorCode: string): void {
    this.rowQueryErrorCodeSignal[1](errorCode);
  }

  public clearRowQuery(): void {
    this.rowQuerySignal[1](null);
    this.rowQueryErrorCodeSignal[1](null);
  }

  public setLoading(): void {
    this.statusSignal[1]("loading");
    this.errorCodeSignal[1](null);
  }

  public setSelection(selection: SheetSelectionView | null): void {
    this.selectionSignal[1](selection);
  }

  public setSelectionRange(range: SheetSelectionRangeView): void {
    this.selectionRangeSignal[1](range);
    this.rangeSummarySignal[1](null);
    this.rangeSummaryErrorCodeSignal[1](null);
  }

  public setRangeSummary(summary: SheetRangeSummaryView): void {
    this.rangeSummarySignal[1](summary);
    this.rangeSummaryErrorCodeSignal[1](null);
  }

  public setRangeSummaryError(errorCode: string): void {
    this.rangeSummarySignal[1](null);
    this.rangeSummaryErrorCodeSignal[1](errorCode);
  }

  public clearSelectionRange(): void {
    this.selectionRangeSignal[1](null);
    this.rangeSummarySignal[1](null);
    this.rangeSummaryErrorCodeSignal[1](null);
  }

  public markDirty(): void {
    this.dirtySignal[1](true);
  }

  public markClean(): void {
    this.dirtySignal[1](false);
  }

  public setDocument(document: SheetDocumentView): void {
    this.documentSignal[1](document);
    this.statusSignal[1]("ready");
    this.errorCodeSignal[1](null);
  }

  public setSelectionEvaluationError(errorCode: string): void {
    const selection = this.selection();
    if (selection === null) {
      return;
    }
    this.selectionSignal[1]({
      ...selection,
      evaluatedValue: null,
      evaluationErrorCode: errorCode,
    });
  }

  private formatKey(worksheetId: string, reference: string): string {
    return `${worksheetId}:${reference}`;
  }

  public setError(errorCode: string): void {
    this.statusSignal[1]("error");
    this.errorCodeSignal[1](errorCode);
  }
}
