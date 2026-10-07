// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-paragraph.tsx
// # 📌 Amac: Writer paragraph runlarini IME-aware contenteditable rich-text yuzeyinde render eder
// # 📌 Modul - FileType: View - TSX
// # Version: 0.2.2
// # Aciklama: Typography, paragraph alignment, clipboard, DOM input ve selection eventlerini Controller'a aktarir
// Bagimli Oldugu Katman: View -> Controller -> Language

import { createEffect, createSignal, Index, onCleanup, type Accessor } from "solid-js";

import { WRITER_PARAGRAPH_MARKER_VALUE } from "../config/dom-contract";
import { CARET_NAVIGATION_KEYS, KEYBOARD_KEYS } from "../config/keyboard";
import { WRITER_INPUT_DEBOUNCE_MS } from "../config/runtime-config";
import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";
import type { WriterParagraphView, WriterResolvedFontView, WriterRunView } from "./writer-types";

interface WriterParagraphProps {
  readonly paragraph: WriterParagraphView;
  readonly controller: WriterController;
  readonly language: LanguageService;
  readonly canMergeWithPrevious: boolean;
  readonly readOnly: boolean;
  readonly renderScale: number;
  readonly fontResolutions: readonly WriterResolvedFontView[];
}

export function WriterParagraphEditor(props: WriterParagraphProps) {
  const [isComposing, setIsComposing] = createSignal(false);
  let editor!: HTMLDivElement;
  let timer: ReturnType<typeof setTimeout> | undefined;
  let suppressNextBlurCommit = false;

  createEffect(() => {
    props.paragraph.runs.map((run) => `${run.id}:${run.text}`).join("|");
    props.paragraph.style.alignment;
    queueMicrotask(() => {
      if (document.activeElement === editor) {
        props.controller.restoreSelection(props.paragraph.id, editor);
      }
    });
  });

  onCleanup(() => {
    if (timer !== undefined) {
      clearTimeout(timer);
    }
  });

  const clearPendingCommit = () => {
    if (timer !== undefined) {
      clearTimeout(timer);
      timer = undefined;
    }
  };

  const captureSelection = () => {
    props.controller.captureSelection(props.paragraph.id, editor);
  };

  const scheduleCommit = () => {
    clearPendingCommit();
    timer = setTimeout(() => {
      timer = undefined;
      void props.controller.commitParagraphFromEditor(props.paragraph.id, editor);
    }, WRITER_INPUT_DEBOUNCE_MS);
  };

  const commitNow = () => {
    clearPendingCommit();
    void props.controller.commitParagraphFromEditor(props.paragraph.id, editor);
  };

  const onKeyDown = (event: KeyboardEvent) => {
    if (props.readOnly) {
      return;
    }
    if (isComposing() || event.isComposing) {
      return;
    }

    if (CARET_NAVIGATION_KEYS.includes(event.key)) {
      props.controller.clearTypingStyle();
    }

    if (event.key === KEYBOARD_KEYS.enter) {
      event.preventDefault();
      clearPendingCommit();
      suppressNextBlurCommit = true;
      void props.controller
        .splitParagraphFromEditor(props.paragraph.id, editor)
        .finally(() => {
          queueMicrotask(() => {
            suppressNextBlurCommit = false;
          });
        });
      return;
    }

    if (
      event.key === KEYBOARD_KEYS.backspace &&
      props.canMergeWithPrevious &&
      props.controller.isCaretAtParagraphStart(props.paragraph.id, editor)
    ) {
      event.preventDefault();
      clearPendingCommit();
      void props.controller.mergeWithPreviousFromEditor(props.paragraph.id, editor);
    }
  };

  const runStyle = (run: WriterRunView) => {
    const resolution = props.fontResolutions.find(
      (item) => item.requestedFamily === run.style.fontFamily,
    );
    return {
      "font-family": resolution?.cssStack ?? run.style.fontFamily,
      "font-size": `${(run.style.fontSizeHalfPoints / 2) * props.renderScale}pt`,
    };
  };

  return (
    <div
      ref={editor}
      class="writer-paragraph"
      contentEditable={!props.readOnly}
      aria-readonly={props.readOnly}
      spellcheck={true}
      role="textbox"
      aria-multiline="false"
      aria-label={props.language.text("paragraphLabel")}
      data-writer-paragraph={WRITER_PARAGRAPH_MARKER_VALUE}
      data-writer-paragraph-id={props.paragraph.id}
      style={{ "text-align": props.paragraph.style.alignment }}
      onFocus={captureSelection}
      onPointerDown={() => props.controller.clearTypingStyle()}
      onMouseUp={captureSelection}
      onKeyUp={captureSelection}
      onInput={() => {
        if (props.readOnly) {
          return;
        }
        captureSelection();
        if (!isComposing()) {
          scheduleCommit();
        }
      }}
      onCopy={(event) => {
        props.controller.copySelection(props.paragraph.id, editor, event);
      }}
      onCut={(event) => {
        if (props.readOnly) {
          return;
        }
        clearPendingCommit();
        void props.controller.cutSelection(props.paragraph.id, editor, event);
      }}
      onPaste={(event) => {
        if (props.readOnly) {
          return;
        }
        clearPendingCommit();
        void props.controller.pasteSelection(props.paragraph.id, editor, event);
      }}
      onBlur={() => {
        if (suppressNextBlurCommit) {
          suppressNextBlurCommit = false;
          return;
        }
        commitNow();
      }}
      onKeyDown={onKeyDown}
      onCompositionStart={() => {
        clearPendingCommit();
        setIsComposing(true);
      }}
      onCompositionEnd={() => {
        setIsComposing(false);
        captureSelection();
        queueMicrotask(commitNow);
      }}
    >
      <Index each={props.paragraph.runs}>
        {(run: Accessor<WriterRunView>) => (
          <span
            data-writer-run-id={run().id}
            classList={{
              "writer-run--bold": run().style.bold,
              "writer-run--italic": run().style.italic,
              "writer-run--underline": run().style.underline,
            }}
            style={runStyle(run())}
          >
            {run().text}
          </span>
        )}
      </Index>
    </div>
  );
}
