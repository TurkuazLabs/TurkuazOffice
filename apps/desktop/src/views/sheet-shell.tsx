// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-shell.tsx
// # 📌 Amac: Turkuaz Office Desktop Sheet grid, format, formula ve filter/sort yuzeyini render eder
// # 📌 Modul - FileType: View - TSX
// Version: 0.12.0
// Aciklama: Hybrid menu, table/filter, conditional formatting, Functions/Charts sidebar, formula bari, grid, properties dock ve status View'larini birlestirir
// Bagimli Oldugu Katman: View -> Controller -> Repo -> Tool -> Language

import { createSignal, For, Match, onCleanup, onMount, Show, Switch } from "solid-js";

import {
  SHEET_FUNCTION_BUTTON_TEXT,
  SHEET_FUNCTION_IDS,
  type SheetFunctionId,
} from "../config/sheet-functions";
import {
  SHEET_ARIA_SHORTCUTS,
  SHEET_SHORTCUT_ACTIONS,
  SHEET_SHORTCUT_HINTS,
} from "../config/keyboard";
import {
  SHEET_DECIMAL_GENERAL_VALUE,
  SHEET_GRID_COLUMN_COUNT,
  SHEET_GRID_COLUMN_HEADER_HEIGHT_PX,
  SHEET_GRID_COLUMN_WIDTH_PX,
  SHEET_GRID_ROW_COUNT,
  SHEET_GRID_ROW_HEADER_WIDTH_PX,
  SHEET_GRID_ROW_HEIGHT_PX,
  SHEET_MAX_DECIMAL_PLACES,
} from "../config/sheet";
import type { SheetController } from "../controllers/sheet.controller";
import type { LanguageService } from "../language/language-service";
import type { SheetSessionRepository } from "../repositories/sheet-session.repository";
import { SheetReferenceTool } from "../tools/sheet-reference.tool";
import { SheetChartsSidebar } from "./sheet-charts-sidebar";
import { SheetFunctionsSidebar } from "./sheet-functions-sidebar";
import { SheetMenubar } from "./sheet-menubar";
import { SheetPropertiesSidebar } from "./sheet-properties-sidebar";
import { SuiteTitlebar } from "./suite-titlebar";
import type {
  SheetCellValueView,
  SheetConditionalFormatModeView,
  SheetConditionalFormatRuleView,
  SheetConditionalFormatStyleView,
  SheetFilterModeView,
  SheetHorizontalAlignmentView,
  SheetSortDirectionView,
  SheetTableView,
} from "./sheet-types";

interface SheetShellProps {
  readonly controller: SheetController;
  readonly repository: SheetSessionRepository;
  readonly language: LanguageService;
  readonly onHome?: () => void;
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
  const [propertiesOpen, setPropertiesOpen] = createSignal(true);
  const [functionsOpen, setFunctionsOpen] = createSignal(false);
  const [chartsOpen, setChartsOpen] = createSignal(false);
  const [queryOpen, setQueryOpen] = createSignal(false);
  const [conditionalFormatOpen, setConditionalFormatOpen] = createSignal(false);
  const [conditionalMode, setConditionalMode] =
    createSignal<SheetConditionalFormatModeView>("numberGreaterThan");
  const [conditionalValue, setConditionalValue] = createSignal("");
  const [conditionalStyle, setConditionalStyle] =
    createSignal<SheetConditionalFormatStyleView>("warning");
  let suppressBlurCommitReference: string | null = null;

  onMount(() => {
    window.addEventListener("keydown", onShortcut);
    void props.controller.initializeSession();
  });

  onCleanup(() => {
    window.removeEventListener("keydown", onShortcut);
  });

  const activeWorksheet = () => props.repository.document()?.worksheets[0] ?? null;
  const selectedFormat = () => props.repository.selection()?.format ?? null;

  const tableAtAddress = (row: number, column: number): SheetTableView | null => {
    const worksheet = activeWorksheet();
    if (worksheet === null) {
      return null;
    }
    return (
      props.repository.document()?.tables.find(
        (table) =>
          table.worksheetId === worksheet.id &&
          row >= table.startRow &&
          row <= table.endRow &&
          column >= table.startColumn &&
          column <= table.endColumn,
      ) ?? null
    );
  };

