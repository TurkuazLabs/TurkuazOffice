// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/language/language-service.test.ts
// # 📌 Amac: Desktop tr-TR/en-US language pack ve reactive runtime switch davranisini regression testiyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.2.0
// Aciklama: Default Turkish label, English switch ve locale option metadata contractlarini kilitler
// Bagimli Oldugu Katman: Language -> Config

import { describe, expect, it } from "vitest";

import { LanguageService } from "./language-service";

describe("LanguageService", () => {
  it("switches between Turkish and English packs at runtime", () => {
    const service = new LanguageService("tr-TR");

    expect(service.text("newDocument")).toBe("Yeni Belge");
    expect(service.locale()).toBe("tr-TR");

    service.setLocale("en-US");

    expect(service.text("newDocument")).toBe("New Document");
    expect(service.text("templateReport")).toBe("Report");
    expect(service.locale()).toBe("en-US");
  });

  it("exposes both supported locale options", () => {
    const service = new LanguageService();

    expect(service.options().map((item) => item.locale)).toEqual(["tr-TR", "en-US"]);
  });
});
