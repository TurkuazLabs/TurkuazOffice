// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/error-codes.ts
// # 📌 Amac: Desktop frontend stabil error code sabitlerini merkezi tutar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.2.0
// # Aciklama: Service ve View katmanlarinda magic error string kullanilmasini engeller
// Bagimli Oldugu Katman: Config

export const ERROR_CODES = {
  unknown: "desktop.unknown_error",
  appRootMissing: "desktop.app_root_missing",
  printUnavailable: "desktop.print_unavailable",
} as const;
