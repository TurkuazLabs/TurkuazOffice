// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-types.ts
// # 📌 Amac: Tauri backend tarafindan gelen Writer read-only View DTO ve UI selection tiplerini tanimlar
// # 📌 Modul - FileType: View - TypeScript
// # Version: 0.2.0
// # Aciklama: Belge, recovery, typography, paragraph alignment, selection ve typing-style kontratlarini typed tutar
// Bagimli Oldugu Katman: View

export type WriterTextAlignmentView = "left" | "center" | "right" | "justify";

export interface WriterCharacterStyleView {
  readonly bold: boolean;
  readonly italic: boolean;
  readonly underline: boolean;
  readonly fontFamily: string;
  readonly fontSizeHalfPoints: number;
}

export interface WriterRunView {
  readonly id: string;
  readonly text: string;
  readonly style: WriterCharacterStyleView;
}

export interface WriterStyledRunInputView {
  readonly text: string;
  readonly style: WriterCharacterStyleView;
}

export interface WriterParagraphStyleView {
  readonly alignment: WriterTextAlignmentView;
}

export interface WriterParagraphView {
  readonly id: string;
  readonly plainText: string;
  readonly style: WriterParagraphStyleView;
  readonly runs: readonly WriterRunView[];
}

export interface WriterImageView {
  readonly id: string;
  readonly assetId: string;
  readonly afterParagraphId: string | null;
  readonly altText: string;
  readonly widthTwips: number | null;
  readonly heightTwips: number | null;
}

export interface WriterAssetView {
  readonly id: string;
  readonly mediaType: string;
  readonly data: readonly number[];
}

export interface WriterPageSettingsView {
  readonly widthTwips: number;
  readonly heightTwips: number;
  readonly marginTopTwips: number;
  readonly marginRightTwips: number;
  readonly marginBottomTwips: number;
  readonly marginLeftTwips: number;
}

export type DocxUnsupportedFeatureView =
  | "table"
  | "image"
  | "numbering"
  | "hyperlink"
  | "header_footer"
  | "comments"
  | "tracked_changes"
  | "fields";

export interface WriterDocxCompatibilityView {
  readonly unsupportedFeatures: readonly DocxUnsupportedFeatureView[];
}

export interface WriterDocxImportView {
  readonly document: WriterDocumentView;
  readonly compatibility: WriterDocxCompatibilityView;
}

export interface WriterDocumentView {
  readonly id: string;
  readonly title: string;
  readonly revision: number;
  readonly plainText: string;
  readonly sectionCount: number;
  readonly pageSettings: WriterPageSettingsView;
  readonly paragraphs: readonly WriterParagraphView[];
  readonly images: readonly WriterImageView[];
}

export interface WriterPageLayoutView {
  readonly pageWidthPx: number;
  readonly pageHeightPx: number;
  readonly marginTopPx: number;
  readonly marginRightPx: number;
  readonly marginBottomPx: number;
  readonly marginLeftPx: number;
  readonly scale: number;
  readonly zoomPercent: number;
}

export interface WriterResolvedFontView {
  readonly requestedFamily: string;
  readonly resolvedFamily: string;
  readonly cssStack: string;
  readonly substituted: boolean;
}



export type WriterExternalChangeStateView = "untracked" | "unchanged" | "modified" | "missing";

export interface WriterFileSessionView {
  readonly path: string | null;
  readonly readOnly: boolean;
  readonly lockOwned: boolean;
  readonly externalState: WriterExternalChangeStateView;
}

export interface WriterReloadView {
  readonly document: WriterDocumentView;
  readonly fileSession: WriterFileSessionView;
}

export interface WriterFileOperationView {
  readonly document: WriterDocumentView;
  readonly path: string;
}

export interface RecoverySnapshotView {
  readonly snapshotId: string;
  readonly documentId: string;
  readonly title: string;
  readonly sourcePath: string | null;
  readonly persistedRevision: number;
  readonly recoveryRevision: number;
  readonly createdAtUnixMs: number;
}

export interface RecoveryRestoreView {
  readonly snapshot: RecoverySnapshotView;
  readonly document: WriterDocumentView;
}

export interface RecoveryComparisonView {
  readonly snapshot: RecoverySnapshotView;
  readonly recoveryPlainText: string;
  readonly sourcePlainText: string | null;
  readonly sourceRevision: number | null;
}

export interface WriterSelectionView {
  readonly paragraphId: string;
  readonly startOffset: number;
  readonly endOffset: number;
}

export interface WriterFormatStateView {
  readonly canFormat: boolean;
  readonly hasRangeSelection: boolean;
  readonly bold: boolean;
  readonly italic: boolean;
  readonly underline: boolean;
  readonly fontFamily: string;
  readonly fontSizeHalfPoints: number;
  readonly alignment: WriterTextAlignmentView;
}

export interface WriterCharacterStylePatchView {
  readonly bold?: boolean;
  readonly italic?: boolean;
  readonly underline?: boolean;
  readonly fontFamily?: string;
  readonly fontSizeHalfPoints?: number;
}

export interface DesktopErrorView {
  readonly code: string;
}
