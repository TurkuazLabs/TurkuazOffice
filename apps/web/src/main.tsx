// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/main.tsx
// # 📌 Amac: M3 Web katmanlarini composition root uzerinden birlestirir
// # 📌 Modul - FileType: Web - TypeScript
// # Version: 0.4.0
// # Aciklama: Browser metadata Repository, Core Tool, Service, Controller ve View baglantilarini kurar
// Bagimli Oldugu Katman: Controller | Service | Repo | Tool | View | Language

import { render } from "solid-js/web";

import { WEB_APP_ROOT_ERROR, WEB_APP_ROOT_ID } from "./config/runtime-config";
import { WebController } from "./controllers/web-controller";
import { BrowserDocumentIndexRepository } from "./repositories/browser-document-index.repository";
import { WebBootstrapService } from "./services/web-bootstrap-service";
import { BrowserCoreContractTool } from "./tools/web-core-tool";
import { App } from "./views/App";
import "./views/app.css";

const repository = new BrowserDocumentIndexRepository(window.localStorage);
const coreTool = new BrowserCoreContractTool();
const service = new WebBootstrapService(repository, coreTool);
const controller = new WebController(service);

const root = document.getElementById(WEB_APP_ROOT_ID);
if (root === null) {
  throw new Error(WEB_APP_ROOT_ERROR);
}

render(() => <App controller={controller} />, root);
