// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/clipboard.service.test.ts
// # 📌 Amac: ClipboardService representation priority ve atomic cut/paste kurallarini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// # Version: 0.2.0
// # Aciklama: Internal MIME, sanitized HTML, plain-text fallback, copy ve cut akisini headless mock sinirlariyla test eder
// Bagimli Oldugu Katman: Test -> Service -> Repo -> Tool

import { describe, expect, it, vi } from "vitest";

import {
  WRITER_CLIPBOARD_FRAGMENT_KIND,
  WRITER_CLIPBOARD_FRAGMENT_SCHEMA_VERSION,
} from "../config/clipboard";
import type {
  ClipboardDomNodeModel,
  ClipboardTransferModel,
  ClipboardWriteModel,
} from "../models/clipboard-model";
import type { WriterSessionRepository } from "../repositories/writer-session.repository";
import type { ClipboardDomTool } from "../tools/clipboard-dom.tool";
import type { ClipboardTool } from "../tools/clipboard.tool";
import type { WriterCharacterStyleView, WriterDocumentView } from "../views/writer-types";
import { ClipboardService } from "./clipboard.service";
import type { WriterSessionService } from "./writer-session.service";

const BASE_STYLE: WriterCharacterStyleView = {
  bold: false,
  italic: false,
  underline: false,
  fontFamily: "Arial",
  fontSizeHalfPoints: 22,
};

const INTERNAL_STYLE: WriterCharacterStyleView = {
  bold: true,
  italic: true,
  underline: false,
  fontFamily: "Georgia",
  fontSizeHalfPoints: 28,
};

const PARAGRAPH_ID = "paragraph-1";

function transfer(overrides: Partial<ClipboardTransferModel> = {}): ClipboardTransferModel {
  return {
    internalFragment: "",
    html: "",
    plainText: "",
    ...overrides,
  };
}

function internalFragment(text: string): string {
  return JSON.stringify({
    kind: WRITER_CLIPBOARD_FRAGMENT_KIND,
    schemaVersion: WRITER_CLIPBOARD_FRAGMENT_SCHEMA_VERSION,
    runs: [
      {
        text,
        style: INTERNAL_STYLE,
      },
    ],
  });
}

function repository(document: WriterDocumentView | null = null): WriterSessionRepository {
  return {
    fileSession: () => null,
    document: () => document,
  } as unknown as WriterSessionRepository;
}

function session(
  startOffset: number,
  endOffset: number,
  replaceSelectionWithStyledRuns: ReturnType<typeof vi.fn>,
): WriterSessionService {
  return {
    captureSelection: vi.fn(() => ({
      paragraphId: PARAGRAPH_ID,
      startOffset,
      endOffset,
    })),
    insertionStyle: vi.fn(() => BASE_STYLE),
    replaceSelectionWithStyledRuns,
  } as unknown as WriterSessionService;
}

function clipboard(
  payload: ClipboardTransferModel,
  writeResult = true,
) {
  return {
    read: vi.fn((_event: ClipboardEvent) => payload),
    readImage: vi.fn(async (_event: ClipboardEvent) => null),
    write: vi.fn((_event: ClipboardEvent, _payload: ClipboardWriteModel) => writeResult),
    consume: vi.fn((_event: ClipboardEvent) => undefined),
  };
}

function paragraphDocument(): WriterDocumentView {
  return {
    id: "document-1",
    title: "Clipboard",
    revision: 1,
    plainText: "AB",
    sectionCount: 1,
    pageSettings: {
      widthTwips: 11_906,
      heightTwips: 16_838,
      marginTopTwips: 1_440,
      marginRightTwips: 1_440,
      marginBottomTwips: 1_440,
      marginLeftTwips: 1_440,
    },
    paragraphs: [
      {
        id: PARAGRAPH_ID,
        plainText: "AB",
        style: { alignment: "left" },
        runs: [
          {
            id: "run-1",
            text: "AB",
            style: BASE_STYLE,
          },
        ],
      },
    ],
  };
}

