// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-shell.tsx
// # 📌 Amac: Turkuaz Office Desktop Sheet temel grid yuzeyini render eder
// # 📌 Modul - FileType: View - TSX
// Version: 0.4.0
// Aciklama: Modul secimi, yeni Sheet komutu, 30x12 editable grid ve session statusbar View'larini birlestirir
// Bagimli Oldugu Katman: View -> Controller -> Repo -> Tool -> Language

import { For, Match, onMount, Switch } from "solid-js";

import { OFFICE_MODULES, type OfficeModule } from "../config/office-modules";
import { SHEET_GRID_COLUMN_COUNT, SHEET_GRID_ROW_COUNT } from "../config/sheet";
import type { SheetController } from "../controllers/sheet.controller";
import type { LanguageService } from "../language/language-service";
import type { SheetSessionRepository } from "../repositories/sheet-session.repository";
import { SheetReferenceTool } from "../tools/sheet-reference.tool";
import type { SheetCellValueView } from "./sheet-types";
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

export function SheetShell(props: SheetShellProps) {
  onMount(() => {
    void props.controller.initializeSession();
  });

  const activeWorksheet = () => props.repository.document()?.worksheets[0] ?? null;

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
    const cell = activeWorksheet()?.cells.find(
      (item) => item.row === row && item.column === column,
    );
    return cell === undefined ? "" : valueText(cell.value);
  };

  const commitCell = (row: number, column: number, value: string): void => {
    if (cellText(row, column) === value) {
      return;
    }
    void props.controller.commitCell(REFERENCE_TOOL.reference(row, column), value);
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

      <div class="sheet-toolbar" aria-label={props.language.text("sheetToolbarLabel")}>
        <button
          type="button"
          class="toolbar-button toolbar-button--primary"
          onClick={() => void props.controller.createDocument()}
        >
          {props.language.text("sheetNewDocument")}
        </button>
        <span>{activeWorksheet()?.name ?? "-"}</span>
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
                <For each={ROWS}>
                  {(row) => (
                    <tr>
                      <th class="sheet-grid__row-header" scope="row">
                        {row}
                      </th>
                      <For each={COLUMNS}>
                        {(column) => {
                          const reference = REFERENCE_TOOL.reference(row, column);
                          return (
                            <td class="sheet-grid__cell">
                              <input
                                class="sheet-grid__input"
                                aria-label={reference}
                                value={cellText(row, column)}
                                onBlur={(event) => commitCell(row, column, event.currentTarget.value)}
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
