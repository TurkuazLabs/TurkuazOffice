// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-properties-sidebar.tsx
// # 📌 Amac: Calc benzeri sag panelde secili hucrenin gercek Sheet bicim ozelliklerini duzenler
// # 📌 Modul - FileType: View - TSX
// Version: 0.5.0
// Aciklama: Secim, deger, karakter bicimi, hizalama ve ondalik ayarlarini mevcut Controller komutlariyla baglar
// Bagimli Oldugu Katman: View -> Controller -> Repo -> Language

import { For, Show } from "solid-js";

import {
  SHEET_DECIMAL_GENERAL_VALUE,
  SHEET_MAX_DECIMAL_PLACES,
} from "../config/sheet";
import type { SheetController } from "../controllers/sheet.controller";
import type { LanguageService } from "../language/language-service";
import type { SheetSessionRepository } from "../repositories/sheet-session.repository";
import type { SheetCellValueView, SheetHorizontalAlignmentView } from "./sheet-types";

interface SheetPropertiesSidebarProps {
  readonly controller: SheetController;
  readonly repository: SheetSessionRepository;
  readonly language: LanguageService;
  readonly onClose: () => void;
}

const DECIMAL_PLACE_OPTIONS = Array.from(
  { length: SHEET_MAX_DECIMAL_PLACES + 1 },
  (_, index) => index,
);

function valueText(value: SheetCellValueView | null): string {
  if (value === null) {
    return "";
  }
  return String(value.value);
}

export function SheetPropertiesSidebar(props: SheetPropertiesSidebarProps) {
  const selection = () => props.repository.selection();
  const format = () => selection()?.format ?? null;

  return (
    <aside class="sheet-sidebar" aria-label={props.language.text("sheetSidebarLabel")}>
      <header class="sheet-sidebar__header">
        <strong>{props.language.text("sheetProperties")}</strong>
        <button
          type="button"
          class="sheet-sidebar__close"
          aria-label={props.language.text("sheetCloseSidebar")}
          onClick={props.onClose}
        >
          x
        </button>
      </header>

      <Show
        when={selection() !== null}
        fallback={<div class="sheet-sidebar__empty">{props.language.text("sheetSelectedCell")}: -</div>}
      >
        <section class="sheet-sidebar__section">
          <h3>{props.language.text("sheetSelectedCell")}</h3>
          <dl class="sheet-sidebar__facts">
            <div>
              <dt>{props.language.text("sheetSelectedCell")}</dt>
              <dd>{selection()?.reference}</dd>
            </div>
            <div>
              <dt>{props.language.text("sheetRawValue")}</dt>
              <dd>{selection()?.rawValue || "-"}</dd>
            </div>
            <div>
              <dt>{props.language.text("sheetEvaluatedValue")}</dt>
              <dd>{(selection()?.evaluationErrorCode ?? valueText(selection()?.evaluatedValue ?? null)) || "-"}</dd>
            </div>
          </dl>
        </section>

        <section class="sheet-sidebar__section">
          <h3>{props.language.text("sheetFormat")}</h3>
          <div class="sheet-sidebar__format-row">
            <button
              type="button"
              class="toolbar-button toolbar-button--format"
              aria-pressed={format()?.bold ?? false}
              onClick={() => void props.controller.toggleBold()}
            >
              {props.language.text("boldShort")}
            </button>
            <button
              type="button"
              class="toolbar-button toolbar-button--format toolbar-button--italic"
              aria-pressed={format()?.italic ?? false}
              onClick={() => void props.controller.toggleItalic()}
            >
              {props.language.text("italicShort")}
            </button>
            <button
              type="button"
              class="toolbar-button toolbar-button--format toolbar-button--underline"
              aria-pressed={format()?.underline ?? false}
              onClick={() => void props.controller.toggleUnderline()}
            >
              {props.language.text("underlineShort")}
            </button>
          </div>

          <label class="sheet-sidebar__field">
            <span>{props.language.text("paragraphLabel")}</span>
            <select
              class="ribbon-select"
              value={format()?.horizontalAlignment ?? "general"}
              onChange={(event) =>
                void props.controller.setHorizontalAlignment(
                  event.currentTarget.value as SheetHorizontalAlignmentView,
                )
              }
            >
              <option value="general">{props.language.text("sheetAlignGeneral")}</option>
              <option value="left">{props.language.text("alignLeft")}</option>
              <option value="center">{props.language.text("alignCenter")}</option>
              <option value="right">{props.language.text("alignRight")}</option>
            </select>
          </label>

          <label class="sheet-sidebar__field">
            <span>{props.language.text("sheetDecimalPlaces")}</span>
            <select
              class="ribbon-select"
              value={
                format()?.decimalPlaces === null || format()?.decimalPlaces === undefined
                  ? SHEET_DECIMAL_GENERAL_VALUE
                  : String(format()?.decimalPlaces)
              }
              onChange={(event) => {
                const value = event.currentTarget.value;
                void props.controller.setDecimalPlaces(
                  value === SHEET_DECIMAL_GENERAL_VALUE ? null : Number(value),
                );
              }}
            >
              <option value={SHEET_DECIMAL_GENERAL_VALUE}>
                {props.language.text("sheetDecimalGeneral")}
              </option>
              <For each={DECIMAL_PLACE_OPTIONS}>
                {(decimalPlaces) => <option value={String(decimalPlaces)}>{decimalPlaces}</option>}
              </For>
            </select>
          </label>
        </section>
      </Show>
    </aside>
  );
}
