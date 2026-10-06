// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/tools/browser-file-transfer.tool.test.ts
// # 📌 Amac: Browser file picker ve download Tool byte tasima davranisini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Pick, cancel, pre-read boyut guardi ve Blob download/object-URL cleanup akislarini test eder
// Bagimli Oldugu Katman: Tool -> Config

import { afterEach, describe, expect, it, vi } from "vitest";

import {
  WEB_DOWNLOAD_URL_REVOKE_DELAY_MS,
  WEB_IMPORT_FILE_TOO_LARGE_ERROR,
} from "../config/runtime-config";
import { BrowserFileTransferTool } from "./browser-file-transfer.tool";

const ACCEPT = ".tko";
const MAX_BYTES = 16;

function currentPicker(): HTMLInputElement {
  const input = document.querySelector<HTMLInputElement>('input[type="file"]');
  if (input === null) {
    throw new Error();
  }
  return input;
}

function setPickedFile(input: HTMLInputElement, file: File): void {
  Object.defineProperty(input, "files", {
    configurable: true,
    value: {
      item: (index: number) => (index === 0 ? file : null),
      length: 1,
    },
  });
}

function fileFixture(
  name: string,
  bytes: readonly number[],
  declaredSize: number = bytes.length,
): File {
  return {
    name,
    type: "application/x-turkuaz-office",
    size: declaredSize,
    arrayBuffer: async () => Uint8Array.from(bytes).buffer,
  } as File;
}

afterEach(() => {
  document.body.innerHTML = "";
  vi.useRealTimers();
  vi.restoreAllMocks();
});

describe("BrowserFileTransferTool", () => {
  it("reads the selected file as bytes and removes the temporary picker", async () => {
    const tool = new BrowserFileTransferTool(document, {
      createObjectURL: vi.fn(),
      revokeObjectURL: vi.fn(),
    });
    const picking = tool.pickFile({ accept: ACCEPT, maxBytes: MAX_BYTES });
    const input = currentPicker();
    setPickedFile(input, fileFixture("sample.tko", [1, 2, 3]));

    input.dispatchEvent(new Event("change"));

    await expect(picking).resolves.toMatchObject({
      name: "sample.tko",
      bytes: new Uint8Array([1, 2, 3]),
    });
    expect(document.querySelector('input[type="file"]')).toBeNull();
  });

  it("returns null when the picker is cancelled", async () => {
    const tool = new BrowserFileTransferTool(document, {
      createObjectURL: vi.fn(),
      revokeObjectURL: vi.fn(),
    });
    const picking = tool.pickFile({ accept: ACCEPT, maxBytes: MAX_BYTES });

    currentPicker().dispatchEvent(new Event("cancel"));

    await expect(picking).resolves.toBeNull();
  });

  it("removes the temporary picker when browser click throws synchronously", async () => {
    vi.spyOn(HTMLInputElement.prototype, "click").mockImplementation(() => {
      throw new Error();
    });
    const tool = new BrowserFileTransferTool(document, {
      createObjectURL: vi.fn(),
      revokeObjectURL: vi.fn(),
    });

    await expect(
      tool.pickFile({ accept: ACCEPT, maxBytes: MAX_BYTES }),
    ).rejects.toThrow();
    expect(document.querySelector('input[type="file"]')).toBeNull();
  });

  it("rejects oversized input before reading file bytes", async () => {
    const arrayBuffer = vi.fn(async () => Uint8Array.from([1]).buffer);
    const file = {
      ...fileFixture("large.tko", [1], MAX_BYTES + 1),
      arrayBuffer,
    } as File;
    const tool = new BrowserFileTransferTool(document, {
      createObjectURL: vi.fn(),
      revokeObjectURL: vi.fn(),
    });
    const picking = tool.pickFile({ accept: ACCEPT, maxBytes: MAX_BYTES });
    const input = currentPicker();
    setPickedFile(input, file);

    input.dispatchEvent(new Event("change"));

    await expect(picking).rejects.toThrow(WEB_IMPORT_FILE_TOO_LARGE_ERROR);
    expect(arrayBuffer).not.toHaveBeenCalled();
  });

  it("delays Blob URL revocation until after the browser can consume the download", () => {
    vi.useFakeTimers();
    const click = vi
      .spyOn(HTMLAnchorElement.prototype, "click")
      .mockImplementation(() => undefined);
    const createObjectURL = vi.fn(() => "blob:test");
    const revokeObjectURL = vi.fn();
    const tool = new BrowserFileTransferTool(document, {
      createObjectURL,
      revokeObjectURL,
    });

    tool.downloadFile({
      fileName: "sample.tko",
      mediaType: "application/x-turkuaz-office",
      bytes: new Uint8Array([4, 5, 6]),
    });

    expect(click).toHaveBeenCalledOnce();
    expect(createObjectURL).toHaveBeenCalledOnce();
    expect(revokeObjectURL).not.toHaveBeenCalled();
    expect(document.querySelector("a[download]")).toBeNull();

    vi.advanceTimersByTime(WEB_DOWNLOAD_URL_REVOKE_DELAY_MS);

    expect(revokeObjectURL).toHaveBeenCalledWith("blob:test");
  });

  it("still schedules delayed URL cleanup when anchor click throws", () => {
    vi.useFakeTimers();
    vi.spyOn(HTMLAnchorElement.prototype, "click").mockImplementation(() => {
      throw new Error();
    });
    const revokeObjectURL = vi.fn();
    const tool = new BrowserFileTransferTool(document, {
      createObjectURL: vi.fn(() => "blob:failed-click"),
      revokeObjectURL,
    });

    expect(() =>
      tool.downloadFile({
        fileName: "sample.tko",
        mediaType: "application/x-turkuaz-office",
        bytes: new Uint8Array([1]),
      }),
    ).toThrow();
    expect(revokeObjectURL).not.toHaveBeenCalled();
    expect(document.querySelector("a[download]")).toBeNull();

    vi.advanceTimersByTime(WEB_DOWNLOAD_URL_REVOKE_DELAY_MS);

    expect(revokeObjectURL).toHaveBeenCalledWith("blob:failed-click");
  });
});
