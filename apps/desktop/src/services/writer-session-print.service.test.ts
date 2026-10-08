// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/writer-session-print.service.test.ts
// # 📌 Amac: Writer dogrudan Yazdir komutunun belge yuklenmeden guvenli no-op olmasini dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.2.5
// Aciklama: Ctrl+P veya menu Yazdir cok erken tetiklendiginde requireDocument hatasi ve native print cagrisi olusmadigini sabitler
// Bagimli Oldugu Katman: Service -> Repo -> Tool

import { describe, expect, it, vi } from "vitest";

import { WriterSessionRepository } from "../repositories/writer-session.repository";
import { TextOffsetTool } from "../tools/text-offset.tool";
import { WriterSessionService } from "./writer-session.service";

describe("WriterSessionService direct print guard", () => {
  it("does nothing when print is requested before a document exists", async () => {
    const repository = new WriterSessionRepository();
    const print = vi.fn();

    const service = new WriterSessionService(
      repository,
      {} as never,
      new TextOffsetTool(),
      {
        activeWriterParagraph: vi.fn().mockReturnValue(null),
      } as never,
      {} as never,
      {} as never,
      { print } as never,
      {} as never,
      {} as never,
    );

    await expect(service.printDocument()).resolves.toBeUndefined();

    expect(print).not.toHaveBeenCalled();
    expect(repository.printPreviewLayout()).toBeNull();
  });
});
