// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/main.tsx
// # 📌 Amac: SolidJS Desktop View agacini secili suite uygulamasina gore Tauri webview rootuna baglar
// # 📌 Modul - FileType: View - TSX
// Version: 0.5.0
// Aciklama: Windows/Linux suite giris noktasini Tool katmanindan okuyup yalnizca Writer veya Sheet shell'ini render eder
// Bagimli Oldugu Katman: View -> Tool -> Config

import { render } from "solid-js/web";

import { APP_CONTAINER } from "./config/app-container";
import { ERROR_CODES } from "./config/error-codes";
import { OFFICE_MODULES } from "./config/office-modules";
import "./config/ui-tokens.css";
import { DesktopLaunchTool } from "./tools/desktop-launch.tool";
import "./views/app.css";
import { SheetShell } from "./views/sheet-shell";
import { WriterShell } from "./views/writer-shell";

async function bootstrapDesktopSuite(): Promise<void> {
  const root = document.getElementById("app");
  if (root === null) {
    throw new Error(ERROR_CODES.appRootMissing);
  }

  const launchTool = new DesktopLaunchTool();
  const activeModule = await launchTool.getModule();

  render(
    () =>
      activeModule === OFFICE_MODULES.sheet ? (
        <SheetShell
          controller={APP_CONTAINER.sheetController}
          repository={APP_CONTAINER.sheetSessionRepository}
          language={APP_CONTAINER.languageService}
        />
      ) : (
        <WriterShell
          controller={APP_CONTAINER.writerController}
          repository={APP_CONTAINER.writerSessionRepository}
          language={APP_CONTAINER.languageService}
        />
      ),
    root,
  );
}

void bootstrapDesktopSuite();
