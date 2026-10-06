// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/main.tsx
// # 📌 Amac: M3 Web katmanlarini composition root uzerinden birlestirir
// # 📌 Modul - FileType: Web - TypeScript
// # Version: 0.4.0
// # Aciklama: Browser metadata Repository, runtime WASM Core Tool, Service, Controller ve View baglantilarini kurar
// Bagimli Oldugu Katman: Controller | Service | Repo | Tool | View | Language

import { render } from "solid-js/web";

import { WEB_APP_ROOT_ERROR, WEB_APP_ROOT_ID } from "./config/runtime-config";
import { WebController } from "./controllers/web-controller";
import { BrowserDocumentIndexRepository } from "./repositories/browser-document-index.repository";
import { IndexedDbDocumentRepository } from "./repositories/indexeddb-document.repository";
import { WebBootstrapService } from "./services/web-bootstrap-service";
import { BrowserIndexedDbDocumentTool } from "./tools/indexed-db-document.tool";
import { loadWebCoreTool } from "./tools/wasm-core-runtime-loader";
import { App } from "./views/App";
import "./views/app.css";

async function bootstrap(): Promise<void> {
  const metadataIndex = new BrowserDocumentIndexRepository(window.localStorage);
  const indexedDbTool = new BrowserIndexedDbDocumentTool(window.indexedDB);
  const repository = new IndexedDbDocumentRepository(indexedDbTool, metadataIndex);
  const loadedCore = await loadWebCoreTool();
  const service = new WebBootstrapService(repository, loadedCore.tool);
  const controller = new WebController(service);
  const state = await controller.initialize();

  const root = document.getElementById(WEB_APP_ROOT_ID);
  if (root === null) {
    throw new Error(WEB_APP_ROOT_ERROR);
  }

  render(() => <App state={state} />, root);
}

void bootstrap();
