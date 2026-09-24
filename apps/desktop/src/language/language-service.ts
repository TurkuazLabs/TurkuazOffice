// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/language/language-service.ts
// # 📌 Amac: Desktop View katmanina secili locale icin reactive typed label degerlerini servis eder
// # 📌 Modul - FileType: Language - TypeScript
// Version: 0.2.0
// Aciklama: tr-TR/en-US runtime secimini Solid signal uzerinden cozer; document locale ile UI locale'i ayri tutar
// Bagimli Oldugu Katman: Language -> Config

import { createSignal, type Accessor } from "solid-js";

import {
  DEFAULT_DESKTOP_LOCALE,
  type DesktopLocale,
} from "../config/localization";
import {
  DESKTOP_LANGUAGE_PACKS,
  DESKTOP_LOCALE_OPTIONS,
  type DesktopLocaleOption,
} from "./language-packs";
import type { DesktopLabelKey } from "./labels";

export class LanguageService {
  private readonly localeSignal;

  public readonly locale: Accessor<DesktopLocale>;

  public constructor(initialLocale: DesktopLocale = DEFAULT_DESKTOP_LOCALE) {
    this.localeSignal = createSignal<DesktopLocale>(initialLocale);
    this.locale = this.localeSignal[0];
  }

  public text(key: DesktopLabelKey): string {
    return DESKTOP_LANGUAGE_PACKS[this.locale()][key];
  }

  public options(): readonly DesktopLocaleOption[] {
    return DESKTOP_LOCALE_OPTIONS;
  }

  public setLocale(locale: DesktopLocale): void {
    this.localeSignal[1](locale);
  }
}
