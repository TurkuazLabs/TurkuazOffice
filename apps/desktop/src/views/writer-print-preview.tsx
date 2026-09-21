// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-print-preview.tsx
// # 📌 Amac: Writer canonical belgeyi session-only print preview yuzeyinde render eder
// # 📌 Modul - FileType: View - TSX
// # Version: 0.2.0
// # Aciklama: Belge modelini mutate etmeden print layout, sistem dialog bilgisi, print ve close komutlarini sunar
// Bagimli Oldugu Katman: View -> Controller -> Language

import { Show } from "solid-js";

import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";
import { WriterPage } from "./writer-page";
import type {
  WriterDocumentView,
  WriterPageLayoutView,
  WriterResolvedFontView,
} from "./writer-types";

interface WriterPrintPreviewProps {
  readonly document: WriterDocumentView;
  readonly controller: WriterController;
  readonly language: LanguageService;
  readonly layout: WriterPageLayoutView;
  readonly fontResolutions: readonly WriterResolvedFontView[];
  readonly errorCode: string | null;
}

export function WriterPrintPreview(props: WriterPrintPreviewProps) {
  return (
    <section
      class="writer-print-preview"
      role="dialog"
      aria-modal="true"
      aria-label={props.language.text("printPreviewTitle")}
    >
      <header class="writer-print-preview__toolbar">
        <div class="writer-print-preview__summary">
          <strong>{props.language.text("printPreviewTitle")}</strong>
          <span>{props.language.text("printPreviewDescription")}</span>
          <Show when={props.errorCode !== null}>
            <span class="writer-print-preview__error" role="alert">
              {props.language.text("printFailed")}: {props.errorCode}
            </span>
          </Show>
        </div>
        <div class="writer-print-preview__actions">
          <button
            type="button"
            class="toolbar-button toolbar-button--primary"
            onClick={() => void props.controller.printDocument()}
          >
            {props.language.text("printNow")}
          </button>
          <button
            type="button"
            class="toolbar-button"
            onClick={() => props.controller.closePrintPreview()}
          >
            {props.language.text("close")}
          </button>
        </div>
      </header>

      <div class="writer-print-preview__surface">
        <WriterPage
          document={props.document}
          controller={props.controller}
          language={props.language}
          ariaLabel={props.language.text("printPreviewDocumentAreaLabel")}
          readOnly={true}
          layout={props.layout}
          fontResolutions={props.fontResolutions}
        />
      </div>
    </section>
  );
}
