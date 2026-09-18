// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/vite.config.ts
// # 📌 Amac: Desktop SolidJS frontend gelistirme ve production build ayarlarini tanimlar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.2.0
// # Aciklama: Tauri dev portunu sabitler ve SolidJS Vite pluginini etkinlestirir
// Bagimli Oldugu Katman: Config

import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

import { DESKTOP_DEV_HOST, DESKTOP_DEV_PORT } from "./src/config/runtime-config";

export default defineConfig({
  plugins: [solidPlugin()],
  clearScreen: false,
  server: {
    host: DESKTOP_DEV_HOST,
    port: DESKTOP_DEV_PORT,
    strictPort: true,
  },
});
