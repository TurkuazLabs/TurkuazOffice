// @vitest-environment jsdom
// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-chart-renderer.test.tsx
// # 📌 Amac: Sheet chart SVG renderer'in asiri buyuk finite sayilarda gecerli geometri uretmesini dogrular
// # 📌 Modul - FileType: View Test - TSX
// Version: 0.11.0
// Aciklama: Bar/Line koordinat normalizasyonu ve Pie scaled-total davranisini Infinity/NaN regressionlarina karsi sabitler
// Bagimli Oldugu Katman: View -> Config

import { render } from "solid-js/web";
import { afterEach, describe, expect, it } from "vitest";

import { SheetChartRenderer } from "./sheet-chart-renderer";

let dispose: (() => void) | null = null;

function mount(
  chartType: "bar" | "line" | "pie",
  points: readonly { category: string; value: number }[],
): HTMLElement {
  const root = document.createElement("div");
  document.body.append(root);
  dispose = render(
    () => (
      <SheetChartRenderer
        chartType={chartType}
        title="Extreme"
        points={points}
        emptyText="Empty"
      />
    ),
    root,
  );
  return root;
}

afterEach(() => {
  dispose?.();
  dispose = null;
  document.body.innerHTML = "";
});

describe("SheetChartRenderer extreme finite values", () => {
  it("keeps extreme finite Bar and Line geometry valid", () => {
    const points = [
      { category: "Max", value: 1e308 },
      { category: "Min", value: -1e308 },
    ] as const;

    for (const chartType of ["bar", "line"] as const) {
      const root = mount(chartType, points);
      expect(root.innerHTML).not.toContain("NaN");
      expect(root.innerHTML).not.toContain("Infinity");
      dispose?.();
      dispose = null;
      root.remove();
    }
  });

  it("keeps extreme Pie slices visible without overflow", () => {
    const root = mount("pie", [
      { category: "A", value: 1e308 },
      { category: "B", value: 1e308 },
    ]);

    const slices = Array.from(root.querySelectorAll<SVGPathElement>("path"));
    expect(slices).toHaveLength(2);
    const paths = slices.map((slice) => slice.getAttribute("d") ?? "");
    expect(paths.join(" ")).not.toContain("NaN");
    expect(paths.join(" ")).not.toContain("Infinity");
    expect(new Set(paths).size).toBe(2);
  });
});
