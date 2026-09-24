// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/language-preference.service.test.ts
// # 📌 Amac: Desktop locale preference load, validation ve persistence davranisini regression testiyle dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.2.0
// Aciklama: Invalid storage fallback, persisted English load ve runtime selection save contractlarini kilitler
// Bagimli Oldugu Katman: Service -> Tool -> Language

import { describe, expect, it } from "vitest";

import { DESKTOP_LOCALE_STORAGE_KEY } from "../config/localization";
import { LanguageService } from "../language/language-service";
import {
  LanguagePreferenceTool,
  type LanguagePreferenceStorage,
} from "../tools/language-preference.tool";
import { LanguagePreferenceService } from "./language-preference.service";

class MemoryStorage implements LanguagePreferenceStorage {
  private readonly values = new Map<string, string>();

  public getItem(key: string): string | null {
    return this.values.get(key) ?? null;
  }

  public setItem(key: string, value: string): void {
    this.values.set(key, value);
  }

  public seed(key: string, value: string): void {
    this.values.set(key, value);
  }
}

describe("LanguagePreferenceService", () => {
  it("loads persisted English locale and saves later changes", () => {
    const storage = new MemoryStorage();
    storage.seed(DESKTOP_LOCALE_STORAGE_KEY, "en-US");
    const language = new LanguageService();
    const tool = new LanguagePreferenceTool(storage);
    const service = new LanguagePreferenceService(language, tool);

    service.initialize();

    expect(service.locale()).toBe("en-US");
    expect(language.text("save")).toBe("Save");

    service.setLocale("tr-TR");

    expect(service.locale()).toBe("tr-TR");
    expect(storage.getItem(DESKTOP_LOCALE_STORAGE_KEY)).toBe("tr-TR");
  });

  it("ignores unsupported locale values", () => {
    const storage = new MemoryStorage();
    const language = new LanguageService();
    const service = new LanguagePreferenceService(
      language,
      new LanguagePreferenceTool(storage),
    );

    service.initialize();
    service.setLocale("invalid-locale");

    expect(service.locale()).toBe("tr-TR");
    expect(storage.getItem(DESKTOP_LOCALE_STORAGE_KEY)).toBeNull();
  });
});
