// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-chart-renderer.tsx
// # 📌 Amac: Canonical Sheet chart verisini dependency-free SVG ciktiya cevirir
// # 📌 Modul - FileType: View - TSX
// Version: 0.11.0
// Aciklama: Bar, Line ve Pie chart tiplerini yalnizca sunum geometrisi ile render eder; domain verisini degistirmez
// Bagimli Oldugu Katman: View -> Config

import { For, Show } from "solid-js";

import {
  SHEET_CHART_BAR_GAP,
  SHEET_CHART_EMPTY_VALUE,
  SHEET_CHART_FULL_CIRCLE_RADIANS,
  SHEET_CHART_PIE_CENTER_X,
  SHEET_CHART_PIE_CENTER_Y,
  SHEET_CHART_PIE_RADIUS,
  SHEET_CHART_PLOT_PADDING,
  SHEET_CHART_POINT_RADIUS,
  SHEET_CHART_SLICE_STYLE_COUNT,
  SHEET_CHART_VIEW_HEIGHT,
  SHEET_CHART_VIEW_WIDTH,
} from "../config/sheet-charts";
import type { SheetChartDataPointView, SheetChartTypeView } from "./sheet-types";

interface SheetChartRendererProps {
  readonly chartType: SheetChartTypeView;
  readonly title: string;
  readonly points: readonly SheetChartDataPointView[];
  readonly emptyText: string;
}

interface PieSliceGeometry {
  readonly path: string;
  readonly point: SheetChartDataPointView;
  readonly styleIndex: number;
}

function valueBounds(points: readonly SheetChartDataPointView[]): {
  readonly scale: number;
  readonly min: number;
  readonly max: number;
  readonly span: number;
} {
  if (points.length === 0) {
    return { scale: 1, min: SHEET_CHART_EMPTY_VALUE, max: 1, span: 1 };
  }

  const scale = Math.max(
    1,
    ...points.map((point) => Math.abs(point.value)),
  );
  const values = points.map((point) => point.value / scale);
  const min = Math.min(SHEET_CHART_EMPTY_VALUE, ...values);
  const max = Math.max(SHEET_CHART_EMPTY_VALUE, ...values);
  const span = max - min;
  return {
    scale,
    min,
    max,
    span: span === SHEET_CHART_EMPTY_VALUE ? 1 : span,
  };
}

function valueY(
  value: number,
  bounds: ReturnType<typeof valueBounds>,
): number {
  const plotHeight = SHEET_CHART_VIEW_HEIGHT - SHEET_CHART_PLOT_PADDING * 2;
  const normalizedValue = value / bounds.scale;
  return (
    SHEET_CHART_PLOT_PADDING +
    ((bounds.max - normalizedValue) / bounds.span) * plotHeight
  );
}

function linePoints(points: readonly SheetChartDataPointView[]): string {
  const bounds = valueBounds(points);
  const plotWidth = SHEET_CHART_VIEW_WIDTH - SHEET_CHART_PLOT_PADDING * 2;
  const divisor = Math.max(1, points.length - 1);
  return points
    .map((point, index) => {
      const x =
        SHEET_CHART_PLOT_PADDING +
        (index / divisor) * plotWidth;
      const y = valueY(point.value, bounds);
      return `${x},${y}`;
    })
    .join(" ");
}

function polarPoint(angle: number): { readonly x: number; readonly y: number } {
  return {
    x: SHEET_CHART_PIE_CENTER_X + Math.cos(angle) * SHEET_CHART_PIE_RADIUS,
    y: SHEET_CHART_PIE_CENTER_Y + Math.sin(angle) * SHEET_CHART_PIE_RADIUS,
  };
}

function pieSlices(points: readonly SheetChartDataPointView[]): readonly PieSliceGeometry[] {
  const positive = points.map((point) => Math.max(SHEET_CHART_EMPTY_VALUE, point.value));
  const scale = Math.max(SHEET_CHART_EMPTY_VALUE, ...positive);
  if (scale <= SHEET_CHART_EMPTY_VALUE) {
    return [];
  }
  const normalized = positive.map((value) => value / scale);
  const total = normalized.reduce((sum, value) => sum + value, SHEET_CHART_EMPTY_VALUE);
  const positiveCount = normalized.filter((value) => value > SHEET_CHART_EMPTY_VALUE).length;

  let angle = -Math.PI / 2;
  return points.flatMap((point, index) => {
    const value = normalized[index] ?? SHEET_CHART_EMPTY_VALUE;
    if (value <= SHEET_CHART_EMPTY_VALUE) {
      return [];
    }
    const sweep = (value / total) * SHEET_CHART_FULL_CIRCLE_RADIANS;
    const start = polarPoint(angle);
    const endAngle = angle + sweep;

    if (positiveCount === 1) {
      const middle = polarPoint(angle + Math.PI);
      const path = [
        `M ${SHEET_CHART_PIE_CENTER_X} ${SHEET_CHART_PIE_CENTER_Y}`,
        `L ${start.x} ${start.y}`,
        `A ${SHEET_CHART_PIE_RADIUS} ${SHEET_CHART_PIE_RADIUS} 0 0 1 ${middle.x} ${middle.y}`,
        `A ${SHEET_CHART_PIE_RADIUS} ${SHEET_CHART_PIE_RADIUS} 0 0 1 ${start.x} ${start.y}`,
        "Z",
      ].join(" ");
      angle = endAngle;
      return [{
        path,
        point,
        styleIndex: index % SHEET_CHART_SLICE_STYLE_COUNT,
      }];
    }

    const end = polarPoint(endAngle);
    const largeArc = sweep > Math.PI ? 1 : 0;
    const path = [
      `M ${SHEET_CHART_PIE_CENTER_X} ${SHEET_CHART_PIE_CENTER_Y}`,
      `L ${start.x} ${start.y}`,
      `A ${SHEET_CHART_PIE_RADIUS} ${SHEET_CHART_PIE_RADIUS} 0 ${largeArc} 1 ${end.x} ${end.y}`,
      "Z",
    ].join(" ");
    angle = endAngle;
    return [{
      path,
      point,
      styleIndex: index % SHEET_CHART_SLICE_STYLE_COUNT,
    }];
  });
}

