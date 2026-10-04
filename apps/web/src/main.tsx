// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/main.tsx
// # 📌 Amac: M3 Web katmanlarini composition root uzerinden birlestirir
// # 📌 Modul - FileType: Web - TypeScript
// # Version: 0.4.0
// # Aciklama: Browser Repository, Core Tool, Service, Controller ve View baglantilarini kurar
// Bagimli Oldugu Katman: Controller | Service | Repo | Tool | View | Language

import { render } from "solid-js/web";

import { WebController } from "./controllers/web-controller";
import { BrowserDocumentRepository } from "./repositories/browser-document-repository";
import { WebBootstrapService } from "./services/web-bootstrap-service";
import { BrowserCoreContractTool } from "./tools/web-core-tool";
import { App } from "./views/App";
import "./views/app.css";

const repository = new BrowserDocumentRepository(window.localStorage);
const coreTool = new BrowserCoreContractTool();
const service = new WebBootstrapService(repository, coreTool);
const controller = new WebController(service);

const root = document.getElementById("app");
if (root === null) {
  throw new Error("Turkuaz Office Web app root bulunamadi.");
}

render(() => <App controller={controller} />, root);
