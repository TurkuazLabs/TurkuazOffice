// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-menubar.tsx
// # 📌 Amac: Modern spreadsheet uygulamalarindaki menu/ribbon akisina uygun Sheet menu satirini sunar
// # 📌 Modul - FileType: View - TSX
// Version: 0.12.0
// Aciklama: Dosya, Giris, Ekle, Bicim, Veri, Formuller, Gorunum ve Yardim siralamasinda gercek Sheet komutlarini gruplar
// Bagimli Oldugu Katman: View -> Language

import { SHEET_SHORTCUT_HINTS } from "../config/keyboard";
import type { LanguageService } from "../language/language-service";

interface SheetMenubarProps {
  readonly language: LanguageService;
  readonly propertiesOpen: boolean;
  readonly queryOpen: boolean;
  readonly conditionalFormatOpen: boolean;
  readonly functionsOpen: boolean;
  readonly chartsOpen: boolean;
  readonly freezeActive: boolean;
  readonly canFreezeAtSelection: boolean;
  readonly canCreateTable: boolean;
  readonly canRemoveTable: boolean;
  readonly onNewDocument: () => void;
  readonly onFind: () => void;
  readonly onToggleProperties: () => void;
  readonly onToggleQuery: () => void;
  readonly onToggleConditionalFormat: () => void;
  readonly onToggleFunctions: () => void;
  readonly onInsertSum: () => void;
  readonly onToggleCharts: () => void;
  readonly onFreezeAtSelection: () => void;
  readonly onFreezeTopRow: () => void;
  readonly onFreezeFirstColumn: () => void;
  readonly onUnfreezePanes: () => void;
  readonly onCreateTable: () => void;
  readonly onRemoveTable: () => void;
}

export function SheetMenubar(props: SheetMenubarProps) {
  return (
    <nav class="sheet-menubar" aria-label={props.language.text("sheetToolbarLabel")}>
      <details class="sheet-menu">
        <summary>{props.language.text("menuFile")}</summary>
        <div class="sheet-menu__popup">
          <button type="button" onClick={props.onNewDocument}>
            <span>{props.language.text("sheetNewDocument")}</span>
            <kbd>{SHEET_SHORTCUT_HINTS.newDocument}</kbd>
          </button>
        </div>
      </details>

      <details class="sheet-menu">
        <summary>{props.language.text("menuEdit")}</summary>
        <div class="sheet-menu__popup">
          <button type="button" aria-keyshortcuts="Control+F Meta+F" onClick={props.onFind}>
            <span>{props.language.text("sheetFind")}</span>
            <kbd>{SHEET_SHORTCUT_HINTS.find}</kbd>
          </button>
        </div>
      </details>

      <span class="sheet-menubar__active">{props.language.text("menuHome")}</span>

      <details class="sheet-menu">
        <summary>{props.language.text("menuInsert")}</summary>
        <div class="sheet-menu__popup">
          <button type="button" aria-pressed={props.chartsOpen} onClick={props.onToggleCharts}>
            {props.language.text("sheetChartsToggle")}
          </button>
          <button type="button" disabled={!props.canCreateTable} onClick={props.onCreateTable}>
            {props.language.text("sheetCreateTable")}
          </button>
        </div>
      </details>

      <details class="sheet-menu">
        <summary>{props.language.text("menuFormat")}</summary>
        <div class="sheet-menu__popup">
          <button type="button" aria-pressed={props.propertiesOpen} onClick={props.onToggleProperties}>
            <span>{props.language.text("sheetPropertiesToggle")}</span>
            <kbd>{SHEET_SHORTCUT_HINTS.toggleProperties}</kbd>
          </button>
          <button
            type="button"
            aria-pressed={props.conditionalFormatOpen}
            onClick={props.onToggleConditionalFormat}
          >
            {props.language.text("sheetConditionalFormatting")}
          </button>
        </div>
      </details>

      <details class="sheet-menu">
        <summary>{props.language.text("menuData")}</summary>
        <div class="sheet-menu__popup">
          <button type="button" disabled={!props.canCreateTable} onClick={props.onCreateTable}>
            {props.language.text("sheetCreateTable")}
          </button>
          <button type="button" disabled={!props.canRemoveTable} onClick={props.onRemoveTable}>
            {props.language.text("sheetRemoveTable")}
          </button>
          <div class="sheet-menu__separator" />
          <button type="button" aria-pressed={props.queryOpen} onClick={props.onToggleQuery}>
            <span>
              {props.queryOpen
                ? props.language.text("sheetHideQuery")
                : props.language.text("sheetShowQuery")}
            </span>
            <kbd>{SHEET_SHORTCUT_HINTS.toggleQuery}</kbd>
          </button>
        </div>
      </details>

      <details class="sheet-menu">
        <summary>{props.language.text("menuFormulas")}</summary>
        <div class="sheet-menu__popup">
          <button
            type="button"
            aria-pressed={props.functionsOpen}
            onClick={props.onToggleFunctions}
          >
            {props.language.text("sheetFunctionsToggle")}
          </button>
          <button type="button" onClick={props.onInsertSum}>
            <span>{props.language.text("sheetFunctionSum")}</span>
            <kbd>{SHEET_SHORTCUT_HINTS.insertSum}</kbd>
          </button>
        </div>
      </details>

      <details class="sheet-menu">
        <summary>{props.language.text("menuView")}</summary>
        <div class="sheet-menu__popup">
          <button type="button" aria-pressed={props.propertiesOpen} onClick={props.onToggleProperties}>
            {props.language.text("sheetPropertiesToggle")}
          </button>
          <button type="button" aria-pressed={props.queryOpen} onClick={props.onToggleQuery}>
            {props.queryOpen
              ? props.language.text("sheetHideQuery")
              : props.language.text("sheetShowQuery")}
          </button>
          <button type="button" aria-pressed={props.chartsOpen} onClick={props.onToggleCharts}>
            {props.language.text("sheetChartsToggle")}
          </button>
          <div class="sheet-menu__separator" />
          <button
            type="button"
            disabled={!props.canFreezeAtSelection}
            onClick={props.onFreezeAtSelection}
          >
            {props.language.text("sheetFreezeAtSelection")}
          </button>
          <button type="button" onClick={props.onFreezeTopRow}>
            {props.language.text("sheetFreezeTopRow")}
          </button>
          <button type="button" onClick={props.onFreezeFirstColumn}>
            {props.language.text("sheetFreezeFirstColumn")}
          </button>
          <button
            type="button"
            disabled={!props.freezeActive}
            onClick={props.onUnfreezePanes}
          >
            {props.language.text("sheetUnfreezePanes")}
          </button>
        </div>
      </details>

      <span class="sheet-menubar__disabled" aria-disabled="true">
        {props.language.text("menuHelp")}
      </span>
    </nav>
  );
}
