// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/main.tsx
// # 📌 Amac: SolidJS Desktop View agacini Tauri webview rootuna baglar
// # 📌 Modul - FileType: View - TSX
// # Version: 0.2.0
// # Aciklama: Composition root bagimliliklarini WriterShell'e aktarir ve global stilleri yukler
// Bagimli Oldugu Katman: View -> Config

import { render } from "solid-js/web";

import { APP_CONTAINER } from "./config/app-container";
import { ERROR_CODES } from "./config/error-codes";
import "./config/ui-tokens.css";
import "./views/app.css";
import { WriterShell } from "./views/writer-shell";

const root = document.getElementById("app");
if (root === null) {
  throw new Error(ERROR_CODES.appRootMissing);
}

render(
  () => (
    <WriterShell
      controller={APP_CONTAINER.writerController}
      repository={APP_CONTAINER.writerSessionRepository}
      language={APP_CONTAINER.languageService}
    />
  ),
  root,
);
