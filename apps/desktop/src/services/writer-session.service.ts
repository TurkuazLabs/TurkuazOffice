// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/writer-session.service.ts
// # 📌 Amac: Desktop Writer oturum, selection, typing-style ve typography is akisini koordine eder
// # 📌 Modul - FileType: Service - TypeScript
// # Version: 0.2.0
// # Aciklama: DOM, Repo, dialog, print ve Tauri Tool uzerinden edit, clipboard mutation, Open/Save, preview, autosave recovery, dirty guard ve history kurallarini yurutur
// Bagimli Oldugu Katman: Service -> Repo -> Tool

import { ERROR_CODES } from "../config/error-codes";
import { WRITER_PRINT_PREVIEW_ZOOM_PERCENT } from "../config/print";
import {
  DEFAULT_WRITER_FONT_FAMILY,
  DEFAULT_WRITER_FONT_SIZE_HALF_POINTS,
} from "../config/typography";
import type { LanguageService } from "../language/language-service";
import type { WriterSessionRepository } from "../repositories/writer-session.repository";
import type { WriterLayoutService } from "./writer-layout.service";
import type { DomSelectionTool } from "../tools/dom-selection.tool";
import type { ImageAssetTool } from "../tools/image-asset.tool";
import type { NativeFileDialogTool } from "../tools/native-file-dialog.tool";
import type { PrintTool } from "../tools/print.tool";
import type { TauriWriterTool } from "../tools/tauri-writer.tool";
import type { TextOffsetTool } from "../tools/text-offset.tool";
import type {
  DesktopErrorView,
  WriterCharacterStylePatchView,
  WriterCharacterStyleView,
  WriterDocumentView,
  WriterFormatStateView,
  WriterParagraphView,
  WriterSelectionView,
  WriterStyledRunInputView,
  WriterTextAlignmentView,
} from "../views/writer-types";

const EMPTY_FORMAT_STATE: WriterFormatStateView = {
  canFormat: false,
  hasRangeSelection: false,
  bold: false,
  italic: false,
  underline: false,
  fontFamily: DEFAULT_WRITER_FONT_FAMILY,
  fontSizeHalfPoints: DEFAULT_WRITER_FONT_SIZE_HALF_POINTS,
  alignment: "left",
};

export class WriterSessionService {
  private mutationQueue: Promise<void> = Promise.resolve();

  public constructor(
    private readonly repository: WriterSessionRepository,
    private readonly writerTool: TauriWriterTool,
    private readonly textOffsetTool: TextOffsetTool,
    private readonly domSelectionTool: DomSelectionTool,
    private readonly fileDialogTool: NativeFileDialogTool,
    private readonly imageAssetTool: ImageAssetTool,
    private readonly printTool: PrintTool,
    private readonly layoutService: WriterLayoutService,
    private readonly language: LanguageService,
  ) {}

  public async initializeSession(): Promise<void> {
    try {
      const candidates = await this.writerTool.listRecoverySnapshots();
      if (candidates.length > 0) {
        this.repository.setRecoveryCandidates(candidates);
        return;
      }
    } catch (error: unknown) {
      this.repository.setRecoveryError(this.errorCode(error));
    }
    await this.createDocument();
  }

  public async autosaveRecovery(): Promise<void> {
    const document = this.repository.document();
    const persistedRevision = this.repository.savedRevision();
    if (
      document === null ||
      persistedRevision === null ||
      !this.repository.dirty() ||
      this.repository.recoveryRevision() === document.revision
    ) {
      return;
    }

    await this.enqueue(async () => {
      const current = this.repository.document();
      const baseline = this.repository.savedRevision();
      if (current === null || baseline === null || current.revision === this.repository.recoveryRevision()) {
        return;
      }
      try {
        this.repository.markRecovery(
          await this.writerTool.createRecoverySnapshot(
            current.id,
            this.repository.filePath(),
            baseline,
          ),
        );
      } catch (error: unknown) {
        this.repository.setRecoveryError(this.errorCode(error));
      }
    });
  }

