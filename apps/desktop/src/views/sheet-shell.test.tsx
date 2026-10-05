// @vitest-environment jsdom

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-shell.test.tsx
// # 📌 Amac: Sheet grid ve formula bar aktif draft'larinin async read-model guncellemelerinde korunmasini dogrular
// # 📌 Modul - FileType: Test - TSX
// Version: 0.7.0
// Aciklama: Aktif draft, table header/filter, conditional formatting, range query, freeze ve yeni Sheet query-reset davranislarini jsdom regression testleriyle sabitler
// Bagimli Oldugu Katman: View -> Controller -> Repo

import { render } from "solid-js/web";
import { afterEach, describe, expect, it } from "vitest";

import type { SheetController } from "../controllers/sheet.controller";
import { LanguageService } from "../language/language-service";
import { SheetSessionRepository } from "../repositories/sheet-session.repository";
import type { SheetDocumentView } from "./sheet-types";
import { SheetShell } from "./sheet-shell";

const DOCUMENT: SheetDocumentView = {
  id: "sheet-document-1",
  title: "Sheet",
  revision: 1,
  worksheets: [
    {
      id: "worksheet-1",
      name: "Sheet1",
      cellCount: 1,
      cells: [
        {
          row: 0,
          column: 0,
          value: { kind: "number", value: 12.34 },
        },
      ],
    },
  ],
  conditionalFormats: [],
  tables: [],
  chartCount: 0,
};

function controllerStub(): SheetController {
  return {
    initializeSession: async () => undefined,
    createDocument: async () => true,
    selectCell: async () => undefined,
    extendSelection: async () => undefined,
    createTableFromSelection: async () => undefined,
    removeTableAtSelection: async () => undefined,
    applyConditionalFormat: async () => undefined,
    removeConditionalFormatAtSelection: async () => undefined,
    freezeAtSelection: () => undefined,
    freezeTopRow: () => undefined,
    freezeFirstColumn: () => undefined,
    unfreezePanes: () => undefined,
    toggleBold: async () => undefined,
    toggleItalic: async () => undefined,
    toggleUnderline: async () => undefined,
    setHorizontalAlignment: async () => undefined,
    setDecimalPlaces: async () => undefined,
    applyRowQuery: async () => undefined,
    clearRowQuery: () => undefined,
    commitCell: async () => undefined,
  } as unknown as SheetController;
}

function mount(repository: SheetSessionRepository): HTMLElement {
  const root = document.createElement("div");
  document.body.append(root);
  dispose = render(
    () => (
      <SheetShell
        controller={controllerStub()}
        repository={repository}
        language={new LanguageService("en-US")}
      />
    ),
    root,
  );
  return root;
}

let dispose: (() => void) | null = null;

afterEach(() => {
  dispose?.();
  dispose = null;
  document.body.innerHTML = "";
});