  const tableAtHeaderCell = (row: number, column: number): SheetTableView | null => {
    const address = REFERENCE_TOOL.domainAddress(row, column);
    const table = tableAtAddress(address.row, address.column);
    return table?.startRow === address.row ? table : null;
  };

  const selectedTable = (): SheetTableView | null => {
    const selection = props.repository.selection();
    return selection === null ? null : tableAtAddress(selection.row, selection.column);
  };

  const selectedConditionalFormatRule = (): SheetConditionalFormatRuleView | null => {
    const selection = props.repository.selection();
    const worksheet = activeWorksheet();
    if (selection === null || worksheet === null) {
      return null;
    }
    return (
      props.repository
        .document()
        ?.conditionalFormats.filter(
          (rule) =>
            rule.worksheetId === worksheet.id &&
            selection.row >= rule.startRow &&
            selection.row <= rule.endRow &&
            selection.column >= rule.startColumn &&
            selection.column <= rule.endColumn,
        )
        .sort((left, right) => left.priority - right.priority)[0] ?? null
    );
  };

  const conditionalFormatStyleAt = (
    row: number,
    column: number,
  ): SheetConditionalFormatStyleView | null => {
    const address = REFERENCE_TOOL.domainAddress(row, column);
    return props.repository.conditionalFormatStyle(address.row, address.column);
  };

  const rangesOverlap = (
    left: { startRow: number; endRow: number; startColumn: number; endColumn: number },
    right: SheetTableView,
  ): boolean =>
    left.startRow <= right.endRow &&
    right.startRow <= left.endRow &&
    left.startColumn <= right.endColumn &&
    right.startColumn <= left.endColumn;

  const canCreateTable = (): boolean => {
    const range = props.repository.selectionRange();
    const worksheet = activeWorksheet();
    if (range === null || worksheet === null || range.startRow >= range.endRow) {
      return false;
    }
    return !(
      props.repository.document()?.tables.some(
        (table) => table.worksheetId === worksheet.id && rangesOverlap(range, table),
      ) ?? false
    );
  };

  const visibleRows = () => {
    const query = props.repository.rowQuery();
    if (query === null) {
      return ROWS;
    }

    const before = ROWS.filter((row) => row - 1 < query.range.startRow);
    const queried = query.rows.map((row) => row + 1);
    const after = ROWS.filter((row) => row - 1 > query.range.endRow);
    return [...before, ...queried, ...after];
  };
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

  const focusCellInput = (reference: string): void => {
    queueMicrotask(() => {
      const input = Array.from(
        document.querySelectorAll<HTMLInputElement>(".sheet-grid__input"),
      ).find((candidate) => candidate.getAttribute("aria-label") === reference);
      if (input === undefined) {
        return;
      }
      input.focus();
      input.setSelectionRange(input.value.length, input.value.length);
    });
  };

  const moveAfterCellCommit = async (
    reference: string,
    row: number,
    column: number,
    value: string,
    rowDelta: number,
  ): Promise<void> => {
    suppressBlurCommitReference = reference;
    await commitCell(reference, row, column, value);
    const targetRow = Math.min(
      SHEET_GRID_ROW_COUNT,
      Math.max(1, row + rowDelta),
    );
    const targetReference = REFERENCE_TOOL.reference(targetRow, column);
    const address = REFERENCE_TOOL.domainAddress(targetRow, column);
    await props.controller.selectCell(targetReference, address.row, address.column);
    focusCellInput(targetReference);
    queueMicrotask(() => {
      if (suppressBlurCommitReference === reference) {
        suppressBlurCommitReference = null;
      }
    });
  };

  const cancelActiveEdit = (): void => {
    if (formulaEditing()) {
      setFormulaEditing(false);
      setFormulaDraft("");
    }
    if (editingReference() !== null) {
      setEditingReference(null);
      setEditingDraft("");
    }
  };

  const focusSelectedCell = (): void => {
    const selection = props.repository.selection();
    if (selection !== null) {
      focusCellInput(selection.reference);
    }
  };

