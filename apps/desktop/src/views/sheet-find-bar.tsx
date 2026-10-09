// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-find-bar.tsx
// # 📌 Amac: Sheet gorunen grid hucreleri uzerinde gercek Bul arayuzu
// # 📌 Modul - FileType: View - TSX
// Version: 0.1.0
// Aciklama: Read-only arama, match count, onceki/sonraki ve hucre secimine navigasyon
// Bagimli Oldugu Katman: View -> Controller -> Service -> Tool -> Language

import { createMemo, createSignal } from "solid-js";

import type { SheetController } from "../controllers/sheet.controller";
import type { LanguageService } from "../language/language-service";
import { SHEET_FIND_MAX_QUERY_SCALARS, type SheetFindMatch } from "../tools/sheet-find.tool";

interface SheetFindBarProps {
  readonly controller: SheetController;
  readonly language: LanguageService;
  readonly visibleRows: readonly number[];
  readonly onNavigate: (match: SheetFindMatch) => Promise<void>;
  readonly onClose: () => void;
  readonly inputRef: (element: HTMLInputElement) => void;
}

export function SheetFindBar(props: SheetFindBarProps) {
  const [query, setQuery] = createSignal("");
  const [activeIndex, setActiveIndex] = createSignal(-1);
  const results = createMemo(() =>
    props.controller.findMatches(query(), props.language.locale(), props.visibleRows),
  );

  const navigate = async (direction: 1 | -1): Promise<void> => {
    const matches = results();
    if (matches.length === 0) {
      return;
    }
    const current = activeIndex();
    const next = current < 0
      ? (direction === 1 ? 0 : matches.length - 1)
      : (current + direction + matches.length) % matches.length;
    const match = matches[next];
    if (match !== undefined) {
      await props.onNavigate(match);
      setActiveIndex(next);
    }
  };

  return (
    <section class="sheet-find-bar" role="search" aria-label={props.language.text("sheetFind")}>
      <input
        ref={props.inputRef}
        type="search"
        class="sheet-find-bar__query"
        aria-label={props.language.text("sheetFind")}
        value={query()}
        maxLength={SHEET_FIND_MAX_QUERY_SCALARS}
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
            void navigate(event.shiftKey ? -1 : 1);
          }
        }}
      />
      <span class="sheet-find-bar__results" aria-live="polite">
        {query().length === 0
          ? "0 / 0"
          : results().length === 0
            ? props.language.text("sheetFindNoMatches")
            : `${activeIndex() >= 0 && activeIndex() < results().length ? activeIndex() + 1 : 0} / ${results().length}`}
      </span>
      <button
        type="button"
        disabled={results().length === 0}
        aria-label={props.language.text("sheetFindPrevious")}
        onClick={() => void navigate(-1)}
      >{props.language.text("sheetFindPrevious")}</button>
      <button
        type="button"
        disabled={results().length === 0}
        aria-label={props.language.text("sheetFindNext")}
        onClick={() => void navigate(1)}
      >{props.language.text("sheetFindNext")}</button>
      <button type="button" aria-label={props.language.text("sheetFindClose")} onClick={props.onClose}>
        {props.language.text("sheetFindClose")}
      </button>
    </section>
  );
}