describe("SheetShell active drafts", () => {
  it("resets local query controls when a new Sheet is created", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const root = mount(repository);

    const showQuery = Array.from(root.querySelectorAll("button")).find(
      (button) => button.textContent?.trim() === "Show filter and sort",
    );
    expect(showQuery).not.toBeUndefined();
    showQuery!.click();
    await Promise.resolve();

    const filter = root.querySelector<HTMLSelectElement>('select[aria-label="Filter"]');
    const sort = root.querySelector<HTMLSelectElement>('select[aria-label="Sort"]');
    expect(filter).not.toBeNull();
    expect(sort).not.toBeNull();

    filter!.value = "nonEmpty";
    filter!.dispatchEvent(new Event("change", { bubbles: true }));
    sort!.value = "ascending";
    sort!.dispatchEvent(new Event("change", { bubbles: true }));

    expect(filter!.value).toBe("nonEmpty");
    expect(sort!.value).toBe("ascending");

    const newSheet = Array.from(root.querySelectorAll("button")).find(
      (button) => button.textContent?.trim() === "New Sheet",
    );
    expect(newSheet).not.toBeUndefined();
    newSheet!.click();
    await Promise.resolve();

    expect(filter!.value).toBe("none");
    expect(sort!.value).toBe("none");
  });

  it("keeps uncommitted grid input when async format cache updates", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const root = mount(repository);

    const input = root.querySelector<HTMLInputElement>('input[aria-label="A1"]');
    expect(input).not.toBeNull();
    input!.focus();
    input!.value = "123";
    input!.dispatchEvent(new Event("input", { bubbles: true }));

    repository.setCellFormat("worksheet-1", "A1", {
      bold: false,
      italic: false,
      underline: false,
      horizontalAlignment: "general",
      decimalPlaces: 2,
    });
    await Promise.resolve();

    expect(input!.value).toBe("123");
  });

  it("renders automatic filter controls on canonical table headers", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument({
      ...DOCUMENT,
      tables: [
        {
          id: "table-1",
          worksheetId: "worksheet-1",
          name: "Table1",
          startRow: 0,
          endRow: 3,
          startColumn: 0,
          endColumn: 1,
        },
      ],
    });

    const root = mount(repository);
    expect(root.querySelector('button[aria-label="Table filter A1"]')).not.toBeNull();
    expect(root.querySelector('button[aria-label="Table filter B1"]')).not.toBeNull();
    expect(root.querySelector('button[aria-label="Table filter A2"]')).toBeNull();
    expect(root.textContent).toContain("Tables: 1");
  });

  it("keeps rows outside a table query range visible", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    repository.setRowQuery({
      column: 0,
      range: {
        startRow: 1,
        endRow: 3,
        startColumn: 0,
        endColumn: 1,
      },
      filterMode: "nonEmpty",
      filterValue: "",
      sortDirection: "none",
      rows: [2],
    });

    const root = mount(repository);
    expect(root.querySelector('input[aria-label="A1"]')).not.toBeNull();
    expect(root.querySelector('input[aria-label="A2"]')).toBeNull();
    expect(root.querySelector('input[aria-label="A3"]')).not.toBeNull();
    expect(root.querySelector('input[aria-label="A4"]')).toBeNull();
    expect(root.querySelector('input[aria-label="A5"]')).not.toBeNull();
  });

  it("preserves backend sort order inside a scoped table query", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    repository.setRowQuery({
      column: 0,
      range: {
        startRow: 1,
        endRow: 3,
        startColumn: 0,
        endColumn: 1,
      },
      filterMode: "none",
      filterValue: "",
      sortDirection: "descending",
      rows: [3, 1],
    });

    const root = mount(repository);
    const firstRows = Array.from(root.querySelectorAll(".sheet-grid__row-header"))
      .slice(0, 4)
      .map((header) => header.textContent?.trim());

    expect(firstRows).toEqual(["1", "4", "2", "5"]);
  });

  it("renders frozen row and column cells with sticky offsets", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    repository.setFreezeState({ rows: 1, columns: 1 });

    const root = mount(repository);
    const a1 = root.querySelector<HTMLInputElement>('input[aria-label="A1"]');
    const cell = a1?.closest<HTMLTableCellElement>("td");
    expect(cell?.style.position).toBe("sticky");
    expect(cell?.style.top).toBe("24px");
    expect(cell?.style.left).toBe("36px");
    expect(cell?.classList.contains("sheet-grid__cell--freeze-row-edge")).toBe(true);
    expect(cell?.classList.contains("sheet-grid__cell--freeze-column-edge")).toBe(true);
  });

  it("renders backend conditional format matches as semantic grid highlights", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument({
      ...DOCUMENT,
      conditionalFormats: [
        {
          id: "conditional-format-1",
          worksheetId: "worksheet-1",
          startRow: 0,
          endRow: 1,
          startColumn: 0,
          endColumn: 0,
          condition: { kind: "numberGreaterThan", value: 10 },
          style: "warning",
          priority: 1,
        },
      ],
    });
    repository.setConditionalFormatMatches([
      { row: 0, column: 0, style: "warning" },
    ]);

    const root = mount(repository);
    const a1 = root.querySelector<HTMLInputElement>('input[aria-label="A1"]');
    expect(
      a1?.closest("td")?.classList.contains("sheet-grid__cell--conditional-warning"),
    ).toBe(true);
    expect(root.textContent).toContain("Conditional rules: 1");
  });

  it("renders range selection and Excel-style status aggregates", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    repository.setSelection({
      reference: "A1",
      row: 0,
      column: 0,
      rawValue: "12.34",
      evaluatedValue: { kind: "number", value: 12.34 },
      evaluationErrorCode: null,
      format: null,
    });
    repository.setSelectionRange({
      startRow: 0,
      endRow: 1,
      startColumn: 0,
      endColumn: 1,
    });
    repository.setRangeSummary({
      count: 4,
      numericCount: 3,
      sum: 30,
      average: 10,
    });

    const root = mount(repository);
    const b2 = root.querySelector<HTMLInputElement>('input[aria-label="B2"]');
    expect(b2?.closest("td")?.classList.contains("sheet-grid__cell--range")).toBe(true);
    expect(root.textContent).toContain("Count: 4");
    expect(root.textContent).toContain("Sum: 30");
    expect(root.textContent).toContain("Average: 10");
  });

  it("keeps uncommitted formula input when selected-cell read model refreshes", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    repository.setSelection({
      reference: "A1",
      row: 0,
      column: 0,
      rawValue: "12.34",
      evaluatedValue: { kind: "number", value: 12.34 },
      evaluationErrorCode: null,
      format: {
        bold: false,
        italic: false,
        underline: false,
        horizontalAlignment: "general",
        decimalPlaces: null,
      },
    });
    const root = mount(repository);

    const formula = root.querySelector<HTMLInputElement>('.sheet-formula-bar__input');
    expect(formula).not.toBeNull();
    formula!.focus();
    formula!.value = "=1+2";
    formula!.dispatchEvent(new Event("input", { bubbles: true }));

    repository.setSelection({
      ...repository.selection()!,
      format: {
        bold: true,
        italic: false,
        underline: false,
        horizontalAlignment: "general",
        decimalPlaces: null,
      },
    });
    await Promise.resolve();

    expect(formula!.value).toBe("=1+2");
  });
});
