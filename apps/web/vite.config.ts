// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/vite.config.ts
// # 📌 Amac: M3 Web SolidJS gelistirme ve production build ayarlarini tanimlar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.4.0
// # Aciklama: Web dev server ve SolidJS Vite plugin kontratini tanimlar
// Bagimli Oldugu Katman: Config

import { defineConfig } from "vitest/config";
import solidPlugin from "vite-plugin-solid";

import { WEB_DEV_HOST, WEB_DEV_PORT } from "./src/config/runtime-config";

export default defineConfig({
  plugins: [solidPlugin()],
  server: {
    host: WEB_DEV_HOST,
    port: WEB_DEV_PORT,
    strictPort: true,
  },
  test: {
    environment: "jsdom",
    include: ["src/**/*.test.ts", "src/**/*.test.tsx", "src/**/*.spec.ts", "src/**/*.spec.tsx"],
  },
});
