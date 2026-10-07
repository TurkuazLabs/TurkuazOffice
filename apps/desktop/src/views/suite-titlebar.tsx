// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/suite-titlebar.tsx
// # 📌 Amac: Writer ve Sheet icin ortak Turkuaz Office uygulama baslik cubugunu render eder
// # 📌 Modul - FileType: View - TSX
// Version: 0.12.0
// Aciklama: Modul ikonu, uygulama adi, belge adi ve Baslangic Merkezi geri donusunu tek View sinirinda standardize eder
// Bagimli Oldugu Katman: View -> Language

import type { LanguageService } from "../language/language-service";
import { SuiteIcon, type SuiteIconKind } from "./suite-icon";

interface SuiteTitlebarProps {
  readonly moduleIcon: Extract<SuiteIconKind, "writer" | "sheet">;
  readonly moduleName: string;
  readonly documentName: string;
  readonly language: LanguageService;
  readonly onHome?: () => void;
}

export function SuiteTitlebar(props: SuiteTitlebarProps) {
  return (
    <header class="office-titlebar">
      <span class="office-titlebar__module-icon" aria-hidden="true">
        <SuiteIcon kind={props.moduleIcon} size={24} decorative />
      </span>
      <strong>
        {props.language.text("appName")} {props.moduleName}
      </strong>
      <span class="office-titlebar__document">{props.documentName}</span>
      {props.onHome !== undefined && (
        <button
          type="button"
          class="office-titlebar__home"
          onClick={props.onHome}
          title={props.language.text("backToStartCenter")}
        >
          <SuiteIcon kind="home" size={18} decorative />
          <span>{props.language.text("startCenterName")}</span>
        </button>
      )}
    </header>
  );
}
