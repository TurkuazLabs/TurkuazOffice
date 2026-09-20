// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/typography.ts
// # 📌 Amac: Writer typography UI seceneklerini merkezi config kontratinda tutar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.2.0
// # Aciklama: Font ailesi ve punto listelerini View icindeki magic degerlerden ayirir
// Bagimli Oldugu Katman: Config

export const WRITER_FONT_FAMILIES = [
  "Arial",
  "Calibri",
  "Times New Roman",
  "Georgia",
  "Verdana",
  "Courier New",
] as const;

export const WRITER_FONT_SIZES_POINTS = [
  8, 9, 10, 11, 12, 14, 16, 18, 20, 24, 28, 32, 36, 48, 72,
] as const;

export const DEFAULT_WRITER_FONT_FAMILY = "Arial";
export const DEFAULT_WRITER_FONT_SIZE_HALF_POINTS = 22;
export const MIN_WRITER_FONT_SIZE_HALF_POINTS = 12;
export const MAX_WRITER_FONT_SIZE_HALF_POINTS = 192;
export const MAX_WRITER_FONT_FAMILY_LENGTH = 128;
