// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/views/App.tsx
// # 📌 Amac: M3 Web foundation durumunu kullaniciya gosteren ana View'i render eder
// # 📌 Modul - FileType: View - TSX
// # Version: 0.4.0
// # Aciklama: Controller'dan gelen read-modeli goruntuler; is kurali veya storage erisimi yapmaz
// Bagimli Oldugu Katman: View

import { createMemo } from "solid-js";

import type { WebController } from "../controllers/web-controller";
import { WEB_TR } from "../language/tr-TR";

export interface AppProps {
  readonly controller: WebController;
}

export function App(props: AppProps) {
  const state = createMemo(() => props.controller.initialize());

  return (
    <main class="app-shell">
      <section class="status-card">
        <p class="eyebrow">{WEB_TR.milestone}</p>
        <h1>{WEB_TR.productName}</h1>
        <p>v{state().version}</p>

        <dl class="status-grid">
          <div>
            <dt>{WEB_TR.coreBridge}</dt>
            <dd>{state().capabilities.bridgeKind}</dd>
          </div>
          <div>
            <dt>{WEB_TR.browserStorage}</dt>
            <dd>{state().capabilities.browserStorage ? WEB_TR.enabled : WEB_TR.disabled}</dd>
          </div>
          <div>
            <dt>{WEB_TR.nativeFileSystem}</dt>
            <dd>{state().capabilities.nativeFileSystemAccess ? WEB_TR.enabled : WEB_TR.disabled}</dd>
          </div>
          <div>
            <dt>{WEB_TR.storedDocuments}</dt>
            <dd>{state().storedDocumentCount}</dd>
          </div>
        </dl>
      </section>
    </main>
  );
}