function BarChart(props: SheetChartRendererProps) {
  const bounds = () => valueBounds(props.points);
  const plotWidth = SHEET_CHART_VIEW_WIDTH - SHEET_CHART_PLOT_PADDING * 2;
  const slotWidth = () => plotWidth / Math.max(1, props.points.length);
  const barWidth = () => Math.max(1, slotWidth() - SHEET_CHART_BAR_GAP);
  const baseline = () => valueY(SHEET_CHART_EMPTY_VALUE, bounds());

  return (
    <svg
      class="sheet-chart-svg"
      viewBox={`0 0 ${SHEET_CHART_VIEW_WIDTH} ${SHEET_CHART_VIEW_HEIGHT}`}
      role="img"
      aria-label={props.title}
    >
      <line
        class="sheet-chart-svg__axis"
        x1={SHEET_CHART_PLOT_PADDING}
        y1={baseline()}
        x2={SHEET_CHART_VIEW_WIDTH - SHEET_CHART_PLOT_PADDING}
        y2={baseline()}
      />
      <For each={props.points}>
        {(point, index) => {
          const y = () => valueY(point.value, bounds());
          const top = () => Math.min(y(), baseline());
          const height = () => Math.max(1, Math.abs(y() - baseline()));
          const x = () =>
            SHEET_CHART_PLOT_PADDING +
            index() * slotWidth() +
            SHEET_CHART_BAR_GAP / 2;
          return (
            <rect
              class="sheet-chart-svg__bar"
              x={x()}
              y={top()}
              width={barWidth()}
              height={height()}
            >
              <title>{point.category}: {point.value}</title>
            </rect>
          );
        }}
      </For>
    </svg>
  );
}

function LineChart(props: SheetChartRendererProps) {
  const bounds = () => valueBounds(props.points);
  const plotWidth = SHEET_CHART_VIEW_WIDTH - SHEET_CHART_PLOT_PADDING * 2;
  const divisor = () => Math.max(1, props.points.length - 1);

  return (
    <svg
      class="sheet-chart-svg"
      viewBox={`0 0 ${SHEET_CHART_VIEW_WIDTH} ${SHEET_CHART_VIEW_HEIGHT}`}
      role="img"
      aria-label={props.title}
    >
      <polyline class="sheet-chart-svg__line" points={linePoints(props.points)} />
      <For each={props.points}>
        {(point, index) => (
          <circle
            class="sheet-chart-svg__point"
            cx={
              SHEET_CHART_PLOT_PADDING +
              (index() / divisor()) * plotWidth
            }
            cy={valueY(point.value, bounds())}
            r={SHEET_CHART_POINT_RADIUS}
          >
            <title>{point.category}: {point.value}</title>
          </circle>
        )}
      </For>
    </svg>
  );
}

function PieChart(props: SheetChartRendererProps) {
  const slices = () => pieSlices(props.points);
  return (
    <Show
      when={slices().length > 0}
      fallback={<div class="sheet-chart-renderer__empty">{props.emptyText}</div>}
    >
      <svg
        class="sheet-chart-svg"
        viewBox={`0 0 ${SHEET_CHART_VIEW_WIDTH} ${SHEET_CHART_VIEW_HEIGHT}`}
        role="img"
        aria-label={props.title}
      >
        <For each={slices()}>
          {(slice) => (
            <path
              class={`sheet-chart-svg__slice sheet-chart-svg__slice--${slice.styleIndex}`}
              d={slice.path}
            >
              <title>{slice.point.category}: {slice.point.value}</title>
            </path>
          )}
        </For>
      </svg>
    </Show>
  );
}

export function SheetChartRenderer(props: SheetChartRendererProps) {
  return (
    <Show
      when={props.points.length > 0}
      fallback={<div class="sheet-chart-renderer__empty">{props.emptyText}</div>}
    >
      <Show when={props.chartType === "bar"}>
        <BarChart {...props} />
      </Show>
      <Show when={props.chartType === "line"}>
        <LineChart {...props} />
      </Show>
      <Show when={props.chartType === "pie"}>
        <PieChart {...props} />
      </Show>
    </Show>
  );
}
