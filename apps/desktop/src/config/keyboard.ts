// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/keyboard.ts
// # 📌 Amac: Desktop keyboard event key degerlerini merkezi sabit olarak tanimlar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.2.0
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
  escape: "escape",
  plus: "+",
  equal: "=",
  minus: "-",
  zero: "0",
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
  zoomIn: "Ctrl++",
  zoomOut: "Ctrl+-",
  zoomReset: "Ctrl+0",
} as const;
