// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/sheet-session.service.test.ts
// # 📌 Amac: Desktop Sheet typed cell input routing davranisini regression testiyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.11.0
// Aciklama: Chart, function draft, dirty-state, table object, conditional formatting, range summary, stale query, freeze, filter-sort ve typed mutation davranislarini dogrular
// Bagimli Oldugu Katman: Service -> Repo -> Tool

import { describe, expect, it } from "vitest";

import { ERROR_CODES } from "../config/error-codes";
import { SHEET_FUNCTION_IDS } from "../config/sheet-functions";
import { LanguageService } from "../language/language-service";
import { SheetSessionRepository } from "../repositories/sheet-session.repository";
import { SheetFormulaHelperTool } from "../tools/sheet-formula-helper.tool";
import type { TauriSheetTool } from "../tools/tauri-sheet.tool";
import type { SheetCellFormatView, SheetDocumentView } from "../views/sheet-types";
import { SheetSessionService } from "./sheet-session.service";

const DOCUMENT: SheetDocumentView = {
  id: "sheet-document-1",
  title: "Sheet",
  revision: 0,
  worksheets: [
    {
      id: "worksheet-1",
      name: "Sheet1",
      cellCount: 0,
      cells: [],
    },
  ],
  conditionalFormats: [],
  tables: [],
  charts: [],
  chartCount: 0,
};

function toolWithCalls(calls: string[]): TauriSheetTool {
  let currentFormat: SheetCellFormatView = {
    bold: false,
    italic: false,
    underline: false,
    horizontalAlignment: "general",
    decimalPlaces: 2,
  };

  return {
    createDocument: async () => DOCUMENT,
    getDocument: async () => DOCUMENT,
    getEvaluatedCell: async (input) => {
      calls.push(`evaluate:${input.reference}`);
      return {
        row: 0,
        column: 2,
        value: { kind: "number", value: 2 },
      };
    },
    getCellFormat: async (input) => {
      calls.push(`format:get:${input.reference}`);
      return currentFormat;
    },
    setCellFormat: async (input, format) => {
      currentFormat = format;
      calls.push(
        `format:set:${input.reference}:${String(format.bold)}:${format.horizontalAlignment}:${String(format.decimalPlaces)}`,
      );
      return DOCUMENT;
    },
    createTable: async (request) => {
      calls.push(
        `table:create:${String(request.range.startRow)}:${String(request.range.endRow)}:${String(request.range.startColumn)}:${String(request.range.endColumn)}`,
      );
      return {
        ...DOCUMENT,
        revision: 1,
        tables: [
          {
            id: "table-1",
            worksheetId: request.worksheetId,
            name: "Table1",
            ...request.range,
          },
        ],
      };
    },
    removeTable: async (request) => {
      calls.push(`table:remove:${request.tableId}`);
      return { ...DOCUMENT, revision: 2, tables: [] };
    },
    createConditionalFormat: async (request) => {
      calls.push(
        `conditional:create:${request.condition.kind}:${request.style}:${String(request.range.startRow)}:${String(request.range.endRow)}:${String(request.range.startColumn)}:${String(request.range.endColumn)}`,
      );
      return {
        ...DOCUMENT,
        revision: 1,
        conditionalFormats: [
          {
            id: "conditional-format-1",
            worksheetId: request.worksheetId,
            ...request.range,
            condition: request.condition,
            style: request.style,
            priority: 1,
          },
        ],
      };
    },
    removeConditionalFormat: async (request) => {
      calls.push(`conditional:remove:${request.ruleId}`);
      return { ...DOCUMENT, revision: 2, conditionalFormats: [] };
    },
    getConditionalFormatMatches: async () => [],
    createChart: async (request) => {
      calls.push(
        `chart:create:${request.chartType}:${request.title}:${String(request.startRow)}:${String(request.endRow)}:${String(request.categoryColumn)}:${String(request.valueColumn)}`,
      );
      const chart = {
        id: "chart-1",
        worksheetId: request.worksheetId,
        chartType: request.chartType,
        title: request.title,
        startRow: request.startRow,
        endRow: request.endRow,
        categoryColumn: request.categoryColumn,
        valueColumn: request.valueColumn,
      };
      return { ...DOCUMENT, revision: 1, charts: [chart], chartCount: 1 };
    },
    removeChart: async (request) => {
      calls.push(`chart:remove:${request.chartId}`);
      return { ...DOCUMENT, revision: 2, charts: [], chartCount: 0 };
    },
    getChartData: async (request) => {
      calls.push(`chart:data:${request.chartId}`);
      return {
        points: [
          { category: "Ocak", value: 10 },
          { category: "Subat", value: 20 },
        ],
      };
    },
    getRangeSummary: async (request) => {
      calls.push(
        `summary:${String(request.range.startRow)}:${String(request.range.endRow)}:${String(request.range.startColumn)}:${String(request.range.endColumn)}`,
      );
      return {
        count: 4,
        numericCount: 3,
        sum: 30,
        average: 10,
      };
    },
    queryRows: async (request) => {
      calls.push(
        `query:${String(request.filter?.column ?? -1)}:${request.filter?.condition.kind ?? "none"}:${request.sort?.direction ?? "none"}:${String(request.range.startRow)}:${String(request.range.endRow)}:${String(request.range.startColumn)}:${String(request.range.endColumn)}`,
      );
      return { rows: [2, 0] };
    },
    clearCell: async () => {
      calls.push("clear");
      return DOCUMENT;
    },
    setFormula: async (_input, expression) => {
      calls.push(`formula:${expression}`);
      return DOCUMENT;
    },
    setBoolean: async (_input, value) => {
      calls.push(`boolean:${String(value)}`);
      return DOCUMENT;
    },
    setNumber: async (_input, value) => {
      calls.push(`number:${String(value)}`);
      return DOCUMENT;
    },
    setText: async (_input, value) => {
      calls.push(`text:${value}`);
      return DOCUMENT;
    },
  } as TauriSheetTool;
}

