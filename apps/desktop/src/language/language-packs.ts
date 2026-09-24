// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/language/language-packs.ts
// # 📌 Amac: Desktop locale kodlarini typed language pack ve kullanici label key'leriyle merkezi esler
// # 📌 Modul - FileType: Language - TypeScript
// Version: 0.2.0
// Aciklama: tr-TR/en-US pack secimini ve locale selector metadata'sini View magic stringlerinden ayirir
// Bagimli Oldugu Katman: Language -> Config

import {
  DESKTOP_LOCALES,
  type DesktopLocale,
} from "../config/localization";
import { EN_LABELS } from "./en";
import type { DesktopLabelKey } from "./labels";
import { TR_LABELS } from "./tr";

export const DESKTOP_LANGUAGE_PACKS: Readonly<
  Record<DesktopLocale, Readonly<Record<DesktopLabelKey, string>>>
> = {
  [DESKTOP_LOCALES[0]]: TR_LABELS,
  [DESKTOP_LOCALES[1]]: EN_LABELS,
};

export interface DesktopLocaleOption {
  readonly locale: DesktopLocale;
  readonly labelKey: DesktopLabelKey;
}

export const DESKTOP_LOCALE_OPTIONS: readonly DesktopLocaleOption[] = [
  { locale: DESKTOP_LOCALES[0], labelKey: "languageTurkish" },
  { locale: DESKTOP_LOCALES[1], labelKey: "languageEnglish" },
];
