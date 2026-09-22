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
