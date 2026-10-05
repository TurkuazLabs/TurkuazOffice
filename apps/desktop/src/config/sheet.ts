// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/sheet.ts
// # 📌 Amac: Desktop Sheet grid ve input davranis sabitlerini merkezi tutar
// # 📌 Modul - FileType: Config - TypeScript
// Version: 0.5.2
// Aciklama: Sheet A-Z viewport, freeze-pane geometrisi, number-format ve typed input tanima kontratini merkezi tanimlar
// Bagimli Oldugu Katman: Config

export const SHEET_GRID_ROW_COUNT = 100;
export const SHEET_GRID_COLUMN_COUNT = 26;
export const SHEET_MAX_DECIMAL_PLACES = 12;
export const SHEET_DECIMAL_GENERAL_VALUE = "general";
export const SHEET_BOOLEAN_TRUE = "true";
export const SHEET_BOOLEAN_FALSE = "false";
export const SHEET_FORMULA_PREFIX = "=";
export const SHEET_NUMBER_PATTERN = /^[+-]?(?:\d+(?:\.\d*)?|\.\d+)$/;

export const SHEET_GRID_ROW_HEIGHT_PX = 24;
export const SHEET_GRID_COLUMN_WIDTH_PX = 88;
export const SHEET_GRID_ROW_HEADER_WIDTH_PX = 36;
export const SHEET_GRID_COLUMN_HEADER_HEIGHT_PX = 24;
