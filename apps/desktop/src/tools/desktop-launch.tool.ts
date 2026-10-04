// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/desktop-launch.tool.ts
// # 📌 Amac: Native suite baslatma hedefini Tauri IPC uzerinden frontend composition rootuna tasir
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.5.0
// Aciklama: Writer/Sheet giris secimini native komuttan typed OfficeModule degerine map eder
// Bagimli Oldugu Katman: Tool -> Config

import { invoke } from "@tauri-apps/api/core";

import { OFFICE_MODULES, type OfficeModule } from "../config/office-modules";

const DESKTOP_LAUNCH_COMMAND = "desktop_get_launch_module";

export class DesktopLaunchTool {
  async getModule(): Promise<OfficeModule> {
    const module = await invoke<string>(DESKTOP_LAUNCH_COMMAND);
    return module === OFFICE_MODULES.sheet ? OFFICE_MODULES.sheet : OFFICE_MODULES.writer;
  }
}
