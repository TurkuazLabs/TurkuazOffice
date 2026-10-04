// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/repositories/browser-document-repository.test.ts
// # 📌 Amac: Browser document Repository persistence davranisini regression testleriyle dogrular
// # 📌 Modul - FileType: Repository - TypeScript
// # Version: 0.4.0
// # Aciklama: Save replace, deterministic list ve remove davranislarini test eder
// Bagimli Oldugu Katman: Repo

import { beforeEach, describe, expect, it } from "vitest";

import { BrowserDocumentRepository } from "./browser-document-repository";

describe("BrowserDocumentRepository", () => {
  beforeEach(() => {
    window.localStorage.clear();
  });

  it("same id save is replacement and list order is deterministic", () => {
    const repository = new BrowserDocumentRepository(window.localStorage);

    repository.save({ id: "b", title: "B", content: "", revision: 1 });
    repository.save({ id: "a", title: "A", content: "ilk", revision: 1 });
    repository.save({ id: "a", title: "A2", content: "son", revision: 2 });

    expect(repository.list()).toEqual([
      { id: "a", title: "A2", content: "son", revision: 2 },
      { id: "b", title: "B", content: "", revision: 1 },
    ]);
  });

  it("remove deletes only requested document", () => {
    const repository = new BrowserDocumentRepository(window.localStorage);

    repository.save({ id: "a", title: "A", content: "", revision: 0 });
    repository.save({ id: "b", title: "B", content: "", revision: 0 });
    repository.remove("a");

    expect(repository.list()).toEqual([
      { id: "b", title: "B", content: "", revision: 0 },
    ]);
  });
});
