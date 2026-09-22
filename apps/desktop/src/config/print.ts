// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/print.ts
// # 📌 Amac: Writer print preview ve sistem yazdirma adaptor sabitlerini merkezi configte tutar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.2.0
// # Aciklama: Preview zoom, dinamik page-style kimligi ve sistem dialog ownership kontratini tanimlar
// Bagimli Oldugu Katman: Config

export const WRITER_PRINT_PREVIEW_ZOOM_PERCENT = 100;
export const WRITER_PRINT_STYLE_ELEMENT_ID = "turkuaz-writer-print-page-style";

export const WRITER_PRINT_SYSTEM_DIALOG_CAPABILITIES = [
  "printer",
  "page_range",
  "copies",
  "orientation",
  "paper_size",
  "margin",
  "scale",
] as const;
