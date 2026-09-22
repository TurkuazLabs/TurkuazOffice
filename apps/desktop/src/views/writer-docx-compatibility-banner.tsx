// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-docx-compatibility-banner.tsx
// # 📌 Amac: DOCX import sirasinda kaybedilen yapisal ozellikleri kullaniciya gorunur sunar
// # 📌 Modul - FileType: View - TSX
// # Version: 0.2.0
// # Aciklama: Typed compatibility feature kodlarini Language katmani uzerinden okunabilir uyarida render eder
// Bagimli Oldugu Katman: View -> Config -> Language

import { For, Show } from "solid-js";

import { DOCX_UNSUPPORTED_FEATURE_LABEL_KEYS } from "../config/docx";
import type { LanguageService } from "../language/language-service";
import type { DocxUnsupportedFeatureView } from "./writer-types";

interface WriterDocxCompatibilityBannerProps {
  readonly features: readonly DocxUnsupportedFeatureView[];
  readonly language: LanguageService;
}

export function WriterDocxCompatibilityBanner(
  props: WriterDocxCompatibilityBannerProps,
) {
  return (
    <Show when={props.features.length > 0}>
      <aside class="writer-docx-banner" role="status">
        <strong>{props.language.text("docxCompatibilityWarning")}</strong>
        <span>
          <For each={props.features}>
            {(feature, index) => (
              <>
                {index() > 0 ? ", " : ""}
                {props.language.text(DOCX_UNSUPPORTED_FEATURE_LABEL_KEYS[feature])}
              </>
            )}
          </For>
        </span>
      </aside>
    </Show>
  );
}
