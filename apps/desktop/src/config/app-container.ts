// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/app-container.ts
// # 📌 Amac: Desktop katman bagimliliklarini tek composition root icinde kurar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.4.0
// # Aciklama: Writer ve Sheet controller, repo, service ve Tauri IPC bagimliliklarini merkezi enjekte eder
// Bagimli Oldugu Katman: Config

import { SheetController } from "../controllers/sheet.controller";
import { WriterController } from "../controllers/writer.controller";
import { ClipboardService } from "../services/clipboard.service";
import { LanguageService } from "../language/language-service";
import { LanguagePreferenceService } from "../services/language-preference.service";
import { KeyboardShortcutService } from "../services/keyboard-shortcut.service";
import { SheetSessionRepository } from "../repositories/sheet-session.repository";
import { WriterSessionRepository } from "../repositories/writer-session.repository";
import { SheetSessionService } from "../services/sheet-session.service";
import { WriterLayoutService } from "../services/writer-layout.service";
import { WriterSessionService } from "../services/writer-session.service";
import { ClipboardDomTool } from "../tools/clipboard-dom.tool";
import { ClipboardTool } from "../tools/clipboard.tool";
import { DomSelectionTool } from "../tools/dom-selection.tool";
import { FontCapabilityTool } from "../tools/font-capability.tool";
import { ImageAssetTool } from "../tools/image-asset.tool";
import { LanguagePreferenceTool } from "../tools/language-preference.tool";
import { NativeFileDialogTool } from "../tools/native-file-dialog.tool";
import { PrintTool } from "../tools/print.tool";
import { TauriSheetTool } from "../tools/tauri-sheet.tool";
import { TauriWriterTool } from "../tools/tauri-writer.tool";
import { TextOffsetTool } from "../tools/text-offset.tool";

const languageService = new LanguageService();
const languagePreferenceTool = new LanguagePreferenceTool();
const languagePreferenceService = new LanguagePreferenceService(
  languageService,
  languagePreferenceTool,
);
languagePreferenceService.initialize();
const sheetSessionRepository = new SheetSessionRepository();
const sheetTool = new TauriSheetTool();
const sheetSessionService = new SheetSessionService(sheetSessionRepository, sheetTool);
const writerSessionRepository = new WriterSessionRepository();
const writerTool = new TauriWriterTool();
const textOffsetTool = new TextOffsetTool();
const domSelectionTool = new DomSelectionTool(textOffsetTool);
const nativeFileDialogTool = new NativeFileDialogTool();
const imageAssetTool = new ImageAssetTool();
const printTool = new PrintTool();
const fontCapabilityTool = new FontCapabilityTool();
const writerLayoutService = new WriterLayoutService(fontCapabilityTool);
const writerSessionService = new WriterSessionService(
  writerSessionRepository,
  writerTool,
  textOffsetTool,
  domSelectionTool,
  nativeFileDialogTool,
  imageAssetTool,
  printTool,
  writerLayoutService,
  languageService,
);
const clipboardTool = new ClipboardTool();
const clipboardDomTool = new ClipboardDomTool();
const clipboardService = new ClipboardService(
  writerSessionRepository,
  writerSessionService,
  clipboardTool,
  clipboardDomTool,
);
const keyboardShortcutService = new KeyboardShortcutService();

export const APP_CONTAINER = {
  sheetController: new SheetController(sheetSessionService),
  sheetSessionRepository,
  writerController: new WriterController(
    writerSessionService,
    clipboardService,
    languagePreferenceService,
    keyboardShortcutService,
  ),
  writerSessionRepository,
  languageService,
} as const;
