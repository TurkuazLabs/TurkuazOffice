// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/native-file-dialog.tool.ts
// # 📌 Amac: Desktop native open/save/confirm dialoglarini Tauri plugin arkasinda kapsuller
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.2.0
// # Aciklama: Service katmanina platform dialog detaylarini sizdirmadan TKO dosya secimi ve discard onayi saglar
// Bagimli Oldugu Katman: Tool -> Config

import { confirm, open, save } from "@tauri-apps/plugin-dialog";

import { TKO_FILE_EXTENSIONS } from "../config/file-format";

export class NativeFileDialogTool {
  public openTko(filterName: string): Promise<string | null> {
    return open({
      multiple: false,
      directory: false,
      filters: [{ name: filterName, extensions: [...TKO_FILE_EXTENSIONS] }],
    });
  }

  public saveTko(filterName: string): Promise<string | null> {
    return save({
      filters: [{ name: filterName, extensions: [...TKO_FILE_EXTENSIONS] }],
    });
  }

  public confirmDiscard(message: string, title: string): Promise<boolean> {
    return confirm(message, { title, kind: "warning" });
  }
}
