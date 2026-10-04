// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/start-center.tsx
// # 📌 Amac: Turkuaz Office suite icin LibreOffice benzeri baslangic merkezi yuzeyini render eder
// # 📌 Modul - FileType: View - TSX
// Version: 0.4.0
// Aciklama: Writer ve Sheet'i ayri process olarak baslatir; planlanan suite modullerini pasif kart olarak gosterir
// Bagimli Oldugu Katman: View -> Tool -> Language -> Config

import { OFFICE_MODULES, type OfficeModule } from "../config/office-modules";
import type { LanguageService } from "../language/language-service";

interface StartCenterProps {
  readonly language: LanguageService;
  readonly onLaunchModule: (module: OfficeModule) => void;
}

export function StartCenter(props: StartCenterProps) {
  return (
    <main class="start-center">
      <header class="start-center__header">
        <div>
          <p class="start-center__eyebrow">{props.language.text("appName")}</p>
          <h1>{props.language.text("startCenterTitle")}</h1>
          <p>{props.language.text("startCenterDescription")}</p>
        </div>
      </header>

      <section class="start-center__apps" aria-label={props.language.text("appName")}>
        <button
          type="button"
          class="start-center-card start-center-card--active"
          onClick={() => props.onLaunchModule(OFFICE_MODULES.writer)}
        >
          <span class="start-center-card__icon" aria-hidden="true">W</span>
          <span class="start-center-card__body">
            <strong>{props.language.text("writerAppName")}</strong>
            <small>{props.language.text("startCenterWriterDescription")}</small>
          </span>
          <span class="start-center-card__action">{props.language.text("launchModule")}</span>
        </button>

        <button
          type="button"
          class="start-center-card start-center-card--active"
          onClick={() => props.onLaunchModule(OFFICE_MODULES.sheet)}
        >
          <span class="start-center-card__icon" aria-hidden="true">S</span>
          <span class="start-center-card__body">
            <strong>{props.language.text("sheetAppName")}</strong>
            <small>{props.language.text("startCenterSheetDescription")}</small>
          </span>
          <span class="start-center-card__action">{props.language.text("launchModule")}</span>
        </button>

        <article class="start-center-card start-center-card--planned" aria-disabled="true">
          <span class="start-center-card__icon" aria-hidden="true">P</span>
          <span class="start-center-card__body">
            <strong>{props.language.text("slidesModule")}</strong>
            <small>{props.language.text("plannedModule")}</small>
          </span>
        </article>

        <article class="start-center-card start-center-card--planned" aria-disabled="true">
          <span class="start-center-card__icon" aria-hidden="true">D</span>
          <span class="start-center-card__body">
            <strong>{props.language.text("drawModule")}</strong>
            <small>{props.language.text("plannedModule")}</small>
          </span>
        </article>
      </section>
    </main>
  );
}
