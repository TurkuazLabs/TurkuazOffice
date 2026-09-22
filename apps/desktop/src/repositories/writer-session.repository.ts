// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/repositories/writer-session.repository.ts
// # 📌 Amac: Desktop Writer oturumunun read-model, recovery, file path, dirty baseline, selection, typing-style ve print preview durumunu bellekte tutar
// # 📌 Modul - FileType: Repo - TypeScript
// # Version: 0.2.0
// # Aciklama: Canonical belgeyi degil backend snapshotini, kayit/recovery baseline'ini ve gecici UI editor/print state'ini saklar
// Bagimli Oldugu Katman: Repo

import { createMemo, createSignal, type Accessor } from "solid-js";

import { WRITER_DEFAULT_ZOOM_PERCENT } from "../config/layout";

import type {
  RecoveryComparisonView,
  RecoveryRestoreView,
  RecoverySnapshotView,
  WriterCharacterStyleView,
  WriterDocumentView,
  WriterFileOperationView,
  WriterFileSessionView,
  WriterPageLayoutView,
  WriterResolvedFontView,
  WriterSelectionView,
} from "../views/writer-types";

export type WriterSessionStatus = "idle" | "loading" | "ready" | "error";

export class WriterSessionRepository {
  private readonly documentSignal = createSignal<WriterDocumentView | null>(null);
  private readonly selectionSignal = createSignal<WriterSelectionView | null>(null);
  private readonly typingStyleSignal = createSignal<WriterCharacterStyleView | null>(null);
  private readonly filePathSignal = createSignal<string | null>(null);
  private readonly fileSessionSignal = createSignal<WriterFileSessionView | null>(null);
  private readonly savedRevisionSignal = createSignal<number | null>(null);
  private readonly statusSignal = createSignal<WriterSessionStatus>("idle");
  private readonly errorCodeSignal = createSignal<string | null>(null);
  private readonly recoveryCandidatesSignal = createSignal<readonly RecoverySnapshotView[]>([]);
  private readonly recoveryComparisonSignal = createSignal<RecoveryComparisonView | null>(null);
  private readonly recoveryRevisionSignal = createSignal<number | null>(null);
  private readonly recoveryErrorCodeSignal = createSignal<string | null>(null);
  private readonly zoomPercentSignal = createSignal<number>(WRITER_DEFAULT_ZOOM_PERCENT);
  private readonly pageLayoutSignal = createSignal<WriterPageLayoutView | null>(null);
  private readonly fontResolutionsSignal = createSignal<readonly WriterResolvedFontView[]>([]);
  private readonly printPreviewLayoutSignal = createSignal<WriterPageLayoutView | null>(null);
  private readonly printErrorCodeSignal = createSignal<string | null>(null);

  public readonly document: Accessor<WriterDocumentView | null> = this.documentSignal[0];
  public readonly selection: Accessor<WriterSelectionView | null> = this.selectionSignal[0];
  public readonly typingStyle: Accessor<WriterCharacterStyleView | null> = this.typingStyleSignal[0];
  public readonly filePath: Accessor<string | null> = this.filePathSignal[0];
  public readonly fileSession: Accessor<WriterFileSessionView | null> = this.fileSessionSignal[0];
  public readonly savedRevision: Accessor<number | null> = this.savedRevisionSignal[0];
  public readonly status: Accessor<WriterSessionStatus> = this.statusSignal[0];
  public readonly errorCode: Accessor<string | null> = this.errorCodeSignal[0];
  public readonly recoveryCandidates: Accessor<readonly RecoverySnapshotView[]> = this.recoveryCandidatesSignal[0];
  public readonly recoveryComparison: Accessor<RecoveryComparisonView | null> = this.recoveryComparisonSignal[0];
  public readonly recoveryRevision: Accessor<number | null> = this.recoveryRevisionSignal[0];
  public readonly recoveryErrorCode: Accessor<string | null> = this.recoveryErrorCodeSignal[0];
  public readonly zoomPercent: Accessor<number> = this.zoomPercentSignal[0];
  public readonly pageLayout: Accessor<WriterPageLayoutView | null> = this.pageLayoutSignal[0];
  public readonly fontResolutions: Accessor<readonly WriterResolvedFontView[]> = this.fontResolutionsSignal[0];
  public readonly printPreviewLayout: Accessor<WriterPageLayoutView | null> = this.printPreviewLayoutSignal[0];
  public readonly printErrorCode: Accessor<string | null> = this.printErrorCodeSignal[0];
  public readonly dirty: Accessor<boolean> = createMemo(() => {
    const document = this.document();
    const baseline = this.savedRevision();
    return document !== null && baseline !== null && document.revision !== baseline;
  });

