// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/shortcut-focus.tool.ts
// # 📌 Amac: Pencere seviyesindeki Writer/Sheet kisayollarinin baska metin alanlarina karismasini engeller
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.1.0
// Aciklama: Sadece gercek editor focus'unu suite action'ina ayirir; browser input/textarea/select ve yabanci contenteditable duzenlemesini korur
// Bagimli Oldugu Katman: Tool -> Config

export function shortcutBelongsToOtherTextControl(
  target: EventTarget | null,
  activeEditorSelector: string,
): boolean {
  if (!(target instanceof Element)) {
    return false;
  }

  // The real Writer paragraph or Sheet grid cell keeps its shortcuts,
  // even though its DOM surface is itself a text-editing control.
  if (target.closest(activeEditorSelector) !== null) {
    return false;
  }

  // Let regular inputs (including the Sheet formula bar), textareas,
  // selects and unrelated rich-text editors handle their own keystrokes.
  return target.closest(
    "input, textarea, select, [contenteditable='true'], [role='textbox']",
  ) !== null;
}