function createService(
  repository: SheetSessionRepository,
  sheetTool: TauriSheetTool,
  confirmDiscard: () => Promise<boolean> = async () => true,
): SheetSessionService {
  return new SheetSessionService(
    repository,
    sheetTool,
    new SheetFormulaHelperTool(),
    { confirmDiscard: async () => confirmDiscard() },
    new LanguageService(),
  );
}

describe("SheetSessionService", () => {
  it("creates the first document during initialization", async () => {
    const repository = new SheetSessionRepository();
    const service = createService(repository, toolWithCalls([]));

    await service.initializeSession();

    expect(repository.document()?.id).toBe(DOCUMENT.id);
    expect(repository.status()).toBe("ready");
  });

  it("returns function drafts only when a cell is selected", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls([]));

    expect(service.functionFormulaDraft(SHEET_FUNCTION_IDS.sum)).toBeNull();

    await service.selectCell("A1", 0, 0);

    expect(service.functionFormulaDraft(SHEET_FUNCTION_IDS.sum)).toBe("=SUM(");
  });

  it("keeps a dirty Sheet when new document discard is cancelled", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument({ ...DOCUMENT, revision: 3 });
    repository.markDirty();
    let createCalls = 0;
    const sheetTool = toolWithCalls([]);
    sheetTool.createDocument = async () => {
      createCalls += 1;
      return { ...DOCUMENT, id: "sheet-document-2" };
    };
    const service = createService(repository, sheetTool, async () => false);

    const created = await service.createDocument();

    expect(created).toBe(false);
    expect(createCalls).toBe(0);
    expect(repository.document()?.id).toBe(DOCUMENT.id);
    expect(repository.dirty()).toBe(true);
  });

  it("does not apply an old Sheet cell commit after a replacement document is installed", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    let resolveCreate!: (value: SheetDocumentView) => void;
    let signalCreateStarted!: () => void;
    const createStarted = new Promise<void>((resolve) => {
      signalCreateStarted = resolve;
    });
    const sheetTool = toolWithCalls(calls);
    sheetTool.createDocument = async () => {
      signalCreateStarted();
      return new Promise<SheetDocumentView>((resolve) => {
        resolveCreate = resolve;
      });
    };
    const service = createService(repository, sheetTool);

    const replacing = service.createDocument();
    await createStarted;
    const oldCommit = service.commitCell("A1", "5");
    resolveCreate({ ...DOCUMENT, id: "sheet-document-2" });
    await replacing;
    await oldCommit;

    expect(repository.document()?.id).toBe("sheet-document-2");
    expect(calls).not.toContain("number:5");
    expect(repository.dirty()).toBe(false);
  });

  it("ignores a row-query response after the query is cleared", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    let resolveQuery!: (value: { rows: number[] }) => void;
    const sheetTool = toolWithCalls([]);
    sheetTool.queryRows = async () =>
      new Promise<{ rows: number[] }>((resolve) => {
        resolveQuery = resolve;
      });
    const service = createService(repository, sheetTool);

    await service.selectCell("A1", 0, 0);
    const pending = service.applyRowQuery("nonEmpty", "", "ascending");
    service.clearRowQuery();
    resolveQuery({ rows: [4, 2, 0] });
    await pending;

    expect(repository.rowQuery()).toBeNull();
    expect(repository.rowQueryErrorCode()).toBeNull();
  });

  it("loads raw formula and evaluated value for the selected cell", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument({
      ...DOCUMENT,
      worksheets: [
        {
          ...DOCUMENT.worksheets[0]!,
          cellCount: 1,
          cells: [
            {
              row: 0,
              column: 2,
              value: { kind: "formula", value: "=1+1" },
            },
          ],
        },
      ],
    });
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("C1", 0, 2);

    expect(calls).toEqual(["evaluate:C1", "format:get:C1"]);
    expect(repository.selection()).toEqual({
      reference: "C1",
      row: 0,
      column: 2,
      rawValue: "=1+1",
      evaluatedValue: { kind: "number", value: 2 },
      evaluationErrorCode: null,
      format: {
        bold: false,
        italic: false,
        underline: false,
        horizontalAlignment: "general",
        decimalPlaces: 2,
      },
    });
  });

  it("creates and selects a canonical chart from an exact two-column range", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("A1", 0, 0);
    await service.extendSelection(2, 1);
    calls.length = 0;
    await service.createChartFromSelection("bar", "Aylik Satis");

    expect(calls).toEqual([
      "chart:create:bar:Aylik Satis:0:2:0:1",
      "chart:data:chart-1",
    ]);
    expect(repository.document()?.charts).toHaveLength(1);
    expect(repository.selectedChartId()).toBe("chart-1");
    expect(repository.chartData()?.points).toEqual([
      { category: "Ocak", value: 10 },
      { category: "Subat", value: 20 },
    ]);
    expect(repository.dirty()).toBe(true);
  });

  it("preserves a newer chart selection while chart creation is pending", async () => {
    const repository = new SheetSessionRepository();
    const existingChart = {
      id: "chart-existing",
      worksheetId: "worksheet-1",
      chartType: "line" as const,
      title: "Mevcut",
      startRow: 0,
      endRow: 1,
      categoryColumn: 0,
      valueColumn: 1,
    };
    repository.setDocument({
      ...DOCUMENT,
      charts: [existingChart],
      chartCount: 1,
    });
    const sheetTool = toolWithCalls([]);
    let resolveCreate!: (value: SheetDocumentView) => void;
    let signalCreateStarted!: () => void;
    const createStarted = new Promise<void>((resolve) => {
      signalCreateStarted = resolve;
    });
    sheetTool.createChart = async (request) => {
      signalCreateStarted();
      return new Promise<SheetDocumentView>((resolve) => {
        resolveCreate = resolve;
      });
    };
    sheetTool.getChartData = async (request) => ({
      points: [{ category: request.chartId, value: 10 }],
    });
    const service = createService(repository, sheetTool);

    await service.selectCell("A1", 0, 0);
    await service.extendSelection(1, 1);
    const creating = service.createChartFromSelection("bar", "Yeni");
    await createStarted;
    await service.selectChart(existingChart.id);

    resolveCreate({
      ...DOCUMENT,
      revision: 2,
      charts: [
        existingChart,
        {
          id: "chart-new",
          worksheetId: "worksheet-1",
          chartType: "bar",
          title: "Yeni",
          startRow: 0,
          endRow: 1,
          categoryColumn: 0,
          valueColumn: 1,
        },
      ],
      chartCount: 2,
    });
    await creating;

    expect(repository.document()?.charts).toHaveLength(2);
    expect(repository.selectedChartId()).toBe(existingChart.id);
    expect(repository.chartData()?.points).toEqual([
      { category: existingChart.id, value: 10 },
    ]);
  });

  it("rejects chart creation unless the selection spans exactly two columns", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("A1", 0, 0);
    await service.extendSelection(2, 2);
    calls.length = 0;
    await service.createChartFromSelection("line", "Gecersiz");

    expect(calls).toEqual([]);
    expect(repository.chartErrorCode()).toBe(ERROR_CODES.sheetInvalidChartRange);
    expect(repository.document()?.charts).toEqual([]);
  });

  it("ignores stale chart data after a newer chart selection", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument({
      ...DOCUMENT,
      charts: [
        {
          id: "chart-1",
          worksheetId: "worksheet-1",
          chartType: "bar",
          title: "Bir",
          startRow: 0,
          endRow: 1,
          categoryColumn: 0,
          valueColumn: 1,
        },
        {
          id: "chart-2",
          worksheetId: "worksheet-1",
          chartType: "line",
          title: "Iki",
          startRow: 0,
          endRow: 1,
          categoryColumn: 0,
          valueColumn: 1,
        },
      ],
      chartCount: 2,
    });
    const sheetTool = toolWithCalls([]);
    let resolveFirst!: (value: { points: { category: string; value: number }[] }) => void;
    let resolveSecond!: (value: { points: { category: string; value: number }[] }) => void;
    sheetTool.getChartData = async (request) =>
      new Promise((resolve) => {
        if (request.chartId === "chart-1") {
          resolveFirst = resolve;
        } else {
          resolveSecond = resolve;
        }
      });
    const service = createService(repository, sheetTool);

    const first = service.selectChart("chart-1");
    const second = service.selectChart("chart-2");

    expect(repository.selectedChartId()).toBe("chart-2");
    expect(repository.chartData()).toBeNull();

    resolveSecond({ points: [{ category: "Yeni", value: 20 }] });
    await second;
    resolveFirst({ points: [{ category: "Eski", value: 10 }] });
    await first;

    expect(repository.selectedChartId()).toBe("chart-2");
    expect(repository.chartData()?.points).toEqual([{ category: "Yeni", value: 20 }]);
  });

  it("removes the selected canonical chart and clears its projected data", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument({
      ...DOCUMENT,
      charts: [
        {
          id: "chart-1",
          worksheetId: "worksheet-1",
          chartType: "pie",
          title: "Dagilim",
          startRow: 0,
          endRow: 1,
          categoryColumn: 0,
          valueColumn: 1,
        },
      ],
      chartCount: 1,
    });
    const service = createService(repository, toolWithCalls(calls));

    await service.selectChart("chart-1");
    calls.length = 0;
    await service.removeChart("chart-1");

    expect(calls).toEqual(["chart:remove:chart-1"]);
    expect(repository.document()?.charts).toEqual([]);
    expect(repository.selectedChartId()).toBeNull();
    expect(repository.chartData()).toBeNull();
    expect(repository.dirty()).toBe(true);
  });

  it("creates a canonical table from the current rectangular selection", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("A1", 0, 0);
    await service.extendSelection(3, 2);
    calls.length = 0;
    await service.createTableFromSelection();

    expect(calls).toEqual(["table:create:0:3:0:2"]);
    expect(repository.document()?.tables).toEqual([
      {
        id: "table-1",
        worksheetId: "worksheet-1",
        name: "Table1",
        startRow: 0,
        endRow: 3,
        startColumn: 0,
        endColumn: 2,
      },
    ]);
    expect(repository.dirty()).toBe(true);
  });

  it("removes the table containing the active cell", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument({
      ...DOCUMENT,
      tables: [
        {
          id: "table-1",
          worksheetId: "worksheet-1",
          name: "Table1",
          startRow: 0,
          endRow: 4,
          startColumn: 0,
          endColumn: 2,
        },
      ],
    });
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("B2", 1, 1);
    calls.length = 0;
    await service.removeTableAtSelection();

    expect(calls).toEqual(["table:remove:table-1"]);
    expect(repository.document()?.tables).toEqual([]);
    expect(repository.dirty()).toBe(true);
  });

  it("uses table data rows when filtering from a table header", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument({
      ...DOCUMENT,
      tables: [
        {
          id: "table-1",
          worksheetId: "worksheet-1",
          name: "Table1",
          startRow: 0,
          endRow: 5,
          startColumn: 1,
          endColumn: 3,
        },
      ],
    });
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("B1", 0, 1);
    calls.length = 0;
    await service.applyRowQuery("nonEmpty", "", "ascending");

    expect(calls).toEqual(["query:1:nonEmpty:ascending:1:5:1:3"]);
    expect(repository.rowQuery()?.range).toEqual({
      startRow: 1,
      endRow: 5,
      startColumn: 1,
      endColumn: 3,
    });
  });

  it("creates a canonical conditional format from the current selection range", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("A1", 0, 0);
    await service.extendSelection(2, 1);
    calls.length = 0;
    await service.applyConditionalFormat("numberGreaterThan", "10", "warning");

    expect(calls).toEqual(["conditional:create:numberGreaterThan:warning:0:2:0:1"]);
    expect(repository.document()?.conditionalFormats).toHaveLength(1);
    expect(repository.document()?.conditionalFormats[0]?.condition).toEqual({
      kind: "numberGreaterThan",
      value: 10,
    });
    expect(repository.dirty()).toBe(true);
  });

  it("rejects invalid conditional format values before invoking the backend", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("A1", 0, 0);
    calls.length = 0;
    await service.applyConditionalFormat("numberEquals", "abc", "accent");
    await service.applyConditionalFormat("textContains", "   ", "success");

    expect(calls).toEqual([]);
    expect(repository.conditionalFormatErrorCode()).toBe(
      ERROR_CODES.sheetInvalidConditionalFormat,
    );
  });

  it("removes the highest-priority conditional format covering the active cell", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument({
      ...DOCUMENT,
      conditionalFormats: [
        {
          id: "conditional-format-2",
          worksheetId: "worksheet-1",
          startRow: 0,
          endRow: 2,
          startColumn: 0,
          endColumn: 2,
          condition: { kind: "numberGreaterThan", value: 5 },
          style: "success",
          priority: 2,
        },
        {
          id: "conditional-format-1",
          worksheetId: "worksheet-1",
          startRow: 0,
          endRow: 2,
          startColumn: 0,
          endColumn: 2,
          condition: { kind: "numberGreaterThan", value: 0 },
          style: "warning",
          priority: 1,
        },
      ],
    });
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("B2", 1, 1);
    calls.length = 0;
    await service.removeConditionalFormatAtSelection();

    expect(calls).toEqual(["conditional:remove:conditional-format-1"]);
    expect(repository.document()?.conditionalFormats).toEqual([]);
  });

  it("refreshes conditional format matches after a cell mutation", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const sheetTool = toolWithCalls([]);
    sheetTool.getConditionalFormatMatches = async () => [
      { row: 0, column: 0, style: "success" },
    ];
    const service = createService(repository, sheetTool);

    await service.commitCell("A1", "5");

    expect(repository.conditionalFormatStyle(0, 0)).toBe("success");
  });

  it("freezes panes above and left of the active cell", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls([]));

    await service.selectCell("C3", 2, 2);
    service.freezeAtSelection();

    expect(repository.freezeState()).toEqual({ rows: 2, columns: 2 });

    service.freezeTopRow();
    expect(repository.freezeState()).toEqual({ rows: 1, columns: 0 });

    service.freezeFirstColumn();
    expect(repository.freezeState()).toEqual({ rows: 0, columns: 1 });

    service.unfreezePanes();
    expect(repository.freezeState()).toEqual({ rows: 0, columns: 0 });
  });

  it("uses the visible filtered row position for freeze-at-selection", async () => {
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    repository.setRowQuery({
      column: 0,
      range: {
        startRow: 0,
        endRow: 99,
        startColumn: 0,
        endColumn: 25,
      },
      filterMode: "nonEmpty",
      filterValue: "",
      sortDirection: "ascending",
      rows: [8, 2, 5],
    });
    const service = createService(repository, toolWithCalls([]));

    await service.selectCell("A6", 5, 0);
    service.freezeAtSelection();

    expect(repository.freezeState()).toEqual({ rows: 2, columns: 0 });
  });

  it("computes freeze position after a scoped table query", async () => {
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
      sortDirection: "ascending",
      rows: [2],
    });
    const service = createService(repository, toolWithCalls([]));

    await service.selectCell("A5", 4, 0);
    service.freezeAtSelection();

    expect(repository.freezeState()).toEqual({ rows: 2, columns: 0 });
  });

  it("extends the active cell into a canonical range summary", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("A1", 0, 0);
    calls.length = 0;
    await service.extendSelection(2, 1);

    expect(calls).toEqual(["summary:0:2:0:1"]);
    expect(repository.selectionRange()).toEqual({
      startRow: 0,
      endRow: 2,
      startColumn: 0,
      endColumn: 1,
    });
    expect(repository.rangeSummary()).toEqual({
      count: 4,
      numericCount: 3,
      sum: 30,
      average: 10,
    });
  });

  it("toggles selected formatting without dropping existing format fields", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("A1", 0, 0);
    calls.length = 0;
    await service.toggleBold();

    expect(calls).toEqual([
      "format:set:A1:true:general:2",
      "evaluate:A1",
      "format:get:A1",
    ]);
    expect(repository.selection()?.format).toEqual({
      bold: true,
      italic: false,
      underline: false,
      horizontalAlignment: "general",
      decimalPlaces: 2,
    });
    expect(repository.cellFormat("worksheet-1", "A1")).toEqual(
      repository.selection()?.format,
    );
  });

  it("updates decimal places without dropping other selected format fields", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("A1", 0, 0);
    await service.toggleBold();
    calls.length = 0;
    await service.setDecimalPlaces(4);

    expect(calls).toEqual([
      "format:set:A1:true:general:4",
      "evaluate:A1",
      "format:get:A1",
    ]);
    expect(repository.cellFormat("worksheet-1", "A1")).toEqual({
      bold: true,
      italic: false,
      underline: false,
      horizontalAlignment: "general",
      decimalPlaces: 4,
    });
  });

  it("refreshes the currently selected cell after another cell mutation completes", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    let resolveMutation!: (value: SheetDocumentView) => void;
    const sheetTool = toolWithCalls(calls);
    sheetTool.setNumber = async () =>
      new Promise<SheetDocumentView>((resolve) => {
        resolveMutation = resolve;
      });
    const service = createService(repository, sheetTool);

    await service.selectCell("A1", 0, 0);
    calls.length = 0;
    const pending = service.commitCell("A1", "5");
    await Promise.resolve();
    await service.selectCell("B1", 0, 1);
    resolveMutation(DOCUMENT);
    await pending;

    expect(calls.filter((call) => call === "evaluate:B1")).toHaveLength(2);
    expect(repository.selection()?.reference).toBe("B1");
  });

  it("does not restore an older selection when format mutation completes late", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    let resolveFormat!: (value: SheetDocumentView) => void;
    const sheetTool = toolWithCalls(calls);
    sheetTool.setCellFormat = async () =>
      new Promise<SheetDocumentView>((resolve) => {
        resolveFormat = resolve;
      });
    const service = createService(repository, sheetTool);

    await service.selectCell("A1", 0, 0);
    const pending = service.toggleBold();
    await Promise.resolve();
    await service.selectCell("B1", 0, 1);
    resolveFormat(DOCUMENT);
    await pending;

    expect(repository.selection()?.reference).toBe("B1");
    expect(repository.cellFormat("worksheet-1", "A1")?.bold).toBe(true);
  });

  it("applies selected-column filter and sort as a non-mutating row query", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("B1", 0, 1);
    calls.length = 0;
    await service.applyRowQuery("numberGreaterThan", "15", "ascending");

    expect(calls).toEqual(["query:1:numberGreaterThan:ascending:0:99:0:25"]);
    expect(repository.rowQuery()).toEqual({
      column: 1,
      range: {
        startRow: 0,
        endRow: 99,
        startColumn: 0,
        endColumn: 25,
      },
      filterMode: "numberGreaterThan",
      filterValue: "15",
      sortDirection: "ascending",
      rows: [2, 0],
    });
  });

  it("rejects invalid numeric filter before invoking the backend", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    repository.setDocument(DOCUMENT);
    const service = createService(repository, toolWithCalls(calls));

    await service.selectCell("B1", 0, 1);
    calls.length = 0;
    await service.applyRowQuery("numberLessThan", "", "none");
    await service.applyRowQuery("numberLessThan", "   ", "none");
    await service.applyRowQuery("numberLessThan", "abc", "none");
    await service.applyRowQuery("numberLessThan", "0x10", "none");

    expect(calls).toEqual([]);
    expect(repository.rowQueryErrorCode()).toBe(ERROR_CODES.sheetInvalidFilter);
  });

  it("routes typed cell inputs without moving parsing into the View", async () => {
    const calls: string[] = [];
    const repository = new SheetSessionRepository();
    const service = createService(repository, toolWithCalls(calls));
    await service.initializeSession();

    await service.commitCell("A1", "");
    await service.commitCell("A2", "=A1+1");
    await service.commitCell("A3", "TRUE");
    await service.commitCell("A4", "12.5");
    await service.commitCell("A5", "  metin  ");

    expect(calls).toEqual([
      "clear",
      "formula:=A1+1",
      "boolean:true",
      "number:12.5",
      "text:  metin  ",
    ]);
    expect(repository.dirty()).toBe(true);
  });
});
