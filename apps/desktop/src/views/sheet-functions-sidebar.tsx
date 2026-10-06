// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-functions-sidebar.tsx
// # 📌 Amac: Sheet destekli fonksiyon katalogunu aranabilir sag panel olarak gosterir ve formul taslagi ekleme istegi uretir
// # 📌 Modul - FileType: View - TSX
// Version: 0.10.0
// Aciklama: SUM, AVERAGE, MIN, MAX ve IF fonksiyonlarini imza, aciklama ve formul cubuguna taslak ekleme aksiyonuyla sunar
// Bagimli Oldugu Katman: View -> Config -> Language

import { createSignal, For } from "solid-js";

import {
  SHEET_FUNCTION_CATALOG,
  type SheetFunctionId,
} from "../config/sheet-functions";
import type { LanguageService } from "../language/language-service";

interface SheetFunctionsSidebarProps {
  readonly language: LanguageService;
  readonly onClose: () => void;
  readonly onInsertDraft: (functionId: SheetFunctionId) => void;
}

export function SheetFunctionsSidebar(props: SheetFunctionsSidebarProps) {
  const [search, setSearch] = createSignal("");

  const visibleFunctions = () => {
    const rawQuery = search().trim();
    if (rawQuery.length === 0) {
      return SHEET_FUNCTION_CATALOG;
    }

    const engineQuery = rawQuery.toLowerCase();
    const localizedQuery = rawQuery.toLocaleLowerCase();

    return SHEET_FUNCTION_CATALOG.filter((definition) => {
      const engineSearch = [
        definition.id,
        definition.signature,
        ...definition.aliases,
      ]
        .join(" ")
        .toLowerCase();
      if (engineSearch.includes(engineQuery)) {
        return true;
      }

      const localizedSearch = [
        props.language.text(definition.labelKey),
        props.language.text(definition.descriptionKey),
      ]
        .join(" ")
        .toLocaleLowerCase();
      return localizedSearch.includes(localizedQuery);
    });
  };

  return (
    <aside
      class="sheet-sidebar sheet-functions-sidebar"
      aria-label={props.language.text("sheetFunctionsSidebarLabel")}
    >
      <header class="sheet-sidebar__header">
        <strong>{props.language.text("sheetFunctions")}</strong>
        <button
          type="button"
          class="sheet-sidebar__close"
          aria-label={props.language.text("sheetCloseSidebar")}
          onClick={props.onClose}
        >
          x
        </button>
      </header>

      <section class="sheet-sidebar__section">
        <label class="sheet-sidebar__field sheet-functions-sidebar__search">
          <span>{props.language.text("sheetFunctionSearch")}</span>
          <input
            type="search"
            value={search()}
            onInput={(event) => setSearch(event.currentTarget.value)}
          />
        </label>
        <p class="sheet-functions-sidebar__hint">
          {props.language.text("sheetFunctionDraftHint")}
        </p>
      </section>

      <section class="sheet-functions-sidebar__catalog">
        <For each={visibleFunctions()}>
          {(definition) => (
            <article class="sheet-function-card">
              <div class="sheet-function-card__title">
                <strong>{props.language.text(definition.labelKey)}</strong>
                <code>{definition.id}</code>
              </div>
              <p>{props.language.text(definition.descriptionKey)}</p>
              <code class="sheet-function-card__signature">{definition.signature}</code>
              <button
                type="button"
                class="toolbar-button toolbar-button--primary"
                onClick={() => props.onInsertDraft(definition.id)}
              >
                {props.language.text("sheetFunctionInsert")}
              </button>
            </article>
          )}
        </For>
      </section>
    </aside>
  );
}
