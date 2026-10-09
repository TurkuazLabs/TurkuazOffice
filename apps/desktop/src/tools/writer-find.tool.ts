// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/writer-find.tool.ts
// # 📌 Amac: Canonical Writer read-model paragraflarinda Unicode-safe metin eslesmelerini bulur
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.1.0
// Aciklama: Writer belge metnini degistirmeden locale-aware arar, Unicode scalar offsetlerini dondurur
// Bagimli Oldugu Katman: Tool -> View Model

import type { WriterDocumentView, WriterSelectionView } from "../views/writer-types";

export type WriterFindMatch = WriterSelectionView;

export const WRITER_FIND_MAX_QUERY_SCALARS = 128;
export const WRITER_FIND_MAX_RESULTS = 1000;

export function findWriterMatches(
  document: WriterDocumentView,
  query: string,
  locale: string,
  maxResults = WRITER_FIND_MAX_RESULTS,
): readonly WriterFindMatch[] {
  const needle = Array.from(query);
  if (
    needle.length === 0 ||
    needle.length > WRITER_FIND_MAX_QUERY_SCALARS ||
    !Number.isSafeInteger(maxResults) ||
    maxResults < 1 ||
    maxResults > WRITER_FIND_MAX_RESULTS + 1
  ) {
    return [];
  }

  const foldedNeedle = query.toLocaleLowerCase(locale);
  const results: WriterFindMatch[] = [];

  for (const paragraph of document.paragraphs) {
    const scalars = Array.from(paragraph.plainText);
    if (scalars.length < needle.length) {
      continue;
    }

    for (let start = 0; start <= scalars.length - needle.length; start++) {
      const candidate = scalars.slice(start, start + needle.length).join("");
      if (candidate.toLocaleLowerCase(locale) === foldedNeedle) {
        results.push({
          paragraphId: paragraph.id,
          startOffset: start,
          endOffset: start + needle.length,
        });
        if (results.length >= maxResults) {
          return results;
        }
      }
    }
  }
  return results;
}
