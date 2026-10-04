// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/main.tsx
// # 📌 Amac: M3 Web katmanlarini generated Rust/WASM runtime ile composition root uzerinden birlestirir
// # 📌 Modul - FileType: Web - TypeScript
// # Version: 0.4.0
// # Aciklama: Browser metadata Repository, WASM Core Tool, Service, Controller ve View baglantilarini async bootstrap ile kurar
// Bagimli Oldugu Katman: Controller | Service | Repo | Tool | View | Language

import { render } from "solid-js/web";

import { WEB_APP_ROOT_ERROR, WEB_APP_ROOT_ID } from "./config/runtime-config";
import { WebController } from "./controllers/web-controller";
import { WEB_TR } from "./language/tr-TR";
import { BrowserDocumentIndexRepository } from "./repositories/browser-document-index.repository";
import { WebBootstrapService } from "./services/web-bootstrap-service";
import { createBrowserWasmRuntimeTool } from "./tools/web-wasm-runtime.tool";
import { App } from "./views/App";
import "./views/app.css";

const root = document.getElementById(WEB_APP_ROOT_ID);
if (root === null) {
  throw new Error(WEB_APP_ROOT_ERROR);
}

async function bootstrap(): Promise<void> {
  const repository = new BrowserDocumentIndexRepository(window.localStorage);
  const coreTool = await createBrowserWasmRuntimeTool().load();
  const service = new WebBootstrapService(repository, coreTool);
  const controller = new WebController(service);

  render(() => <App controller={controller} />, root);
}

void bootstrap().catch((error: unknown) => {
  console.error(error);
  root.textContent = WEB_TR.wasmBootstrapError;
});
