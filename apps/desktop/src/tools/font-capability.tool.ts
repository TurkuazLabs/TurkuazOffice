// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/font-capability.tool.ts
// # 📌 Amac: Desktop webview ortaminda logical font family kullanilabilirligini sorgular
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.2.0
// # Aciklama: document.fonts API'sini Service katmanina izole ederek platform font capability adaptorunu saglar
// Bagimli Oldugu Katman: Tool

export class FontCapabilityTool {
  public isAvailable(fontFamily: string): boolean {
    if (typeof document === "undefined" || document.fonts === undefined) {
      return false;
    }
    try {
      return document.fonts.check(`12px ${this.quote(fontFamily)}`);
    } catch {
      return false;
    }
  }

  private quote(fontFamily: string): string {
    const escaped = fontFamily.replaceAll("\\", "\\\\").replaceAll('"', '\\"');
    return `"${escaped}"`;
  }
}
