// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/error-codes.ts
// # 📌 Amac: Desktop frontend stabil error code sabitlerini merkezi tutar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.6.0
// # Aciklama: Writer ve Sheet Service/View katmanlarinda magic error string kullanilmasini engeller
// Bagimli Oldugu Katman: Config

export const ERROR_CODES = {
  unknown: "desktop.unknown_error",
  appRootMissing: "desktop.app_root_missing",
  printUnavailable: "desktop.print_unavailable",
  sheetWorksheetNotFound: "sheet.worksheet_not_found",
  sheetInvalidFilter: "sheet.invalid_filter",
  sheetTableNotFound: "sheet.table_not_found",
  sheetInvalidTableRange: "sheet.invalid_table_range",
  sheetTableRangeOverlap: "sheet.table_range_overlap",
} as const;
