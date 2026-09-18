// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/ribbon.ts
// # 📌 Amac: Writer ribbon tab ve paragraph alignment command metadata'sini merkezi config olarak tanimlar
// # 📌 Modul - FileType: Config - TypeScript
// Version: 0.2.0
// Aciklama: View icindeki inline ribbon command listelerini ve magic key degerlerini merkezi konfigurasyona tasir
// Bagimli Oldugu Katman: Config

export const WRITER_RIBBON_TABS = [
  { id: "file", languageKey: "menuFile", active: false, enabled: false },
  { id: "home", languageKey: "menuHome", active: true, enabled: true },
  { id: "insert", languageKey: "menuInsert", active: false, enabled: false },
  { id: "view", languageKey: "menuView", active: false, enabled: false },
] as const;

export type WriterRibbonTabConfig = (typeof WRITER_RIBBON_TABS)[number];

export const WRITER_ALIGNMENT_COMMANDS = [
  { alignment: "left", languageKey: "alignLeft", shortLanguageKey: "alignLeftShort" },
  { alignment: "center", languageKey: "alignCenter", shortLanguageKey: "alignCenterShort" },
  { alignment: "right", languageKey: "alignRight", shortLanguageKey: "alignRightShort" },
  { alignment: "justify", languageKey: "alignJustify", shortLanguageKey: "alignJustifyShort" },
] as const;

export type WriterAlignmentCommandConfig = (typeof WRITER_ALIGNMENT_COMMANDS)[number];
