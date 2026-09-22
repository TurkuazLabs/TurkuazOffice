// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/tauri-writer.tool.ts
// # 📌 Amac: Desktop frontend ile Tauri Rust backend arasindaki Writer IPC adaptasyonunu yapar
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.2.0
// # Aciklama: Text, typing-style, character style ve paragraph alignment commandlarini typed tasir
// Bagimli Oldugu Katman: Tool

import { invoke } from "@tauri-apps/api/core";

import { IPC_COMMANDS } from "../config/ipc-commands";
import type {
  RecoveryComparisonView,
  RecoveryRestoreView,
  RecoverySnapshotView,
  WriterAssetView,
  WriterCharacterStylePatchView,
  WriterCharacterStyleView,
  WriterDocumentView,
  WriterDocxImportView,
  WriterFileOperationView,
  WriterFileSessionView,
  WriterReloadView,
  WriterStyledRunInputView,
  WriterTextAlignmentView,
} from "../views/writer-types";

export class TauriWriterTool {
  public createDocument(): Promise<WriterDocumentView> {
    return invoke<WriterDocumentView>(IPC_COMMANDS.writerCreateDocument);
  }

  public openDocument(path: string): Promise<WriterFileOperationView> {
    return invoke<WriterFileOperationView>(IPC_COMMANDS.writerOpenDocument, { path });
  }

  public importDocx(path: string): Promise<WriterDocxImportView> {
    return invoke<WriterDocxImportView>(IPC_COMMANDS.writerImportDocx, { path });
  }

  public exportDocx(documentId: string, path: string): Promise<string> {
    return invoke<string>(IPC_COMMANDS.writerExportDocx, { documentId, path });
  }

  public listRecoverySnapshots(): Promise<readonly RecoverySnapshotView[]> {
    return invoke<readonly RecoverySnapshotView[]>(IPC_COMMANDS.writerListRecoverySnapshots);
  }

  public createRecoverySnapshot(
    documentId: string,
    sourcePath: string | null,
    persistedRevision: number,
  ): Promise<RecoverySnapshotView> {
    return invoke<RecoverySnapshotView>(IPC_COMMANDS.writerCreateRecoverySnapshot, {
      documentId,
      sourcePath,
      persistedRevision,
    });
  }

  public restoreRecoverySnapshot(snapshotId: string): Promise<RecoveryRestoreView> {
    return invoke<RecoveryRestoreView>(IPC_COMMANDS.writerRestoreRecoverySnapshot, { snapshotId });
  }

  public compareRecoverySnapshot(snapshotId: string): Promise<RecoveryComparisonView> {
    return invoke<RecoveryComparisonView>(IPC_COMMANDS.writerCompareRecoverySnapshot, { snapshotId });
  }

  public discardRecoverySnapshot(snapshotId: string): Promise<void> {
    return invoke<void>(IPC_COMMANDS.writerDiscardRecoverySnapshot, { snapshotId });
  }

  public clearDocumentRecovery(documentId: string): Promise<void> {
    return invoke<void>(IPC_COMMANDS.writerClearDocumentRecovery, { documentId });
  }

  public saveDocument(documentId: string, path: string): Promise<WriterFileOperationView> {
    return invoke<WriterFileOperationView>(IPC_COMMANDS.writerSaveDocument, {
      documentId,
      path,
    });
  }

  public getFileSession(documentId: string): Promise<WriterFileSessionView> {
    return invoke<WriterFileSessionView>(IPC_COMMANDS.writerGetFileSession, { documentId });
  }

  public acknowledgeExternalChange(documentId: string): Promise<WriterFileSessionView> {
    return invoke<WriterFileSessionView>(IPC_COMMANDS.writerAcknowledgeExternalChange, { documentId });
  }

  public reloadFromDisk(documentId: string): Promise<WriterReloadView> {
    return invoke<WriterReloadView>(IPC_COMMANDS.writerReloadFromDisk, { documentId });
  }

  public getAsset(documentId: string, assetId: string): Promise<WriterAssetView> {
    return invoke<WriterAssetView>(IPC_COMMANDS.writerGetAsset, {
      documentId,
      assetId,
    });
  }

  public insertImageData(
    documentId: string,
    afterParagraphId: string,
    mediaType: string,
    data: readonly number[],
    altText: string,
    widthTwips: number | null,
    heightTwips: number | null,
  ): Promise<WriterDocumentView> {
    return invoke<WriterDocumentView>(IPC_COMMANDS.writerInsertImageData, {
      documentId,
      afterParagraphId,
      mediaType,
      data,
      altText,
      widthTwips,
      heightTwips,
    });
  }

  public replaceParagraphText(
    documentId: string,
    paragraphId: string,
    text: string,
    typingStyle: WriterCharacterStyleView | null,
  ): Promise<WriterDocumentView> {
    return invoke<WriterDocumentView>(IPC_COMMANDS.writerReplaceParagraphText, {
      documentId,
      paragraphId,
      text,
      typingStyle,
    });
  }

  public replaceRangeWithStyledRuns(
    documentId: string,
    paragraphId: string,
    startOffset: number,
    endOffset: number,
    runs: readonly WriterStyledRunInputView[],
  ): Promise<WriterDocumentView> {
    return invoke<WriterDocumentView>(IPC_COMMANDS.writerReplaceRangeWithStyledRuns, {
      documentId,
      paragraphId,
      startOffset,
      endOffset,
      runs,
    });
  }

  public applyCharacterStyle(
    documentId: string,
    paragraphId: string,
    startOffset: number,
    endOffset: number,
    patch: WriterCharacterStylePatchView,
  ): Promise<WriterDocumentView> {
    return invoke<WriterDocumentView>(IPC_COMMANDS.writerApplyCharacterStyle, {
      documentId,
      paragraphId,
      startOffset,
      endOffset,
      bold: patch.bold ?? null,
      italic: patch.italic ?? null,
      underline: patch.underline ?? null,
      fontFamily: patch.fontFamily ?? null,
      fontSizeHalfPoints: patch.fontSizeHalfPoints ?? null,
    });
  }

  public applyParagraphAlignment(
    documentId: string,
    paragraphId: string,
    alignment: WriterTextAlignmentView,
  ): Promise<WriterDocumentView> {
    return invoke<WriterDocumentView>(IPC_COMMANDS.writerApplyParagraphAlignment, {
      documentId,
      paragraphId,
      alignment,
    });
  }

  public splitParagraph(
    documentId: string,
    paragraphId: string,
    offset: number,
  ): Promise<WriterDocumentView> {
    return invoke<WriterDocumentView>(IPC_COMMANDS.writerSplitParagraph, {
      documentId,
      paragraphId,
      offset,
    });
  }

  public mergeWithPrevious(
    documentId: string,
    paragraphId: string,
  ): Promise<WriterDocumentView> {
    return invoke<WriterDocumentView>(IPC_COMMANDS.writerMergeWithPrevious, {
      documentId,
      paragraphId,
    });
  }

  public undo(documentId: string): Promise<WriterDocumentView> {
    return invoke<WriterDocumentView>(IPC_COMMANDS.writerUndo, { documentId });
  }

  public redo(documentId: string): Promise<WriterDocumentView> {
    return invoke<WriterDocumentView>(IPC_COMMANDS.writerRedo, { documentId });
  }
}
