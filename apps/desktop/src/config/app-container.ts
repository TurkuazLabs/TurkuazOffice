// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/app-container.ts
// # 📌 Amac: Desktop katman bagimliliklarini tek composition root icinde kurar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.2.0
// # Aciklama: Writer IME, clipboard, DOM selection, native file dialog, Tauri IPC ve session bagimliliklarini merkezi enjekte eder
// Bagimli Oldugu Katman: Config

import { WriterController } from "../controllers/writer.controller";
import { ClipboardService } from "../services/clipboard.service";
import { LanguageService } from "../language/language-service";
import { WriterSessionRepository } from "../repositories/writer-session.repository";
import { WriterLayoutService } from "../services/writer-layout.service";
import { WriterSessionService } from "../services/writer-session.service";
import { ClipboardDomTool } from "../tools/clipboard-dom.tool";
import { ClipboardTool } from "../tools/clipboard.tool";
import { DomSelectionTool } from "../tools/dom-selection.tool";
import { FontCapabilityTool } from "../tools/font-capability.tool";
import { ImageAssetTool } from "../tools/image-asset.tool";
import { NativeFileDialogTool } from "../tools/native-file-dialog.tool";
import { TauriWriterTool } from "../tools/tauri-writer.tool";
import { TextOffsetTool } from "../tools/text-offset.tool";

const languageService = new LanguageService();
const writerSessionRepository = new WriterSessionRepository();
const writerTool = new TauriWriterTool();
const textOffsetTool = new TextOffsetTool();
const domSelectionTool = new DomSelectionTool(textOffsetTool);
const nativeFileDialogTool = new NativeFileDialogTool();
const imageAssetTool = new ImageAssetTool();
const fontCapabilityTool = new FontCapabilityTool();
const writerLayoutService = new WriterLayoutService(fontCapabilityTool);
const writerSessionService = new WriterSessionService(
  writerSessionRepository,
  writerTool,
  textOffsetTool,
  domSelectionTool,
  nativeFileDialogTool,
  imageAssetTool,
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

export const APP_CONTAINER = {
  writerController: new WriterController(writerSessionService, clipboardService),
  writerSessionRepository,
  languageService,
} as const;
