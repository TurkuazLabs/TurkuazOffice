// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/office-module-switcher.tsx
// # 📌 Amac: Desktop Writer ve Sheet modul gecisini View katmaninda sunar
// # 📌 Modul - FileType: View - TSX
// Version: 0.4.0
// Aciklama: Aktif modulu gosterir ve composition root tarafindan verilen modul secim callbackini cagirir
// Bagimli Oldugu Katman: View -> Language -> Config

import { OFFICE_MODULES, type OfficeModule } from "../config/office-modules";
import type { LanguageService } from "../language/language-service";

interface OfficeModuleSwitcherProps {
  readonly activeModule: OfficeModule;
  readonly language: LanguageService;
  readonly onSelectModule: (module: OfficeModule) => void;
}

export function OfficeModuleSwitcher(props: OfficeModuleSwitcherProps) {
  return (
    <nav class="office-module-switcher" aria-label={props.language.text("moduleSwitcherLabel")}>
      <button
        type="button"
        class="office-module-switcher__button"
        classList={{
          "office-module-switcher__button--active": props.activeModule === OFFICE_MODULES.writer,
        }}
        aria-pressed={props.activeModule === OFFICE_MODULES.writer}
        onClick={() => props.onSelectModule(OFFICE_MODULES.writer)}
      >
        {props.language.text("writerModule")}
      </button>
      <button
        type="button"
        class="office-module-switcher__button"
        classList={{
          "office-module-switcher__button--active": props.activeModule === OFFICE_MODULES.sheet,
        }}
        aria-pressed={props.activeModule === OFFICE_MODULES.sheet}
        onClick={() => props.onSelectModule(OFFICE_MODULES.sheet)}
      >
        {props.language.text("sheetModule")}
      </button>
    </nav>
  );
}