  public async recoverSnapshot(snapshotId: string): Promise<void> {
    this.repository.setLoading();
    try {
      this.repository.setRecoveredDocument(await this.writerTool.restoreRecoverySnapshot(snapshotId));
      this.refreshLayoutEnvironment();
      await this.refreshFileSession();
    } catch (error: unknown) {
      this.repository.setError(this.errorCode(error));
    }
  }

  public async compareRecoverySnapshot(snapshotId: string): Promise<void> {
    try {
      this.repository.setRecoveryComparison(await this.writerTool.compareRecoverySnapshot(snapshotId));
    } catch (error: unknown) {
      this.repository.setRecoveryError(this.errorCode(error));
    }
  }

  public closeRecoveryComparison(): void {
    this.repository.setRecoveryComparison(null);
  }

  public async discardRecoverySnapshot(snapshotId: string): Promise<void> {
    try {
      await this.writerTool.discardRecoverySnapshot(snapshotId);
      const candidates = await this.writerTool.listRecoverySnapshots();
      this.repository.setRecoveryCandidates(candidates);
      this.repository.setRecoveryComparison(null);
      if (candidates.length === 0 && this.repository.document() === null) {
        await this.createDocument();
      }
    } catch (error: unknown) {
      this.repository.setRecoveryError(this.errorCode(error));
    }
  }

  public async createDocument(): Promise<void> {
    await this.prepareFileOperation();
    if (!(await this.confirmDiscardIfNeeded())) {
      return;
    }
    await this.clearCurrentRecoveryForDiscard();
    this.repository.setSelection(null);
    this.repository.setTypingStyle(null);
    await this.enqueue(async () => {
      this.repository.setLoading();
      try {
        this.repository.setNewDocument(await this.writerTool.createDocument());
        this.refreshLayoutEnvironment();
      } catch (error: unknown) {
        this.repository.setError(this.errorCode(error));
      }
    });
  }

  public async openDocument(): Promise<void> {
    await this.prepareFileOperation();
    if (!(await this.confirmDiscardIfNeeded())) {
      return;
    }
    const path = await this.fileDialogTool.openTko(this.language.text("tkoFileFilter"));
    if (path === null) {
      return;
    }
    await this.clearCurrentRecoveryForDiscard();

    this.repository.setSelection(null);
    this.repository.setTypingStyle(null);
    await this.enqueue(async () => {
      this.repository.setLoading();
      try {
        this.repository.setOpenedDocument(await this.writerTool.openDocument(path));
        this.refreshLayoutEnvironment();
        await this.refreshFileSession();
      } catch (error: unknown) {
        this.repository.setError(this.errorCode(error));
      }
    });
  }

  public async saveDocument(): Promise<void> {
    await this.prepareFileOperation();
    const document = this.requireDocument();
    let path = this.repository.filePath();
    if (path === null) {
      path = await this.fileDialogTool.saveTko(this.language.text("tkoFileFilter"));
    }
    if (path === null) {
      return;
    }

    await this.enqueue(async () => {
      this.repository.setLoading();
      try {
        this.repository.setSavedDocument(await this.writerTool.saveDocument(document.id, path));
        this.refreshLayoutEnvironment();
        await this.refreshFileSession();
      } catch (error: unknown) {
        this.repository.setError(this.errorCode(error));
      }
    });
  }

  public async openPrintPreview(): Promise<void> {
    await this.prepareFileOperation();
    const document = this.repository.document();
    if (document === null) {
      return;
    }
    this.repository.setPrintPreviewLayout(
      this.layoutService.pageLayout(document.pageSettings, WRITER_PRINT_PREVIEW_ZOOM_PERCENT),
    );
  }

  public closePrintPreview(): void {
    this.repository.setPrintPreviewLayout(null);
    this.restoreSessionSelection();
  }

  public async printDocument(): Promise<void> {
    await this.prepareFileOperation();
    const document = this.requireDocument();
    this.repository.setPrintPreviewLayout(
      this.layoutService.pageLayout(document.pageSettings, WRITER_PRINT_PREVIEW_ZOOM_PERCENT),
    );
    try {
      this.printTool.print(document.pageSettings);
    } catch (error: unknown) {
      this.repository.setPrintError(this.errorCode(error));
    }
  }

