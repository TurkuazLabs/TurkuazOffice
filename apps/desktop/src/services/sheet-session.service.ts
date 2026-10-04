// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/sheet-session.service.ts
// # 📌 Amac: Desktop Sheet oturum ve cell edit is akisini koordine eder
// # 📌 Modul - FileType: Service - TypeScript
// Version: 0.4.1
// Aciklama: Dirty-state korumali create, cell edit/format ve stale-response guvenli filter-sort akislarini koordine eder
// Bagimli Oldugu Katman: Service -> Repo -> Tool -> Language

import { ERROR_CODES } from "../config/error-codes";
import {
  SHEET_BOOLEAN_FALSE,
  SHEET_BOOLEAN_TRUE,
  SHEET_FORMULA_PREFIX,
  SHEET_GRID_COLUMN_COUNT,
  SHEET_GRID_ROW_COUNT,
  SHEET_NUMBER_PATTERN,
} from "../config/sheet";
import type { LanguageService } from "../language/language-service";
import type { SheetSessionRepository } from "../repositories/sheet-session.repository";
import type { NativeFileDialogTool } from "../tools/native-file-dialog.tool";
import type { TauriSheetTool } from "../tools/tauri-sheet.tool";
import type {
  SheetCellFormatView,
  SheetCellValueView,
  SheetDesktopErrorView,
  SheetDocumentView,
  SheetFilterConditionView,
  SheetFilterModeView,
  SheetHorizontalAlignmentView,
  SheetRowQueryRequestView,
  SheetSortDirectionView,
} from "../views/sheet-types";

export class SheetSessionService {
  private mutationQueue: Promise<void> = Promise.resolve();
  private rowQueryGeneration = 0;

  public constructor(
    private readonly repository: SheetSessionRepository,
    private readonly sheetTool: TauriSheetTool,
    private readonly fileDialogTool: Pick<NativeFileDialogTool, "confirmDiscard">,
    private readonly language: LanguageService,
  ) {}

  public async initializeSession(): Promise<void> {
    if (this.repository.document() !== null) {
      return;
    }
    await this.createDocument();
  }

  public createDocument(): Promise<boolean> {
    return this.enqueue(async () => {
      if (!(await this.confirmDiscardIfNeeded())) {
        return false;
      }

      this.repository.setLoading();
      try {
        const document = await this.sheetTool.createDocument();
        this.invalidateRowQueryRequests();
        this.repository.setSelection(null);
        this.repository.clearCellFormats();
        this.repository.clearRowQuery();
        this.repository.setDocument(document);
        this.repository.markClean();
        return true;
      } catch (error: unknown) {
        this.repository.setError(this.errorCode(error));
        return false;
      }
    });
  }

  public async selectCell(reference: string, row: number, column: number): Promise<void> {
    const document = this.requireDocument();
    const worksheet = document.worksheets[0];
    if (worksheet === undefined) {
      this.repository.setError(ERROR_CODES.sheetWorksheetNotFound);
      return;
    }

    const rawCell = worksheet.cells.find(
      (cell) => cell.row === row && cell.column === column,
    );
    this.repository.setSelection({
      reference,
      row,
      column,
      rawValue: rawCell === undefined ? "" : this.valueText(rawCell.value),
      evaluatedValue: null,
      evaluationErrorCode: null,
      format: null,
    });
    await Promise.all([
      this.refreshSelectionEvaluation(),
      this.refreshSelectionFormat(),
    ]);
  }

  public toggleBold(): Promise<void> {
    return this.updateSelectedFormat((format) => ({ ...format, bold: !format.bold }));
  }

  public toggleItalic(): Promise<void> {
    return this.updateSelectedFormat((format) => ({ ...format, italic: !format.italic }));
  }

  public toggleUnderline(): Promise<void> {
    return this.updateSelectedFormat((format) => ({ ...format, underline: !format.underline }));
  }

  public setHorizontalAlignment(alignment: SheetHorizontalAlignmentView): Promise<void> {
    return this.updateSelectedFormat((format) => ({
      ...format,
      horizontalAlignment: alignment,
    }));
  }

  public setDecimalPlaces(decimalPlaces: number | null): Promise<void> {
    return this.updateSelectedFormat((format) => ({
      ...format,
      decimalPlaces,
    }));
  }

  public async applyRowQuery(
    filterMode: SheetFilterModeView,
    filterValue: string,
    sortDirection: SheetSortDirectionView,
  ): Promise<void> {
    const generation = this.nextRowQueryGeneration();
    const selection = this.repository.selection();
    if (selection === null) {
      return;
    }
    if (filterMode === "none" && sortDirection === "none") {
      this.repository.clearRowQuery();
      return;
    }
    await this.runRowQuery(selection.column, filterMode, filterValue, sortDirection, generation);
  }

