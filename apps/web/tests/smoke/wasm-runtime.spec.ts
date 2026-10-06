// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/tests/smoke/wasm-runtime.spec.ts
// # 📌 Amac: Production Web buildinde generated Rust WASM Core modulunun gercek browser runtimeinda yuklendigini dogrular
// # 📌 Modul - FileType: Test - TypeScript
// Version: 0.4.0
// Aciklama: Browser fallback yerine rust-wasm bridge, ABI ve native-filesystem guvenlik kontratini smoke seviyesinde test eder
// Bagimli Oldugu Katman: Tool -> Service -> Controller -> View

import { expect, test } from "@playwright/test";

test("loads generated Rust WASM Core instead of browser fallback", async ({ page }) => {
  const consoleErrors: string[] = [];
  page.on("console", (message) => {
    if (message.type() === "error") {
      consoleErrors.push(message.text());
    }
  });

  await page.goto("/");

  const statusCard = page.locator(".status-card");
  await expect(statusCard).toContainText("rust-wasm");
  await expect(statusCard).toContainText("1");
  await expect(statusCard).toContainText("Kapali");
  expect(consoleErrors).toEqual([]);
});