  public async saveDocumentAs(): Promise<void> {
    await this.prepareFileOperation();
    const document = this.requireDocument();
    const path = await this.fileDialogTool.saveTko(this.language.text("tkoFileFilter"));
    if (path === null) {
      return;
    }
    await this.enqueue(async () => {
      this.repository.setLoading();
      try {
        this.repository.setSavedDocument(await this.writerTool.saveDocument(document.id, path));
        this.refreshLayoutEnvironment();
        await this.refreshFileSession();
      } catch (error: unknown) {
        this.repository.setError(this.errorCode(error));
      }
    });
  }

  public async pollExternalChange(): Promise<void> {
    const document = this.repository.document();
    if (document === null || this.repository.filePath() === null) {
      return;
    }
    try {
      this.repository.setFileSession(await this.writerTool.getFileSession(document.id));
    } catch (error: unknown) {
      this.repository.setRecoveryError(this.errorCode(error));
    }
  }

  public async reloadFromDisk(): Promise<void> {
    await this.prepareFileOperation();
    if (!(await this.confirmDiscardIfNeeded())) {
      return;
    }
    const document = this.requireDocument();
    await this.clearCurrentRecoveryForDiscard();
    this.repository.setSelection(null);
    this.repository.setTypingStyle(null);
    this.repository.setLoading();
    try {
      const result = await this.writerTool.reloadFromDisk(document.id);
      const path = result.fileSession.path;
      if (path === null) {
        throw new Error(ERROR_CODES.unknown);
      }
      this.repository.setSavedDocument({ document: result.document, path });
      this.refreshLayoutEnvironment();
      this.repository.setFileSession(result.fileSession);
    } catch (error: unknown) {
      this.repository.setError(this.errorCode(error));
    }
  }

  public async keepLocalVersion(): Promise<void> {
    await this.prepareFileOperation();
    const document = this.requireDocument();
    const path = this.repository.filePath();
    if (path === null) {
      return;
    }
    await this.enqueue(async () => {
      this.repository.setLoading();
      try {
        this.repository.setFileSession(await this.writerTool.acknowledgeExternalChange(document.id));
        this.repository.setSavedDocument(await this.writerTool.saveDocument(document.id, path));
        this.refreshLayoutEnvironment();
        await this.refreshFileSession();
      } catch (error: unknown) {
        this.repository.setError(this.errorCode(error));
      }
    });
  }

  public zoomIn(): void {
    this.repository.setZoomPercent(this.layoutService.zoomIn(this.repository.zoomPercent()));
    this.refreshPageLayout();
  }

  public zoomOut(): void {
    this.repository.setZoomPercent(this.layoutService.zoomOut(this.repository.zoomPercent()));
    this.refreshPageLayout();
  }

  public resetZoom(): void {
    this.repository.setZoomPercent(this.layoutService.resetZoom());
    this.refreshPageLayout();
  }

  public captureSelection(paragraphId: string, editor: HTMLElement): WriterSelectionView | null {
    const selection = this.domSelectionTool.readParagraphSelection(editor, paragraphId);
    this.repository.setSelection(selection);
    return selection;
  }

  public restoreSelection(paragraphId: string, editor: HTMLElement): boolean {
    const selection = this.repository.selection();
    if (selection === null || selection.paragraphId !== paragraphId) {
      return false;
    }
    return this.domSelectionTool.restoreParagraphSelection(editor, selection);
  }

  public restoreSessionSelection(): boolean {
    const selection = this.repository.selection();
    if (selection === null) {
      return false;
    }
    return this.domSelectionTool.focusAndRestoreParagraphSelection(selection);
  }

  public clearTypingStyle(): void {
    this.repository.setTypingStyle(null);
  }

