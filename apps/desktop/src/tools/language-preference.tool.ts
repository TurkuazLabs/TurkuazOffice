// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/language-preference.tool.ts
// # 📌 Amac: Desktop UI locale tercihini browser local storage adaptorunun arkasinda okur ve yazar
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.2.0
// Aciklama: localStorage erisimini Language ve View katmanlarindan ayirir; gecersiz degerde default locale'e doner
// Bagimli Oldugu Katman: Tool -> Config

import {
  DEFAULT_DESKTOP_LOCALE,
  DESKTOP_LOCALE_STORAGE_KEY,
  isDesktopLocale,
  type DesktopLocale,
} from "../config/localization";

export interface LanguagePreferenceStorage {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
}

export class LanguagePreferenceTool {
  public constructor(
    private readonly storage: LanguagePreferenceStorage | null =
      typeof window === "undefined" ? null : window.localStorage,
  ) {}

  public load(): DesktopLocale {
    if (this.storage === null) {
      return DEFAULT_DESKTOP_LOCALE;
    }
    try {
      const value = this.storage.getItem(DESKTOP_LOCALE_STORAGE_KEY);
      return value !== null && isDesktopLocale(value) ? value : DEFAULT_DESKTOP_LOCALE;
    } catch {
      return DEFAULT_DESKTOP_LOCALE;
    }
  }

  public save(locale: DesktopLocale): void {
    if (this.storage === null) {
      return;
    }
    try {
      this.storage.setItem(DESKTOP_LOCALE_STORAGE_KEY, locale);
    } catch {
      // Preference persistence must not block Writer usage.
    }
  }
}