  public setLoading(): void {
    this.statusSignal[1]("loading");
    this.errorCodeSignal[1](null);
  }

  public setDocument(document: WriterDocumentView): void {
    this.documentSignal[1](document);
    this.statusSignal[1]("ready");
    this.errorCodeSignal[1](null);
  }

  public setNewDocument(document: WriterDocumentView): void {
    this.filePathSignal[1](null);
    this.fileSessionSignal[1](null);
    this.savedRevisionSignal[1](document.revision);
    this.recoveryRevisionSignal[1](null);
    this.setPrintPreviewLayout(null);
    this.setDocument(document);
  }

  public setOpenedDocument(result: WriterFileOperationView): void {
    this.filePathSignal[1](result.path);
    this.savedRevisionSignal[1](result.document.revision);
    this.recoveryRevisionSignal[1](null);
    this.setPrintPreviewLayout(null);
    this.setDocument(result.document);
  }

  public setSavedDocument(result: WriterFileOperationView): void {
    this.filePathSignal[1](result.path);
    this.savedRevisionSignal[1](result.document.revision);
    this.recoveryRevisionSignal[1](null);
    this.setDocument(result.document);
  }


  public setFileSession(fileSession: WriterFileSessionView | null): void {
    this.fileSessionSignal[1](fileSession);
  }


  public setZoomPercent(zoomPercent: number): void {
    this.zoomPercentSignal[1](zoomPercent);
  }

  public setPageLayout(layout: WriterPageLayoutView | null): void {
    this.pageLayoutSignal[1](layout);
  }

  public setFontResolutions(resolutions: readonly WriterResolvedFontView[]): void {
    this.fontResolutionsSignal[1](resolutions);
  }

  public setPrintPreviewLayout(layout: WriterPageLayoutView | null): void {
    this.printPreviewLayoutSignal[1](layout);
    this.printErrorCodeSignal[1](null);
  }

  public setPrintError(errorCode: string): void {
    this.printErrorCodeSignal[1](errorCode);
  }

  public setSelection(selection: WriterSelectionView | null): void {
    this.selectionSignal[1](selection);
  }

  public setTypingStyle(style: WriterCharacterStyleView | null): void {
    this.typingStyleSignal[1](style);
  }

  public setRecoveryCandidates(candidates: readonly RecoverySnapshotView[]): void {
    this.recoveryCandidatesSignal[1](candidates);
    this.statusSignal[1]("ready");
  }

  public setRecoveryComparison(comparison: RecoveryComparisonView | null): void {
    this.recoveryComparisonSignal[1](comparison);
  }

  public markRecovery(snapshot: RecoverySnapshotView): void {
    this.recoveryRevisionSignal[1](snapshot.recoveryRevision);
    this.recoveryErrorCodeSignal[1](null);
  }

  public setRecoveryError(errorCode: string): void {
    this.recoveryErrorCodeSignal[1](errorCode);
  }

  public setRecoveredDocument(result: RecoveryRestoreView): void {
    this.filePathSignal[1](result.snapshot.sourcePath);
    this.savedRevisionSignal[1](result.snapshot.persistedRevision);
    this.recoveryRevisionSignal[1](result.snapshot.recoveryRevision);
    this.recoveryCandidatesSignal[1]([]);
    this.recoveryComparisonSignal[1](null);
    this.setPrintPreviewLayout(null);
    this.setDocument(result.document);
  }

  public setError(errorCode: string): void {
    this.statusSignal[1]("error");
    this.errorCodeSignal[1](errorCode);
  }
}
