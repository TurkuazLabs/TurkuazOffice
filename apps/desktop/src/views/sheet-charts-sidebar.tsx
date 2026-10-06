// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-charts-sidebar.tsx
// # 📌 Amac: Canonical Sheet grafiklerini olusturma, secme, render etme ve kaldirma yuzeyini sag panelde sunar
// # 📌 Modul - FileType: View - TSX
// Version: 0.11.0
// Aciklama: Secili iki kolonluk araliktan Bar/Line/Pie chart olusturur; chart verisini Controller/Repo uzerinden SVG renderer'a aktarir
// Bagimli Oldugu Katman: View -> Controller -> Repo -> Language -> Config

import { For, Show, createSignal, onMount } from "solid-js";

import {
  SHEET_CHART_DEFAULT_TYPE,
  SHEET_CHART_LABEL_KEYS,
  SHEET_CHART_REQUIRED_COLUMN_COUNT,
  SHEET_CHART_TYPES,
  type SheetChartType,
} from "../config/sheet-charts";
import type { SheetController } from "../controllers/sheet.controller";
import type { LanguageService } from "../language/language-service";
import type { SheetSessionRepository } from "../repositories/sheet-session.repository";
import { SheetChartRenderer } from "./sheet-chart-renderer";

interface SheetChartsSidebarProps {
  readonly controller: SheetController;
  readonly repository: SheetSessionRepository;
  readonly language: LanguageService;
  readonly onClose: () => void;
}

export function SheetChartsSidebar(props: SheetChartsSidebarProps) {
  const [chartType, setChartType] = createSignal<SheetChartType>(SHEET_CHART_DEFAULT_TYPE);
  const [title, setTitle] = createSignal("");

  const charts = () => props.repository.document()?.charts ?? [];
  const selectedChart = () =>
    charts().find((chart) => chart.id === props.repository.selectedChartId()) ?? null;
  const selectionIsValid = () => {
    const range = props.repository.selectionRange();
    return (
      range !== null &&
      range.endColumn - range.startColumn + 1 === SHEET_CHART_REQUIRED_COLUMN_COUNT
    );
  };

  onMount(() => {
    if (props.repository.selectedChartId() === null) {
      const firstChart = charts()[0];
      if (firstChart !== undefined) {
        void props.controller.selectChart(firstChart.id);
      }
    }
  });

  return (
    <aside class="sheet-sidebar sheet-charts-sidebar" aria-label={props.language.text("sheetChartsSidebarLabel")}>
      <header class="sheet-sidebar__header">
        <strong>{props.language.text("sheetCharts")}</strong>
        <button
          type="button"
          class="sheet-sidebar__close"
          aria-label={props.language.text("sheetCloseSidebar")}
          onClick={props.onClose}
        >
          x
        </button>
      </header>

      <section class="sheet-sidebar__section sheet-charts-sidebar__create">
        <h3>{props.language.text("sheetChartCreate")}</h3>
        <p class="sheet-charts-sidebar__hint">
          {props.language.text("sheetChartSelectionHint")}
        </p>

        <label class="sheet-sidebar__field">
          <span>{props.language.text("sheetChartType")}</span>
          <select
            class="ribbon-select"
            value={chartType()}
            onChange={(event) => setChartType(event.currentTarget.value as SheetChartType)}
          >
            <For each={SHEET_CHART_TYPES}>
              {(type) => (
                <option value={type}>
                  {props.language.text(SHEET_CHART_LABEL_KEYS[type])}
                </option>
              )}
            </For>
          </select>
        </label>

        <label class="sheet-sidebar__field">
          <span>{props.language.text("sheetChartTitle")}</span>
          <input
            value={title()}
            onInput={(event) => setTitle(event.currentTarget.value)}
          />
        </label>

        <button
          type="button"
          class="toolbar-button toolbar-button--primary"
          disabled={!selectionIsValid() || title().trim().length === 0}
          onClick={() => void props.controller.createChartFromSelection(chartType(), title())}
        >
          {props.language.text("sheetChartCreate")}
        </button>
      </section>

      <section class="sheet-sidebar__section">
        <h3>{props.language.text("sheetCharts")}</h3>
        <Show
          when={charts().length > 0}
          fallback={<div class="sheet-sidebar__empty">{props.language.text("sheetChartNoCharts")}</div>}
        >
          <div class="sheet-charts-sidebar__list">
            <For each={charts()}>
              {(chart) => (
                <button
                  type="button"
                  class="sheet-charts-sidebar__item"
                  aria-pressed={props.repository.selectedChartId() === chart.id}
                  onClick={() => void props.controller.selectChart(chart.id)}
                >
                  <strong>{chart.title}</strong>
                  <span>{props.language.text(SHEET_CHART_LABEL_KEYS[chart.chartType])}</span>
                </button>
              )}
            </For>
          </div>
        </Show>
      </section>

      <Show when={props.repository.chartErrorCode() !== null}>
        <section class="sheet-sidebar__section">
          <div class="sheet-charts-sidebar__error">
            {props.language.text("sheetChartDataError")}: {props.repository.chartErrorCode()}
          </div>
        </section>
      </Show>

      <Show when={selectedChart()}>
        {(chart) => (
          <section class="sheet-sidebar__section sheet-charts-sidebar__preview">
            <div class="sheet-charts-sidebar__preview-header">
              <h3>{chart().title}</h3>
              <button
                type="button"
                class="toolbar-button"
                onClick={() => void props.controller.removeChart(chart().id)}
              >
                {props.language.text("sheetChartRemove")}
              </button>
            </div>

            <Show
              when={props.repository.chartData()}
              fallback={
                <div class="sheet-chart-renderer__empty">
                  {props.language.text("sheetChartDataEmpty")}
                </div>
              }
            >
              {(data) => (
                <SheetChartRenderer
                  chartType={chart().chartType}
                  title={chart().title}
                  points={data().points}
                  emptyText={props.language.text("sheetChartDataEmpty")}
                />
              )}
            </Show>
          </section>
        )}
      </Show>
    </aside>
  );
}
