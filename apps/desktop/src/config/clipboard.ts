// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/clipboard.ts
// # 📌 Amac: Writer clipboard MIME, schema, limit ve sanitizer sabitlerini merkezi tutar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.2.0
// # Aciklama: Internal Turkuaz fragment, HTML ve plain-text representation kontratini magic string olmadan tanimlar
// Bagimli Oldugu Katman: Config

export const WRITER_CLIPBOARD_MIME = {
  internal: "application/x-turkuaz-office-fragment+json",
  html: "text/html",
  plainText: "text/plain",
} as const;

export const WRITER_CLIPBOARD_FRAGMENT_KIND = "turkuaz-writer-styled-fragment";
export const WRITER_CLIPBOARD_FRAGMENT_SCHEMA_VERSION = 1;
export const WRITER_CLIPBOARD_MAX_RUNS = 1_024;
export const WRITER_CLIPBOARD_MAX_TEXT_LENGTH = 1_000_000;
export const WRITER_CLIPBOARD_MAX_PAYLOAD_CHARS = 2_000_000;
export const WRITER_CLIPBOARD_MAX_DOM_NODES = 4_096;
export const WRITER_CLIPBOARD_MAX_DOM_DEPTH = 64;

export const WRITER_CLIPBOARD_BLOCKED_TAGS: ReadonlySet<string> = new Set([
  "script",
  "style",
  "iframe",
  "object",
  "embed",
  "svg",
  "math",
  "link",
  "meta",
]);

export const WRITER_CLIPBOARD_BOLD_TAGS: ReadonlySet<string> = new Set(["b", "strong"]);
export const WRITER_CLIPBOARD_ITALIC_TAGS: ReadonlySet<string> = new Set(["i", "em"]);
export const WRITER_CLIPBOARD_UNDERLINE_TAGS: ReadonlySet<string> = new Set(["u"]);
export const WRITER_CLIPBOARD_BREAK_TAGS: ReadonlySet<string> = new Set(["br"]);
export const WRITER_CLIPBOARD_BLOCK_TAGS: ReadonlySet<string> = new Set([
  "p",
  "div",
  "li",
  "pre",
  "blockquote",
  "h1",
  "h2",
  "h3",
  "h4",
  "h5",
  "h6",
]);
