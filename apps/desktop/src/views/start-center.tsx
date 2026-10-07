// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/start-center.tsx
// # 📌 Amac: Turkuaz Office masaustu icin modern suite Baslangic Merkezi arayuzunu render eder
// # 📌 Modul - FileType: View - TSX
// Version: 0.12.0
// Aciklama: Writer ve Sheet'e hizli gecis, son belgeler/sablon alanlari ve gelecek modul kartlarini konsept tasarima uygun sunar
// Bagimli Oldugu Katman: View -> Language

import type { LanguageService } from "../language/language-service";
import { SuiteIcon } from "./suite-icon";

interface StartCenterProps {
  readonly language: LanguageService;
  readonly onOpenWriter: () => void;
  readonly onOpenSheet: () => void;
}

export function StartCenter(props: StartCenterProps) {
  return (
    <main class="start-center">
      <aside class="start-center__sidebar">
        <div class="start-center__brand">
          <SuiteIcon kind="brand" size={52} decorative />
          <div>
            <strong>{props.language.text("appName")}</strong>
            <span>{props.language.text("startCenterBrandTagline")}</span>
          </div>
        </div>

        <nav class="start-center__navigation" aria-label={props.language.text("startCenterNavigation")}>
          <span class="start-center__nav-item start-center__nav-item--active">
            <SuiteIcon kind="home" size={20} decorative />
            {props.language.text("startCenterName")}
          </span>
          <span class="start-center__nav-item">{props.language.text("startCenterRecent")}</span>
          <span class="start-center__nav-item">{props.language.text("startCenterTemplates")}</span>
        </nav>

        <div class="start-center__sidebar-actions">
          <span>{props.language.text("startCenterSettings")}</span>
          <span>{props.language.text("startCenterHelp")}</span>
        </div>
      </aside>

      <section class="start-center__content">
        <header class="start-center__hero">
          <div>
            <span class="start-center__eyebrow">{props.language.text("startCenterName")}</span>
            <h1>{props.language.text("startCenterTitle")}</h1>
            <p>{props.language.text("startCenterSubtitle")}</p>
          </div>
          <div class="start-center__search" aria-hidden="true">
            <span>⌕</span>
            <span>{props.language.text("startCenterSearch")}</span>
          </div>
        </header>

        <section class="start-center__modules" aria-label={props.language.text("startCenterApplications")}>
          <button type="button" class="start-center__module-card start-center__module-card--writer" onClick={props.onOpenWriter}>
            <SuiteIcon kind="writer" size={54} decorative />
            <span class="start-center__module-copy">
              <strong>{props.language.text("writerModule")}</strong>
              <span>{props.language.text("startCenterWriterDescription")}</span>
            </span>
            <span class="start-center__module-arrow" aria-hidden="true">›</span>
          </button>

          <button type="button" class="start-center__module-card start-center__module-card--sheet" onClick={props.onOpenSheet}>
            <SuiteIcon kind="sheet" size={54} decorative />
            <span class="start-center__module-copy">
              <strong>{props.language.text("sheetModule")}</strong>
              <span>{props.language.text("startCenterSheetDescription")}</span>
            </span>
            <span class="start-center__module-arrow" aria-hidden="true">›</span>
          </button>

          <article class="start-center__module-card start-center__module-card--disabled">
            <SuiteIcon kind="presentation" size={54} decorative />
            <span class="start-center__module-copy">
              <strong>{props.language.text("startCenterPresentation")}</strong>
              <span>{props.language.text("startCenterComingSoon")}</span>
            </span>
          </article>

          <article class="start-center__module-card start-center__module-card--disabled">
            <SuiteIcon kind="pdf" size={54} decorative />
            <span class="start-center__module-copy">
              <strong>{props.language.text("startCenterPdf")}</strong>
              <span>{props.language.text("startCenterComingSoon")}</span>
            </span>
          </article>
        </section>

        <section class="start-center__section">
          <div class="start-center__section-heading">
            <div>
              <h2>{props.language.text("startCenterRecent")}</h2>
              <p>{props.language.text("startCenterRecentDescription")}</p>
            </div>
          </div>
          <div class="start-center__recent-grid">
            <article class="start-center__empty-card">
              <SuiteIcon kind="writer" size={38} decorative />
              <div>
                <strong>{props.language.text("startCenterRecentWriterTitle")}</strong>
                <span>{props.language.text("startCenterRecentWriterDescription")}</span>
              </div>
            </article>
            <article class="start-center__empty-card">
              <SuiteIcon kind="sheet" size={38} decorative />
              <div>
                <strong>{props.language.text("startCenterRecentSheetTitle")}</strong>
                <span>{props.language.text("startCenterRecentSheetDescription")}</span>
              </div>
            </article>
          </div>
        </section>

        <section class="start-center__section start-center__section--templates">
          <div class="start-center__section-heading">
            <div>
              <h2>{props.language.text("startCenterTemplates")}</h2>
              <p>{props.language.text("startCenterTemplatesDescription")}</p>
            </div>
          </div>
          <div class="start-center__template-grid">
            <button type="button" onClick={props.onOpenWriter}>
              <SuiteIcon kind="writer" size={32} decorative />
              <span>{props.language.text("templateBlank")}</span>
            </button>
            <button type="button" onClick={props.onOpenWriter}>
              <SuiteIcon kind="writer" size={32} decorative />
              <span>{props.language.text("templateLetter")}</span>
            </button>
            <button type="button" onClick={props.onOpenWriter}>
              <SuiteIcon kind="writer" size={32} decorative />
              <span>{props.language.text("templateReport")}</span>
            </button>
            <button type="button" onClick={props.onOpenSheet}>
              <SuiteIcon kind="sheet" size={32} decorative />
              <span>{props.language.text("startCenterBudgetTemplate")}</span>
            </button>
          </div>
        </section>
      </section>
    </main>
  );
}