  public flushFocusedParagraph(): Promise<void> {
    const editor = this.domSelectionTool.activeWriterParagraph();
    const paragraphId = editor?.dataset.writerParagraphId ?? null;
    if (editor === null || paragraphId === null) {
      return Promise.resolve();
    }
    return this.commitParagraphFromEditor(paragraphId, editor);
  }

  public isCaretAtParagraphStart(paragraphId: string, editor: HTMLElement): boolean {
    const selection = this.captureSelection(paragraphId, editor);
    return selection !== null && selection.startOffset === 0 && selection.endOffset === 0;
  }

  public commitParagraphFromEditor(paragraphId: string, editor: HTMLElement): Promise<void> {
    if (this.isReadOnly()) {
      return Promise.resolve();
    }
    const text = this.domSelectionTool.plainText(editor);
    const selection = this.domSelectionTool.readParagraphSelection(editor, paragraphId);
    if (selection !== null) {
      this.repository.setSelection(selection);
    }
    return this.replaceParagraphText(paragraphId, text, this.repository.typingStyle());
  }

  public async loadImageAssetUrl(documentId: string, assetId: string): Promise<string> {
    const asset = await this.writerTool.getAsset(documentId, assetId);
    return this.imageAssetTool.createObjectUrl(asset);
  }

  public releaseImageAssetUrl(url: string): void {
    this.imageAssetTool.revokeObjectUrl(url);
  }

  public insertImageData(
    afterParagraphId: string,
    mediaType: string,
    data: readonly number[],
    altText = "",
  ): Promise<void> {
    if (this.isReadOnly()) {
      return Promise.resolve();
    }
    return this.enqueue(async () => {
      const document = this.requireDocument();
      const changed = await this.run(() =>
        this.writerTool.insertImageData(
          document.id,
          afterParagraphId,
          mediaType,
          data,
          altText,
          null,
          null,
        ),
      );
      if (changed) {
        this.repository.setSelection(null);
        this.repository.setTypingStyle(null);
      }
    });
  }

  public replaceParagraphText(
    paragraphId: string,
    text: string,
    typingStyle: WriterCharacterStyleView | null = null,
  ): Promise<void> {
    if (this.isReadOnly()) {
      return Promise.resolve();
    }
    return this.enqueue(async () => {
      const document = this.requireDocument();
      await this.run(() =>
        this.writerTool.replaceParagraphText(document.id, paragraphId, text, typingStyle),
      );
    });
  }

  public splitParagraphFromEditor(paragraphId: string, editor: HTMLElement): Promise<void> {
    if (this.isReadOnly()) {
      return Promise.resolve();
    }
    const selection = this.captureSelection(paragraphId, editor);
    const currentText = this.domSelectionTool.plainText(editor);
    const fallbackOffset = this.textOffsetTool.logicalLength(currentText);
    const splitOffset = selection?.startOffset ?? fallbackOffset;
    const committedText = selection === null
      ? currentText
      : this.textOffsetTool.removeLogicalRange(
          currentText,
          selection.startOffset,
          selection.endOffset,
        );
    const typingStyle = this.repository.typingStyle();

    this.repository.setSelection(null);
    return this.enqueue(async () => {
      const document = this.requireDocument();
      const committed = await this.run(() =>
        this.writerTool.replaceParagraphText(document.id, paragraphId, committedText, typingStyle),
      );
      if (!committed) {
        return;
      }
      const updatedDocument = this.requireDocument();
      await this.run(() =>
        this.writerTool.splitParagraph(updatedDocument.id, paragraphId, splitOffset),
      );
    });
  }

  public mergeWithPreviousFromEditor(paragraphId: string, editor: HTMLElement): Promise<void> {
    if (this.isReadOnly()) {
      return Promise.resolve();
    }
    const currentText = this.domSelectionTool.plainText(editor);
    const typingStyle = this.repository.typingStyle();
    this.repository.setSelection(null);
    this.repository.setTypingStyle(null);
    return this.enqueue(async () => {
      const document = this.requireDocument();
      const committed = await this.run(() =>
        this.writerTool.replaceParagraphText(document.id, paragraphId, currentText, typingStyle),
      );
      if (!committed) {
        return;
      }
      const updatedDocument = this.requireDocument();
      await this.run(() => this.writerTool.mergeWithPrevious(updatedDocument.id, paragraphId));
    });
  }

