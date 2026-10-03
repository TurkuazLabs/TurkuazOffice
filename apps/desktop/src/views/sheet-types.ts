// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-types.ts
// # 📌 Amac: Tauri backend tarafindan gelen Sheet read-only DTO kontratlarini typed tanimlar
// # 📌 Modul - FileType: View - TypeScript
// Version: 0.4.0
// Aciklama: Sheet belge, worksheet, cell ve stabil desktop error read-model tiplerini tasir
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
