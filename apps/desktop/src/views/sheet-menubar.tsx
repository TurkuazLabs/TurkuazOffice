// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/sheet-menubar.tsx
// # 📌 Amac: LibreOffice klasik menusu ile Excel hizli akislarini birlestiren Sheet menu satirini sunar
// # 📌 Modul - FileType: View - TSX
// Version: 0.10.0
// Aciklama: Dosya, Bicim, Ekle, Gorunum ve Veri menulerinde gercek Sheet komutlarini; Functions sidebar dahil aktif yuzeyleri sunar
// Bagimli Oldugu Katman: View -> Language

import type { LanguageService } from "../language/language-service";

interface SheetMenubarProps {
  readonly language: LanguageService;
  readonly propertiesOpen: boolean;
  readonly queryOpen: boolean;
  readonly conditionalFormatOpen: boolean;
  readonly functionsOpen: boolean;
  readonly freezeActive: boolean;
  readonly canFreezeAtSelection: boolean;
  readonly canCreateTable: boolean;
  readonly canRemoveTable: boolean;
  readonly onNewDocument: () => void;
  readonly onToggleProperties: () => void;
  readonly onToggleQuery: () => void;
  readonly onToggleConditionalFormat: () => void;
  readonly onToggleFunctions: () => void;
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
            {props.language.text("sheetNewDocument")}
          </button>
        </div>
      </details>

      <span class="sheet-menubar__disabled" aria-disabled="true">
        {props.language.text("menuEdit")}
      </span>

      <details class="sheet-menu">
        <summary>{props.language.text("menuFormat")}</summary>
        <div class="sheet-menu__popup">
          <button type="button" onClick={props.onToggleProperties}>
            {props.language.text("sheetPropertiesToggle")}
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
        <summary>{props.language.text("menuInsert")}</summary>
        <div class="sheet-menu__popup">
          <button
            type="button"
            aria-pressed={props.functionsOpen}
            onClick={props.onToggleFunctions}
          >
            {props.language.text("sheetFunctionsToggle")}
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
          <button
            type="button"
            aria-pressed={props.functionsOpen}
            onClick={props.onToggleFunctions}
          >
            {props.language.text("sheetFunctionsToggle")}
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
            {props.queryOpen
              ? props.language.text("sheetHideQuery")
              : props.language.text("sheetShowQuery")}
          </button>
        </div>
      </details>

      <span class="sheet-menubar__disabled" aria-disabled="true">
        {props.language.text("menuTools")}
      </span>
      <span class="sheet-menubar__disabled" aria-disabled="true">
        {props.language.text("menuWindow")}
      </span>
      <span class="sheet-menubar__disabled" aria-disabled="true">
        {props.language.text("menuHelp")}
      </span>
    </nav>
  );
}
