// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/sheet-functions.ts
// # 📌 Amac: Sheet Functions sidebar tarafinda kullanilan destekli fonksiyon katalogunu merkezi tutar
// # 📌 Modul - FileType: Config - TypeScript
// Version: 0.10.0
// Aciklama: Formula engine tarafinda desteklenen SUM, AVERAGE, MIN, MAX ve IF fonksiyonlarinin UI metadata ve taslaklarini tanimlar
// Bagimli Oldugu Katman: Config

export const SHEET_FUNCTION_BUTTON_TEXT = "fx";

export const SHEET_FUNCTION_IDS = {
  sum: "SUM",
  average: "AVERAGE",
  min: "MIN",
  max: "MAX",
  if: "IF",
} as const;

export type SheetFunctionId =
  (typeof SHEET_FUNCTION_IDS)[keyof typeof SHEET_FUNCTION_IDS];

export const SHEET_FUNCTION_CATALOG = [
  {
    id: SHEET_FUNCTION_IDS.sum,
    aliases: [],
    labelKey: "sheetFunctionSum",
    descriptionKey: "sheetFunctionSumDescription",
    signature: "SUM(number1, number2, ...)",
    draft: "=SUM(",
  },
  {
    id: SHEET_FUNCTION_IDS.average,
    aliases: ["AVG"],
    labelKey: "sheetFunctionAverage",
    descriptionKey: "sheetFunctionAverageDescription",
    signature: "AVERAGE(number1, number2, ...)",
    draft: "=AVERAGE(",
  },
  {
    id: SHEET_FUNCTION_IDS.min,
    aliases: [],
    labelKey: "sheetFunctionMin",
    descriptionKey: "sheetFunctionMinDescription",
    signature: "MIN(number1, number2, ...)",
    draft: "=MIN(",
  },
  {
    id: SHEET_FUNCTION_IDS.max,
    aliases: [],
    labelKey: "sheetFunctionMax",
    descriptionKey: "sheetFunctionMaxDescription",
    signature: "MAX(number1, number2, ...)",
    draft: "=MAX(",
  },
  {
    id: SHEET_FUNCTION_IDS.if,
    aliases: [],
    labelKey: "sheetFunctionIf",
    descriptionKey: "sheetFunctionIfDescription",
    signature: "IF(condition, true_value, false_value)",
    draft: "=IF(",
  },
] as const;

