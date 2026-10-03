// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/writer-session-remount.service.test.ts
// # 📌 Amac: Desktop Writer oturumunun modul remount sirasinda korunmasini regression testiyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.2.1
// Aciklama: Mevcut Writer document varsa initializeSession akisinin yeni belge olusturmadan erken donmesini dogrular
// Bagimli Oldugu Katman: Service -> Repo

import { describe, expect, it } from "vitest";

import { WriterSessionRepository } from "../repositories/writer-session.repository";
import type { WriterDocumentView } from "../views/writer-types";
import { WriterSessionService } from "./writer-session.service";

const DOCUMENT: WriterDocumentView = {
  id: "writer-document-1",
  title: "Korunan Oturum",
  revision: 7,
  plainText: "Mevcut belge",
  sectionCount: 1,
  pageSettings: {
    widthTwips: 11906,
    heightTwips: 16838,
    marginTopTwips: 1440,
    marginRightTwips: 1440,
    marginBottomTwips: 1440,
    marginLeftTwips: 1440,
  },
  paragraphs: [],
  images: [],
};

describe("WriterSessionService remount", () => {
  it("keeps an existing writer document during initializeSession", async () => {
    const repository = new WriterSessionRepository();
    repository.setNewDocument(DOCUMENT);

    const service = new WriterSessionService(
      repository,
      {} as never,
      {} as never,
      {} as never,
      {} as never,
      {} as never,
      {} as never,
      {} as never,
      {} as never,
    );

    await service.initializeSession();

    expect(repository.document()).toBe(DOCUMENT);
    expect(repository.document()?.revision).toBe(7);
  });
});
