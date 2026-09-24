// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/localization.ts
// # 📌 Amac: Desktop UI desteklenen locale, default locale ve preference storage key degerlerini merkezi tanimlar
// # 📌 Modul - FileType: Config - TypeScript
// Version: 0.2.0
// Aciklama: Turkish/English runtime locale secimini View magic stringlerinden ayirir
// Bagimli Oldugu Katman: Config

export const DESKTOP_LOCALES = ["tr-TR", "en-US"] as const;
export type DesktopLocale = (typeof DESKTOP_LOCALES)[number];

export const DEFAULT_DESKTOP_LOCALE: DesktopLocale = "tr-TR";
export const DESKTOP_LOCALE_STORAGE_KEY = "turkuaz-office.ui-locale";

export function isDesktopLocale(value: string): value is DesktopLocale {
  return DESKTOP_LOCALES.some((locale) => locale === value);
}
