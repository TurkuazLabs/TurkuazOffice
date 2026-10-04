// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/repositories/browser-document-index.repository.test.ts
// # 📌 Amac: Browser document index Repository persistence davranisini regression testleriyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// # Version: 0.4.0
// # Aciklama: Metadata replace, deterministic list, corrupt-storage fallback ve remove davranislarini test eder
// Bagimli Oldugu Katman: Repo

import { beforeEach, describe, expect, it } from "vitest";

import { WEB_STORAGE_NAMESPACE } from "../config/runtime-config";
import { BrowserDocumentIndexRepository } from "./browser-document-index.repository";

describe("BrowserDocumentIndexRepository", () => {
  beforeEach(() => {
    window.localStorage.clear();
  });

  it("same id save is replacement and list order is deterministic", () => {
    const repository = new BrowserDocumentIndexRepository(window.localStorage);

    repository.save({ id: "b", title: "B", revision: 1 });
    repository.save({ id: "a", title: "A", revision: 1 });
    repository.save({ id: "a", title: "A2", revision: 2 });

    expect(repository.list()).toEqual([
      { id: "a", title: "A2", revision: 2 },
      { id: "b", title: "B", revision: 1 },
    ]);
  });

  it("corrupt storage degrades to an empty document list", () => {
    window.localStorage.setItem(WEB_STORAGE_NAMESPACE, "{broken");
    const repository = new BrowserDocumentIndexRepository(window.localStorage);

    expect(repository.list()).toEqual([]);
  });

  it("rejects payload-shaped entries from the metadata index", () => {
    window.localStorage.setItem(
      WEB_STORAGE_NAMESPACE,
      JSON.stringify([
        { id: "a", title: "A", revision: 1, content: "canonical-like-payload" },
      ]),
    );
    const repository = new BrowserDocumentIndexRepository(window.localStorage);

    expect(repository.list()).toEqual([{ id: "a", title: "A", revision: 1 }]);
  });

  it("remove deletes only requested document metadata", () => {
    const repository = new BrowserDocumentIndexRepository(window.localStorage);

    repository.save({ id: "a", title: "A", revision: 0 });
    repository.save({ id: "b", title: "B", revision: 0 });
    repository.remove("a");

    expect(repository.list()).toEqual([
      { id: "b", title: "B", revision: 0 },
    ]);
  });
});
