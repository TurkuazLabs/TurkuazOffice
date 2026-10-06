// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/playwright.config.ts
// # 📌 Amac: Generated WASM browser smoke testi icin headless Chromium ve Vite preview kontratini tanimlar
// # 📌 Modul - FileType: Config - TypeScript
// Version: 0.4.0
// Aciklama: Production Web buildini localhost uzerinden acip M3 WASM runtime smoke testini calistirir
// Bagimli Oldugu Katman: Config | Tool | View

import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./tests/smoke",
  fullyParallel: false,
  workers: 1,
  retries: 0,
  reporter: "line",
  use: {
    baseURL: "http://127.0.0.1:4173",
    ...devices["Desktop Chrome"],
  },
  webServer: {
    command: "npm run preview -- --host 127.0.0.1 --port 4173",
    url: "http://127.0.0.1:4173",
    reuseExistingServer: false,
  },
});
