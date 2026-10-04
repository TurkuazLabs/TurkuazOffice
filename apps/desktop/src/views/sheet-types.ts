// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-types.ts
// # 📌 Amac: Tauri backend tarafindan gelen Sheet read-only DTO kontratlarini typed tanimlar
// # 📌 Modul - FileType: View - TypeScript
// Version: 0.4.0
// Aciklama: Sheet belge, cell/format, selection, row-query ve stabil desktop error read-model tiplerini tasir
// Bagimli Oldugu Katman: View

export type SheetCellValueView =
  | { readonly kind: "text"; readonly value: string }
  | { readonly kind: "number"; readonly value: number }
  | { readonly kind: "boolean"; readonly value: boolean }
  | { readonly kind: "formula"; readonly value: string };

export interface SheetCellView {
  readonly row: number;
  readonly column: number;
  readonly value: SheetCellValueView;
}

export interface SheetWorksheetView {
  readonly id: string;
  readonly name: string;
  readonly cellCount: number;
  readonly cells: readonly SheetCellView[];
}

export type SheetHorizontalAlignmentView = "general" | "left" | "center" | "right";

export interface SheetCellFormatView {
  readonly bold: boolean;
  readonly italic: boolean;
  readonly underline: boolean;
  readonly horizontalAlignment: SheetHorizontalAlignmentView;
  readonly decimalPlaces: number | null;
}

export type SheetFilterModeView =
  | "none"
  | "nonEmpty"
  | "textContains"
  | "numberGreaterThan"
  | "numberLessThan"
  | "booleanTrue"
  | "booleanFalse";

export type SheetSortDirectionView = "none" | "ascending" | "descending";

export type SheetFilterConditionView =
  | { readonly kind: "nonEmpty" }
  | { readonly kind: "textContains"; readonly value: string }
  | { readonly kind: "numberGreaterThan"; readonly value: number }
  | { readonly kind: "numberLessThan"; readonly value: number }
  | { readonly kind: "booleanEquals"; readonly value: boolean };

export interface SheetRowQueryRequestView {
  readonly documentId: string;
  readonly worksheetId: string;
  readonly range: {
    readonly startRow: number;
    readonly endRow: number;
    readonly startColumn: number;
    readonly endColumn: number;
  };
  readonly filter:
    | {
        readonly column: number;
        readonly condition: SheetFilterConditionView;
      }
    | null;
  readonly sort:
    | {
        readonly column: number;
        readonly direction: Exclude<SheetSortDirectionView, "none">;
      }
    | null;
}

export interface SheetRowQueryResultView {
  readonly rows: readonly number[];
}

export interface SheetRowQueryStateView {
  readonly column: number;
  readonly filterMode: SheetFilterModeView;
  readonly filterValue: string;
  readonly sortDirection: SheetSortDirectionView;
  readonly rows: readonly number[];
}

export interface SheetSelectionView {
  readonly reference: string;
  readonly row: number;
  readonly column: number;
  readonly rawValue: string;
  readonly evaluatedValue: SheetCellValueView | null;
  readonly evaluationErrorCode: string | null;
  readonly format: SheetCellFormatView | null;
}

export interface SheetDocumentView {
  readonly id: string;
  readonly title: string;
  readonly revision: number;
  readonly worksheets: readonly SheetWorksheetView[];
  readonly chartCount: number;
}

export interface SheetDesktopErrorView {
  readonly code: string;
}
