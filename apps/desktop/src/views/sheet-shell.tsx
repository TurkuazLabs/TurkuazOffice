// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-shell.tsx
// # 📌 Amac: Turkuaz Office Desktop Sheet grid, format, formula ve filter/sort yuzeyini render eder
// # 📌 Modul - FileType: View - TSX
// Version: 0.4.1
// Aciklama: Zero-based adaptasyon, aktif draft, dirty-safe create, query-control reset, format toolbar, row query, formula bari ve grid View'larini birlestirir
// Bagimli Oldugu Katman: View -> Controller -> Repo -> Tool -> Language

import { createSignal, For, Match, onMount, Switch } from "solid-js";

import { OFFICE_MODULES, type OfficeModule } from "../config/office-modules";
import {
  SHEET_DECIMAL_GENERAL_VALUE,
  SHEET_GRID_COLUMN_COUNT,
  SHEET_GRID_ROW_COUNT,
  SHEET_MAX_DECIMAL_PLACES,
} from "../config/sheet";
import type { SheetController } from "../controllers/sheet.controller";
import type { LanguageService } from "../language/language-service";
import type { SheetSessionRepository } from "../repositories/sheet-session.repository";
import { SheetReferenceTool } from "../tools/sheet-reference.tool";
import type {
  SheetCellValueView,
  SheetFilterModeView,
  SheetHorizontalAlignmentView,
  SheetSortDirectionView,
} from "./sheet-types";
import { OfficeModuleSwitcher } from "./office-module-switcher";

interface SheetShellProps {
  readonly controller: SheetController;
  readonly repository: SheetSessionRepository;
  readonly language: LanguageService;
  readonly onSelectModule: (module: OfficeModule) => void;
}

const ROWS = Array.from({ length: SHEET_GRID_ROW_COUNT }, (_, index) => index + 1);
const COLUMNS = Array.from({ length: SHEET_GRID_COLUMN_COUNT }, (_, index) => index + 1);
const REFERENCE_TOOL = new SheetReferenceTool();
const DECIMAL_PLACE_OPTIONS = Array.from(
  { length: SHEET_MAX_DECIMAL_PLACES + 1 },
  (_, index) => index,
);

