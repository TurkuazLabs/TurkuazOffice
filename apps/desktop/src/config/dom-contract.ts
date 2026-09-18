// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/dom-contract.ts
// # 📌 Amac: Writer DOM marker degerlerini ve selector kontratini tek merkezde tanimlar
// # 📌 Modul - FileType: Config - TypeScript
// Version: 0.2.0
// Aciklama: View ve Tool katmanlarinin contenteditable paragraph marker kontratini magic string kullanmadan paylasmasini saglar
// Bagimli Oldugu Katman: Config

export const WRITER_PARAGRAPH_MARKER_VALUE = "writer-paragraph" as const;
export const WRITER_PARAGRAPH_SELECTOR = `[data-writer-paragraph="${WRITER_PARAGRAPH_MARKER_VALUE}"]` as const;
