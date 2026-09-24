// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/controllers/writer.controller.ts
// # 📌 Amac: Writer View requestlerini alip yalnizca WriterSessionService cagirir
// # 📌 Modul - FileType: Controller - TypeScript
// # Version: 0.2.0
// # Aciklama: File, recovery, DOCX, print, IME, clipboard, selection, typography, paragraph ve history requestleri icin ince Controller siniridir
// Bagimli Oldugu Katman: Controller -> Service

import type { ClipboardService } from "../services/clipboard.service";
import type { LanguagePreferenceService } from "../services/language-preference.service";
import type {
  KeyboardShortcutInput,
  KeyboardShortcutService,
} from "../services/keyboard-shortcut.service";
import type { WriterSessionService } from "../services/writer-session.service";
import type { DesktopLocale } from "../config/localization";
import type { DesktopLocaleOption } from "../language/language-packs";
import type { WriterShortcutAction } from "../config/keyboard";
import type {
  RecentFileView,
  WriterFormatStateView,
  WriterSelectionView,
  WriterStyledRunInputView,
  WriterTextAlignmentView,
  WriterTemplateView,
} from "../views/writer-types";

export class WriterController {
  public constructor(
    private readonly service: WriterSessionService,
    private readonly clipboardService: ClipboardService,
    private readonly languagePreferenceService: LanguagePreferenceService,
    private readonly keyboardShortcutService: KeyboardShortcutService,
  ) {}

  public locale(): DesktopLocale {
    return this.languagePreferenceService.locale();
  }

  public localeOptions(): readonly DesktopLocaleOption[] {
    return this.languagePreferenceService.options();
  }

  public setLocale(locale: string): void {
    this.languagePreferenceService.setLocale(locale);
  }

  public resolveKeyboardShortcut(
    input: KeyboardShortcutInput,
  ): WriterShortcutAction | null {
    return this.keyboardShortcutService.resolve(input);
  }

  public initializeSession(): Promise<void> {
    return this.service.initializeSession();
  }

  public autosaveRecovery(): Promise<void> {
    return this.service.autosaveRecovery();
  }

  public recoverSnapshot(snapshotId: string): Promise<void> {
    return this.service.recoverSnapshot(snapshotId);
  }

  public compareRecoverySnapshot(snapshotId: string): Promise<void> {
    return this.service.compareRecoverySnapshot(snapshotId);
  }

  public closeRecoveryComparison(): void {
    this.service.closeRecoveryComparison();
  }

  public discardRecoverySnapshot(snapshotId: string): Promise<void> {
    return this.service.discardRecoverySnapshot(snapshotId);
  }

  public createDocument(): Promise<void> {
    return this.service.createDocument();
  }

  public templates(): readonly WriterTemplateView[] {
    return this.service.templates();
  }

  public createDocumentFromTemplate(templateId: string): Promise<void> {
    return this.service.createDocumentFromTemplate(templateId);
  }

  public openDocument(): Promise<void> {
    return this.service.openDocument();
  }

  public openRecentFile(path: string): Promise<void> {
    return this.service.openRecentFile(path);
  }

  public recentFiles(): readonly RecentFileView[] {
    return this.service.recentFiles();
  }

  public saveDocument(): Promise<void> {
    return this.service.saveDocument();
  }

  public importDocx(): Promise<void> {
    return this.service.importDocx();
  }

  public exportDocx(): Promise<void> {
    return this.service.exportDocx();
  }

  public exportPdf(): Promise<void> {
    return this.service.exportPdf();
  }

  public saveDocumentAs(): Promise<void> {
    return this.service.saveDocumentAs();
  }

  public openPrintPreview(): Promise<void> {
    return this.service.openPrintPreview();
  }

  public closePrintPreview(): void {
    this.service.closePrintPreview();
  }

  public printDocument(): Promise<void> {
    return this.service.printDocument();
  }

  public pollExternalChange(): Promise<void> {
    return this.service.pollExternalChange();
  }

  public reloadFromDisk(): Promise<void> {
    return this.service.reloadFromDisk();
  }

