// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-recovery-panel.tsx
// # 📌 Amac: Startup recovery adaylarini Recover, Compare ve Discard aksiyonlariyla sunar
// # 📌 Modul - FileType: View - TSX
// # Version: 0.2.0
// # Aciklama: Recovery read-modelini mutate etmeden kullanici kararini Controller katmanina aktarir
// Bagimli Oldugu Katman: View -> Controller -> Repo -> Language

import { For, Show, type Accessor } from "solid-js";

import type { WriterController } from "../controllers/writer.controller";
import type { LanguageService } from "../language/language-service";
import type { WriterSessionRepository } from "../repositories/writer-session.repository";
import type { RecoveryComparisonView, RecoverySnapshotView } from "./writer-types";

interface WriterRecoveryPanelProps {
  readonly controller: WriterController;
  readonly repository: WriterSessionRepository;
  readonly language: LanguageService;
}

export function WriterRecoveryPanel(props: WriterRecoveryPanelProps) {
  return (
    <main class="writer-recovery" aria-labelledby="writer-recovery-title">
      <section class="writer-recovery__card">
        <h1 id="writer-recovery-title">{props.language.text("recoveryTitle")}</h1>
        <p>{props.language.text("recoveryDescription")}</p>

        <Show
          when={props.repository.recoveryComparison()}
          fallback={
            <div class="writer-recovery__list">
              <For each={props.repository.recoveryCandidates()}>
                {(snapshot: RecoverySnapshotView) => (
                  <article class="writer-recovery__item">
                    <div>
                      <strong>{snapshot.title || props.language.text("untitledDocument")}</strong>
                      <div>{props.language.text("recoverySource")}: {snapshot.sourcePath ?? props.language.text("untitledDocument")}</div>
                      <div>{props.language.text("savedRevision")}: {snapshot.persistedRevision}</div>
                      <div>{props.language.text("recoveryRevision")}: {snapshot.recoveryRevision}</div>
                    </div>
                    <div class="writer-recovery__actions">
                      <button class="toolbar-button toolbar-button--primary" onClick={() => void props.controller.recoverSnapshot(snapshot.snapshotId)}>
                        {props.language.text("recover")}
                      </button>
                      <button class="toolbar-button" onClick={() => void props.controller.compareRecoverySnapshot(snapshot.snapshotId)}>
                        {props.language.text("compare")}
                      </button>
                      <button class="toolbar-button" onClick={() => void props.controller.discardRecoverySnapshot(snapshot.snapshotId)}>
                        {props.language.text("discard")}
                      </button>
                    </div>
                  </article>
                )}
              </For>
            </div>
          }
        >
          {(comparison: Accessor<RecoveryComparisonView>) => (
            <div class="writer-recovery__comparison">
              <div>
                <h2>{props.language.text("diskCopy")}</h2>
                <pre>{comparison().sourcePlainText ?? props.language.text("sourceUnavailable")}</pre>
              </div>
              <div>
                <h2>{props.language.text("recoveryCopy")}</h2>
                <pre>{comparison().recoveryPlainText}</pre>
              </div>
              <button class="toolbar-button" onClick={() => props.controller.closeRecoveryComparison()}>
                {props.language.text("close")}
              </button>
            </div>
          )}
        </Show>
        <Show when={props.repository.recoveryErrorCode() !== null}>
          <p role="alert">{props.language.text("autosaveRecoveryFailed")}: {props.repository.recoveryErrorCode()}</p>
        </Show>
      </section>
    </main>
  );
}
