// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/views/App.tsx
// # 📌 Amac: M3 Web foundation durumunu kullaniciya gosteren ana View'i render eder
// # 📌 Modul - FileType: View - TSX
// # Version: 0.4.0
// # Aciklama: Controller'dan gelen read-modeli goruntuler; is kurali veya storage erisimi yapmaz
// Bagimli Oldugu Katman: View -> Language

import { WEB_TR } from "../language/tr-TR";
import type { WebBootstrapViewModel } from "../models/web-models";

export interface AppProps {
  readonly state: WebBootstrapViewModel;
}

export function App(props: AppProps) {
  return (
    <main class="app-shell">
      <section class="status-card">
        <p class="eyebrow">{WEB_TR.milestone}</p>
        <h1>{WEB_TR.productName}</h1>
        <p>v{props.state.version}</p>

        <dl class="status-grid">
          <div>
            <dt>{WEB_TR.coreBridge}</dt>
            <dd>{props.state.capabilities.bridgeKind}</dd>
          </div>
          <div>
            <dt>{WEB_TR.coreAbi}</dt>
            <dd>{props.state.capabilities.abiVersion}</dd>
          </div>
          <div>
            <dt>{WEB_TR.browserMetadataStorage}</dt>
            <dd>
              {props.state.capabilities.browserMetadataStorage ? WEB_TR.enabled : WEB_TR.disabled}
            </dd>
          </div>
          <div>
            <dt>{WEB_TR.nativeFileSystem}</dt>
            <dd>{props.state.capabilities.nativeFileSystemAccess ? WEB_TR.enabled : WEB_TR.disabled}</dd>
          </div>
          <div>
            <dt>{WEB_TR.canonicalDocumentStorage}</dt>
            <dd>
              {props.state.documentStorageAvailable
                ? `${WEB_TR.enabled} (${props.state.documentStorageKind})`
                : WEB_TR.unavailable}
            </dd>
          </div>
          <div>
            <dt>{WEB_TR.storedDocuments}</dt>
            <dd>{props.state.storedDocumentCount ?? "-"}</dd>
          </div>
        </dl>
      </section>
    </main>
  );
}
