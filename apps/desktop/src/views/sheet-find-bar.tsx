// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-find-bar.tsx
// # 📌 Amac: Sheet visible-grid Find sorgusunu ortak Suite Find arayuzune baglar
// # 📌 Modul - FileType: View - TSX
// Version: 0.1.0
// Aciklama: Sheet hucre navigasyonu ve sparse query modulu ozeldir, sayac ve klavye davranisi ortaktir
// Bagimli Oldugu Katman: View -> Controller -> Suite View -> Language

import type { SheetController } from "../controllers/sheet.controller";
import type { LanguageService } from "../language/language-service";
import { SHEET_FIND_MAX_QUERY_SCALARS, type SheetFindMatch } from "../tools/sheet-find.tool";
import { SuiteFindBar, type SuiteFindBarLabels } from "./suite-find-bar";

interface SheetFindBarProps {
  readonly controller: SheetController;
  readonly language: LanguageService;
  readonly visibleRows: readonly number[];
  readonly onNavigate: (match: SheetFindMatch) => Promise<void>;
  readonly onClose: () => void;
  readonly inputRef: (element: HTMLInputElement) => void;
}

const SHEET_FIND_LABELS: SuiteFindBarLabels = {
  find: "sheetFind",
  previous: "sheetFindPrevious",
  next: "sheetFindNext",
  close: "sheetFindClose",
  noMatches: "sheetFindNoMatches",
};

export function SheetFindBar(props: SheetFindBarProps) {
  return (
    <SuiteFindBar
      className="sheet-find-bar"
      labels={SHEET_FIND_LABELS}
      language={props.language}
      maxQueryScalars={SHEET_FIND_MAX_QUERY_SCALARS}
      findMatches={(query) =>
        props.controller.findMatches(query, props.language.locale(), props.visibleRows)}
      onNavigate={props.onNavigate}
      onClose={props.onClose}
      inputRef={props.inputRef}
    />
  );
}
