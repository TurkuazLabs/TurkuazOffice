// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/repositories/web-writer-session.repository.test.ts
// # 📌 Amac: Writer Web session Repository byte izolasyonu ve lifecycle davranisini dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Replace/read clone, clear ve rich TKO kaynak byte koruma regressionlarini sabitler
// Bagimli Oldugu Katman: Repo -> Model

import { describe, expect, it } from "vitest";

import type { WebWriterSessionDocument } from "../models/web-models";
import { InMemoryWebWriterSessionRepository } from "./web-writer-session.repository";

function documentFixture(): WebWriterSessionDocument {
  return {
    fileName: "belge.tko",
    bytes: new Uint8Array([1, 2, 3]),
    summary: {
      id: "writer-1",
      title: "Belge",
      schemaVersion: 1,
      revision: 4,
      sectionCount: 2,
      assetCount: 1,
    },
  };
}

describe("InMemoryWebWriterSessionRepository", () => {
  it("isolates stored TKO bytes from caller mutation", () => {
    const repository = new InMemoryWebWriterSessionRepository();
    const source = documentFixture();

    repository.replace(source);
    source.bytes[0] = 9;

    expect(repository.active()?.bytes).toEqual(new Uint8Array([1, 2, 3]));
  });

  it("returns an isolated copy of active session data", () => {
    const repository = new InMemoryWebWriterSessionRepository();
    repository.replace(documentFixture());

    const first = repository.active();
    if (first === null) {
      throw new Error("Session fixture missing");
    }
    first.bytes[1] = 8;

    expect(repository.active()?.bytes).toEqual(new Uint8Array([1, 2, 3]));
  });

  it("clears the active Writer session", () => {
    const repository = new InMemoryWebWriterSessionRepository();
    repository.replace(documentFixture());

    repository.clear();

    expect(repository.active()).toBeNull();
  });
});
