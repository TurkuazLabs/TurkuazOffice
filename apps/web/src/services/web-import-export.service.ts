// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/services/web-import-export.service.ts
// # 📌 Amac: M3 Web browser import/export byte transport is kurallarini yonetir
// # 📌 Modul - FileType: Service - TypeScript
// Version: 0.4.0
// Aciklama: TKO file selection/download profilini yonetir; format decode/encode isini Tool katmaninda taklit etmez
// Bagimli Oldugu Katman: Service -> Tool -> Config

import {
  WEB_BROWSER_IMPORT_MAX_BYTES,
  WEB_NATIVE_DOCUMENT_ACCEPT,
  WEB_IMPORT_FILE_TOO_LARGE_ERROR,
  WEB_NATIVE_DOCUMENT_EXTENSION,
  WEB_NATIVE_DOCUMENT_MIME_TYPE,
  WEB_UNSUPPORTED_IMPORT_FILE_ERROR,
} from "../config/runtime-config";
import type {
  WebDownloadFile,
  WebImportExportCapabilities,
  WebPickedFile,
} from "../models/web-models";
import type { WebBrowserFileTransferTool } from "../tools/browser-file-transfer.tool";

export class WebImportExportService {
  public constructor(private readonly fileTransferTool: WebBrowserFileTransferTool) {}

  public capabilities(): WebImportExportCapabilities {
    return {
      browserFileTransferAvailable: true,
      nativeTkoCodecAvailable: false,
    };
  }

  public async pickNativeDocumentBytes(): Promise<WebPickedFile | null> {
    const picked = await this.fileTransferTool.pickFile({
      accept: WEB_NATIVE_DOCUMENT_ACCEPT,
      maxBytes: WEB_BROWSER_IMPORT_MAX_BYTES,
    });
    if (picked === null) {
      return null;
    }

    if (picked.bytes.byteLength > WEB_BROWSER_IMPORT_MAX_BYTES) {
      throw new Error(WEB_IMPORT_FILE_TOO_LARGE_ERROR);
    }
    if (!picked.name.toLocaleLowerCase("en-US").endsWith(WEB_NATIVE_DOCUMENT_EXTENSION)) {
      throw new Error(WEB_UNSUPPORTED_IMPORT_FILE_ERROR);
    }
    return picked;
  }

  public downloadNativeDocumentBytes(fileName: string, bytes: Uint8Array): void {
    const normalizedName = fileName
      .toLocaleLowerCase("en-US")
      .endsWith(WEB_NATIVE_DOCUMENT_EXTENSION)
      ? fileName
      : `${fileName}${WEB_NATIVE_DOCUMENT_EXTENSION}`;

    const file: WebDownloadFile = {
      fileName: normalizedName,
      mediaType: WEB_NATIVE_DOCUMENT_MIME_TYPE,
      bytes,
    };
    this.fileTransferTool.downloadFile(file);
  }
}
