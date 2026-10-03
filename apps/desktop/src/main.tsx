// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/main.tsx
// # 📌 Amac: SolidJS Desktop View agacini Tauri webview rootuna baglar
// # 📌 Modul - FileType: View - TSX
// Version: 0.4.0
// Aciklama: Composition root bagimliliklarini Writer ve Sheet shell'lerine aktarir ve modul secimini yonetir
// Bagimli Oldugu Katman: View -> Config

import { createSignal, Match, Switch } from "solid-js";
import { render } from "solid-js/web";

import { APP_CONTAINER } from "./config/app-container";
import { ERROR_CODES } from "./config/error-codes";
import { OFFICE_MODULES, type OfficeModule } from "./config/office-modules";
import "./config/ui-tokens.css";
import "./views/app.css";
import { SheetShell } from "./views/sheet-shell";
import { WriterShell } from "./views/writer-shell";

const root = document.getElementById("app");
if (root === null) {
  throw new Error(ERROR_CODES.appRootMissing);
}

const [activeModule, setActiveModule] = createSignal<OfficeModule>(OFFICE_MODULES.writer);

render(
  () => (
    <Switch>
      <Match when={activeModule() === OFFICE_MODULES.sheet}>
        <SheetShell
          controller={APP_CONTAINER.sheetController}
          repository={APP_CONTAINER.sheetSessionRepository}
          language={APP_CONTAINER.languageService}
          onSelectModule={setActiveModule}
        />
      </Match>
      <Match when={true}>
        <WriterShell
          controller={APP_CONTAINER.writerController}
          repository={APP_CONTAINER.writerSessionRepository}
          language={APP_CONTAINER.languageService}
          onSelectModule={setActiveModule}
        />
      </Match>
    </Switch>
  ),
  root,
);