  public clearRowQuery(): void {
    this.invalidateRowQueryRequests();
    this.repository.clearRowQuery();
  }

  public commitCell(reference: string, rawValue: string): Promise<void> {
    return this.enqueue(async () => {
      const document = this.requireDocument();
      const worksheet = document.worksheets[0];
      if (worksheet === undefined) {
        this.repository.setError(ERROR_CODES.sheetWorksheetNotFound);
        return;
      }

      const input = {
        documentId: document.id,
        worksheetId: worksheet.id,
        reference,
      };
      const normalized = rawValue.trim();

      this.repository.setLoading();
      try {
        let updated: SheetDocumentView;
        if (normalized.length === 0) {
          updated = await this.sheetTool.clearCell(input);
        } else if (normalized.startsWith(SHEET_FORMULA_PREFIX)) {
          updated = await this.sheetTool.setFormula(input, normalized);
        } else if (normalized.toLowerCase() === SHEET_BOOLEAN_TRUE) {
          updated = await this.sheetTool.setBoolean(input, true);
        } else if (normalized.toLowerCase() === SHEET_BOOLEAN_FALSE) {
          updated = await this.sheetTool.setBoolean(input, false);
        } else if (SHEET_NUMBER_PATTERN.test(normalized)) {
          updated = await this.sheetTool.setNumber(input, Number(normalized));
        } else {
          updated = await this.sheetTool.setText(input, rawValue);
        }
        this.repository.setDocument(updated);
        this.repository.markDirty();
        const selection = this.repository.selection();
        if (selection !== null) {
          await this.selectCell(selection.reference, selection.row, selection.column);
        }
        await this.refreshRowQuery();
      } catch (error: unknown) {
        this.repository.setError(this.errorCode(error));
      }
    });
  }

  private async runRowQuery(
    column: number,
    filterMode: SheetFilterModeView,
    filterValue: string,
    sortDirection: SheetSortDirectionView,
    generation: number,
  ): Promise<void> {
    const document = this.requireDocument();
    const documentId = document.id;
    const worksheet = document.worksheets[0];
    if (worksheet === undefined) {
      this.repository.setError(ERROR_CODES.sheetWorksheetNotFound);
      return;
    }

    let condition: SheetFilterConditionView | null = null;
    switch (filterMode) {
      case "none":
        break;
      case "nonEmpty":
        condition = { kind: "nonEmpty" };
        break;
      case "textContains":
        condition = { kind: "textContains", value: filterValue };
        break;
      case "numberGreaterThan":
      case "numberLessThan": {
        const normalized = filterValue.trim();
        if (normalized.length === 0 || !SHEET_NUMBER_PATTERN.test(normalized)) {
          this.repository.setRowQueryError(ERROR_CODES.sheetInvalidFilter);
          return;
        }
        const value = Number(normalized);
        if (!Number.isFinite(value)) {
          this.repository.setRowQueryError(ERROR_CODES.sheetInvalidFilter);
          return;
        }
        condition =
          filterMode === "numberGreaterThan"
            ? { kind: "numberGreaterThan", value }
            : { kind: "numberLessThan", value };
        break;
      }
      case "booleanTrue":
        condition = { kind: "booleanEquals", value: true };
        break;
      case "booleanFalse":
        condition = { kind: "booleanEquals", value: false };
        break;
    }

    const request: SheetRowQueryRequestView = {
      documentId: document.id,
      worksheetId: worksheet.id,
      range: {
        startRow: 0,
        endRow: SHEET_GRID_ROW_COUNT - 1,
        startColumn: 0,
        endColumn: SHEET_GRID_COLUMN_COUNT - 1,
      },
      filter: condition === null ? null : { column, condition },
      sort:
        sortDirection === "none"
          ? null
          : {
              column,
              direction: sortDirection,
            },
    };

    try {
      const result = await this.sheetTool.queryRows(request);
      if (!this.isCurrentRowQueryRequest(generation, documentId)) {
        return;
      }
      this.repository.setRowQuery({
        column,
        filterMode,
        filterValue,
        sortDirection,
        rows: result.rows,
      });
    } catch (error: unknown) {
      if (this.isCurrentRowQueryRequest(generation, documentId)) {
        this.repository.setRowQueryError(this.errorCode(error));
      }
    }
  }

