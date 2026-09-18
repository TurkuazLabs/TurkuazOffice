// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/layout.ts
// # 📌 Amac: Writer sayfa olcumu, zoom ve font fallback kontratlarini merkezi configte tutar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.2.0
// # Aciklama: Twip/CSS donusumu, zoom sinirlari ve platform font fallback profillerini tanimlar
// Bagimli Oldugu Katman: Config

export const TWIPS_PER_INCH = 1440;
export const CSS_REFERENCE_PIXELS_PER_INCH = 96;

export const WRITER_DEFAULT_ZOOM_PERCENT = 100;
export const WRITER_MIN_ZOOM_PERCENT = 50;
export const WRITER_MAX_ZOOM_PERCENT = 200;
export const WRITER_ZOOM_STEP_PERCENT = 10;

export const WRITER_GENERIC_FONT_FAMILY = "sans-serif";

export interface WriterFontFallbackProfile {
  readonly candidates: readonly string[];
  readonly genericFamily: "sans-serif" | "serif" | "monospace";
}

export const WRITER_FONT_FALLBACK_PROFILES: Readonly<Record<string, WriterFontFallbackProfile>> = {
  Arial: {
    candidates: ["Arial", "Liberation Sans", "Nimbus Sans"],
    genericFamily: "sans-serif",
  },
  Calibri: {
    candidates: ["Calibri", "Carlito", "Arial", "Liberation Sans"],
    genericFamily: "sans-serif",
  },
  "Times New Roman": {
    candidates: ["Times New Roman", "Liberation Serif", "Nimbus Roman"],
    genericFamily: "serif",
  },
  Georgia: {
    candidates: ["Georgia", "Liberation Serif", "Nimbus Roman"],
    genericFamily: "serif",
  },
  Verdana: {
    candidates: ["Verdana", "DejaVu Sans", "Liberation Sans"],
    genericFamily: "sans-serif",
  },
  "Courier New": {
    candidates: ["Courier New", "Liberation Mono", "Nimbus Mono PS"],
    genericFamily: "monospace",
  },
};
