// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-find-bar.tsx
// # 📌 Amac: Writer canonical belge Find servisini ortak Suite Find arayuzune baglar
// # 📌 Modul - FileType: View - TSX
// Version: 0.1.0
// Aciklama: Writer metin eslesme/odak davranisi kendi Controller'indadir, genel navigasyon SuiteFindBar'dadir
// Bagimli Oldugu Katman: View -> Controller -> Suite View -> Language

import { WRITER_FIND_MAX_QUERY_SCALARS } from "../tools/writer-find.tool";
import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";
import { SuiteFindBar, type SuiteFindBarLabels } from "./suite-find-bar";

interface WriterFindBarProps {
  readonly controller: WriterController;
  readonly language: LanguageService;
  readonly onClose: () => void;
  readonly inputRef: (element: HTMLInputElement) => void;
}

const WRITER_FIND_LABELS: SuiteFindBarLabels = {
  find: "writerFind",
  previous: "writerFindPrevious",
  next: "writerFindNext",
  close: "writerFindClose",
  noMatches: "writerFindNoMatches",
};

export function WriterFindBar(props: WriterFindBarProps) {
  return (
    <SuiteFindBar
      className="writer-find-bar"
      labels={WRITER_FIND_LABELS}
      language={props.language}
      maxQueryScalars={WRITER_FIND_MAX_QUERY_SCALARS}
      findMatches={(query) => props.controller.findMatches(query, props.language.locale())}
      onNavigate={(match) => props.controller.focusFindMatch(match)}
      replaceOne={{
        label: "writerReplaceOne",
        placeholder: "writerReplaceWith",
        canReplace: () => props.controller.canReplaceFoundMatch(),
        perform: (match, query, replacement) =>
          props.controller.replaceFoundMatch(match, query, replacement, props.language.locale()),
      }}
      onClose={props.onClose}
      inputRef={props.inputRef}
    />
  );
}
