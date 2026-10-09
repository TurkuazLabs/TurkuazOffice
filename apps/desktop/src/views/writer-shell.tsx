// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-shell.tsx
// # 📌 Amac: Turkuaz Office Desktop Writer ana pencere kompozisyonunu render eder
// # 📌 Modul - FileType: View - TSX
// # Version: 0.12.0
// # Aciklama: Modul secici, ribbon, rich-text page, loading/error ve statusbar View'larini birlestirir
// Bagimli Oldugu Katman: View -> Controller -> Repo -> Language

import { createSignal, Match, onCleanup, onMount, Show, Switch } from "solid-js";

import { WRITER_PARAGRAPH_MARKER_VALUE, WRITER_PARAGRAPH_SELECTOR } from "../config/dom-contract";
import { KEYBOARD_MODIFIER_STATES, WRITER_SHORTCUT_ACTIONS } from "../config/keyboard";
import { WRITER_AUTOSAVE_INTERVAL_MS, WRITER_EXTERNAL_CHANGE_POLL_MS } from "../config/runtime-config";
import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";
import type { WriterSessionRepository } from "../repositories/writer-session.repository";
import { shortcutBelongsToOtherTextControl } from "../tools/shortcut-focus.tool";
import { WriterPage } from "./writer-page";
import { WriterPrintPreview } from "./writer-print-preview";
import { WriterDocxCompatibilityBanner } from "./writer-docx-compatibility-banner";
import { WriterFileProtectionBanner } from "./writer-file-protection-banner";
import { WriterFindBar } from "./writer-find-bar";
import { SuiteTitlebar } from "./suite-titlebar";
import { WriterStatusbar } from "./writer-statusbar";
import { WriterRecoveryPanel } from "./writer-recovery-panel";
import { WriterRibbon } from "./writer-ribbon";

interface WriterShellProps {
  readonly controller: WriterController;
  readonly repository: WriterSessionRepository;
  readonly language: LanguageService;
  readonly onHome?: () => void;
}

