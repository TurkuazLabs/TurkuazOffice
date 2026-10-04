// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-shell.tsx
// # 📌 Amac: Turkuaz Office Desktop Writer ana pencere kompozisyonunu render eder
// # 📌 Modul - FileType: View - TSX
// # Version: 0.5.0
// # Aciklama: Modul secici, ribbon, rich-text page, loading/error ve statusbar View'larini birlestirir
// Bagimli Oldugu Katman: View -> Controller -> Repo -> Language

import { Match, onCleanup, onMount, Show, Switch } from "solid-js";

import { WRITER_PARAGRAPH_MARKER_VALUE } from "../config/dom-contract";
import { WRITER_SHORTCUT_ACTIONS } from "../config/keyboard";
import { WRITER_AUTOSAVE_INTERVAL_MS, WRITER_EXTERNAL_CHANGE_POLL_MS } from "../config/runtime-config";
import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";
import type { WriterSessionRepository } from "../repositories/writer-session.repository";
import { WriterPage } from "./writer-page";
import { WriterPrintPreview } from "./writer-print-preview";
import { WriterDocxCompatibilityBanner } from "./writer-docx-compatibility-banner";
import { WriterFileProtectionBanner } from "./writer-file-protection-banner";
import { WriterStatusbar } from "./writer-statusbar";
import { WriterRecoveryPanel } from "./writer-recovery-panel";
import { WriterRibbon } from "./writer-ribbon";

interface WriterShellProps {
  readonly controller: WriterController;
  readonly repository: WriterSessionRepository;
  readonly language: LanguageService;
}

export function WriterShell(props: WriterShellProps) {
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
      case WRITER_SHORTCUT_ACTIONS.bold:
        void props.controller.toggleBold();
        return;
      case WRITER_SHORTCUT_ACTIONS.italic:
        void props.controller.toggleItalic();
        return;
      case WRITER_SHORTCUT_ACTIONS.underline:
        void props.controller.toggleUnderline();
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
    <div class="office-shell">
      <header class="office-titlebar">
        <strong>{props.language.text("appName")} {props.language.text("writerModule")}</strong>
        <span class="office-titlebar__document">
          {props.repository.filePath() ?? props.language.text("untitledDocument")}
        </span>
      </header>
      <div class="office-command-area">
        <Show when={props.repository.document() !== null || props.repository.recoveryCandidates().length === 0}>
          <WriterRibbon controller={props.controller} language={props.language} />
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
          <WriterPage
            document={props.repository.document()!}
            controller={props.controller}
            language={props.language}
            ariaLabel={props.language.text("documentAreaLabel")}
            readOnly={props.repository.fileSession()?.readOnly === true}
            layout={props.repository.pageLayout()!}
            fontResolutions={props.repository.fontResolutions()}
          />
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