  public replaceSelectionWithStyledRuns(
    paragraphId: string,
    startOffset: number,
    endOffset: number,
    runs: readonly WriterStyledRunInputView[],
  ): Promise<void> {
    if (this.isReadOnly()) {
      return Promise.resolve();
    }

    const insertedLength = runs.reduce(
      (total, run) => total + this.textOffsetTool.logicalLength(run.text),
      0,
    );

    return this.enqueue(async () => {
      const currentDocument = this.requireDocument();
      const changed = await this.run(() =>
        this.writerTool.replaceRangeWithStyledRuns(
          currentDocument.id,
          paragraphId,
          startOffset,
          endOffset,
          runs,
        ),
      );
      if (changed) {
        const caretOffset = startOffset + insertedLength;
        this.repository.setSelection({
          paragraphId,
          startOffset: caretOffset,
          endOffset: caretOffset,
        });
      }
      this.repository.setTypingStyle(null);
    });
  }

  public insertionStyle(
    paragraphId: string,
    offset: number,
  ): WriterCharacterStyleView {
    const paragraph = this.repository
      .document()
      ?.paragraphs.find((item) => item.id === paragraphId);
    return (
      this.repository.typingStyle() ??
      (paragraph === undefined ? null : this.styleAtCaret(paragraph, offset)) ?? {
        bold: false,
        italic: false,
        underline: false,
        fontFamily: DEFAULT_WRITER_FONT_FAMILY,
        fontSizeHalfPoints: DEFAULT_WRITER_FONT_SIZE_HALF_POINTS,
      }
    );
  }

  public toggleBold(): Promise<void> {
    const state = this.formatState();
    return this.applyCharacterStyle({ bold: !state.bold });
  }

  public toggleItalic(): Promise<void> {
    const state = this.formatState();
    return this.applyCharacterStyle({ italic: !state.italic });
  }

  public toggleUnderline(): Promise<void> {
    const state = this.formatState();
    return this.applyCharacterStyle({ underline: !state.underline });
  }

  public setFontFamily(fontFamily: string): Promise<void> {
    return this.applyCharacterStyle({ fontFamily });
  }

  public setFontSizeHalfPoints(fontSizeHalfPoints: number): Promise<void> {
    return this.applyCharacterStyle({ fontSizeHalfPoints });
  }

  public setParagraphAlignment(alignment: WriterTextAlignmentView): Promise<void> {
    if (this.isReadOnly()) {
      return Promise.resolve();
    }
    const selection = this.repository.selection();
    if (selection === null) {
      return Promise.resolve();
    }
    const activeEditor = this.domSelectionTool.activeWriterParagraph();
    const activeParagraphId = activeEditor?.dataset.writerParagraphId ?? null;
    const activeText = activeEditor === null ? null : this.domSelectionTool.plainText(activeEditor);
    const typingStyle = this.repository.typingStyle();

    return this.enqueue(async () => {
      let document = this.requireDocument();
      if (activeParagraphId === selection.paragraphId && activeText !== null) {
        const committed = await this.run(() =>
          this.writerTool.replaceParagraphText(
            document.id,
            selection.paragraphId,
            activeText,
            typingStyle,
          ),
        );
        if (!committed) {
          return;
        }
        document = this.requireDocument();
      }
      await this.run(() =>
        this.writerTool.applyParagraphAlignment(
          document.id,
          selection.paragraphId,
          alignment,
        ),
      );
    });
  }

