// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/services/web-import-export.service.ts
// # 📌 Amac: M3 Web browser file-transfer ve Writer TKO codec composition is kurallarini yonetir
// # 📌 Modul - FileType: Service - TypeScript
// Version: 0.4.0
// Aciklama: TKO secim/limit kurallarini, Rust-backed inspect validation ve canonical re-encode download akisini tek Service sinirinda compose eder
// Bagimli Oldugu Katman: Service -> Tool -> Config

import {
  WEB_APP_VERSION,
  WEB_BROWSER_IMPORT_MAX_BYTES,
  WEB_NATIVE_DOCUMENT_ACCEPT,
  WEB_IMPORT_FILE_TOO_LARGE_ERROR,
  WEB_NATIVE_DOCUMENT_EXTENSION,
  WEB_NATIVE_DOCUMENT_MIME_TYPE,
  WEB_TKO_CODEC_UNAVAILABLE_ERROR,
  WEB_UNSUPPORTED_IMPORT_FILE_ERROR,
} from "../config/runtime-config";
import type {
  WebDownloadFile,
  WebImportExportCapabilities,
  WebNativeDocumentImport,
  WebPickedFile,
} from "../models/web-models";
import type { WebBrowserFileTransferTool } from "../tools/browser-file-transfer.tool";
import type { WebWriterTkoTool } from "../tools/writer-tko.tool";

export class WebImportExportService {
  public constructor(
    private readonly fileTransferTool: WebBrowserFileTransferTool,
    private readonly writerTkoTool: WebWriterTkoTool | null,
  ) {}

  public capabilities(): WebImportExportCapabilities {
    return {
      browserFileTransferAvailable: true,
      nativeTkoCodecAvailable: this.writerTkoTool !== null,
    };
  }

  public async importNativeDocument(): Promise<WebNativeDocumentImport | null> {
    const codec = this.requireTkoCodec();
    const picked = await this.pickNativeDocumentBytes();
    if (picked === null) {
      return null;
    }

    return {
      fileName: picked.name,
      bytes: picked.bytes.slice(),
      summary: codec.inspect(picked.bytes),
    };
  }

  public exportNativeDocument(fileName: string, bytes: Uint8Array): void {
    const codec = this.requireTkoCodec();
    const canonicalBytes = codec.reencode(bytes, WEB_APP_VERSION);
    this.downloadNativeDocumentBytes(fileName, canonicalBytes);
  }

  private requireTkoCodec(): WebWriterTkoTool {
    if (this.writerTkoTool === null) {
      throw new Error(WEB_TKO_CODEC_UNAVAILABLE_ERROR);
    }
    return this.writerTkoTool;
  }

  private async pickNativeDocumentBytes(): Promise<WebPickedFile | null> {
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

  private downloadNativeDocumentBytes(fileName: string, bytes: Uint8Array): void {
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
