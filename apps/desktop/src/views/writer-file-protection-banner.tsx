// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-file-protection-banner.tsx
// # 📌 Amac: File lock ve external-change durumlarini kullaniciya eylemli bir banner olarak gosterir
// # 📌 Modul - FileType: View - TSX
// # Version: 0.2.0
// # Aciklama: Read-only, modified ve missing durumlarinda reload, keep-local ve save-as komutlarini Controller'a iletir
// Bagimli Oldugu Katman: View -> Controller -> Repo -> Language

import { Show } from "solid-js";

import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";
import type { WriterSessionRepository } from "../repositories/writer-session.repository";

interface WriterFileProtectionBannerProps {
  readonly controller: WriterController;
  readonly repository: WriterSessionRepository;
  readonly language: LanguageService;
}

export function WriterFileProtectionBanner(props: WriterFileProtectionBannerProps) {
  const session = () => props.repository.fileSession();
  const hasExternalChange = () => {
    const state = session()?.externalState;
    return state === "modified" || state === "missing";
  };
  const visible = () => session()?.readOnly === true || hasExternalChange();
  const message = () => {
    const current = session();
    if (current?.externalState === "modified") {
      return props.language.text("externalChangeModified");
    }
    if (current?.externalState === "missing") {
      return props.language.text("externalChangeMissing");
    }
    return props.language.text("fileLockedDescription");
  };

  return (
    <Show when={visible()}>
      <aside class="writer-protection-banner" role="status" aria-live="polite">
        <div class="writer-protection-banner__text">
          <strong>
            {hasExternalChange()
              ? props.language.text("externalChangeTitle")
              : props.language.text("fileLocked")}
          </strong>
          <span>{message()}</span>
        </div>
        <div class="writer-protection-banner__actions">
          <Show when={session()?.externalState === "modified"}>
            <button type="button" class="toolbar-button" onClick={() => void props.controller.reloadFromDisk()}>
              {props.language.text("reloadFromDisk")}
            </button>
          </Show>
          <Show when={session()?.externalState === "missing" || session()?.externalState === "modified"}>
            <button
              type="button"
              class="toolbar-button"
              disabled={session()?.readOnly === true}
              onClick={() => void props.controller.keepLocalVersion()}
            >
              {props.language.text("keepLocalVersion")}
            </button>
          </Show>
          <button type="button" class="toolbar-button" onClick={() => void props.controller.saveDocumentAs()}>
            {props.language.text("saveAs")}
          </button>
        </div>
      </aside>
    </Show>
  );
}
