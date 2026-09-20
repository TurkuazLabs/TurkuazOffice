// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/clipboard.tool.ts
// # 📌 Amac: Browser ClipboardEvent ve DataTransfer API detaylarini Service katmanindan izole eder
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.2.0
// # Aciklama: Internal MIME, HTML ve plain-text representation okuma/yazma ile default event tuketimini yapar
// Bagimli Oldugu Katman: Tool -> Config -> Model

import { WRITER_CLIPBOARD_MIME } from "../config/clipboard";
import type { ClipboardTransferModel, ClipboardWriteModel } from "../models/clipboard-model";

export class ClipboardTool {
  public read(event: ClipboardEvent): ClipboardTransferModel | null {
    const clipboardData = event.clipboardData;
    if (clipboardData === null) {
      return null;
    }
    return {
      internalFragment: this.readType(clipboardData, WRITER_CLIPBOARD_MIME.internal),
      html: this.readType(clipboardData, WRITER_CLIPBOARD_MIME.html),
      plainText: this.readType(clipboardData, WRITER_CLIPBOARD_MIME.plainText),
    };
  }

  public write(event: ClipboardEvent, payload: ClipboardWriteModel): boolean {
    const clipboardData = event.clipboardData;
    if (clipboardData === null) {
      return false;
    }

    let wrote = false;
    wrote = this.writeType(clipboardData, WRITER_CLIPBOARD_MIME.plainText, payload.plainText) || wrote;
    wrote = this.writeType(clipboardData, WRITER_CLIPBOARD_MIME.html, payload.html) || wrote;
    wrote =
      this.writeType(clipboardData, WRITER_CLIPBOARD_MIME.internal, payload.internalFragment) || wrote;

    if (wrote) {
      event.preventDefault();
    }
    return wrote;
  }

  public consume(event: ClipboardEvent): void {
    event.preventDefault();
  }

  private readType(clipboardData: DataTransfer, mime: string): string {
    try {
      return clipboardData.getData(mime);
    } catch {
      return "";
    }
  }

  private writeType(clipboardData: DataTransfer, mime: string, value: string): boolean {
    try {
      clipboardData.setData(mime, value);
      return true;
    } catch {
      return false;
    }
  }
}