export function WriterShell(props: WriterShellProps) {
  const [findOpen, setFindOpen] = createSignal(false);
  let findInput: HTMLInputElement | undefined;

  const openFind = async (): Promise<void> => {
    await props.controller.flushFocusedParagraph();
    setFindOpen(true);
    queueMicrotask(() => findInput?.focus());
  };

  const closeFind = (): void => {
    setFindOpen(false);
    queueMicrotask(() => props.controller.restoreSessionSelection());
  };

  const commitFocusedParagraph = () => {
    const activeElement = document.activeElement;
    if (
      activeElement instanceof HTMLElement &&
      activeElement.dataset.writerParagraph === WRITER_PARAGRAPH_MARKER_VALUE
    ) {
      activeElement.blur();
    }
  };

  const onShortcut = (event: KeyboardEvent) => {
    const action = props.controller.resolveKeyboardShortcut({
      key: event.key,
      ctrlKey: event.ctrlKey,
      metaKey: event.metaKey,
      altKey: event.altKey,
      altGraphKey: event.getModifierState(KEYBOARD_MODIFIER_STATES.altGraph),
      shiftKey: event.shiftKey,
      isComposing: event.isComposing,
      printPreviewOpen: props.repository.printPreviewLayout() !== null,
      recoveryBlocking:
        props.repository.document() === null &&
        props.repository.recoveryCandidates().length > 0,
    });
    if (action === null) {
      return;
    }

    // Document-wide file commands keep working even when a form control is focused.
    // Text-editing and formatting commands belong to the focused input instead.
    if (
      shortcutBelongsToOtherTextControl(event.target, WRITER_PARAGRAPH_SELECTOR) &&
      action !== WRITER_SHORTCUT_ACTIONS.closePrintPreview &&
      action !== WRITER_SHORTCUT_ACTIONS.print &&
      action !== WRITER_SHORTCUT_ACTIONS.printPreview &&
      action !== WRITER_SHORTCUT_ACTIONS.newDocument &&
      action !== WRITER_SHORTCUT_ACTIONS.open &&
      action !== WRITER_SHORTCUT_ACTIONS.save &&
      action !== WRITER_SHORTCUT_ACTIONS.saveAs &&
      action !== WRITER_SHORTCUT_ACTIONS.find
    ) {
      return;
    }

    event.preventDefault();
    switch (action) {
      case WRITER_SHORTCUT_ACTIONS.closePrintPreview:
        props.controller.closePrintPreview();
        return;
      case WRITER_SHORTCUT_ACTIONS.print:
        void props.controller.printDocument();
        return;
      case WRITER_SHORTCUT_ACTIONS.zoomIn:
        props.controller.zoomIn();
        return;
      case WRITER_SHORTCUT_ACTIONS.zoomOut:
        props.controller.zoomOut();
        return;
      case WRITER_SHORTCUT_ACTIONS.zoomReset:
        props.controller.resetZoom();
        return;
      case WRITER_SHORTCUT_ACTIONS.undo:
        commitFocusedParagraph();
        void props.controller.undo();
        return;
      case WRITER_SHORTCUT_ACTIONS.redo:
        commitFocusedParagraph();
        void props.controller.redo();
        return;
      case WRITER_SHORTCUT_ACTIONS.newDocument:
        void props.controller.createDocument();
        return;
      case WRITER_SHORTCUT_ACTIONS.open:
        void props.controller.openDocument();
        return;
      case WRITER_SHORTCUT_ACTIONS.printPreview:
        void props.controller.openPrintPreview();
        return;
      case WRITER_SHORTCUT_ACTIONS.saveAs:
        void props.controller.saveDocumentAs();
        return;
      case WRITER_SHORTCUT_ACTIONS.save:
        void props.controller.saveDocument();
        return;
      case WRITER_SHORTCUT_ACTIONS.find:
        void openFind();
        return;
      case WRITER_SHORTCUT_ACTIONS.bold:
        void props.controller.toggleBold();
        return;
      case WRITER_SHORTCUT_ACTIONS.italic:
        void props.controller.toggleItalic();
        return;
      case WRITER_SHORTCUT_ACTIONS.underline:
        void props.controller.toggleUnderline();
        return;
      case WRITER_SHORTCUT_ACTIONS.alignLeft:
        void props.controller.setParagraphAlignment("left");
        return;
      case WRITER_SHORTCUT_ACTIONS.alignCenter:
        void props.controller.setParagraphAlignment("center");
        return;
      case WRITER_SHORTCUT_ACTIONS.alignRight:
        void props.controller.setParagraphAlignment("right");
        return;
      case WRITER_SHORTCUT_ACTIONS.alignJustify:
        void props.controller.setParagraphAlignment("justify");
        return;
      case WRITER_SHORTCUT_ACTIONS.increaseFontSize:
        void props.controller.increaseFontSize();
        return;
      case WRITER_SHORTCUT_ACTIONS.decreaseFontSize:
        void props.controller.decreaseFontSize();
        return;
    }
  };

  let autosaveTimer: ReturnType<typeof setInterval> | undefined;
  let externalChangeTimer: ReturnType<typeof setInterval> | undefined;

  onMount(() => {
    window.addEventListener("keydown", onShortcut);
    void props.controller.initializeSession();
    autosaveTimer = setInterval(() => {
      void props.controller.autosaveRecovery();
    }, WRITER_AUTOSAVE_INTERVAL_MS);
    externalChangeTimer = setInterval(() => {
      void props.controller.pollExternalChange();
    }, WRITER_EXTERNAL_CHANGE_POLL_MS);
  });

  onCleanup(() => {
    window.removeEventListener("keydown", onShortcut);
    if (autosaveTimer !== undefined) {
      clearInterval(autosaveTimer);
    }
    if (externalChangeTimer !== undefined) {
      clearInterval(externalChangeTimer);
    }
  });

  const formatState = () => props.controller.formatState();

  const statusText = () => {
    const status = props.repository.status();
    if (status === "loading") {
      return props.language.text("writerLoading");
    }
    if (status === "error") {
      return `${props.language.text("writerError")}: ${props.repository.errorCode() ?? "-"}`;
    }
    return props.language.text("writerReady");
  };

  return (
    <div class="office-shell office-shell--writer">
      <SuiteTitlebar
        moduleIcon="writer"
        moduleName={props.language.text("writerModule")}
        documentName={props.repository.filePath() ?? props.language.text("untitledDocument")}
        language={props.language}
        onHome={props.onHome}
      />
      <div class="office-command-area">
        <Show when={props.repository.document() !== null || props.repository.recoveryCandidates().length === 0}>
          <WriterRibbon controller={props.controller} language={props.language} onFind={() => void openFind()} />
        </Show>
        <WriterFileProtectionBanner
          controller={props.controller}
          repository={props.repository}
          language={props.language}
        />
        <WriterDocxCompatibilityBanner
          features={props.repository.docxCompatibilityFeatures()}
          language={props.language}
        />
        <Show when={findOpen() && props.repository.document() !== null}>
          <WriterFindBar
            controller={props.controller}
            language={props.language}
            onClose={closeFind}
            inputRef={(element) => { findInput = element; }}
          />
        </Show>
      </div>
      <Switch>
        <Match when={props.repository.document() === null && props.repository.recoveryCandidates().length > 0}>
          <WriterRecoveryPanel
            controller={props.controller}
            repository={props.repository}
            language={props.language}
          />
        </Match>
        <Match when={props.repository.document() !== null && props.repository.pageLayout() !== null}>
          <div class="writer-editor-layout">
            <aside class="writer-pages-panel" aria-label={props.language.text("writerPages")}>
              <div class="writer-panel__header">
                <strong>{props.language.text("writerPages")}</strong>
                <span aria-hidden="true">«</span>
              </div>
              <div class="writer-page-thumbnail-wrap" aria-current="page">
                <div class="writer-page-thumbnail" aria-hidden="true">
                  <span />
                  <span />
                  <span />
                  <span class="writer-page-thumbnail__accent" />
                  <span />
                  <span />
                </div>
                <span>{props.language.text("writerPageOne")}</span>
              </div>
            </aside>

            <WriterPage
              document={props.repository.document()!}
              controller={props.controller}
              language={props.language}
              ariaLabel={props.language.text("documentAreaLabel")}
              readOnly={props.repository.fileSession()?.readOnly === true}
              layout={props.repository.pageLayout()!}
              fontResolutions={props.repository.fontResolutions()}
            />

            <aside class="writer-properties-panel" aria-label={props.language.text("writerProperties")}>
              <div class="writer-panel__header">
                <strong>{props.language.text("writerProperties")}</strong>
              </div>

              <section class="writer-properties-panel__section">
                <h3>{props.language.text("fontGroup")}</h3>
                <div class="writer-properties-panel__value">
                  <span>{props.language.text("fontFamily")}</span>
                  <strong>{formatState().fontFamily || "-"}</strong>
                </div>
                <div class="writer-properties-panel__value">
                  <span>{props.language.text("fontSize")}</span>
                  <strong>
                    {formatState().fontSizeHalfPoints > 0
                      ? formatState().fontSizeHalfPoints / 2
                      : "-"}
                  </strong>
                </div>
                <div class="writer-properties-panel__format-row">
                  <button
                    type="button"
                    class="toolbar-button toolbar-button--format"
                    aria-label={props.language.text("bold")}
                    aria-pressed={formatState().bold}
                    disabled={!formatState().canFormat}
                    onClick={() => void props.controller.toggleBold()}
                  >
                    {props.language.text("boldShort")}
                  </button>
                  <button
                    type="button"
                    class="toolbar-button toolbar-button--format toolbar-button--italic"
                    aria-label={props.language.text("italic")}
                    aria-pressed={formatState().italic}
                    disabled={!formatState().canFormat}
                    onClick={() => void props.controller.toggleItalic()}
                  >
                    {props.language.text("italicShort")}
                  </button>
                  <button
                    type="button"
                    class="toolbar-button toolbar-button--format toolbar-button--underline"
                    aria-label={props.language.text("underline")}
                    aria-pressed={formatState().underline}
                    disabled={!formatState().canFormat}
                    onClick={() => void props.controller.toggleUnderline()}
                  >
                    {props.language.text("underlineShort")}
                  </button>
                </div>
              </section>

              <section class="writer-properties-panel__section">
                <h3>{props.language.text("paragraphGroup")}</h3>
                <div class="writer-properties-panel__format-row">
                  <button
                    type="button"
                    class="toolbar-button toolbar-button--format"
                    aria-label={props.language.text("alignLeft")}
                    aria-pressed={formatState().alignment === "left"}
                    disabled={!formatState().canFormat}
                    onClick={() => void props.controller.setParagraphAlignment("left")}
                  >
                    {props.language.text("alignLeftShort")}
                  </button>
                  <button
                    type="button"
                    class="toolbar-button toolbar-button--format"
                    aria-label={props.language.text("alignCenter")}
                    aria-pressed={formatState().alignment === "center"}
                    disabled={!formatState().canFormat}
                    onClick={() => void props.controller.setParagraphAlignment("center")}
                  >
                    {props.language.text("alignCenterShort")}
                  </button>
                  <button
                    type="button"
                    class="toolbar-button toolbar-button--format"
                    aria-label={props.language.text("alignRight")}
                    aria-pressed={formatState().alignment === "right"}
                    disabled={!formatState().canFormat}
                    onClick={() => void props.controller.setParagraphAlignment("right")}
                  >
                    {props.language.text("alignRightShort")}
                  </button>
                  <button
                    type="button"
                    class="toolbar-button toolbar-button--format"
                    aria-label={props.language.text("alignJustify")}
                    aria-pressed={formatState().alignment === "justify"}
                    disabled={!formatState().canFormat}
                    onClick={() => void props.controller.setParagraphAlignment("justify")}
                  >
                    {props.language.text("alignJustifyShort")}
                  </button>
                </div>
              </section>

              <section class="writer-properties-panel__section">
                <h3>{props.language.text("writerDocumentInfo")}</h3>
                <dl class="writer-properties-panel__facts">
                  <div>
                    <dt>{props.language.text("paragraphs")}</dt>
                    <dd>{props.repository.document()!.paragraphs.length}</dd>
                  </div>
                  <div>
                    <dt>{props.language.text("revision")}</dt>
                    <dd>{props.repository.document()!.revision}</dd>
                  </div>
                </dl>
              </section>
            </aside>
          </div>
        </Match>
        <Match when={props.repository.status() === "error"}>
          <main class="writer-workspace writer-workspace--message" role="alert">
            {statusText()}
          </main>
        </Match>
        <Match when={true}>
          <main class="writer-workspace writer-workspace--message" aria-live="polite">
            {props.language.text("writerLoading")}
          </main>
        </Match>
      </Switch>
      <WriterStatusbar
        controller={props.controller}
        document={props.repository.document()}
        language={props.language}
        statusText={statusText()}
        filePath={props.repository.filePath()}
        dirty={props.repository.dirty()}
        recoveryRevision={props.repository.recoveryRevision()}
        recoveryErrorCode={props.repository.recoveryErrorCode()}
        fileSession={props.repository.fileSession()}
        zoomPercent={props.repository.zoomPercent()}
        fontResolutions={props.repository.fontResolutions()}
      />
      <Show when={props.repository.printPreviewLayout() !== null && props.repository.document() !== null}>
        <WriterPrintPreview
          document={props.repository.document()!}
          controller={props.controller}
          language={props.language}
          layout={props.repository.printPreviewLayout()!}
          fontResolutions={props.repository.fontResolutions()}
          errorCode={props.repository.printErrorCode()}
        />
      </Show>
    </div>
  );
}
