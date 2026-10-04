// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/main.tsx
// # 📌 Amac: SolidJS Desktop View agacini suite launch contextine gore Tauri webview rootuna baglar
// # 📌 Modul - FileType: View - TSX
// Version: 0.4.0
// Aciklama: Start Center, Writer veya Sheet shell'ini process launch modulune gore ayri uygulama yuzeyi olarak render eder
// Bagimli Oldugu Katman: View -> Config -> Tool

import { Match, Switch } from "solid-js";
import { render } from "solid-js/web";

import { APP_CONTAINER } from "./config/app-container";
import { ERROR_CODES } from "./config/error-codes";
import { OFFICE_MODULES } from "./config/office-modules";
import "./config/ui-tokens.css";
import "./views/app.css";
import { SheetShell } from "./views/sheet-shell";
import { StartCenter } from "./views/start-center";
import { WriterShell } from "./views/writer-shell";

const root = document.getElementById("app");
if (root === null) {
  throw new Error(ERROR_CODES.appRootMissing);
}

async function bootstrapDesktop(): Promise<void> {
  const launchContext = await APP_CONTAINER.desktopLaunchTool
    .getLaunchContext()
    .catch(() => ({ module: OFFICE_MODULES.start }));

  render(
    () => (
      <Switch
        fallback={
          <StartCenter
            language={APP_CONTAINER.languageService}
            onLaunchModule={(module) => {
              void APP_CONTAINER.desktopLaunchTool.launchModule(module);
            }}
          />
        }
      >
        <Match when={launchContext.module === OFFICE_MODULES.writer}>
          <WriterShell
            controller={APP_CONTAINER.writerController}
            repository={APP_CONTAINER.writerSessionRepository}
            language={APP_CONTAINER.languageService}
          />
        </Match>
        <Match when={launchContext.module === OFFICE_MODULES.sheet}>
          <SheetShell
            controller={APP_CONTAINER.sheetController}
            repository={APP_CONTAINER.sheetSessionRepository}
            language={APP_CONTAINER.languageService}
          />
        </Match>
      </Switch>
    ),
    root,
  );
}

void bootstrapDesktop();
