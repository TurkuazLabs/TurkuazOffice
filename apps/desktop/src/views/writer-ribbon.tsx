// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-ribbon.tsx
// # 📌 Amac: Writer klasik menu ve iki katmanli kelime islemci arac cubugunu render eder
// # 📌 Modul - FileType: View - TSX
// Version: 0.12.0
// Aciklama: LibreOffice benzeri menu satiri, standart belge komutlari, font ve paragraph kontrollerini Turkuaz Office kimligiyle sunar
// Bagimli Oldugu Katman: View -> Controller -> Language -> Config

import { For, Show } from "solid-js";

import { WRITER_ARIA_SHORTCUTS, WRITER_SHORTCUT_HINTS } from "../config/keyboard";
import { RECENT_FILES_RIBBON_LIMIT } from "../config/recent-files";
import {
  WRITER_ALIGNMENT_COMMANDS,
  type WriterAlignmentCommandConfig,
} from "../config/ribbon";
import {
  WRITER_FONT_FAMILIES,
  WRITER_FONT_SIZES_POINTS,
} from "../config/typography";
import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";
import type { DesktopLabelKey } from "../language/labels";
import type { WriterTextAlignmentView } from "./writer-types";

interface WriterRibbonProps {
  readonly controller: WriterController;
  readonly language: LanguageService;
}

type WriterToolbarIconName =
  | "new"
  | "open"
  | "save"
  | "pdf"
  | "print"
  | "undo"
  | "redo";

function WriterToolbarIcon(props: { readonly name: WriterToolbarIconName }) {
  switch (props.name) {
    case "new":
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M6 3h8l4 4v14H6z" />
          <path d="M14 3v5h5M12 11v6M9 14h6" />
        </svg>
      );
    case "open":
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M3 7h7l2 2h9l-2 10H4z" />
          <path d="M4 7V5h6l2 2" />
        </svg>
      );
    case "save":
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M5 3h12l2 2v16H5z" />
          <path d="M8 3v6h8V3M8 15h8v6H8z" />
        </svg>
      );
    case "pdf":
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M6 3h9l4 4v14H6z" />
          <path d="M15 3v5h5M9 13h6M9 16h6" />
        </svg>
      );
    case "print":
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M7 8V3h10v5M7 17H4v-7h16v7h-3" />
          <path d="M7 14h10v7H7z" />
        </svg>
      );
    case "undo":
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M9 7 4 12l5 5" />
          <path d="M5 12h8a6 6 0 0 1 6 6" />
        </svg>
      );
    case "redo":
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="m15 7 5 5-5 5" />
          <path d="M19 12h-8a6 6 0 0 0-6 6" />
        </svg>
      );
  }
}

function AlignmentIcon(props: { readonly alignment: WriterTextAlignmentView }) {
  const starts =
    props.alignment === "center"
      ? [5, 7, 4, 6]
      : props.alignment === "right"
        ? [7, 5, 8, 6]
        : [3, 3, 3, 3];
  const ends =
    props.alignment === "center"
      ? [19, 17, 20, 18]
      : props.alignment === "right"
        ? [21, 21, 21, 21]
        : props.alignment === "justify"
          ? [21, 21, 21, 21]
          : [19, 17, 20, 16];

  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <For each={[0, 1, 2, 3]}>
        {(index) => <path d={`M${starts[index]} ${6 + index * 4}H${ends[index]}`} />}
      </For>
    </svg>
  );
}

