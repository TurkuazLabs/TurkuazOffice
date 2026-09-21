// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/image-asset.tool.ts
// # 📌 Amac: Writer binary asset bytes'larini browser image ObjectURL yasam dongusune adapte eder
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.2.0
// # Aciklama: Blob/ObjectURL browser API detaylarini Service ve View katmanlarindan izole eder
// Bagimli Oldugu Katman: Tool -> View

import type { WriterAssetView } from "../views/writer-types";

export class ImageAssetTool {
  public createObjectUrl(asset: WriterAssetView): string {
    const bytes = Uint8Array.from(asset.data);
    return URL.createObjectURL(new Blob([bytes], { type: asset.mediaType }));
  }

  public revokeObjectUrl(url: string): void {
    URL.revokeObjectURL(url);
  }
}