describe("ClipboardService", () => {
  it("internal MIME has paste priority", async () => {
    const replace = vi.fn(async () => undefined);
    const clipboardMock = clipboard(
      transfer({
        internalFragment: internalFragment("Internal"),
        html: "<strong>HTML</strong>",
        plainText: "Plain",
      }),
    );
    const domParse = vi.fn(() => {
      throw new Error("HTML path must not run");
    });
    const service = new ClipboardService(
      repository(),
      session(1, 1, replace),
      clipboardMock as unknown as ClipboardTool,
      { parse: domParse } as unknown as ClipboardDomTool,
    );

    await service.pasteSelection(
      PARAGRAPH_ID,
      {} as HTMLElement,
      {} as ClipboardEvent,
    );

    expect(domParse).not.toHaveBeenCalled();
    expect(clipboardMock.consume).toHaveBeenCalledOnce();
    expect(replace).toHaveBeenCalledWith(PARAGRAPH_ID, 1, 1, [
      { text: "Internal", style: INTERNAL_STYLE },
    ]);
  });

  it("invalid internal payload falls back to sanitized HTML", async () => {
    const replace = vi.fn(async () => undefined);
    const htmlNodes: readonly ClipboardDomNodeModel[] = [
      {
        kind: "element",
        tagName: "strong",
        style: {
          fontFamily: "",
          fontSize: "",
          fontWeight: "",
          fontStyle: "",
          textDecoration: "",
        },
        children: [{ kind: "text", text: "Rich" }],
      },
    ];
    const clipboardMock = clipboard(
      transfer({
        internalFragment: "{invalid",
        html: "<strong>Rich</strong>",
        plainText: "Plain",
      }),
    );
    const service = new ClipboardService(
      repository(),
      session(0, 0, replace),
      clipboardMock as unknown as ClipboardTool,
      { parse: vi.fn(() => htmlNodes) } as unknown as ClipboardDomTool,
    );

    await service.pasteSelection(
      PARAGRAPH_ID,
      {} as HTMLElement,
      {} as ClipboardEvent,
    );

    expect(replace).toHaveBeenCalledWith(PARAGRAPH_ID, 0, 0, [
      {
        text: "Rich",
        style: { ...BASE_STYLE, bold: true },
      },
    ]);
  });

  it("invalid rich payload falls back to plain text", async () => {
    const replace = vi.fn(async () => undefined);
    const clipboardMock = clipboard(
      transfer({
        internalFragment: "{invalid",
        html: "<div>Too large</div>",
        plainText: "Line A\r\nLine B",
      }),
    );
    const service = new ClipboardService(
      repository(),
      session(2, 2, replace),
      clipboardMock as unknown as ClipboardTool,
      { parse: vi.fn(() => null) } as unknown as ClipboardDomTool,
    );

    await service.pasteSelection(
      PARAGRAPH_ID,
      {} as HTMLElement,
      {} as ClipboardEvent,
    );

    expect(replace).toHaveBeenCalledWith(PARAGRAPH_ID, 2, 2, [
      { text: "Line A Line B", style: BASE_STYLE },
    ]);
  });

  it("copy writes internal HTML and plain-text representations", () => {
    const replace = vi.fn(async () => undefined);
    const clipboardMock = clipboard(transfer());
    const service = new ClipboardService(
      repository(paragraphDocument()),
      session(0, 2, replace),
      clipboardMock as unknown as ClipboardTool,
      { parse: vi.fn(() => []) } as unknown as ClipboardDomTool,
    );

    service.copySelection(
      PARAGRAPH_ID,
      {} as HTMLElement,
      {} as ClipboardEvent,
    );

    expect(clipboardMock.write).toHaveBeenCalledOnce();
    const payload = clipboardMock.write.mock.calls[0]?.[1];
    expect(payload?.plainText).toBe("AB");
    expect(payload?.html).toContain("<span");
    expect(JSON.parse(payload?.internalFragment ?? "{}")).toMatchObject({
      kind: WRITER_CLIPBOARD_FRAGMENT_KIND,
      schemaVersion: WRITER_CLIPBOARD_FRAGMENT_SCHEMA_VERSION,
    });
  });

  it("cut writes clipboard before atomic empty fragment replace", async () => {
    const order: string[] = [];
    const replace = vi.fn(async () => {
      order.push("replace");
    });
    const clipboardMock = {
      ...clipboard(transfer()),
      write: vi.fn((_event: ClipboardEvent, _payload: ClipboardWriteModel) => {
        order.push("write");
        return true;
      }),
    };
    const service = new ClipboardService(
      repository(paragraphDocument()),
      session(0, 2, replace),
      clipboardMock as unknown as ClipboardTool,
      { parse: vi.fn(() => []) } as unknown as ClipboardDomTool,
    );

    await service.cutSelection(
      PARAGRAPH_ID,
      {} as HTMLElement,
      {} as ClipboardEvent,
    );

    expect(order).toEqual(["write", "replace"]);
    expect(replace).toHaveBeenCalledWith(PARAGRAPH_ID, 0, 2, []);
  });
});

  it("image-only paste consumes default DOM paste and inserts canonical asset", async () => {
    const replace = vi.fn(async () => undefined);
    const insertImageData = vi.fn(async () => undefined);
    const clipboardMock = {
      ...clipboard(transfer()),
      readImage: vi.fn(async () => ({
        mediaType: "image/png",
        data: [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a],
      })),
    };
    const writerSession = {
      ...session(1, 1, replace),
      insertImageData,
    };
    const service = new ClipboardService(
      repository(),
      writerSession as unknown as WriterSessionService,
      clipboardMock as unknown as ClipboardTool,
      { parse: vi.fn(() => null) } as unknown as ClipboardDomTool,
    );

    await service.pasteSelection(
      PARAGRAPH_ID,
      {} as HTMLElement,
      {} as ClipboardEvent,
    );

    expect(clipboardMock.consume).toHaveBeenCalledOnce();
    expect(insertImageData).toHaveBeenCalledWith(
      PARAGRAPH_ID,
      "image/png",
      [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a],
    );
    expect(replace).not.toHaveBeenCalled();
  });
