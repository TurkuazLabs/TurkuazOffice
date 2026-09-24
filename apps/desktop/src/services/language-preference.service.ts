// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/language-preference.service.ts
// # 📌 Amac: Desktop UI locale secimi ve kalici preference is kurallarini Language/Tool katmanlari arasinda koordine eder
// # 📌 Modul - FileType: Service - TypeScript
// Version: 0.2.0
// Aciklama: Locale validation, initial preference load ve runtime language switch akislarini merkezi Service katmaninda tutar
// Bagimli Oldugu Katman: Service -> Tool -> Language

import {
  isDesktopLocale,
  type DesktopLocale,
} from "../config/localization";
import type {
  DesktopLocaleOption,
} from "../language/language-packs";
import type { LanguageService } from "../language/language-service";
import type { LanguagePreferenceTool } from "../tools/language-preference.tool";

export class LanguagePreferenceService {
  public constructor(
    private readonly language: LanguageService,
    private readonly preferenceTool: LanguagePreferenceTool,
  ) {}

  public initialize(): void {
    this.language.setLocale(this.preferenceTool.load());
  }

  public locale(): DesktopLocale {
    return this.language.locale();
  }

  public options(): readonly DesktopLocaleOption[] {
    return this.language.options();
  }

  public setLocale(value: string): void {
    if (!isDesktopLocale(value)) {
      return;
    }
    this.language.setLocale(value);
    this.preferenceTool.save(value);
  }
}
