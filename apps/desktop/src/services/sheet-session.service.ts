// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/sheet-session.service.ts
// # 📌 Amac: Desktop Sheet oturum ve cell edit is akisini koordine eder
// # 📌 Modul - FileType: Service - TypeScript
// Version: 0.4.0
// Aciklama: Repo ve Tauri Tool uzerinden yeni Sheet, typed cell input ve hata akisini business kurallariyla yonetir
// Bagimli Oldugu Katman: Service -> Repo -> Tool

import { ERROR_CODES } from "../config/error-codes";
import {
  SHEET_BOOLEAN_FALSE,
  SHEET_BOOLEAN_TRUE,
  SHEET_FORMULA_PREFIX,
  SHEET_NUMBER_PATTERN,
} from "../config/sheet";
import type { SheetSessionRepository } from "../repositories/sheet-session.repository";
import type { TauriSheetTool } from "../tools/tauri-sheet.tool";
import type { SheetDesktopErrorView, SheetDocumentView } from "../views/sheet-types";

export class SheetSessionService {
  private mutationQueue: Promise<void> = Promise.resolve();

  public constructor(
    private readonly repository: SheetSessionRepository,
    private readonly sheetTool: TauriSheetTool,
  ) {}

  public async initializeSession(): Promise<void> {
    if (this.repository.document() !== null) {
      return;
    }
    await this.createDocument();
  }

  public async createDocument(): Promise<void> {
    await this.enqueue(async () => {
      this.repository.setLoading();
      try {
        this.repository.setDocument(await this.sheetTool.createDocument());
      } catch (error: unknown) {
        this.repository.setError(this.errorCode(error));
      }
    });
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
      } catch (error: unknown) {
        this.repository.setError(this.errorCode(error));
      }
    });
  }

  private enqueue(operation: () => Promise<void>): Promise<void> {
    const next = this.mutationQueue.then(operation, operation);
    this.mutationQueue = next.catch(() => undefined);
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
