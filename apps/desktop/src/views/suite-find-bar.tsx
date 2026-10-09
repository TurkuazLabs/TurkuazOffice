// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/suite-find-bar.tsx
// # 📌 Amac: Writer ve Sheet Find panellerinin gercek ortak query/navigasyon arayuzunu render eder
// # 📌 Modul - FileType: View - TSX
// Version: 0.1.0
// Aciklama: Arama motorunu ortaklastirmadan sayac, wraparound, keyboard ve focus navigation davranisini paylastirir
// Bagimli Oldugu Katman: View -> Language

import { createMemo, createSignal } from "solid-js";

import type { LanguageService } from "../language/language-service";
import type { DesktopLabelKey } from "../language/labels";

export interface SuiteFindBarLabels {
  readonly find: DesktopLabelKey;
  readonly next: DesktopLabelKey;
  readonly previous: DesktopLabelKey;
  readonly close: DesktopLabelKey;
  readonly noMatches: DesktopLabelKey;
}

interface SuiteFindBarProps<TMatch> {
  readonly className: "writer-find-bar" | "sheet-find-bar";
  readonly labels: SuiteFindBarLabels;
  readonly language: LanguageService;
  readonly maxQueryScalars: number;
  readonly findMatches: (query: string) => readonly TMatch[];
  readonly onNavigate: (match: TMatch) => boolean | void | Promise<boolean | void>;
  readonly onClose: () => void;
  readonly inputRef: (element: HTMLInputElement) => void;
}

export function SuiteFindBar<TMatch>(props: SuiteFindBarProps<TMatch>) {
  const [query, setQuery] = createSignal("");
  const [activeIndex, setActiveIndex] = createSignal(-1);
  const matches = createMemo(() => props.findMatches(query()));
  let generation = 0;
  let navigating = false;

  const navigate = (direction: 1 | -1): void => {
    const results = matches();
    if (navigating || results.length === 0) {
      return;
    }

    const current = activeIndex();
    const next = current < 0
      ? (direction === 1 ? 0 : results.length - 1)
      : (current + direction + results.length) % results.length;
    const match = results[next];
    if (match === undefined) {
      return;
    }

    const requestGeneration = generation;
    const outcome = props.onNavigate(match);
    if (outcome instanceof Promise) {
      navigating = true;
      void outcome.then(
        (accepted) => {
          if (accepted !== false && requestGeneration === generation) {
            setActiveIndex(next);
          }
        },
        () => undefined,
      ).finally(() => {
        navigating = false;
      });
    } else if (outcome !== false && requestGeneration === generation) {
      setActiveIndex(next);
    }
  };

  return (
    <section class={props.className} role="search" aria-label={props.language.text(props.labels.find)}>
      <input
        ref={props.inputRef}
        type="search"
        class={`${props.className}__query`}
        aria-label={props.language.text(props.labels.find)}
        value={query()}
        maxLength={props.maxQueryScalars}
        onInput={(event) => {
          generation += 1;
          setQuery(event.currentTarget.value);
          setActiveIndex(-1);
        }}
        onKeyDown={(event) => {
          if (event.key === "Escape") {
            event.preventDefault();
            generation += 1;
            props.onClose();
          } else if (event.key === "Enter") {
            event.preventDefault();
            navigate(event.shiftKey ? -1 : 1);
          }
        }}
      />
      <span class={`${props.className}__results`} aria-live="polite">
        {query().length === 0
          ? "0 / 0"
          : matches().length === 0
            ? props.language.text(props.labels.noMatches)
            : `${activeIndex() >= 0 && activeIndex() < matches().length ? activeIndex() + 1 : 0} / ${matches().length}`}
      </span>
      <button
        type="button"
        disabled={matches().length === 0}
        aria-label={props.language.text(props.labels.previous)}
        onClick={() => navigate(-1)}
      >{props.language.text(props.labels.previous)}</button>
      <button
        type="button"
        disabled={matches().length === 0}
        aria-label={props.language.text(props.labels.next)}
        onClick={() => navigate(1)}
      >{props.language.text(props.labels.next)}</button>
      <button
        type="button"
        aria-label={props.language.text(props.labels.close)}
        onClick={() => {
          generation += 1;
          props.onClose();
        }}
      >{props.language.text(props.labels.close)}</button>
    </section>
  );
}
