// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-page.tsx
// # 📌 Amac: Writer document read-modelini canonical page geometry ile Desktop yuzeyinde render eder
// # 📌 Modul - FileType: View - TSX
// # Version: 0.2.0
// # Aciklama: PageSettings, zoom layout ve font resolution read-modelini rich-text paragraph yuzeyine uygular
// Bagimli Oldugu Katman: View -> Controller -> Language

import { Index, type Accessor } from "solid-js";

import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";
import { WriterParagraphEditor } from "./writer-paragraph";
import type {
  WriterDocumentView,
  WriterPageLayoutView,
  WriterParagraphView,
  WriterResolvedFontView,
} from "./writer-types";

interface WriterPageProps {
  readonly document: WriterDocumentView;
  readonly controller: WriterController;
  readonly language: LanguageService;
  readonly ariaLabel: string;
  readonly readOnly: boolean;
  readonly layout: WriterPageLayoutView;
  readonly fontResolutions: readonly WriterResolvedFontView[];
}

export function WriterPage(props: WriterPageProps) {
  const pageStyle = () => ({
    width: `${props.layout.pageWidthPx}px`,
    "min-height": `${props.layout.pageHeightPx}px`,
    padding: `${props.layout.marginTopPx}px ${props.layout.marginRightPx}px ${props.layout.marginBottomPx}px ${props.layout.marginLeftPx}px`,
  });

  return (
    <main class="writer-workspace" aria-label={props.ariaLabel}>
      <section class="writer-page" aria-label={props.document.title} style={pageStyle()}>
        <Index each={props.document.paragraphs}>
          {(paragraph: Accessor<WriterParagraphView>, index: number) => (
            <WriterParagraphEditor
              paragraph={paragraph()}
              controller={props.controller}
              language={props.language}
              canMergeWithPrevious={index > 0}
              readOnly={props.readOnly}
              renderScale={props.layout.scale}
              fontResolutions={props.fontResolutions}
            />
          )}
        </Index>
      </section>
    </main>
  );
}
