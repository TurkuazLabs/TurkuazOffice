// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-statusbar.tsx
// # 📌 Amac: Writer Desktop oturum, font fallback ve zoom durumunu alt bilgi cubugunda gosterir
// # 📌 Modul - FileType: View - TSX
// # Version: 0.2.0
// # Aciklama: File path, dirty state, revision, fallback bilgisi ve session-only zoom kontrollerini render eder
// Bagimli Oldugu Katman: View -> Controller -> Language

import { WRITER_ARIA_SHORTCUTS } from "../config/keyboard";
import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";
import type {
  WriterDocumentView,
  WriterFileSessionView,
  WriterResolvedFontView,
} from "./writer-types";

interface WriterStatusbarProps {
  readonly controller: WriterController;
  readonly document: WriterDocumentView | null;
  readonly language: LanguageService;
  readonly statusText: string;
  readonly filePath: string | null;
  readonly dirty: boolean;
  readonly recoveryRevision: number | null;
  readonly recoveryErrorCode: string | null;
  readonly fileSession: WriterFileSessionView | null;
  readonly zoomPercent: number;
  readonly fontResolutions: readonly WriterResolvedFontView[];
}

export function WriterStatusbar(props: WriterStatusbarProps) {
  const hasSubstitution = () => props.fontResolutions.some((font) => font.substituted);

  return (
    <footer class="writer-statusbar">
      <span>{props.statusText}</span>
      <span class="writer-statusbar__spacer" />
      <span>{props.language.text("offlineLocal")}</span>
      <span>{props.dirty ? props.language.text("unsavedState") : props.language.text("savedState")}</span>
      <span>{props.filePath ?? props.language.text("untitledDocument")}</span>
      {props.fileSession?.readOnly === true && <span>{props.language.text("readOnlyState")}</span>}
      {props.fileSession?.lockOwned === true && <span>{props.language.text("fileLockOwned")}</span>}
      {props.recoveryRevision !== null && (
        <span>{props.language.text("autosaveRecovery")}: {props.recoveryRevision}</span>
      )}
      {props.recoveryErrorCode !== null && (
        <span>{props.language.text("autosaveRecoveryFailed")}</span>
      )}
      {hasSubstitution() && <span>{props.language.text("fontSubstituted")}</span>}
      {props.document !== null && (
        <>
          <span>{props.language.text("paragraphs")}: {props.document.paragraphs.length}</span>
          <span>{props.language.text("revision")}: {props.document.revision}</span>
        </>
      )}
      <label class="writer-statusbar__language">
        <span>{props.language.text("language")}</span>
        <select
          aria-label={props.language.text("language")}
          value={props.controller.locale()}
          onChange={(event) => props.controller.setLocale(event.currentTarget.value)}
        >
          {props.controller.localeOptions().map((option) => (
            <option value={option.locale}>
              {props.language.text(option.labelKey)}
            </option>
          ))}
        </select>
      </label>
      <div class="writer-statusbar__zoom" aria-label={props.language.text("zoom")}>
        <button
          type="button"
          class="statusbar-button"
          aria-label={props.language.text("zoomOut")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.zoomOut}
          onClick={() => props.controller.zoomOut()}
        >
          -
        </button>
        <button
          type="button"
          class="statusbar-button statusbar-button--zoom"
          aria-label={props.language.text("zoomReset")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.zoomReset}
          onClick={() => props.controller.resetZoom()}
        >
          {props.zoomPercent}%
        </button>
        <button
          type="button"
          class="statusbar-button"
          aria-label={props.language.text("zoomIn")}
          aria-keyshortcuts={WRITER_ARIA_SHORTCUTS.zoomIn}
          onClick={() => props.controller.zoomIn()}
        >
          +
        </button>
      </div>
    </footer>
  );
}
