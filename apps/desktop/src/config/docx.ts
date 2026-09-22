// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/docx.ts
// # 📌 Amac: DOCX compatibility feature kodlarini Language label anahtarlarina merkezi map eder
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.2.0
// # Aciklama: View icinde foreign-format feature magic string kullanilmasini engeller
// Bagimli Oldugu Katman: Config -> Language -> View

import type { DesktopLabelKey } from "../language/labels";
import type { DocxUnsupportedFeatureView } from "../views/writer-types";

export const DOCX_UNSUPPORTED_FEATURE_LABEL_KEYS: Readonly<
  Record<DocxUnsupportedFeatureView, DesktopLabelKey>
> = {
  table: "docxFeatureTable",
  image: "docxFeatureImage",
  numbering: "docxFeatureNumbering",
  hyperlink: "docxFeatureHyperlink",
  header_footer: "docxFeatureHeaderFooter",
  comments: "docxFeatureComments",
  tracked_changes: "docxFeatureTrackedChanges",
  fields: "docxFeatureFields",
};
