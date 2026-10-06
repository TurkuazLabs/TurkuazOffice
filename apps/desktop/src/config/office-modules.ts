// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/office-modules.ts
// # 📌 Amac: Desktop uygulama modul secim degerlerini merkezi kontrat olarak tanimlar
// # 📌 Modul - FileType: Config - TypeScript
// Version: 0.12.0
// Aciklama: Baslangic Merkezi, Writer ve Sheet modul anahtarlarinda magic string kullanimini engeller
// Bagimli Oldugu Katman: Config

export const OFFICE_MODULES = {
  home: "home",
  writer: "writer",
  sheet: "sheet",
} as const;

export type OfficeModule = (typeof OFFICE_MODULES)[keyof typeof OFFICE_MODULES];