  const focusFirstCell = async (): Promise<void> => {
    const reference = REFERENCE_TOOL.reference(1, 1);
    const address = REFERENCE_TOOL.domainAddress(1, 1);
    await props.controller.selectCell(reference, address.row, address.column);
    focusCellInput(reference);
  };

  const onShortcut = (event: KeyboardEvent): void => {
    const action = props.controller.resolveKeyboardShortcut({
      key: event.key,
      ctrlKey: event.ctrlKey,
      metaKey: event.metaKey,
      shiftKey: event.shiftKey,
      altKey: event.altKey,
      isComposing: event.isComposing,
      editingActive: editingReference() !== null || formulaEditing(),
    });
    if (action === null) {
      return;
    }

    event.preventDefault();
    switch (action) {
      case SHEET_SHORTCUT_ACTIONS.newDocument:
        void createDocument();
        return;
      case SHEET_SHORTCUT_ACTIONS.bold:
        void props.controller.toggleBold();
        return;
      case SHEET_SHORTCUT_ACTIONS.italic:
        void props.controller.toggleItalic();
        return;
      case SHEET_SHORTCUT_ACTIONS.underline:
        void props.controller.toggleUnderline();
        return;
      case SHEET_SHORTCUT_ACTIONS.editCell:
        focusSelectedCell();
        return;
      case SHEET_SHORTCUT_ACTIONS.cancelEdit:
        cancelActiveEdit();
        return;
      case SHEET_SHORTCUT_ACTIONS.firstCell:
        void focusFirstCell();
        return;
      case SHEET_SHORTCUT_ACTIONS.toggleProperties:
        togglePropertiesSidebar();
        return;
      case SHEET_SHORTCUT_ACTIONS.toggleQuery:
        setQueryOpen((value) => !value);
        return;
      case SHEET_SHORTCUT_ACTIONS.insertSum:
        insertFunctionDraft(SHEET_FUNCTION_IDS.sum);
        return;
    }
  };

  const formulaEditorValue = (): string =>
    formulaEditing() ? formulaDraft() : (props.repository.selection()?.rawValue ?? "");

  let formulaEditGeneration = 0;

  const beginFormulaEditGeneration = (): number => {
    formulaEditGeneration += 1;
    return formulaEditGeneration;
  };

  const commitFormulaBar = async (value: string): Promise<void> => {
    const commitGeneration = formulaEditGeneration;
    const selection = props.repository.selection();
    if (selection !== null && selection.rawValue !== value) {
      await props.controller.commitCell(selection.reference, value);
    }
    if (commitGeneration !== formulaEditGeneration) {
      return;
    }
    setFormulaEditing(false);
    setFormulaDraft("");
  };

  let formulaInput: HTMLInputElement | undefined;

  const togglePropertiesSidebar = (): void => {
    setFunctionsOpen(false);
    setChartsOpen(false);
    setPropertiesOpen((value) => !value);
  };

  const toggleFunctionsSidebar = (): void => {
    setPropertiesOpen(false);
    setChartsOpen(false);
    setFunctionsOpen((value) => !value);
  };

  const toggleChartsSidebar = (): void => {
    setPropertiesOpen(false);
    setFunctionsOpen(false);
    setChartsOpen((value) => !value);
  };