  public formatState(): WriterFormatStateView {
    if (this.repository.fileSession()?.readOnly === true) {
      return EMPTY_FORMAT_STATE;
    }
    const document = this.repository.document();
    const selection = this.repository.selection();
    if (document === null || selection === null) {
      return EMPTY_FORMAT_STATE;
    }
    const paragraph = document.paragraphs.find((item) => item.id === selection.paragraphId);
    if (paragraph === undefined) {
      return EMPTY_FORMAT_STATE;
    }

    const hasRangeSelection = selection.startOffset !== selection.endOffset;
    if (!hasRangeSelection) {
      const style = this.repository.typingStyle() ?? this.styleAtCaret(paragraph, selection.startOffset);
      return {
        canFormat: style !== null,
        hasRangeSelection: false,
        bold: style?.bold ?? false,
        italic: style?.italic ?? false,
        underline: style?.underline ?? false,
        fontFamily: style?.fontFamily ?? DEFAULT_WRITER_FONT_FAMILY,
        fontSizeHalfPoints: style?.fontSizeHalfPoints ?? DEFAULT_WRITER_FONT_SIZE_HALF_POINTS,
        alignment: paragraph.style.alignment,
      };
    }

    const selectedRuns = this.selectedRuns(paragraph, selection);
    if (selectedRuns.length === 0) {
      return EMPTY_FORMAT_STATE;
    }
    const firstRun = selectedRuns[0];
    if (firstRun === undefined) {
      return EMPTY_FORMAT_STATE;
    }
    const first = firstRun.style;
    const sameFamily = selectedRuns.every((run) => run.style.fontFamily === first.fontFamily);
    const sameSize = selectedRuns.every(
      (run) => run.style.fontSizeHalfPoints === first.fontSizeHalfPoints,
    );
    return {
      canFormat: true,
      hasRangeSelection: true,
      bold: selectedRuns.every((run) => run.style.bold),
      italic: selectedRuns.every((run) => run.style.italic),
      underline: selectedRuns.every((run) => run.style.underline),
      fontFamily: sameFamily ? first.fontFamily : "",
      fontSizeHalfPoints: sameSize ? first.fontSizeHalfPoints : 0,
      alignment: paragraph.style.alignment,
    };
  }

  public undo(): Promise<void> {
    if (this.isReadOnly()) {
      return Promise.resolve();
    }
    this.repository.setSelection(null);
    this.repository.setTypingStyle(null);
    return this.enqueue(async () => {
      const document = this.requireDocument();
      await this.run(() => this.writerTool.undo(document.id));
    });
  }

  public redo(): Promise<void> {
    if (this.isReadOnly()) {
      return Promise.resolve();
    }
    this.repository.setSelection(null);
    this.repository.setTypingStyle(null);
    return this.enqueue(async () => {
      const document = this.requireDocument();
      await this.run(() => this.writerTool.redo(document.id));
    });
  }

  private isReadOnly(): boolean {
    return this.repository.fileSession()?.readOnly === true;
  }

  private refreshLayoutEnvironment(): void {
    this.refreshPageLayout();
    const document = this.repository.document();
    this.repository.setFontResolutions(
      document === null ? [] : this.layoutService.resolveDocumentFonts(document),
    );
  }

  private refreshPageLayout(): void {
    const document = this.repository.document();
    this.repository.setPageLayout(
      document === null
        ? null
        : this.layoutService.pageLayout(document.pageSettings, this.repository.zoomPercent()),
    );
  }

  private async refreshFileSession(): Promise<void> {
    const document = this.repository.document();
    if (document === null || this.repository.filePath() === null) {
      this.repository.setFileSession(null);
      return;
    }
    this.repository.setFileSession(await this.writerTool.getFileSession(document.id));
  }

  private async clearCurrentRecoveryForDiscard(): Promise<void> {
    const document = this.repository.document();
    if (document === null || !this.repository.dirty()) {
      return;
    }
    try {
      await this.writerTool.clearDocumentRecovery(document.id);
    } catch (error: unknown) {
      this.repository.setRecoveryError(this.errorCode(error));
    }
  }

  private async prepareFileOperation(): Promise<void> {
    await this.flushFocusedParagraph();
    await this.mutationQueue;
  }

  private async confirmDiscardIfNeeded(): Promise<boolean> {
    if (!this.repository.dirty()) {
      return true;
    }
    return this.fileDialogTool.confirmDiscard(
      this.language.text("unsavedChangesMessage"),
      this.language.text("unsavedChangesTitle"),
    );
  }

