// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/ribbon.ts
// # 📌 Amac: Writer menu ve paragraph alignment command metadata'sini merkezi config olarak tanimlar
// # 📌 Modul - FileType: Config - TypeScript
// Version: 0.12.0
// Aciklama: Writer ust menu siralamasini klasik kelime islemci modeline tasir ve alignment command metadata'sini merkezi tutar
// Bagimli Oldugu Katman: Config

export const WRITER_RIBBON_TABS = [
  { id: "file", languageKey: "menuFile", active: false, enabled: false },
  { id: "edit", languageKey: "menuEdit", active: false, enabled: false },
  { id: "view", languageKey: "menuView", active: false, enabled: false },
  { id: "insert", languageKey: "menuInsert", active: false, enabled: false },
  { id: "format", languageKey: "menuFormat", active: false, enabled: false },
  { id: "table", languageKey: "menuTable", active: false, enabled: false },
  { id: "tools", languageKey: "menuTools", active: false, enabled: false },
  { id: "help", languageKey: "menuHelp", active: false, enabled: false },
] as const;

export type WriterRibbonTabConfig = (typeof WRITER_RIBBON_TABS)[number];

export const WRITER_ALIGNMENT_COMMANDS = [
  { alignment: "left", languageKey: "alignLeft", shortLanguageKey: "alignLeftShort" },
  { alignment: "center", languageKey: "alignCenter", shortLanguageKey: "alignCenterShort" },
  { alignment: "right", languageKey: "alignRight", shortLanguageKey: "alignRightShort" },
  { alignment: "justify", languageKey: "alignJustify", shortLanguageKey: "alignJustifyShort" },
] as const;

export type WriterAlignmentCommandConfig = (typeof WRITER_ALIGNMENT_COMMANDS)[number];
