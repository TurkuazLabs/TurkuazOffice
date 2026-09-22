// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-ribbon.tsx
// # 📌 Amac: Writer menu sekmeleri ve Giris ribbon command gruplarini render eder
// # 📌 Modul - FileType: View - TSX
// Version: 0.2.0
// Aciklama: New/Open/Save, DOCX import/export, print, gecmis, font, inline style ve paragraph alignment kontrollerini erisilebilir sunar
// Bagimli Oldugu Katman: View -> Controller -> Language -> Config

import { For } from "solid-js";

import {
  WRITER_ALIGNMENT_COMMANDS,
  WRITER_RIBBON_TABS,
  type WriterAlignmentCommandConfig,
  type WriterRibbonTabConfig,
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

export function WriterRibbon(props: WriterRibbonProps) {
  const formatState = () => props.controller.formatState();
  const preserveEditorSelection = (event: MouseEvent) => event.preventDefault();
  const flushBeforeSelectFocus = () => void props.controller.flushFocusedParagraph();
  const restoreEditorSelection = () => queueMicrotask(() => props.controller.restoreSessionSelection());

  const setFontFamily = async (fontFamily: string) => {
    await props.controller.setFontFamily(fontFamily);
    restoreEditorSelection();
  };

  const setFontSize = async (fontSizeHalfPoints: number) => {
    await props.controller.setFontSizeHalfPoints(fontSizeHalfPoints);
    restoreEditorSelection();
  };

  return (
    <section class="writer-ribbon" aria-label={props.language.text("ribbonLabel")}>
      <nav class="writer-ribbon__tabs" role="tablist" aria-label={props.language.text("ribbonLabel")}>
        <For each={WRITER_RIBBON_TABS}>
          {(tab: WriterRibbonTabConfig) => (
            <button
              type="button"
              class={`writer-ribbon__tab${tab.active ? " writer-ribbon__tab--active" : ""}`}
              role="tab"
              aria-selected={tab.active}
              disabled={!tab.enabled}
              title={tab.enabled ? undefined : props.language.text("menuComingSoon")}
            >
              {props.language.text(tab.languageKey as DesktopLabelKey)}
            </button>
          )}
        </For>
      </nav>

      <div class="writer-ribbon__band" role="toolbar" aria-label={props.language.text("menuHome")}>
        <div class="ribbon-group">
          <div class="ribbon-group__commands">
            <button
              type="button"
              class="toolbar-button toolbar-button--primary"
              onClick={() => void props.controller.createDocument()}
            >
              {props.language.text("newDocument")}
            </button>
            <button
              type="button"
              class="toolbar-button"
              onClick={() => void props.controller.openDocument()}
            >
              {props.language.text("open")}
            </button>
            <button
              type="button"
              class="toolbar-button"
              onClick={() => void props.controller.saveDocument()}
            >
              {props.language.text("save")}
            </button>
            <button
              type="button"
              class="toolbar-button"
              onClick={() => void props.controller.saveDocumentAs()}
            >
              {props.language.text("saveAs")}
            </button>
            <button
              type="button"
              class="toolbar-button"
              onClick={() => void props.controller.importDocx()}
            >
              {props.language.text("importDocx")}
            </button>
            <button
              type="button"
              class="toolbar-button"
              onClick={() => void props.controller.exportDocx()}
            >
              {props.language.text("exportDocx")}
            </button>
            <button
              type="button"
              class="toolbar-button"
              onClick={() => void props.controller.openPrintPreview()}
            >
              {props.language.text("print")}
            </button>
          </div>
          <span class="ribbon-group__label">{props.language.text("documentGroup")}</span>
        </div>

        <div class="writer-ribbon__separator" aria-hidden="true" />

        <div class="ribbon-group">
          <div class="ribbon-group__commands">
            <button type="button" class="toolbar-button" onClick={() => void props.controller.undo()}>
              {props.language.text("undo")}
            </button>
            <button type="button" class="toolbar-button" onClick={() => void props.controller.redo()}>
              {props.language.text("redo")}
            </button>
          </div>
          <span class="ribbon-group__label">{props.language.text("historyGroup")}</span>
        </div>

        <div class="writer-ribbon__separator" aria-hidden="true" />

        <div class="ribbon-group ribbon-group--font">
          <div class="ribbon-group__commands ribbon-group__commands--stacked">
            <div class="ribbon-select-row">
              <label class="sr-only" for="writer-font-family">
                {props.language.text("fontFamily")}
              </label>
              <select
                id="writer-font-family"
                class="ribbon-select ribbon-select--font"
                disabled={!formatState().canFormat}
                value={formatState().fontFamily}
                onMouseDown={flushBeforeSelectFocus}
                onChange={(event: Event & { currentTarget: HTMLSelectElement }) =>
                  void setFontFamily(event.currentTarget.value)
                }
              >
                {formatState().fontFamily === "" && (
                  <option value="" disabled>{props.language.text("mixedValue")}</option>
                )}
                <For each={WRITER_FONT_FAMILIES}>
                  {(family: string) => <option value={family}>{family}</option>}
                </For>
              </select>

              <label class="sr-only" for="writer-font-size">
                {props.language.text("fontSize")}
              </label>
              <select
                id="writer-font-size"
                class="ribbon-select ribbon-select--size"
                disabled={!formatState().canFormat}
                value={formatState().fontSizeHalfPoints === 0 ? "" : String(formatState().fontSizeHalfPoints)}
                onMouseDown={flushBeforeSelectFocus}
                onChange={(event: Event & { currentTarget: HTMLSelectElement }) =>
                  void setFontSize(Number(event.currentTarget.value))
                }
              >
                {formatState().fontSizeHalfPoints === 0 && (
                  <option value="" disabled>{props.language.text("mixedValue")}</option>
                )}
                <For each={WRITER_FONT_SIZES_POINTS}>
                  {(size: number) => <option value={String(size * 2)}>{size}</option>}
                </For>
              </select>
            </div>

            <div class="ribbon-inline-format-row">
              <button
                type="button"
                class="toolbar-button toolbar-button--format"
                disabled={!formatState().canFormat}
                aria-label={props.language.text("bold")}
                aria-pressed={formatState().bold}
                onMouseDown={preserveEditorSelection}
                onClick={() => void props.controller.toggleBold()}
              >
                {props.language.text("boldShort")}
              </button>
              <button
                type="button"
                class="toolbar-button toolbar-button--format toolbar-button--italic"
                disabled={!formatState().canFormat}
                aria-label={props.language.text("italic")}
                aria-pressed={formatState().italic}
                onMouseDown={preserveEditorSelection}
                onClick={() => void props.controller.toggleItalic()}
              >
                {props.language.text("italicShort")}
              </button>
              <button
                type="button"
                class="toolbar-button toolbar-button--format toolbar-button--underline"
                disabled={!formatState().canFormat}
                aria-label={props.language.text("underline")}
                aria-pressed={formatState().underline}
                onMouseDown={preserveEditorSelection}
                onClick={() => void props.controller.toggleUnderline()}
              >
                {props.language.text("underlineShort")}
              </button>
            </div>
          </div>
          <span class="ribbon-group__label">{props.language.text("fontGroup")}</span>
        </div>

        <div class="writer-ribbon__separator" aria-hidden="true" />

        <div class="ribbon-group">
          <div class="ribbon-group__commands ribbon-group__commands--alignment">
            <For each={WRITER_ALIGNMENT_COMMANDS}>
              {(item: WriterAlignmentCommandConfig) => (
                <button
                  type="button"
                  class="toolbar-button toolbar-button--format"
                  disabled={!formatState().canFormat}
                  aria-label={props.language.text(item.languageKey as DesktopLabelKey)}
                  aria-pressed={formatState().alignment === item.alignment}
                  onMouseDown={preserveEditorSelection}
                  onClick={() =>
                    void props.controller.setParagraphAlignment(item.alignment as WriterTextAlignmentView)
                  }
                >
                  {props.language.text(item.shortLanguageKey as DesktopLabelKey)}
                </button>
              )}
            </For>
          </div>
          <span class="ribbon-group__label">{props.language.text("paragraphGroup")}</span>
        </div>
      </div>
    </section>
  );
}
