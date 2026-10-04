// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/tauri-desktop-launch.tool.ts
// # 📌 Amac: Desktop Start Center ile Tauri suite launch IPC siniri arasinda typed adaptor saglar
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.4.0
// Aciklama: Baslangic modulunu okur ve Writer/Sheet'i ayri process olarak baslatir
// Bagimli Oldugu Katman: Tool -> Config

import { invoke } from "@tauri-apps/api/core";

import { IPC_COMMANDS } from "../config/ipc-commands";
import type { OfficeModule } from "../config/office-modules";

export interface DesktopLaunchContextView {
  readonly module: OfficeModule;
}

export class TauriDesktopLaunchTool {
  public getLaunchContext(): Promise<DesktopLaunchContextView> {
    return invoke<DesktopLaunchContextView>(IPC_COMMANDS.desktopGetLaunchContext);
  }

  public launchModule(module: OfficeModule): Promise<void> {
    return invoke<void>(IPC_COMMANDS.desktopLaunchModule, { module });
  }
}
