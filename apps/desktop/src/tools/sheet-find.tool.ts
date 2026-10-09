// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/sheet-find.tool.ts
// # 📌 Amac: Sheet aktif worksheet read-modelindeki ekranda erisilebilir hucreleri arar
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.1.0
// Aciklama: Sparse canonical cell degerlerinde read-only locale-aware Bul; filtrelenmis/gorunmeyen hucreleri sonuca eklemez
// Bagimli Oldugu Katman: Tool -> Config -> View Model

import {
  SHEET_GRID_COLUMN_COUNT,
  SHEET_GRID_ROW_COUNT,
} from "../config/sheet";
import type { SheetCellValueView, SheetDocumentView } from "../views/sheet-types";

export interface SheetFindMatch {
  readonly row: number;
  readonly column: number;
}

export const SHEET_FIND_MAX_QUERY_SCALARS = 128;
export const SHEET_FIND_MAX_RESULTS = 1000;

function rawCellText(value: SheetCellValueView): string {
  switch (value.kind) {
    case "text":
    case "formula":
      return value.value;
    case "number":
    case "boolean":
      return String(value.value);
  }
}

export function findSheetMatches(
  document: SheetDocumentView,
  query: string,
  locale: string,
  visibleRows: readonly number[],
): readonly SheetFindMatch[] {
  const length = Array.from(query).length;
  if (length === 0 || length > SHEET_FIND_MAX_QUERY_SCALARS) {
    return [];
  }

  const worksheet = document.worksheets[0];
  if (worksheet === undefined) {
    return [];
  }
  const rows = new Set(visibleRows);
  const needle = query.toLocaleLowerCase(locale);
  return worksheet.cells
    .filter(
      (cell) =>
        cell.row >= 0 &&
        cell.row < SHEET_GRID_ROW_COUNT &&
        cell.column >= 0 &&
        cell.column < SHEET_GRID_COLUMN_COUNT &&
        rows.has(cell.row) &&
        rawCellText(cell.value).toLocaleLowerCase(locale).includes(needle),
    )
    .sort((left, right) => left.row - right.row || left.column - right.column)
    .slice(0, SHEET_FIND_MAX_RESULTS)
    .map((cell) => ({ row: cell.row, column: cell.column }));
}
