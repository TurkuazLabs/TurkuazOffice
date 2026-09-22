// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/print.tool.ts
// # 📌 Amac: Writer canonical page geometrysini sistem yazdirma dialoguna aktarir
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.2.0
// # Aciklama: Dinamik @page fiziksel boyut kuralini kurar ve WebView browser print capability'sini cagirir
// Bagimli Oldugu Katman: Tool -> Config

import { ERROR_CODES } from "../config/error-codes";
import { TWIPS_PER_INCH } from "../config/layout";
import { WRITER_PRINT_STYLE_ELEMENT_ID } from "../config/print";
import type { WriterPageSettingsView } from "../views/writer-types";

export class PrintTool {
  public print(pageSettings: WriterPageSettingsView): void {
    if (typeof window.print !== "function") {
      throw this.unavailableError();
    }

    const existing = document.getElementById(WRITER_PRINT_STYLE_ELEMENT_ID);
    existing?.remove();

    const style = document.createElement("style");
    style.id = WRITER_PRINT_STYLE_ELEMENT_ID;
    style.textContent = this.pageRule(pageSettings);
    document.head.append(style);

    try {
      window.print();
    } catch {
      throw this.unavailableError();
    } finally {
      style.remove();
    }
  }

  public pageRule(pageSettings: WriterPageSettingsView): string {
    const widthInches = pageSettings.widthTwips / TWIPS_PER_INCH;
    const heightInches = pageSettings.heightTwips / TWIPS_PER_INCH;
    return `@page { size: ${widthInches}in ${heightInches}in; margin: 0; }`;
  }

  private unavailableError(): { readonly code: string } {
    return { code: ERROR_CODES.printUnavailable };
  }
}