export function SheetShell(props: SheetShellProps) {
  const [editingReference, setEditingReference] = createSignal<string | null>(null);
  const [editingDraft, setEditingDraft] = createSignal("");
  const [formulaEditing, setFormulaEditing] = createSignal(false);
  const [formulaDraft, setFormulaDraft] = createSignal("");
  const [filterMode, setFilterMode] = createSignal<SheetFilterModeView>("none");
  const [filterValue, setFilterValue] = createSignal("");
  const [sortDirection, setSortDirection] = createSignal<SheetSortDirectionView>("none");

  onMount(() => {
    void props.controller.initializeSession();
  });

  const activeWorksheet = () => props.repository.document()?.worksheets[0] ?? null;
  const selectedFormat = () => props.repository.selection()?.format ?? null;
  const visibleRows = () => props.repository.rowQuery()?.rows.map((row) => row + 1) ?? ROWS;
  const filterNeedsValue = () =>
    filterMode() === "textContains" ||
    filterMode() === "numberGreaterThan" ||
    filterMode() === "numberLessThan";

  const queryColumnLabel = (): string => {
    const selection = props.repository.selection();
    return selection === null ? "-" : REFERENCE_TOOL.columnLabel(selection.column + 1);
  };

  const valueText = (value: SheetCellValueView): string => {
    switch (value.kind) {
      case "text":
      case "formula":
        return value.value;
      case "number":
      case "boolean":
        return String(value.value);
    }
  };

  const cellText = (row: number, column: number): string => {
    const worksheet = activeWorksheet();
    const address = REFERENCE_TOOL.domainAddress(row, column);
    const cell = worksheet?.cells.find(
      (item) => item.row === address.row && item.column === address.column,
    );
    if (cell === undefined || worksheet === null) {
      return "";
    }

    const reference = REFERENCE_TOOL.reference(row, column);
    const format = props.repository.cellFormat(worksheet.id, reference);
    if (
      cell.value.kind === "number" &&
      format?.decimalPlaces !== null &&
      format?.decimalPlaces !== undefined
    ) {
      return cell.value.value.toFixed(format.decimalPlaces);
    }
    return valueText(cell.value);
  };

  const beginCellEdit = (reference: string, value: string): void => {
    setEditingReference(reference);
    setEditingDraft(value);
  };

  const editorValue = (reference: string, row: number, column: number): string =>
    editingReference() === reference ? editingDraft() : cellText(row, column);

  const commitCell = async (
    reference: string,
    row: number,
    column: number,
    value: string,
  ): Promise<void> => {
    if (cellText(row, column) !== value) {
      await props.controller.commitCell(reference, value);
    }
    if (editingReference() === reference) {
      setEditingReference(null);
      setEditingDraft("");
    }
  };

  const formulaEditorValue = (): string =>
    formulaEditing() ? formulaDraft() : (props.repository.selection()?.rawValue ?? "");

  const commitFormulaBar = async (value: string): Promise<void> => {
    const selection = props.repository.selection();
    if (selection !== null && selection.rawValue !== value) {
      await props.controller.commitCell(selection.reference, value);
    }
    setFormulaEditing(false);
    setFormulaDraft("");
  };

  const resetQueryControls = (): void => {
    setFilterMode("none");
    setFilterValue("");
    setSortDirection("none");
  };

  const createDocument = async (): Promise<void> => {
    if (await props.controller.createDocument()) {
      resetQueryControls();
    }
  };

  const clearQuery = (): void => {
    resetQueryControls();
    props.controller.clearRowQuery();
  };

  const cellStyle = (reference: string): Record<string, string> => {
    const worksheet = activeWorksheet();
    if (worksheet === null) {
      return {};
    }
    const format = props.repository.cellFormat(worksheet.id, reference);
    if (format === null) {
      return {};
    }

    const alignment: SheetHorizontalAlignmentView = format.horizontalAlignment;
    return {
      "font-weight": format.bold ? "700" : "400",
      "font-style": format.italic ? "italic" : "normal",
      "text-decoration": format.underline ? "underline" : "none",
      "text-align": alignment === "general" ? "start" : alignment,
    };
  };

  const evaluatedText = (): string => {
    const selection = props.repository.selection();
    if (selection === null) {
      return "-";
    }
    if (selection.evaluationErrorCode !== null) {
      return selection.evaluationErrorCode;
    }
    return selection.evaluatedValue === null ? "" : valueText(selection.evaluatedValue);
  };

  const statusText = () => {
    if (props.repository.status() === "loading") {
      return props.language.text("sheetLoading");
    }
    if (props.repository.status() === "error") {
      return `${props.language.text("sheetError")}: ${props.repository.errorCode() ?? "-"}`;
    }
    return props.language.text("sheetReady");
  };

  return (
    <div class="office-shell">
      <header class="office-titlebar">
        <strong>{props.language.text("appName")}</strong>
        <span class="office-titlebar__document">
          {props.repository.document()?.title ?? props.language.text("sheetModule")}
        </span>
        <OfficeModuleSwitcher
          activeModule={OFFICE_MODULES.sheet}
          language={props.language}
          onSelectModule={props.onSelectModule}
        />
      </header>

      <div class="sheet-command-area">
        <div class="sheet-toolbar" aria-label={props.language.text("sheetToolbarLabel")}>
          <button
            type="button"
            class="toolbar-button toolbar-button--primary"
            onClick={() => void createDocument()}
          >
            {props.language.text("sheetNewDocument")}
          </button>
          <span>{activeWorksheet()?.name ?? "-"}</span>
          <span class="sheet-toolbar__separator" />
          <button
            type="button"
            class="toolbar-button toolbar-button--format"
            aria-label={props.language.text("bold")}
            aria-pressed={selectedFormat()?.bold ?? false}
            disabled={selectedFormat() === null}
            onClick={() => void props.controller.toggleBold()}
          >
            {props.language.text("boldShort")}
          </button>
          <button
            type="button"
            class="toolbar-button toolbar-button--format toolbar-button--italic"
            aria-label={props.language.text("italic")}
            aria-pressed={selectedFormat()?.italic ?? false}
            disabled={selectedFormat() === null}
            onClick={() => void props.controller.toggleItalic()}
          >
            {props.language.text("italicShort")}
          </button>
          <button
            type="button"
            class="toolbar-button toolbar-button--format toolbar-button--underline"
            aria-label={props.language.text("underline")}
            aria-pressed={selectedFormat()?.underline ?? false}
            disabled={selectedFormat() === null}
            onClick={() => void props.controller.toggleUnderline()}
          >
            {props.language.text("underlineShort")}
          </button>
          <span class="sheet-toolbar__separator" />
          <button
            type="button"
            class="toolbar-button"
            aria-label={props.language.text("sheetAlignGeneral")}
            aria-pressed={selectedFormat()?.horizontalAlignment === "general"}
            disabled={selectedFormat() === null}
            onClick={() => void props.controller.setHorizontalAlignment("general")}
          >
            {props.language.text("sheetAlignGeneralShort")}
          </button>
          <button
            type="button"
            class="toolbar-button toolbar-button--format"
            aria-label={props.language.text("alignLeft")}
            aria-pressed={selectedFormat()?.horizontalAlignment === "left"}
            disabled={selectedFormat() === null}
            onClick={() => void props.controller.setHorizontalAlignment("left")}
          >
            {props.language.text("alignLeftShort")}
          </button>
          <button
            type="button"
            class="toolbar-button toolbar-button--format"
            aria-label={props.language.text("alignCenter")}
            aria-pressed={selectedFormat()?.horizontalAlignment === "center"}
            disabled={selectedFormat() === null}
            onClick={() => void props.controller.setHorizontalAlignment("center")}
          >
            {props.language.text("alignCenterShort")}
          </button>
          <button
            type="button"
            class="toolbar-button toolbar-button--format"
            aria-label={props.language.text("alignRight")}
            aria-pressed={selectedFormat()?.horizontalAlignment === "right"}
            disabled={selectedFormat() === null}
            onClick={() => void props.controller.setHorizontalAlignment("right")}
          >
            {props.language.text("alignRightShort")}
          </button>
          <span class="sheet-toolbar__separator" />
          <label class="sheet-toolbar__decimal-format">
            <span>{props.language.text("sheetDecimalPlaces")}</span>
            <select
              class="ribbon-select ribbon-select--size"
              disabled={selectedFormat() === null}
              value={
                selectedFormat()?.decimalPlaces === null ||
                selectedFormat()?.decimalPlaces === undefined
                  ? SHEET_DECIMAL_GENERAL_VALUE
                  : String(selectedFormat()?.decimalPlaces)
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
                {(decimalPlaces) => (
                  <option value={String(decimalPlaces)}>{decimalPlaces}</option>
                )}
              </For>
            </select>
          </label>
        </div>

        <div class="sheet-query-bar">
          <span class="sheet-query-bar__column">
            {props.language.text("sheetQueryColumn")}: {queryColumnLabel()}
          </span>
          <label>
            <span>{props.language.text("sheetFilter")}</span>
            <select
              class="ribbon-select"
              aria-label={props.language.text("sheetFilter")}
              value={filterMode()}
              onChange={(event) => setFilterMode(event.currentTarget.value as SheetFilterModeView)}
            >
              <option value="none">{props.language.text("sheetFilterNone")}</option>
              <option value="nonEmpty">{props.language.text("sheetFilterNonEmpty")}</option>
              <option value="textContains">{props.language.text("sheetFilterTextContains")}</option>
              <option value="numberGreaterThan">
                {props.language.text("sheetFilterNumberGreaterThan")}
              </option>
              <option value="numberLessThan">
                {props.language.text("sheetFilterNumberLessThan")}
              </option>
              <option value="booleanTrue">{props.language.text("sheetFilterBooleanTrue")}</option>
              <option value="booleanFalse">{props.language.text("sheetFilterBooleanFalse")}</option>
            </select>
          </label>
          <input
            class="sheet-query-bar__value"
            aria-label={props.language.text("sheetFilterValue")}
            placeholder={props.language.text("sheetFilterValue")}
            disabled={!filterNeedsValue()}
            value={filterValue()}
            onInput={(event) => setFilterValue(event.currentTarget.value)}
          />
          <label>
            <span>{props.language.text("sheetSort")}</span>
            <select
              class="ribbon-select"
              aria-label={props.language.text("sheetSort")}
              value={sortDirection()}
              onChange={(event) =>
                setSortDirection(event.currentTarget.value as SheetSortDirectionView)
              }
            >
              <option value="none">{props.language.text("sheetSortNone")}</option>
              <option value="ascending">{props.language.text("sheetSortAscending")}</option>
              <option value="descending">{props.language.text("sheetSortDescending")}</option>
            </select>
          </label>
          <button
            type="button"
            class="toolbar-button toolbar-button--primary"
            disabled={props.repository.selection() === null}
            onClick={() =>
              void props.controller.applyRowQuery(filterMode(), filterValue(), sortDirection())
            }
          >
            {props.language.text("sheetApplyQuery")}
          </button>
          <button
            type="button"
            class="toolbar-button"
            disabled={props.repository.rowQuery() === null}
            onClick={clearQuery}
          >
            {props.language.text("sheetClearQuery")}
          </button>
          <span class="sheet-query-bar__error" aria-live="polite">
            {props.repository.rowQueryErrorCode() ?? ""}
          </span>
        </div>

        <div class="sheet-formula-bar" aria-label={props.language.text("sheetFormulaBarLabel")}>
          <span class="sheet-formula-bar__reference" title={props.language.text("sheetSelectedCell")}>
            {props.repository.selection()?.reference ?? "-"}
          </span>
          <input
            class="sheet-formula-bar__input"
            aria-label={props.language.text("sheetFormulaBarLabel")}
            disabled={props.repository.selection() === null}
            value={formulaEditorValue()}
            onFocus={(event) => {
              setFormulaEditing(true);
              setFormulaDraft(event.currentTarget.value);
            }}
            onInput={(event) => {
              if (formulaEditing()) {
                setFormulaDraft(event.currentTarget.value);
              }
            }}
            onBlur={(event) => {
              void commitFormulaBar(event.currentTarget.value);
            }}
            onKeyDown={(event) => {
              if (event.key === "Enter") {
                event.currentTarget.blur();
              }
            }}
          />
          <span class="sheet-formula-bar__evaluated">
            {props.language.text("sheetEvaluatedValue")}: {evaluatedText()}
          </span>
        </div>
      </div>

      <Switch>
        <Match when={props.repository.document() !== null}>
          <main class="sheet-workspace">
            <table class="sheet-grid" aria-label={props.language.text("sheetGridLabel")}>
              <thead>
                <tr>
                  <th class="sheet-grid__corner" scope="col" />
                  <For each={COLUMNS}>
                    {(column) => (
                      <th class="sheet-grid__column-header" scope="col">
                        {REFERENCE_TOOL.columnLabel(column)}
                      </th>
                    )}
                  </For>
                </tr>
              </thead>
              <tbody>
                <For each={visibleRows()}>
                  {(row) => (
                    <tr>
                      <th class="sheet-grid__row-header" scope="row">
                        {row}
                      </th>
                      <For each={COLUMNS}>
                        {(column) => {
                          const reference = REFERENCE_TOOL.reference(row, column);
                          return (
                            <td
                              class="sheet-grid__cell"
                              classList={{
                                "sheet-grid__cell--selected":
                                  props.repository.selection()?.reference === reference,
                              }}
                            >
                              <input
                                class="sheet-grid__input"
                                aria-label={reference}
                                value={editorValue(reference, row, column)}
                                style={cellStyle(reference)}
                                onFocus={(event) => {
                                  beginCellEdit(reference, event.currentTarget.value);
                                  const address = REFERENCE_TOOL.domainAddress(row, column);
                                  void props.controller.selectCell(
                                    reference,
                                    address.row,
                                    address.column,
                                  );
                                }}
                                onInput={(event) => {
                                  if (editingReference() === reference) {
                                    setEditingDraft(event.currentTarget.value);
                                  }
                                }}
                                onBlur={(event) => {
                                  void commitCell(
                                    reference,
                                    row,
                                    column,
                                    event.currentTarget.value,
                                  );
                                }}
                                onKeyDown={(event) => {
                                  if (event.key === "Enter") {
                                    event.currentTarget.blur();
                                  }
                                }}
                              />
                            </td>
                          );
                        }}
                      </For>
                    </tr>
                  )}
                </For>
              </tbody>
            </table>
          </main>
        </Match>
        <Match when={true}>
          <main class="writer-workspace writer-workspace--message" aria-live="polite">
            {statusText()}
          </main>
        </Match>
      </Switch>

      <footer class="sheet-statusbar">
        <span>{statusText()}</span>
        <span>
          {props.language.text("sheetSelectedCell")}: {props.repository.selection()?.reference ?? "-"}
        </span>
        <span>
          {props.language.text("sheetVisibleRows")}: {visibleRows().length}
        </span>
        <span class="sheet-statusbar__spacer" />
        <span>
          {props.language.text("revision")}: {props.repository.document()?.revision ?? 0}
        </span>
        <span>
          {props.language.text("sheetCells")}: {activeWorksheet()?.cellCount ?? 0}
        </span>
      </footer>
    </div>
  );
}
