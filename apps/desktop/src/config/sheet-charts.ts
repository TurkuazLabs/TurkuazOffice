// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/sheet-charts.ts
// # 📌 Amac: Sheet Basic Charts Desktop UI tip, varsayilan ve SVG geometri sabitlerini merkezi tutar
// # 📌 Modul - FileType: Config - TypeScript
// Version: 0.11.0
// Aciklama: Bar/Line/Pie chart tipi, secim kontrati ve dependency-free SVG renderer olculerini magic string/sayi dagilimini engelleyecek sekilde tanimlar
// Bagimli Oldugu Katman: Config

export const SHEET_CHART_TYPES = ["bar", "line", "pie"] as const;
export type SheetChartType = (typeof SHEET_CHART_TYPES)[number];

export const SHEET_CHART_DEFAULT_TYPE: SheetChartType = "bar";
export const SHEET_CHART_LABEL_KEYS: Readonly<
  Record<SheetChartType, "sheetChartBar" | "sheetChartLine" | "sheetChartPie">
> = {
  bar: "sheetChartBar",
  line: "sheetChartLine",
  pie: "sheetChartPie",
};
export const SHEET_CHART_REQUIRED_COLUMN_COUNT = 2;

export const SHEET_CHART_VIEW_WIDTH = 280;
export const SHEET_CHART_VIEW_HEIGHT = 180;
export const SHEET_CHART_PLOT_PADDING = 20;
export const SHEET_CHART_BAR_GAP = 6;
export const SHEET_CHART_POINT_RADIUS = 3;
export const SHEET_CHART_PIE_RADIUS = 64;
export const SHEET_CHART_PIE_CENTER_X = SHEET_CHART_VIEW_WIDTH / 2;
export const SHEET_CHART_PIE_CENTER_Y = SHEET_CHART_VIEW_HEIGHT / 2;
export const SHEET_CHART_FULL_CIRCLE_RADIANS = Math.PI * 2;
export const SHEET_CHART_EMPTY_VALUE = 0;
export const SHEET_CHART_SLICE_STYLE_COUNT = 5;
