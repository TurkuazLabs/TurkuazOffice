// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/browser-file-transfer.tool.ts
// # 📌 Amac: Browser file picker ve download API erisimini Web Tool katmaninda izole eder
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.4.0
// Aciklama: Native filesystem yetkisi istemeden input-file ingress ve Blob object-URL egress byte tasimasini saglar
// Bagimli Oldugu Katman: Tool -> Config

import {
  WEB_DOWNLOAD_URL_REVOKE_DELAY_MS,
  WEB_IMPORT_FILE_TOO_LARGE_ERROR,
} from "../config/runtime-config";
import type {
  WebDownloadFile,
  WebFilePickRequest,
  WebPickedFile,
} from "../models/web-models";

export interface WebBrowserFileTransferTool {
  pickFile(request: WebFilePickRequest): Promise<WebPickedFile | null>;
  downloadFile(file: WebDownloadFile): void;
}

export interface WebObjectUrlApi {
  createObjectURL(blob: Blob): string;
  revokeObjectURL(url: string): void;
}

export class BrowserFileTransferTool implements WebBrowserFileTransferTool {
  public constructor(
    private readonly document: Document,
    private readonly objectUrlApi: WebObjectUrlApi,
  ) {}

  public pickFile(request: WebFilePickRequest): Promise<WebPickedFile | null> {
    return new Promise<WebPickedFile | null>((resolve, reject) => {
      const input = this.document.createElement("input");
      input.type = "file";
      input.accept = request.accept;
      input.hidden = true;
      this.document.body.append(input);

      let completed = false;
      const cleanup = (): void => {
        input.remove();
      };
      const finish = (value: WebPickedFile | null): void => {
        if (completed) {
          return;
        }
        completed = true;
        cleanup();
        resolve(value);
      };
      const fail = (error: unknown): void => {
        if (completed) {
          return;
        }
        completed = true;
        cleanup();
        reject(error);
      };

      input.addEventListener(
        "change",
        () => {
          const file = input.files?.item(0) ?? null;
          if (file === null) {
            finish(null);
            return;
          }
          if (file.size > request.maxBytes) {
            fail(new Error(WEB_IMPORT_FILE_TOO_LARGE_ERROR));
            return;
          }

          void file.arrayBuffer().then(
            (buffer) => {
              finish({
                name: file.name,
                mediaType: file.type,
                bytes: new Uint8Array(buffer),
              });
            },
            fail,
          );
        },
        { once: true },
      );
      input.addEventListener("cancel", () => finish(null), { once: true });
      input.click();
    });
  }

  public downloadFile(file: WebDownloadFile): void {
    const blob = new Blob([file.bytes.slice().buffer], {
      type: file.mediaType,
    });
    const url = this.objectUrlApi.createObjectURL(blob);
    const anchor = this.document.createElement("a");
    anchor.href = url;
    anchor.download = file.fileName;
    anchor.hidden = true;
    this.document.body.append(anchor);

    try {
      anchor.click();
    } finally {
      anchor.remove();
      globalThis.setTimeout(() => {
        this.objectUrlApi.revokeObjectURL(url);
      }, WEB_DOWNLOAD_URL_REVOKE_DELAY_MS);
    }
  }
}
