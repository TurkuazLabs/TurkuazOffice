// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-shell.tsx
// # 📌 Amac: Turkuaz Office Desktop Writer ana pencere kompozisyonunu render eder
// # 📌 Modul - FileType: View - TSX
// # Version: 0.2.0
// # Aciklama: Ribbon, local file shortcut, rich-text page, loading/error ve statusbar View'larini birlestirir
// Bagimli Oldugu Katman: View -> Controller -> Repo -> Language

import { Match, onCleanup, onMount, Show, Switch } from "solid-js";

import { WRITER_PARAGRAPH_MARKER_VALUE } from "../config/dom-contract";
import { KEYBOARD_KEYS } from "../config/keyboard";
import { WRITER_AUTOSAVE_INTERVAL_MS, WRITER_EXTERNAL_CHANGE_POLL_MS } from "../config/runtime-config";
import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";
import type { WriterSessionRepository } from "../repositories/writer-session.repository";
import { WriterPage } from "./writer-page";
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
    const modifier = event.ctrlKey || event.metaKey;
    if (!modifier || event.isComposing) {
      return;
    }

    if (props.repository.document() === null && props.repository.recoveryCandidates().length > 0) {
      return;
    }

    const key = event.key.toLowerCase();
    if (key === KEYBOARD_KEYS.plus || key === KEYBOARD_KEYS.equal) {
      event.preventDefault();
      props.controller.zoomIn();
      return;
    }
    if (key === KEYBOARD_KEYS.minus) {
      event.preventDefault();
      props.controller.zoomOut();
      return;
    }
    if (key === KEYBOARD_KEYS.zero) {
      event.preventDefault();
      props.controller.resetZoom();
      return;
    }
    if (key === KEYBOARD_KEYS.z && !event.shiftKey) {
      event.preventDefault();
      commitFocusedParagraph();
      void props.controller.undo();
      return;
    }
    if (key === KEYBOARD_KEYS.y || (key === KEYBOARD_KEYS.z && event.shiftKey)) {
      event.preventDefault();
      commitFocusedParagraph();
      void props.controller.redo();
      return;
    }
    if (key === KEYBOARD_KEYS.n) {
      event.preventDefault();
      void props.controller.createDocument();
      return;
    }
    if (key === KEYBOARD_KEYS.o) {
      event.preventDefault();
      void props.controller.openDocument();
      return;
    }
    if (key === KEYBOARD_KEYS.s && event.shiftKey) {
      event.preventDefault();
      void props.controller.saveDocumentAs();
      return;
    }
    if (key === KEYBOARD_KEYS.s) {
      event.preventDefault();
      void props.controller.saveDocument();
      return;
    }
    if (key === KEYBOARD_KEYS.b) {
      event.preventDefault();
      void props.controller.toggleBold();
      return;
    }
    if (key === KEYBOARD_KEYS.i) {
      event.preventDefault();
      void props.controller.toggleItalic();
      return;
    }
    if (key === KEYBOARD_KEYS.u) {
      event.preventDefault();
      void props.controller.toggleUnderline();
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
        <strong>{props.language.text("appName")}</strong>
        <span>{props.repository.filePath() ?? props.language.text("untitledDocument")}</span>
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
    </div>
  );
}
