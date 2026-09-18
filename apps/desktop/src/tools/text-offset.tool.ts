// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/text-offset.tool.ts
// # 📌 Amac: Browser UTF-16 offsetleri ile Writer logical Unicode scalar offsetleri arasinda ceviri yapar
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.2.0
// # Aciklama: Selection, IME ve rich-text edit akisini browser string indeks semantiginden ayirir
// Bagimli Oldugu Katman: Tool

export class TextOffsetTool {
  public utf16ToLogical(text: string, utf16Offset: number): number {
    return Array.from(text.slice(0, utf16Offset)).length;
  }

  public logicalToUtf16(text: string, logicalOffset: number): number {
    return Array.from(text).slice(0, logicalOffset).join("").length;
  }

  public logicalLength(text: string): number {
    return Array.from(text).length;
  }

  public removeLogicalRange(text: string, startOffset: number, endOffset: number): string {
    const chars = Array.from(text);
    const start = Math.min(startOffset, endOffset);
    const end = Math.max(startOffset, endOffset);
    return chars.slice(0, start).concat(chars.slice(end)).join("");
  }
}
