// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/keyboard.ts
// # 📌 Amac: Desktop keyboard event key degerlerini merkezi sabit olarak tanimlar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.4.1
// # Aciklama: Writer shortcut ve caret navigation magic string kullanimini engeller
// Bagimli Oldugu Katman: Config

export const KEYBOARD_KEYS = {
  enter: "Enter",
  backspace: "Backspace",
  arrowLeft: "ArrowLeft",
  arrowRight: "ArrowRight",
  arrowUp: "ArrowUp",
  arrowDown: "ArrowDown",
  home: "Home",
  end: "End",
  pageUp: "PageUp",
  pageDown: "PageDown",
  z: "z",
  y: "y",
  n: "n",
  o: "o",
  s: "s",
  b: "b",
  i: "i",
  u: "u",
  p: "p",
  l: "l",
  e: "e",
  r: "r",
  j: "j",
  one: "1",
  f2: "f2",
  tab: "Tab",
  escape: "escape",
  plus: "+",
  greaterThan: ">",
  lessThan: "<",
  equal: "=",
  minus: "-",
  zero: "0",
} as const;

export const KEYBOARD_MODIFIER_STATES = {
  altGraph: "AltGraph",
} as const;

export const CARET_NAVIGATION_KEYS: readonly string[] = [
  KEYBOARD_KEYS.arrowLeft,
  KEYBOARD_KEYS.arrowRight,
  KEYBOARD_KEYS.arrowUp,
  KEYBOARD_KEYS.arrowDown,
  KEYBOARD_KEYS.home,
  KEYBOARD_KEYS.end,
  KEYBOARD_KEYS.pageUp,
  KEYBOARD_KEYS.pageDown,
];


export const WRITER_SHORTCUT_ACTIONS = {
  closePrintPreview: "close-print-preview",
  print: "print",
  zoomIn: "zoom-in",
  zoomOut: "zoom-out",
  zoomReset: "zoom-reset",
  undo: "undo",
  redo: "redo",
  newDocument: "new-document",
  open: "open",
  printPreview: "print-preview",
  saveAs: "save-as",
  save: "save",
  bold: "bold",
  italic: "italic",
  underline: "underline",
  alignLeft: "align-left",
  alignCenter: "align-center",
  alignRight: "align-right",
  alignJustify: "align-justify",
  increaseFontSize: "increase-font-size",
  decreaseFontSize: "decrease-font-size",
} as const;

export type WriterShortcutAction =
  (typeof WRITER_SHORTCUT_ACTIONS)[keyof typeof WRITER_SHORTCUT_ACTIONS];

export const WRITER_ARIA_SHORTCUTS = {
  newDocument: "Control+N Meta+N",
  open: "Control+O Meta+O",
  save: "Control+S Meta+S",
  saveAs: "Control+Shift+S Meta+Shift+S",
  print: "Control+P Meta+P",
  printPreview: "Control+Shift+P Meta+Shift+P",
  undo: "Control+Z Meta+Z",
  redo: "Control+Y Meta+Y Control+Shift+Z Meta+Shift+Z",
  bold: "Control+B Meta+B",
  italic: "Control+I Meta+I",
  underline: "Control+U Meta+U",
  alignLeft: "Control+L Meta+L",
  alignCenter: "Control+E Meta+E",
  alignRight: "Control+R Meta+R",
  alignJustify: "Control+J Meta+J",
  increaseFontSize: "Control+Shift+> Meta+Shift+>",
  decreaseFontSize: "Control+Shift+< Meta+Shift+<",
  zoomIn: "Control++ Meta++ Control+= Meta+=",
  zoomOut: "Control+- Meta+-",
  zoomReset: "Control+0 Meta+0",
} as const;


export const WRITER_SHORTCUT_HINTS = {
  newDocument: "Ctrl+N",
  open: "Ctrl+O",
  save: "Ctrl+S",
  saveAs: "Ctrl+Shift+S",
  print: "Ctrl+P",
  printPreview: "Ctrl+Shift+P",
  undo: "Ctrl+Z",
  redo: "Ctrl+Y / Ctrl+Shift+Z",
  bold: "Ctrl+B",
  italic: "Ctrl+I",
  underline: "Ctrl+U",
  alignLeft: "Ctrl+L",
  alignCenter: "Ctrl+E",
  alignRight: "Ctrl+R",
  alignJustify: "Ctrl+J",
  increaseFontSize: "Ctrl+Shift+>",
  decreaseFontSize: "Ctrl+Shift+<",
  zoomIn: "Ctrl++",
  zoomOut: "Ctrl+-",
  zoomReset: "Ctrl+0",
} as const;


export const SHEET_SHORTCUT_ACTIONS = {
  newDocument: "new-document",
  bold: "bold",
  italic: "italic",
  underline: "underline",
  editCell: "edit-cell",
  cancelEdit: "cancel-edit",
  firstCell: "first-cell",
  toggleProperties: "toggle-properties",
  toggleQuery: "toggle-query",
  insertSum: "insert-sum",
} as const;

export type SheetShortcutAction =
  (typeof SHEET_SHORTCUT_ACTIONS)[keyof typeof SHEET_SHORTCUT_ACTIONS];

export const SHEET_ARIA_SHORTCUTS = {
  newDocument: "Control+N Meta+N",
  bold: "Control+B Meta+B",
  italic: "Control+I Meta+I",
  underline: "Control+U Meta+U",
  editCell: "F2",
  cancelEdit: "Escape",
  firstCell: "Control+Home Meta+Home",
  toggleProperties: "Control+1 Meta+1",
  toggleQuery: "Control+Shift+L Meta+Shift+L",
  insertSum: "Alt+=",
  nextRow: "Enter",
  previousRow: "Shift+Enter",
  nextCell: "Tab",
  previousCell: "Shift+Tab",
} as const;

export const SHEET_SHORTCUT_HINTS = {
  newDocument: "Ctrl+N",
  bold: "Ctrl+B",
  italic: "Ctrl+I",
  underline: "Ctrl+U",
  editCell: "F2",
  cancelEdit: "Esc",
  firstCell: "Ctrl+Home",
  toggleProperties: "Ctrl+1",
  toggleQuery: "Ctrl+Shift+L",
  insertSum: "Alt+=",
  nextRow: "Enter",
  previousRow: "Shift+Enter",
  nextCell: "Tab",
  previousCell: "Shift+Tab",
} as const;