  public keepLocalVersion(): Promise<void> {
    return this.service.keepLocalVersion();
  }

  public zoomIn(): void {
    this.service.zoomIn();
  }

  public zoomOut(): void {
    this.service.zoomOut();
  }

  public resetZoom(): void {
    this.service.resetZoom();
  }

  public captureSelection(paragraphId: string, editor: HTMLElement): WriterSelectionView | null {
    return this.service.captureSelection(paragraphId, editor);
  }

  public restoreSelection(paragraphId: string, editor: HTMLElement): boolean {
    return this.service.restoreSelection(paragraphId, editor);
  }

  public restoreSessionSelection(): boolean {
    return this.service.restoreSessionSelection();
  }

  public clearTypingStyle(): void {
    this.service.clearTypingStyle();
  }

  public flushFocusedParagraph(): Promise<void> {
    return this.service.flushFocusedParagraph();
  }

  public isCaretAtParagraphStart(paragraphId: string, editor: HTMLElement): boolean {
    return this.service.isCaretAtParagraphStart(paragraphId, editor);
  }

  public commitParagraphFromEditor(paragraphId: string, editor: HTMLElement): Promise<void> {
    return this.service.commitParagraphFromEditor(paragraphId, editor);
  }

  public loadImageAssetUrl(documentId: string, assetId: string): Promise<string> {
    return this.service.loadImageAssetUrl(documentId, assetId);
  }

  public releaseImageAssetUrl(url: string): void {
    this.service.releaseImageAssetUrl(url);
  }

  public insertImageData(
    afterParagraphId: string,
    mediaType: string,
    data: readonly number[],
    altText = "",
  ): Promise<void> {
    return this.service.insertImageData(afterParagraphId, mediaType, data, altText);
  }

  public splitParagraphFromEditor(paragraphId: string, editor: HTMLElement): Promise<void> {
    return this.service.splitParagraphFromEditor(paragraphId, editor);
  }

  public mergeWithPreviousFromEditor(paragraphId: string, editor: HTMLElement): Promise<void> {
    return this.service.mergeWithPreviousFromEditor(paragraphId, editor);
  }

  public replaceSelectionWithStyledRuns(
    paragraphId: string,
    startOffset: number,
    endOffset: number,
    runs: readonly WriterStyledRunInputView[],
  ): Promise<void> {
    return this.service.replaceSelectionWithStyledRuns(
      paragraphId,
      startOffset,
      endOffset,
      runs,
    );
  }

  public copySelection(
    paragraphId: string,
    editor: HTMLElement,
    event: ClipboardEvent,
  ): void {
    this.clipboardService.copySelection(paragraphId, editor, event);
  }

  public cutSelection(
    paragraphId: string,
    editor: HTMLElement,
    event: ClipboardEvent,
  ): Promise<void> {
    return this.clipboardService.cutSelection(paragraphId, editor, event);
  }

  public pasteSelection(
    paragraphId: string,
    editor: HTMLElement,
    event: ClipboardEvent,
  ): Promise<void> {
    return this.clipboardService.pasteSelection(paragraphId, editor, event);
  }

  public toggleBold(): Promise<void> {
    return this.service.toggleBold();
  }

  public toggleItalic(): Promise<void> {
    return this.service.toggleItalic();
  }

  public toggleUnderline(): Promise<void> {
    return this.service.toggleUnderline();
  }

  public setFontFamily(fontFamily: string): Promise<void> {
    return this.service.setFontFamily(fontFamily);
  }

  public setFontSizeHalfPoints(fontSizeHalfPoints: number): Promise<void> {
    return this.service.setFontSizeHalfPoints(fontSizeHalfPoints);
  }

  public setParagraphAlignment(alignment: WriterTextAlignmentView): Promise<void> {
    return this.service.setParagraphAlignment(alignment);
  }

  public formatState(): WriterFormatStateView {
    return this.service.formatState();
  }

  public undo(): Promise<void> {
    return this.service.undo();
  }

  public redo(): Promise<void> {
    return this.service.redo();
  }
}