export function WriterRibbon(props: WriterRibbonProps) {
  const formatState = () => props.controller.formatState();
  const preserveEditorSelection = (event: MouseEvent) => event.preventDefault();
  const flushBeforeSelectFocus = () => void props.controller.flushFocusedParagraph();
  const restoreEditorSelection = () =>
    queueMicrotask(() => props.controller.restoreSessionSelection());

  const setFontFamily = async (fontFamily: string) => {
    await props.controller.setFontFamily(fontFamily);
    restoreEditorSelection();
  };

  const setFontSize = async (fontSizeHalfPoints: number) => {
    await props.controller.setFontSizeHalfPoints(fontSizeHalfPoints);
    restoreEditorSelection();
  };

  const applyMenuFormat = async (command: () => Promise<void>) => {
    await command();
    restoreEditorSelection();
  };

  return (
    <section class="writer-ribbon" aria-label={props.language.text("ribbonLabel")}>
      <nav class="writer-menubar" aria-label={props.language.text("ribbonLabel")}>
        <details class="writer-menu">
          <summary>{props.language.text("menuFile")}</summary>
          <div class="writer-menu__popup">
            <button type="button" onClick={() => void props.controller.createDocument()}>
              <span>{props.language.text("newDocument")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.newDocument}</kbd>
            </button>
            <button type="button" onClick={() => void props.controller.openDocument()}>
              <span>{props.language.text("open")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.open}</kbd>
            </button>
            <button type="button" onClick={() => void props.controller.saveDocument()}>
              <span>{props.language.text("save")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.save}</kbd>
            </button>
            <button type="button" onClick={() => void props.controller.saveDocumentAs()}>
              <span>{props.language.text("saveAs")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.saveAs}</kbd>
            </button>
            <div class="writer-menu__separator" />
            <button type="button" onClick={() => void props.controller.importDocx()}>
              {props.language.text("importDocx")}
            </button>
            <button type="button" onClick={() => void props.controller.exportDocx()}>
              {props.language.text("exportDocx")}
            </button>
            <button type="button" onClick={() => void props.controller.exportPdf()}>
              {props.language.text("exportPdf")}
            </button>
            <button type="button" onClick={() => void props.controller.printDocument()}>
              <span>{props.language.text("print")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.print}</kbd>
            </button>
            <button type="button" onClick={() => void props.controller.openPrintPreview()}>
              <span>{props.language.text("printPreviewTitle")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.printPreview}</kbd>
            </button>

            <Show when={props.controller.templates().some((item) => item.quickCreate)}>
              <div class="writer-menu__separator" />
              <For each={props.controller.templates().filter((item) => item.quickCreate)}>
                {(item) => (
                  <button
                    type="button"
                    title={props.language.text(item.descriptionKey as DesktopLabelKey)}
                    onClick={() => void props.controller.createDocumentFromTemplate(item.id)}
                  >
                    {props.language.text(item.nameKey as DesktopLabelKey)}
                  </button>
                )}
              </For>
            </Show>

            <Show when={props.controller.recentFiles().length > 0}>
              <div class="writer-menu__separator" />
              <For each={props.controller.recentFiles().slice(0, RECENT_FILES_RIBBON_LIMIT)}>
                {(item) => (
                  <button type="button" title={item.path} onClick={() => void props.controller.openRecentFile(item.path)}>
                    {item.title}
                  </button>
                )}
              </For>
            </Show>
          </div>
        </details>

        <details class="writer-menu">
          <summary>{props.language.text("menuEdit")}</summary>
          <div class="writer-menu__popup">
            <button type="button" onClick={() => void props.controller.undo()}>
              <span>{props.language.text("undo")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.undo}</kbd>
            </button>
            <button type="button" onClick={() => void props.controller.redo()}>
              <span>{props.language.text("redo")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.redo}</kbd>
            </button>
          </div>
        </details>

        <details class="writer-menu">
          <summary>{props.language.text("menuView")}</summary>
          <div class="writer-menu__popup">
            <button type="button" onClick={() => void props.controller.openPrintPreview()}>
              <span>{props.language.text("printPreviewTitle")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.printPreview}</kbd>
            </button>
          </div>
        </details>

        <span class="writer-menubar__disabled" aria-disabled="true">
          {props.language.text("menuInsert")}
        </span>

        <details class="writer-menu">
          <summary onMouseDown={flushBeforeSelectFocus}>
            {props.language.text("menuFormat")}
          </summary>
          <div class="writer-menu__popup">
            <button
              type="button"
              disabled={!formatState().canFormat}
              aria-pressed={formatState().bold}
              onMouseDown={preserveEditorSelection}
              onClick={() => void applyMenuFormat(() => props.controller.toggleBold())}
            >
              <span>{props.language.text("bold")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.bold}</kbd>
            </button>
            <button
              type="button"
              disabled={!formatState().canFormat}
              aria-pressed={formatState().italic}
              onMouseDown={preserveEditorSelection}
              onClick={() => void applyMenuFormat(() => props.controller.toggleItalic())}
            >
              <span>{props.language.text("italic")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.italic}</kbd>
            </button>
            <button
              type="button"
              disabled={!formatState().canFormat}
              aria-pressed={formatState().underline}
              onMouseDown={preserveEditorSelection}
              onClick={() => void applyMenuFormat(() => props.controller.toggleUnderline())}
            >
              <span>{props.language.text("underline")}</span>
              <kbd>{WRITER_SHORTCUT_HINTS.underline}</kbd>
            </button>
          </div>
        </details>

        <span class="writer-menubar__disabled" aria-disabled="true">
          {props.language.text("menuStyles")}
        </span>
        <span class="writer-menubar__disabled" aria-disabled="true">
          {props.language.text("menuTable")}
        </span>
        <span class="writer-menubar__disabled" aria-disabled="true">
          {props.language.text("menuForm")}
        </span>
        <span class="writer-menubar__disabled" aria-disabled="true">
          {props.language.text("menuTools")}
        </span>
        <span class="writer-menubar__disabled" aria-disabled="true">
          {props.language.text("menuWindow")}
        </span>
        <span class="writer-menubar__disabled" aria-disabled="true">
          {props.language.text("menuHelp")}
        </span>
      </nav>

      <div class="writer-toolbar writer-toolbar--standard" role="toolbar" aria-label={props.language.text("documentGroup")}>
        <button
          type="button"
          class="writer-tool-button"
          title={`${props.language.text("newDocument")} — ${WRITER_SHORTCUT_HINTS.newDocument}`}
          aria-label={props.language.text("newDocument")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.newDocument}
          onClick={() => void props.controller.createDocument()}
        >
          <WriterToolbarIcon name="new" />
        </button>
        <button
          type="button"
          class="writer-tool-button"
          title={`${props.language.text("open")} — ${WRITER_SHORTCUT_HINTS.open}`}
          aria-label={props.language.text("open")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.open}
          onClick={() => void props.controller.openDocument()}
        >
          <WriterToolbarIcon name="open" />
        </button>
        <button
          type="button"
          class="writer-tool-button"
          title={`${props.language.text("save")} — ${WRITER_SHORTCUT_HINTS.save}`}
          aria-label={props.language.text("save")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.save}
          onClick={() => void props.controller.saveDocument()}
        >
          <WriterToolbarIcon name="save" />
        </button>

        <span class="writer-toolbar__separator" />

        <button
          type="button"
          class="writer-tool-button"
          title={props.language.text("exportPdf")}
          aria-label={props.language.text("exportPdf")}
          onClick={() => void props.controller.exportPdf()}
        >
          <WriterToolbarIcon name="pdf" />
        </button>
        <button
          type="button"
          class="writer-tool-button"
          title={`${props.language.text("print")} — ${WRITER_SHORTCUT_HINTS.print}`}
          aria-label={props.language.text("print")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.print}
          onClick={() => void props.controller.printDocument()}
        >
          <WriterToolbarIcon name="print" />
        </button>

        <span class="writer-toolbar__separator" />

        <button
          type="button"
          class="writer-tool-button"
          title={props.language.text("undo")}
          aria-label={props.language.text("undo")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.undo}
          onClick={() => void props.controller.undo()}
        >
          <WriterToolbarIcon name="undo" />
        </button>
        <button
          type="button"
          class="writer-tool-button"
          title={props.language.text("redo")}
          aria-label={props.language.text("redo")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.redo}
          onClick={() => void props.controller.redo()}
        >
          <WriterToolbarIcon name="redo" />
        </button>
      </div>

      <div class="writer-toolbar writer-toolbar--format" role="toolbar" aria-label={props.language.text("fontGroup")}>
        <label class="sr-only" for="writer-font-family">
          {props.language.text("fontFamily")}
        </label>
        <select
          id="writer-font-family"
          class="ribbon-select writer-toolbar__font"
          disabled={!formatState().canFormat}
          value={formatState().fontFamily}
          onMouseDown={flushBeforeSelectFocus}
          onChange={(event) => void setFontFamily(event.currentTarget.value)}
        >
          {formatState().fontFamily === "" && (
            <option value="" disabled>{props.language.text("mixedValue")}</option>
          )}
          <For each={WRITER_FONT_FAMILIES}>
            {(family) => <option value={family}>{family}</option>}
          </For>
        </select>

        <label class="sr-only" for="writer-font-size">
          {props.language.text("fontSize")}
        </label>
        <select
          id="writer-font-size"
          class="ribbon-select writer-toolbar__size"
          disabled={!formatState().canFormat}
          value={formatState().fontSizeHalfPoints === 0 ? "" : String(formatState().fontSizeHalfPoints)}
          onMouseDown={flushBeforeSelectFocus}
          onChange={(event) => void setFontSize(Number(event.currentTarget.value))}
        >
          {formatState().fontSizeHalfPoints === 0 && (
            <option value="" disabled>{props.language.text("mixedValue")}</option>
          )}
          <For each={WRITER_FONT_SIZES_POINTS}>
            {(size) => <option value={String(size * 2)}>{size}</option>}
          </For>
        </select>

        <span class="writer-toolbar__separator" />

        <button
          type="button"
          class="writer-tool-button writer-tool-button--text"
          disabled={!formatState().canFormat}
          aria-label={props.language.text("bold")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.bold}
          aria-pressed={formatState().bold}
          onMouseDown={preserveEditorSelection}
          onClick={() => void props.controller.toggleBold()}
        >
          <strong>{props.language.text("boldShort")}</strong>
        </button>
        <button
          type="button"
          class="writer-tool-button writer-tool-button--text writer-tool-button--italic"
          disabled={!formatState().canFormat}
          aria-label={props.language.text("italic")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.italic}
          aria-pressed={formatState().italic}
          onMouseDown={preserveEditorSelection}
          onClick={() => void props.controller.toggleItalic()}
        >
          {props.language.text("italicShort")}
        </button>
        <button
          type="button"
          class="writer-tool-button writer-tool-button--text writer-tool-button--underline"
          disabled={!formatState().canFormat}
          aria-label={props.language.text("underline")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.underline}
          aria-pressed={formatState().underline}
          onMouseDown={preserveEditorSelection}
          onClick={() => void props.controller.toggleUnderline()}
        >
          {props.language.text("underlineShort")}
        </button>

        <span class="writer-toolbar__separator" />

        <For each={WRITER_ALIGNMENT_COMMANDS}>
          {(item: WriterAlignmentCommandConfig) => (
            <button
              type="button"
              class="writer-tool-button"
              disabled={!formatState().canFormat}
              aria-label={props.language.text(item.languageKey as DesktopLabelKey)}
              aria-pressed={formatState().alignment === item.alignment}
              aria-keyshortcuts={
                item.alignment === "left"
                  ? WRITER_ARIA_SHORTCUTS.alignLeft
                  : item.alignment === "center"
                    ? WRITER_ARIA_SHORTCUTS.alignCenter
                    : item.alignment === "right"
                      ? WRITER_ARIA_SHORTCUTS.alignRight
                      : WRITER_ARIA_SHORTCUTS.alignJustify
              }
              onMouseDown={preserveEditorSelection}
              onClick={() => void props.controller.setParagraphAlignment(item.alignment as WriterTextAlignmentView)}
            >
              <AlignmentIcon alignment={item.alignment as WriterTextAlignmentView} />
            </button>
          )}
        </For>
      </div>
    </section>
  );
}
