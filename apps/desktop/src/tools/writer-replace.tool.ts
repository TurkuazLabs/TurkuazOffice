// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/writer-replace.tool.ts
// # 📌 Amac: Writer tek eslesme Replace isleminin salt-okunur ve Unicode guvenli planini olusturur
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.1.0
// Aciklama: Degisen araligi ve orijinal run stilini dogrular; mutasyon sadece mevcut Rust command'da yapilir
// Bagimli Oldugu Katman: Tool -> View Model

import type {
  WriterDocumentView,
  WriterSelectionView,
  WriterStyledRunInputView,
} from "../views/writer-types";
import { WRITER_FIND_MAX_QUERY_SCALARS } from "./writer-find.tool";

const WRITER_REPLACE_MAX_SCALARS = 4096;

export interface WriterReplaceOnePlan {
  readonly documentId: string;
  readonly revision: number;
  readonly paragraphId: string;
  readonly startOffset: number;
  readonly endOffset: number;
  readonly runs: readonly WriterStyledRunInputView[];
}

export function planWriterReplaceOne(
  document: WriterDocumentView,
  match: WriterSelectionView,
  expectedQuery: string,
  replacement: string,
  locale: string,
): WriterReplaceOnePlan | null {
  const queryLength = Array.from(expectedQuery).length;
  const replacementLength = Array.from(replacement).length;
  if (
    queryLength === 0 ||
    queryLength > WRITER_FIND_MAX_QUERY_SCALARS ||
    replacementLength > WRITER_REPLACE_MAX_SCALARS ||
    /[\r\n]/u.test(replacement) ||
    !Number.isSafeInteger(match.startOffset) ||
    !Number.isSafeInteger(match.endOffset) ||
    match.startOffset < 0 ||
    match.endOffset <= match.startOffset
  ) {
    return null;
  }

  const paragraph = document.paragraphs.find((item) => item.id === match.paragraphId);
  if (paragraph === undefined) {
    return null;
  }

  const characters = Array.from(paragraph.plainText);
  if (match.endOffset > characters.length) {
    return null;
  }
  const original = characters.slice(match.startOffset, match.endOffset).join("");
  if (
    original.toLocaleLowerCase(locale) !== expectedQuery.toLocaleLowerCase(locale) ||
    original === replacement
  ) {
    return null;
  }

  let cursor = 0;
  let insertionStyle: WriterStyledRunInputView["style"] | null = null;
  for (const run of paragraph.runs) {
    const end = cursor + Array.from(run.text).length;
    if (match.startOffset >= cursor && match.startOffset < end) {
      insertionStyle = run.style;
    }
    cursor = end;
  }
  // An inconsistent DTO must never produce a formatting-losing mutation.
  if (cursor !== characters.length || insertionStyle === null) {
    return null;
  }

  return {
    documentId: document.id,
    revision: document.revision,
    paragraphId: match.paragraphId,
    startOffset: match.startOffset,
    endOffset: match.endOffset,
    runs: replacement.length === 0 ? [] : [{ text: replacement, style: insertionStyle }],
  };
}