  const insertFunctionDraft = (functionId: SheetFunctionId): void => {
    const draft = props.controller.functionFormulaDraft(functionId);
    if (draft === null) {
      return;
    }
    beginFormulaEditGeneration();
    setFormulaEditing(true);
    setFormulaDraft(draft);
    queueMicrotask(() => {
      formulaInput?.focus();
      formulaInput?.setSelectionRange(draft.length, draft.length);
    });
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

  const sameQueryRange = (
    left: { startRow: number; endRow: number; startColumn: number; endColumn: number },
    right: { startRow: number; endRow: number; startColumn: number; endColumn: number },
  ): boolean =>
    left.startRow === right.startRow &&
    left.endRow === right.endRow &&
    left.startColumn === right.startColumn &&
    left.endColumn === right.endColumn;

  const openTableFilter = async (
    reference: string,
    row: number,
    column: number,
  ): Promise<void> => {
    const table = tableAtHeaderCell(row, column);
    if (table === null) {
      return;
    }
    const address = REFERENCE_TOOL.domainAddress(row, column);
    const tableRange = {
      startRow: table.startRow + 1,
      endRow: table.endRow,
      startColumn: table.startColumn,
      endColumn: table.endColumn,
    };
    const query = props.repository.rowQuery();
    if (
      query !== null &&
      query.column === address.column &&
      sameQueryRange(query.range, tableRange)
    ) {
      setFilterMode(query.filterMode);
      setFilterValue(query.filterValue);
      setSortDirection(query.sortDirection);
    } else {
      props.controller.clearRowQuery();
      resetQueryControls();
    }

    await props.controller.selectCell(reference, address.row, address.column);
    setQueryOpen(true);
  };

  const frozenCellStyle = (
    visibleRowIndex: number,
    columnIndex: number,
  ): Record<string, string> => {
    const freeze = props.repository.freezeState();
    const frozenRow = visibleRowIndex < freeze.rows;
    const frozenColumn = columnIndex < freeze.columns;
    if (!frozenRow && !frozenColumn) {
      return {};
    }

    return {
      position: "sticky",
      ...(frozenRow
        ? {
            top: `${SHEET_GRID_COLUMN_HEADER_HEIGHT_PX + visibleRowIndex * SHEET_GRID_ROW_HEIGHT_PX}px`,
          }
        : {}),
      ...(frozenColumn
        ? {
            left: `${SHEET_GRID_ROW_HEADER_WIDTH_PX + columnIndex * SHEET_GRID_COLUMN_WIDTH_PX}px`,
          }
        : {}),
      "z-index": frozenRow && frozenColumn ? "6" : "5",
    };
  };

  const cellStyle = (reference: string): Record<string, string> => {
    const worksheet = activeWorksheet();
    const format = worksheet === null ? null : props.repository.cellFormat(worksheet.id, reference);
    const alignment: SheetHorizontalAlignmentView = format?.horizontalAlignment ?? "general";
    return {
      ...(format === null
        ? {}
        : {
            "font-weight": format.bold ? "700" : "400",
            "font-style": format.italic ? "italic" : "normal",
            "text-decoration": format.underline ? "underline" : "none",
            "text-align": alignment === "general" ? "start" : alignment,
          }),
    };
  };

  const frozenColumnHeaderStyle = (columnIndex: number): Record<string, string> =>
    columnIndex < props.repository.freezeState().columns
      ? {
          left: `${SHEET_GRID_ROW_HEADER_WIDTH_PX + columnIndex * SHEET_GRID_COLUMN_WIDTH_PX}px`,
          "z-index": "5",
        }
      : {};

  const frozenRowHeaderStyle = (visibleRowIndex: number): Record<string, string> =>
    visibleRowIndex < props.repository.freezeState().rows
      ? {
          top: `${SHEET_GRID_COLUMN_HEADER_HEIGHT_PX + visibleRowIndex * SHEET_GRID_ROW_HEIGHT_PX}px`,
          "z-index": "5",
        }
      : {};

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

  const isCellInSelectionRange = (row: number, column: number): boolean => {
    const range = props.repository.selectionRange();
    if (range === null) {
      return false;
    }
    const address = REFERENCE_TOOL.domainAddress(row, column);
    return (
      address.row >= range.startRow &&
      address.row <= range.endRow &&
      address.column >= range.startColumn &&
      address.column <= range.endColumn
    );
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
    <div class="office-shell office-shell--sheet">
      <SuiteTitlebar
        moduleIcon="sheet"
        moduleName={props.language.text("sheetModule")}
        documentName={props.repository.document()?.title ?? props.language.text("sheetModule")}
        language={props.language}
        onHome={props.onHome}
      />

      <div class="sheet-command-area">
        <SheetMenubar
          language={props.language}
          propertiesOpen={propertiesOpen()}
          queryOpen={queryOpen()}
          conditionalFormatOpen={conditionalFormatOpen()}
          functionsOpen={functionsOpen()}
          chartsOpen={chartsOpen()}
          freezeActive={
            props.repository.freezeState().rows > 0 ||
            props.repository.freezeState().columns > 0
          }
          canFreezeAtSelection={props.repository.selection() !== null}
          canCreateTable={canCreateTable()}
          canRemoveTable={selectedTable() !== null}
          onNewDocument={() => void createDocument()}
          onToggleProperties={togglePropertiesSidebar}
          onToggleQuery={() => setQueryOpen((value) => !value)}
          onToggleConditionalFormat={() => setConditionalFormatOpen((value) => !value)}
          onToggleFunctions={toggleFunctionsSidebar}
          onInsertSum={() => insertFunctionDraft(SHEET_FUNCTION_IDS.sum)}
          onToggleCharts={toggleChartsSidebar}
          onFreezeAtSelection={() => props.controller.freezeAtSelection()}
          onFreezeTopRow={() => props.controller.freezeTopRow()}
          onFreezeFirstColumn={() => props.controller.freezeFirstColumn()}
          onUnfreezePanes={() => props.controller.unfreezePanes()}
          onCreateTable={() => void props.controller.createTableFromSelection()}
          onRemoveTable={() => void props.controller.removeTableAtSelection()}
        />
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
            aria-keyshortcuts={SHEET_ARIA_SHORTCUTS.bold}
            title={`${props.language.text("bold")} — ${SHEET_SHORTCUT_HINTS.bold}`}
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
            aria-keyshortcuts={SHEET_ARIA_SHORTCUTS.italic}
            title={`${props.language.text("italic")} — ${SHEET_SHORTCUT_HINTS.italic}`}
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
            aria-keyshortcuts={SHEET_ARIA_SHORTCUTS.underline}
            title={`${props.language.text("underline")} — ${SHEET_SHORTCUT_HINTS.underline}`}
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

        <Show when={queryOpen()}>
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
        </Show>

        <Show when={conditionalFormatOpen()}>
          <div class="sheet-conditional-format-bar">
            <strong>{props.language.text("sheetConditionalFormatting")}</strong>
            <label>
              <span>{props.language.text("sheetConditionalCondition")}</span>
              <select
                class="ribbon-select"
                aria-label={props.language.text("sheetConditionalCondition")}
                value={conditionalMode()}
                onChange={(event) =>
                  setConditionalMode(
                    event.currentTarget.value as SheetConditionalFormatModeView,
                  )
                }
              >
                <option value="numberGreaterThan">
                  {props.language.text("sheetConditionalNumberGreaterThan")}
                </option>
                <option value="numberLessThan">
                  {props.language.text("sheetConditionalNumberLessThan")}
                </option>
                <option value="numberEquals">
                  {props.language.text("sheetConditionalNumberEquals")}
                </option>
                <option value="textContains">
                  {props.language.text("sheetConditionalTextContains")}
                </option>
              </select>
            </label>
            <input
              class="sheet-conditional-format-bar__value"
              aria-label={props.language.text("sheetConditionalValue")}
              placeholder={props.language.text("sheetConditionalValue")}
              value={conditionalValue()}
              onInput={(event) => setConditionalValue(event.currentTarget.value)}
            />
            <label>
              <span>{props.language.text("sheetConditionalStyle")}</span>
              <select
                class="ribbon-select"
                aria-label={props.language.text("sheetConditionalStyle")}
                value={conditionalStyle()}
                onChange={(event) =>
                  setConditionalStyle(
                    event.currentTarget.value as SheetConditionalFormatStyleView,
                  )
                }
              >
                <option value="warning">
                  {props.language.text("sheetConditionalStyleWarning")}
                </option>
                <option value="success">
                  {props.language.text("sheetConditionalStyleSuccess")}
                </option>
                <option value="accent">
                  {props.language.text("sheetConditionalStyleAccent")}
                </option>
              </select>
            </label>
            <button
              type="button"
              class="toolbar-button toolbar-button--primary"
              disabled={props.repository.selection() === null}
              onClick={() =>
                void props.controller.applyConditionalFormat(
                  conditionalMode(),
                  conditionalValue(),
                  conditionalStyle(),
                )
              }
            >
              {props.language.text("sheetConditionalApply")}
            </button>
            <button
              type="button"
              class="toolbar-button"
              disabled={selectedConditionalFormatRule() === null}
              onClick={() => void props.controller.removeConditionalFormatAtSelection()}
            >
              {props.language.text("sheetConditionalRemove")}
            </button>
            <span class="sheet-query-bar__error" aria-live="polite">
              {props.repository.conditionalFormatErrorCode() ?? ""}
            </span>
          </div>
        </Show>

        <div class="sheet-formula-bar" aria-label={props.language.text("sheetFormulaBarLabel")}>
          <span class="sheet-formula-bar__reference" title={props.language.text("sheetSelectedCell")}>
            {props.repository.selection()?.reference ?? "-"}
          </span>
          <button
            type="button"
            class="sheet-formula-bar__functions-button"
            aria-label={props.language.text("sheetFunctionsToggle")}
            aria-pressed={functionsOpen()}
            disabled={props.repository.selection() === null}
            onClick={toggleFunctionsSidebar}
          >
            {SHEET_FUNCTION_BUTTON_TEXT}
          </button>
          <input
            ref={(element) => {
              formulaInput = element;
            }}
            class="sheet-formula-bar__input"
            aria-label={props.language.text("sheetFormulaBarLabel")}
            disabled={props.repository.selection() === null}
            value={formulaEditorValue()}
            onFocus={(event) => {
              beginFormulaEditGeneration();
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

      <div class="sheet-body">
        <Switch>
        <Match when={props.repository.document() !== null}>
          <main class="sheet-workspace">
            <table class="sheet-grid" aria-label={props.language.text("sheetGridLabel")}>
              <thead>
                <tr>
                  <th class="sheet-grid__corner" scope="col" />
                  <For each={COLUMNS}>
                    {(column) => {
                      const columnIndex = column - 1;
                      return (
                        <th
                          class="sheet-grid__column-header"
                          classList={{
                            "sheet-grid__column-header--freeze-edge":
                              props.repository.freezeState().columns > 0 &&
                              columnIndex === props.repository.freezeState().columns - 1,
                          }}
                          scope="col"
                          style={frozenColumnHeaderStyle(columnIndex)}
                        >
                          {REFERENCE_TOOL.columnLabel(column)}
                        </th>
                      );
                    }}
                  </For>
                </tr>
              </thead>
              <tbody>
                <For each={visibleRows()}>
                  {(row, rowIndex) => (
                    <tr>
                      <th
                        class="sheet-grid__row-header"
                        classList={{
                          "sheet-grid__row-header--freeze-edge":
                            props.repository.freezeState().rows > 0 &&
                            rowIndex() === props.repository.freezeState().rows - 1,
                        }}
                        scope="row"
                        style={frozenRowHeaderStyle(rowIndex())}
                      >
                        {row}
                      </th>
                      <For each={COLUMNS}>
                        {(column) => {
                          const reference = REFERENCE_TOOL.reference(row, column);
                          return (
                            <td
                              class="sheet-grid__cell"
                              style={frozenCellStyle(rowIndex(), column - 1)}
                              classList={{
                                "sheet-grid__cell--frozen":
                                  rowIndex() < props.repository.freezeState().rows ||
                                  column - 1 < props.repository.freezeState().columns,
                                "sheet-grid__cell--selected":
                                  props.repository.selection()?.reference === reference,
                                "sheet-grid__cell--range": isCellInSelectionRange(row, column),
                                "sheet-grid__cell--conditional-warning":
                                  conditionalFormatStyleAt(row, column) === "warning",
                                "sheet-grid__cell--conditional-success":
                                  conditionalFormatStyleAt(row, column) === "success",
                                "sheet-grid__cell--conditional-accent":
                                  conditionalFormatStyleAt(row, column) === "accent",
                                "sheet-grid__cell--table-header":
                                  tableAtHeaderCell(row, column) !== null,
                                "sheet-grid__cell--table-body": (() => {
                                  const address = REFERENCE_TOOL.domainAddress(row, column);
                                  const table = tableAtAddress(address.row, address.column);
                                  return table !== null && table.startRow !== address.row;
                                })(),
                                "sheet-grid__cell--freeze-row-edge":
                                  props.repository.freezeState().rows > 0 &&
                                  rowIndex() === props.repository.freezeState().rows - 1,
                                "sheet-grid__cell--freeze-column-edge":
                                  props.repository.freezeState().columns > 0 &&
                                  column - 1 === props.repository.freezeState().columns - 1,
                              }}
                            >
                              <input
                                class="sheet-grid__input"
                                aria-label={reference}
                                value={editorValue(reference, row, column)}
                                style={cellStyle(reference)}
                                onMouseDown={(event) => {
                                  if (
                                    event.button === 0 &&
                                    event.shiftKey &&
                                    props.repository.selection() !== null
                                  ) {
                                    event.preventDefault();
                                    const address = REFERENCE_TOOL.domainAddress(row, column);
                                    void props.controller.extendSelection(
                                      address.row,
                                      address.column,
                                    );
                                  }
                                }}
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
                                  if (suppressBlurCommitReference === reference) {
                                    suppressBlurCommitReference = null;
                                    return;
                                  }
                                  void commitCell(
                                    reference,
                                    row,
                                    column,
                                    event.currentTarget.value,
                                  );
                                }}
                                onKeyDown={(event) => {
                                  if (event.key === "Enter") {
                                    event.preventDefault();
                                    void moveAfterCellCommit(
                                      reference,
                                      row,
                                      column,
                                      event.currentTarget.value,
                                      event.shiftKey ? -1 : 1,
                                    );
                                  }
                                }}
                              />
                              <Show when={tableAtHeaderCell(row, column) !== null}>
                                <button
                                  type="button"
                                  class="sheet-table-filter-button"
                                  aria-label={`${props.language.text("sheetTableFilter")} ${reference}`}
                                  onMouseDown={(event) => {
                                    event.preventDefault();
                                    event.stopPropagation();
                                  }}
                                  onClick={(event) => {
                                    event.stopPropagation();
                                    void openTableFilter(reference, row, column);
                                  }}
                                >
                                  &#9662;
                                </button>
                              </Show>
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
        <Show when={propertiesOpen()}>
          <SheetPropertiesSidebar
            controller={props.controller}
            repository={props.repository}
            language={props.language}
            onClose={() => setPropertiesOpen(false)}
          />
        </Show>
        <Show when={functionsOpen()}>
          <SheetFunctionsSidebar
            language={props.language}
            onClose={() => setFunctionsOpen(false)}
            onInsertDraft={insertFunctionDraft}
          />
        </Show>
        <Show when={chartsOpen()}>
          <SheetChartsSidebar
            controller={props.controller}
            repository={props.repository}
            language={props.language}
            onClose={() => setChartsOpen(false)}
          />
        </Show>
      </div>

      <footer class="sheet-statusbar">
        <span>{statusText()}</span>
        <span>
          {props.language.text("sheetSelectedCell")}: {props.repository.selection()?.reference ?? "-"}
        </span>
        <span>
          {props.language.text("sheetVisibleRows")}: {visibleRows().length}
        </span>
        <span>
          {props.language.text("sheetStatusValue")}: {evaluatedText()}
        </span>
        <Show when={props.repository.rangeSummary() !== null}>
          <span>
            {props.language.text("sheetStatusCount")}: {props.repository.rangeSummary()?.count ?? 0}
          </span>
          <Show when={(props.repository.rangeSummary()?.numericCount ?? 0) > 0}>
            <span>
              {props.language.text("sheetStatusSum")}: {props.repository.rangeSummary()?.sum ?? 0}
            </span>
            <span>
              {props.language.text("sheetStatusAverage")}: {props.repository.rangeSummary()?.average ?? 0}
            </span>
          </Show>
        </Show>
        <Show when={props.repository.rangeSummaryErrorCode() !== null}>
          <span>{props.repository.rangeSummaryErrorCode()}</span>
        </Show>
        <span class="sheet-statusbar__spacer" />
        <span>
          {props.language.text("revision")}: {props.repository.document()?.revision ?? 0}
        </span>
        <span>
          {props.language.text("sheetCells")}: {activeWorksheet()?.cellCount ?? 0}
        </span>
        <span>
          {props.language.text("sheetTables")}: {props.repository.document()?.tables.length ?? 0}
        </span>
        <span>
          {props.language.text("sheetConditionalRules")}: {props.repository.document()?.conditionalFormats.length ?? 0}
        </span>
        <span>
          {props.language.text("sheetCharts")}: {props.repository.document()?.charts.length ?? 0}
        </span>
      </footer>
    </div>
  );
}
