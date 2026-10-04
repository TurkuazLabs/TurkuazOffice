// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/sheet-session.service.test.ts
// # 📌 Amac: Desktop Sheet typed cell input routing davranisini regression testiyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.1
// Aciklama: Dirty-state, discard onayi, stale query korumasi, format cache, filter-sort ve typed mutation davranislarini dogrular
// Bagimli Oldugu Katman: Service -> Repo -> Tool

import { describe, expect, it } from "vitest";

import { ERROR_CODES } from "../config/error-codes";
import { LanguageService } from "../language/language-service";
import { SheetSessionRepository } from "../repositories/sheet-session.repository";
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
    queryRows: async (request) => {
      calls.push(
        `query:${String(request.filter?.column ?? -1)}:${request.filter?.condition.kind ?? "none"}:${request.sort?.direction ?? "none"}:${String(request.range.endRow)}:${String(request.range.endColumn)}`,
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
    const sheetTool = toolWithCalls(calls);
    sheetTool.createDocument = async () =>
      new Promise<SheetDocumentView>((resolve) => {
        resolveCreate = resolve;
      });
    const service = createService(repository, sheetTool);

    const replacing = service.createDocument();
    await Promise.resolve();
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

    expect(calls).toEqual(["query:1:numberGreaterThan:ascending:29:11"]);
    expect(repository.rowQuery()).toEqual({
      column: 1,
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
