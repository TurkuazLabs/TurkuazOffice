// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/language/language-service.ts
// # 📌 Amac: Desktop View katmanina secili dil label degerlerini servis eder
// # 📌 Modul - FileType: Language - TypeScript
// # Version: 0.2.0
// # Aciklama: M1 icin Turkce kaynagi tek erisim noktasi uzerinden sunar ve gelecekte locale secimine aciktir
// Bagimli Oldugu Katman: Language

import type { DesktopLabelKey } from "./labels";
import { TR_LABELS } from "./tr";

export class LanguageService {
  public text(key: DesktopLabelKey): string {
    return TR_LABELS[key];
  }
}
