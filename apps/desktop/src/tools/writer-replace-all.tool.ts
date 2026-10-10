// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/writer-replace-all.tool.ts
// # 📌 Amac: Writer Replace All icin salt-okunur, eksiksiz, atomik batch'e uygun plan olusturur
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.1.0
// Aciklama: Bounded +1 truncation guard, overlap policy, scalar offset ve mevcut stil-preserving Replace planini kullanir
// Bagimli Oldugu Katman: Tool -> Writer canonical read model

import type { WriterDocumentView } from "../views/writer-types";
import {
  findWriterMatches,
  WRITER_FIND_MAX_RESULTS,
} from "./writer-find.tool";
import {
  planWriterReplaceOne,
  type WriterReplaceOnePlan,
} from "./writer-replace.tool";

export interface WriterReplaceAllPlan {
  readonly documentId: string;
  readonly revision: number;
  // Descending document/paragraph offsets ensure earlier text positions are stable
  // if the future native batch accepts scalar coordinates.
  readonly replacements: readonly WriterReplaceOnePlan[];
}

export function planWriterReplaceAll(
  document: WriterDocumentView,
  query: string,
  replacement: string,
  locale: string,
): WriterReplaceAllPlan | null {
  // Detect a truncated search instead of silently replacing a partial document.
  const candidates = findWriterMatches(
    document, query, locale, WRITER_FIND_MAX_RESULTS + 1,
  );
  if (candidates.length === 0 || candidates.length > WRITER_FIND_MAX_RESULTS) {
    return null;
  }

  const selected: WriterReplaceOnePlan[] = [];
  const lastEndByParagraph = new Map<string, number>();
  for (const candidate of candidates) {
    const previousEnd = lastEndByParagraph.get(candidate.paragraphId) ?? -1;
    if (candidate.startOffset < previousEnd) {
      // Left-to-right, non-overlapping replacements, matching common editor semantics.
      continue;
    }
    const plan = planWriterReplaceOne(
      document, candidate, query, replacement, locale,
    );
    if (plan === null || plan.documentId !== document.id || plan.revision !== document.revision) {
      return null;
    }
    selected.push(plan);
    lastEndByParagraph.set(candidate.paragraphId, candidate.endOffset);
  }

  if (selected.length === 0) {
    return null;
  }

  // Each paragraph is searched in canonical order, so reverse order prevents
  // earlier scalar ranges shifting as later matches are replaced.
  return {
    documentId: document.id,
    revision: document.revision,
    replacements: selected.reverse(),
  };
}