  private applyCharacterStyle(patch: WriterCharacterStylePatchView): Promise<void> {
    if (this.isReadOnly()) {
      return Promise.resolve();
    }
    const selection = this.repository.selection();
    const document = this.repository.document();
    if (selection === null || document === null) {
      return Promise.resolve();
    }
    const paragraph = document.paragraphs.find((item) => item.id === selection.paragraphId);
    if (paragraph === undefined) {
      return Promise.resolve();
    }

    if (selection.startOffset === selection.endOffset) {
      const base = this.repository.typingStyle() ?? this.styleAtCaret(paragraph, selection.startOffset);
      if (base !== null) {
        this.repository.setTypingStyle(this.patchStyle(base, patch));
      }
      return Promise.resolve();
    }

    const activeEditor = this.domSelectionTool.activeWriterParagraph();
    const activeParagraphId = activeEditor?.dataset.writerParagraphId ?? null;
    const activeText = activeEditor === null ? null : this.domSelectionTool.plainText(activeEditor);
    const typingStyle = this.repository.typingStyle();

    return this.enqueue(async () => {
      let currentDocument = this.requireDocument();
      if (activeParagraphId === selection.paragraphId && activeText !== null) {
        const committed = await this.run(() =>
          this.writerTool.replaceParagraphText(
            currentDocument.id,
            selection.paragraphId,
            activeText,
            typingStyle,
          ),
        );
        if (!committed) {
          return;
        }
        currentDocument = this.requireDocument();
      }

      await this.run(() =>
        this.writerTool.applyCharacterStyle(
          currentDocument.id,
          selection.paragraphId,
          selection.startOffset,
          selection.endOffset,
          patch,
        ),
      );
    });
  }

  private patchStyle(
    style: WriterCharacterStyleView,
    patch: WriterCharacterStylePatchView,
  ): WriterCharacterStyleView {
    return {
      bold: patch.bold ?? style.bold,
      italic: patch.italic ?? style.italic,
      underline: patch.underline ?? style.underline,
      fontFamily: patch.fontFamily ?? style.fontFamily,
      fontSizeHalfPoints: patch.fontSizeHalfPoints ?? style.fontSizeHalfPoints,
    };
  }

  private styleAtCaret(
    paragraph: WriterParagraphView,
    offset: number,
  ): WriterCharacterStyleView | null {
    let cursor = 0;
    for (const run of paragraph.runs) {
      const end = cursor + this.textOffsetTool.logicalLength(run.text);
      if (offset <= end) {
        return run.style;
      }
      cursor = end;
    }
    return paragraph.runs.at(-1)?.style ?? null;
  }

  private selectedRuns(paragraph: WriterParagraphView, selection: WriterSelectionView) {
    let cursor = 0;
    return paragraph.runs.filter((run) => {
      const length = this.textOffsetTool.logicalLength(run.text);
      const start = cursor;
      const end = cursor + length;
      cursor = end;
      return selection.startOffset < end && selection.endOffset > start;
    });
  }

  private enqueue(operation: () => Promise<void>): Promise<void> {
    const next = this.mutationQueue.then(operation, operation);
    this.mutationQueue = next.catch(() => undefined);
    return next;
  }

  private requireDocument(): WriterDocumentView {
    const document = this.repository.document();
    if (document === null) {
      throw new Error(ERROR_CODES.unknown);
    }
    return document;
  }

  private async run(operation: () => Promise<WriterDocumentView>): Promise<boolean> {
    this.repository.setLoading();
    try {
      this.repository.setDocument(await operation());
      this.refreshLayoutEnvironment();
      return true;
    } catch (error: unknown) {
      this.repository.setError(this.errorCode(error));
      return false;
    }
  }

  private errorCode(error: unknown): string {
    if (typeof error === "object" && error !== null && "code" in error) {
      return String((error as DesktopErrorView).code);
    }
    return ERROR_CODES.unknown;
  }
}
