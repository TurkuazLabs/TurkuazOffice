// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-find-bar.tsx
// # 📌 Amac: Writer canonical belge eslesmeleri icin arama ve onceki/sonraki navigasyon cizgisi
// # 📌 Modul - FileType: View - TSX
// Version: 0.1.0
// Aciklama: Read-only query, match count ve secime odaklanma; replace veya HTML icinde arama yapmaz
// Bagimli Oldugu Katman: View -> Controller -> Service -> Tool -> Language

import { createMemo, createSignal } from "solid-js";

import { WRITER_FIND_MAX_QUERY_SCALARS } from "../tools/writer-find.tool";
import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";

interface WriterFindBarProps {
  readonly controller: WriterController;
  readonly language: LanguageService;
  readonly onClose: () => void;
  readonly inputRef: (element: HTMLInputElement) => void;
}

export function WriterFindBar(props: WriterFindBarProps) {
  const [query, setQuery] = createSignal("");
  const [activeIndex, setActiveIndex] = createSignal(-1);
  const matches = createMemo(() =>
    props.controller.findMatches(query(), props.language.locale()),
  );

  const navigate = (direction: 1 | -1): void => {
    const results = matches();
    if (results.length === 0) {
      return;
    }
    const current = activeIndex();
    const next = current < 0
      ? (direction === 1 ? 0 : results.length - 1)
      : (current + direction + results.length) % results.length;
    const match = results[next];
    if (match !== undefined && props.controller.focusFindMatch(match)) {
      setActiveIndex(next);
    }
  };

  return (
    <section class="writer-find-bar" role="search" aria-label={props.language.text("writerFind")}>
      <input
        ref={props.inputRef}
        type="search"
        class="writer-find-bar__query"
        aria-label={props.language.text("writerFind")}
        value={query()}
        maxLength={WRITER_FIND_MAX_QUERY_SCALARS}
        onInput={(event) => {
          setQuery(event.currentTarget.value);
          setActiveIndex(-1);
        }}
        onKeyDown={(event) => {
          if (event.key === "Escape") {
            event.preventDefault();
            props.onClose();
          } else if (event.key === "Enter") {
            event.preventDefault();
            navigate(event.shiftKey ? -1 : 1);
          }
        }}
      />
      <span class="writer-find-bar__results" aria-live="polite">
        {query().length === 0
          ? "0 / 0"
          : matches().length === 0
            ? props.language.text("writerFindNoMatches")
            : `${activeIndex() >= 0 && activeIndex() < matches().length ? activeIndex() + 1 : 0} / ${matches().length}`}
      </span>
      <button type="button" disabled={matches().length === 0}
        aria-label={props.language.text("writerFindPrevious")}
        onClick={() => navigate(-1)}>
        {props.language.text("writerFindPrevious")}
      </button>
      <button type="button" disabled={matches().length === 0}
        aria-label={props.language.text("writerFindNext")}
        onClick={() => navigate(1)}>
        {props.language.text("writerFindNext")}
      </button>
      <button type="button" aria-label={props.language.text("writerFindClose")}
        onClick={props.onClose}>
        {props.language.text("writerFindClose")}
      </button>
    </section>
  );
}
