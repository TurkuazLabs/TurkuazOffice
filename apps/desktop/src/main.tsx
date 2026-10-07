// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/main.tsx
// # 📌 Amac: SolidJS Desktop View agacini Baslangic Merkezi veya secili suite uygulamasina gore Tauri webview rootuna baglar
// # 📌 Modul - FileType: View - TSX
// Version: 0.12.0
// Aciklama: Windows/Linux suite giris noktasini Tool katmanindan okuyup Start Center, Writer veya Sheet shell'ini ayni pencere icinde compose eder
// Bagimli Oldugu Katman: View -> Tool -> Config

import { createSignal, Match, Switch } from "solid-js";
import { render } from "solid-js/web";

import { APP_CONTAINER } from "./config/app-container";
import { ERROR_CODES } from "./config/error-codes";
import { OFFICE_MODULES, type OfficeModule } from "./config/office-modules";
import "./config/ui-tokens.css";
import { DesktopLaunchTool } from "./tools/desktop-launch.tool";
import "./views/app.css";
import { SheetShell } from "./views/sheet-shell";
import { StartCenter } from "./views/start-center";
import { WriterShell } from "./views/writer-shell";

async function bootstrapDesktopSuite(): Promise<void> {
  const root = document.getElementById("app");
  if (root === null) {
    throw new Error(ERROR_CODES.appRootMissing);
  }

  const launchTool = new DesktopLaunchTool();
  const initialModule = await launchTool.getModule();
  const [activeModule, setActiveModule] = createSignal<OfficeModule>(initialModule);
  const openModule = (module: OfficeModule): void => {
    setActiveModule(module);
  };

  render(
    () => (
      <Switch>
        <Match when={activeModule() === OFFICE_MODULES.writer}>
          <WriterShell
            controller={APP_CONTAINER.writerController}
            repository={APP_CONTAINER.writerSessionRepository}
            language={APP_CONTAINER.languageService}
            onHome={() => openModule(OFFICE_MODULES.home)}
          />
        </Match>
        <Match when={activeModule() === OFFICE_MODULES.sheet}>
          <SheetShell
            controller={APP_CONTAINER.sheetController}
            repository={APP_CONTAINER.sheetSessionRepository}
            language={APP_CONTAINER.languageService}
            onHome={() => openModule(OFFICE_MODULES.home)}
          />
        </Match>
        <Match when={true}>
          <StartCenter
            language={APP_CONTAINER.languageService}
            onOpenWriter={() => openModule(OFFICE_MODULES.writer)}
            onOpenSheet={() => openModule(OFFICE_MODULES.sheet)}
          />
        </Match>
      </Switch>
    ),
    root,
  );
}

void bootstrapDesktopSuite();