  private async refreshRowQuery(): Promise<void> {
    const query = this.repository.rowQuery();
    if (query === null) {
      return;
    }
    const generation = this.nextRowQueryGeneration();
    await this.runRowQuery(
      query.column,
      query.filterMode,
      query.filterValue,
      query.sortDirection,
      generation,
    );
  }

  private async refreshSelectionFormat(): Promise<void> {
    const selection = this.repository.selection();
    const document = this.repository.document();
    const worksheet = document?.worksheets[0];
    if (selection === null || document === null || worksheet === undefined) {
      return;
    }

    const input = {
      documentId: document.id,
      worksheetId: worksheet.id,
      reference: selection.reference,
    };
    try {
      const format = await this.sheetTool.getCellFormat(input);
      const current = this.repository.selection();
      if (current?.reference !== selection.reference) {
        return;
      }
      this.repository.setCellFormat(worksheet.id, selection.reference, format);
      this.repository.setSelection({
        ...current,
        format,
      });
    } catch (error: unknown) {
      this.repository.setError(this.errorCode(error));
    }
  }

  private updateSelectedFormat(
    transform: (format: SheetCellFormatView) => SheetCellFormatView,
  ): Promise<void> {
    return this.enqueue(async () => {
      const selection = this.repository.selection();
      const document = this.requireDocument();
      const worksheet = document.worksheets[0];
      if (selection === null || selection.format === null || worksheet === undefined) {
        return;
      }

      const input = {
        documentId: document.id,
        worksheetId: worksheet.id,
        reference: selection.reference,
      };
      const nextFormat = transform(selection.format);
      this.repository.setLoading();
      try {
        this.repository.setDocument(
          await this.sheetTool.setCellFormat(input, nextFormat),
        );
        this.repository.markDirty();
        this.repository.setCellFormat(worksheet.id, selection.reference, nextFormat);
        const current = this.repository.selection();
        if (current?.reference === selection.reference) {
          await this.selectCell(selection.reference, selection.row, selection.column);
        }
      } catch (error: unknown) {
        this.repository.setError(this.errorCode(error));
      }
    });
  }

  private async refreshSelectionEvaluation(): Promise<void> {
    const selection = this.repository.selection();
    const document = this.repository.document();
    const worksheet = document?.worksheets[0];
    if (selection === null || document === null || worksheet === undefined) {
      return;
    }

    const input = {
      documentId: document.id,
      worksheetId: worksheet.id,
      reference: selection.reference,
    };
    try {
      const evaluated = await this.sheetTool.getEvaluatedCell(input);
      const current = this.repository.selection();
      if (current?.reference !== selection.reference) {
        return;
      }
      this.repository.setSelection({
        ...current,
        evaluatedValue: evaluated?.value ?? null,
        evaluationErrorCode: null,
      });
    } catch (error: unknown) {
      if (this.repository.selection()?.reference === selection.reference) {
        this.repository.setSelectionEvaluationError(this.errorCode(error));
      }
    }
  }

  private valueText(value: SheetCellValueView): string {
    switch (value.kind) {
      case "text":
      case "formula":
        return value.value;
      case "number":
      case "boolean":
        return String(value.value);
    }
  }

  private confirmDiscardIfNeeded(): Promise<boolean> {
    if (!this.repository.dirty()) {
      return Promise.resolve(true);
    }
    return this.fileDialogTool.confirmDiscard(
      this.language.text("unsavedChangesMessage"),
      this.language.text("unsavedChangesTitle"),
    );
  }

  private nextRowQueryGeneration(): number {
    this.rowQueryGeneration += 1;
    return this.rowQueryGeneration;
  }

  private invalidateRowQueryRequests(): void {
    this.nextRowQueryGeneration();
  }

  private isCurrentRowQueryRequest(generation: number, documentId: string): boolean {
    return (
      generation === this.rowQueryGeneration &&
      this.repository.document()?.id === documentId
    );
  }

  private enqueue<T>(operation: () => Promise<T>): Promise<T> {
    const next = this.mutationQueue.then(operation, operation);
    this.mutationQueue = next.then(
      () => undefined,
      () => undefined,
    );
    return next;
  }

  private requireDocument(): SheetDocumentView {
    const document = this.repository.document();
    if (document === null) {
      throw new Error(ERROR_CODES.unknown);
    }
    return document;
  }

  private errorCode(error: unknown): string {
    if (typeof error === "object" && error !== null && "code" in error) {
      return String((error as SheetDesktopErrorView).code);
    }
    return ERROR_CODES.unknown;
  }
}
