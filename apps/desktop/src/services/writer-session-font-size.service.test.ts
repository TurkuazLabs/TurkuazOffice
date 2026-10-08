// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/writer-session-font-size.service.test.ts
// # 📌 Amac: Writer font buyut/kucult shortcut servis davranisini regression testiyle sabitler
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.2.6
// Aciklama: Collapsed caret icin standart punto listesindeki bir sonraki/onceki font boyutunun typing-style'a uygulandigini dogrular
// Bagimli Oldugu Katman: Service -> Repo -> Config

import { describe, expect, it } from "vitest";

import { WriterSessionRepository } from "../repositories/writer-session.repository";
import { TextOffsetTool } from "../tools/text-offset.tool";
import type { WriterDocumentView } from "../views/writer-types";
import { WriterSessionService } from "./writer-session.service";

const DOCUMENT: WriterDocumentView = {
  id: "writer-font-size-test",
  title: "Font Size",
  revision: 0,
  plainText: "abc",
  sectionCount: 1,
  pageSettings: {
    widthTwips: 11906,
    heightTwips: 16838,
    marginTopTwips: 1440,
    marginRightTwips: 1440,
    marginBottomTwips: 1440,
    marginLeftTwips: 1440,
  },
  paragraphs: [
    {
      id: "p1",
      plainText: "abc",
      style: { alignment: "left" },
      runs: [
        {
          id: "r1",
          text: "abc",
          style: {
            bold: false,
            italic: false,
            underline: false,
            fontFamily: "Arial",
            fontSizeHalfPoints: 22,
          },
        },
      ],
    },
  ],
  images: [],
};

function fixture() {
  const repository = new WriterSessionRepository();
  repository.setNewDocument(DOCUMENT);
  repository.setSelection({
    paragraphId: "p1",
    startOffset: 1,
    endOffset: 1,
  });

  const service = new WriterSessionService(
    repository,
    {} as never,
    new TextOffsetTool(),
    {} as never,
    {} as never,
    {} as never,
    {} as never,
    {} as never,
    {} as never,
  );

  return { repository, service };
}

describe("WriterSessionService font size stepping", () => {
  it("steps from 11pt to 12pt and back using the standard size list", async () => {
    const { repository, service } = fixture();

    await service.increaseFontSize();

    expect(repository.typingStyle()?.fontSizeHalfPoints).toBe(24);

    await service.decreaseFontSize();

    expect(repository.typingStyle()?.fontSizeHalfPoints).toBe(22);
  });
});
