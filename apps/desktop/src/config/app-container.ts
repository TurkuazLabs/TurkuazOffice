// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/config/app-container.ts
// # 📌 Amac: Desktop katman bagimliliklarini tek composition root icinde kurar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.2.0
// # Aciklama: Writer IME, DOM selection, native file dialog, Tauri IPC ve session bagimliliklarini merkezi enjekte eder
// Bagimli Oldugu Katman: Config

import { WriterController } from "../controllers/writer.controller";
import { LanguageService } from "../language/language-service";
import { WriterSessionRepository } from "../repositories/writer-session.repository";
import { WriterLayoutService } from "../services/writer-layout.service";
import { WriterSessionService } from "../services/writer-session.service";
import { DomSelectionTool } from "../tools/dom-selection.tool";
import { FontCapabilityTool } from "../tools/font-capability.tool";
import { NativeFileDialogTool } from "../tools/native-file-dialog.tool";
import { TauriWriterTool } from "../tools/tauri-writer.tool";
import { TextOffsetTool } from "../tools/text-offset.tool";

const languageService = new LanguageService();
const writerSessionRepository = new WriterSessionRepository();
const writerTool = new TauriWriterTool();
const textOffsetTool = new TextOffsetTool();
const domSelectionTool = new DomSelectionTool(textOffsetTool);
const nativeFileDialogTool = new NativeFileDialogTool();
const fontCapabilityTool = new FontCapabilityTool();
const writerLayoutService = new WriterLayoutService(fontCapabilityTool);
const writerSessionService = new WriterSessionService(
  writerSessionRepository,
  writerTool,
  textOffsetTool,
  domSelectionTool,
  nativeFileDialogTool,
  writerLayoutService,
  languageService,
);

export const APP_CONTAINER = {
  writerController: new WriterController(writerSessionService),
  writerSessionRepository,
  languageService,
} as const;
