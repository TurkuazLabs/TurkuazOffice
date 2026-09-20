// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/clipboard.service.ts
// # 📌 Amac: Writer copy/cut/paste representation secimi, sanitizer ve fragment mapping kurallarini yurutur
// # 📌 Modul - FileType: Service - TypeScript
// # Version: 0.2.0
// # Aciklama: Internal Turkuaz MIME > sanitized HTML > plain-text onceligini paragraph-local canonical styled-run mutationina baglar
// Bagimli Oldugu Katman: Service -> Repo -> Tool -> Model

import {
  WRITER_CLIPBOARD_BLOCKED_TAGS,
  WRITER_CLIPBOARD_BLOCK_TAGS,
  WRITER_CLIPBOARD_BOLD_TAGS,
  WRITER_CLIPBOARD_BREAK_TAGS,
  WRITER_CLIPBOARD_FRAGMENT_KIND,
  WRITER_CLIPBOARD_FRAGMENT_SCHEMA_VERSION,
  WRITER_CLIPBOARD_ITALIC_TAGS,
  WRITER_CLIPBOARD_MAX_PAYLOAD_CHARS,
  WRITER_CLIPBOARD_MAX_RUNS,
  WRITER_CLIPBOARD_MAX_TEXT_LENGTH,
  WRITER_CLIPBOARD_UNDERLINE_TAGS,
} from "../config/clipboard";
import {
  MAX_WRITER_FONT_FAMILY_LENGTH,
  MAX_WRITER_FONT_SIZE_HALF_POINTS,
  MIN_WRITER_FONT_SIZE_HALF_POINTS,
} from "../config/typography";
import type {
  ClipboardCharacterStyleModel,
  ClipboardDomNodeModel,
  ClipboardStyledRunModel,
  ClipboardTransferModel,
  ClipboardWriteModel,
  WriterClipboardFragmentModel,
} from "../models/clipboard-model";
import type { WriterSessionRepository } from "../repositories/writer-session.repository";
import type { ClipboardDomTool } from "../tools/clipboard-dom.tool";
import type { ClipboardTool } from "../tools/clipboard.tool";
import type {
  WriterCharacterStyleView,
  WriterParagraphView,
  WriterSelectionView,
  WriterStyledRunInputView,
} from "../views/writer-types";
import type { WriterSessionService } from "./writer-session.service";

export class ClipboardService {
  public constructor(
    private readonly repository: WriterSessionRepository,
    private readonly writerSessionService: WriterSessionService,
    private readonly clipboardTool: ClipboardTool,
    private readonly clipboardDomTool: ClipboardDomTool,
  ) {}

  public copySelection(
    paragraphId: string,
    editor: HTMLElement,
    event: ClipboardEvent,
  ): void {
    const selection = this.writerSessionService.captureSelection(paragraphId, editor);
    if (selection === null || selection.startOffset === selection.endOffset) {
      return;
    }
    const fragment = this.fragmentFromSelection(selection);
    if (fragment === null) {
      return;
    }
    this.clipboardTool.write(event, this.toWritePayload(fragment));
  }

  public async cutSelection(
    paragraphId: string,
    editor: HTMLElement,
    event: ClipboardEvent,
  ): Promise<void> {
    if (this.repository.fileSession()?.readOnly === true) {
      return;
    }
    const selection = this.writerSessionService.captureSelection(paragraphId, editor);
    if (selection === null || selection.startOffset === selection.endOffset) {
      return;
    }
    const fragment = this.fragmentFromSelection(selection);
    if (fragment === null || !this.clipboardTool.write(event, this.toWritePayload(fragment))) {
      return;
    }
    await this.writerSessionService.replaceSelectionWithStyledRuns(
      paragraphId,
      selection.startOffset,
      selection.endOffset,
      [],
    );
  }

  public async pasteSelection(
    paragraphId: string,
    editor: HTMLElement,
    event: ClipboardEvent,
  ): Promise<void> {
    if (this.repository.fileSession()?.readOnly === true) {
      return;
    }
    const selection = this.writerSessionService.captureSelection(paragraphId, editor);
    const transfer = this.clipboardTool.read(event);
    if (selection === null || transfer === null) {
      return;
    }

    const baseStyle = this.writerSessionService.insertionStyle(
      paragraphId,
      selection.startOffset,
    );
    const runs = this.selectPasteRuns(transfer, baseStyle);
    if (runs === null || runs.length === 0) {
      return;
    }

    this.clipboardTool.consume(event);
    await this.writerSessionService.replaceSelectionWithStyledRuns(
      paragraphId,
      selection.startOffset,
      selection.endOffset,
      runs,
    );
  }

  private fragmentFromSelection(
    selection: WriterSelectionView,
  ): WriterClipboardFragmentModel | null {
    const paragraph = this.repository
      .document()
      ?.paragraphs.find((item) => item.id === selection.paragraphId);
    if (paragraph === undefined) {
      return null;
    }

    const runs = this.sliceParagraphRuns(paragraph, selection);
    if (runs.length === 0) {
      return null;
    }
    return {
      kind: WRITER_CLIPBOARD_FRAGMENT_KIND,
      schemaVersion: WRITER_CLIPBOARD_FRAGMENT_SCHEMA_VERSION,
      runs,
    };
  }

  private sliceParagraphRuns(
    paragraph: WriterParagraphView,
    selection: WriterSelectionView,
  ): readonly ClipboardStyledRunModel[] {
    const output: ClipboardStyledRunModel[] = [];
    let cursor = 0;
    for (const run of paragraph.runs) {
      const codePoints = Array.from(run.text);
      const runStart = cursor;
      const runEnd = cursor + codePoints.length;
      cursor = runEnd;

      const overlapStart = Math.max(selection.startOffset, runStart);
      const overlapEnd = Math.min(selection.endOffset, runEnd);
      if (overlapStart >= overlapEnd) {
        continue;
      }
      const text = codePoints
        .slice(overlapStart - runStart, overlapEnd - runStart)
        .join("");
      this.pushRun(output, {
        text,
        style: this.toClipboardStyle(run.style),
      });
    }
    return output;
  }

  private toWritePayload(fragment: WriterClipboardFragmentModel): ClipboardWriteModel {
    return {
      internalFragment: JSON.stringify(fragment),
      html: fragment.runs.map((run) => this.runToHtml(run)).join(""),
      plainText: fragment.runs.map((run) => run.text).join(""),
    };
  }

  private selectPasteRuns(
    transfer: ClipboardTransferModel,
    baseStyle: WriterCharacterStyleView,
  ): readonly WriterStyledRunInputView[] | null {
    const internal = this.parseInternalFragment(transfer.internalFragment);
    if (internal !== null) {
      return internal.map((run) => this.toWriterRun(run));
    }

    const html = this.parseHtmlFragment(transfer.html, baseStyle);
    if (html !== null) {
      return html.map((run) => this.toWriterRun(run));
    }

    const plainText = this.normalizeInlineText(transfer.plainText);
    if (plainText.length === 0 || Array.from(plainText).length > WRITER_CLIPBOARD_MAX_TEXT_LENGTH) {
      return null;
    }
    return [{ text: plainText, style: baseStyle }];
  }

  private parseInternalFragment(raw: string): readonly ClipboardStyledRunModel[] | null {
    if (raw.length === 0 || raw.length > WRITER_CLIPBOARD_MAX_PAYLOAD_CHARS) {
      return null;
    }
    let parsed: unknown;
    try {
      parsed = JSON.parse(raw);
    } catch {
      return null;
    }
    if (!this.isRecord(parsed)) {
      return null;
    }
    if (
      parsed.kind !== WRITER_CLIPBOARD_FRAGMENT_KIND ||
      parsed.schemaVersion !== WRITER_CLIPBOARD_FRAGMENT_SCHEMA_VERSION ||
      !Array.isArray(parsed.runs) ||
      parsed.runs.length === 0 ||
      parsed.runs.length > WRITER_CLIPBOARD_MAX_RUNS
    ) {
      return null;
    }

    const runs: ClipboardStyledRunModel[] = [];
    let totalLength = 0;
    for (const rawRun of parsed.runs) {
      const run = this.parseInternalRun(rawRun);
      if (run === null) {
        return null;
      }
      totalLength += Array.from(run.text).length;
      if (totalLength > WRITER_CLIPBOARD_MAX_TEXT_LENGTH) {
        return null;
      }
      this.pushRun(runs, run);
    }
    return runs.length === 0 ? null : runs;
  }

  private parseInternalRun(value: unknown): ClipboardStyledRunModel | null {
    if (!this.isRecord(value) || typeof value.text !== "string" || !this.isRecord(value.style)) {
      return null;
    }
    const style = value.style;
    if (
      typeof style.bold !== "boolean" ||
      typeof style.italic !== "boolean" ||
      typeof style.underline !== "boolean" ||
      typeof style.fontFamily !== "string" ||
      style.fontFamily.length === 0 ||
      style.fontFamily.length > MAX_WRITER_FONT_FAMILY_LENGTH ||
      typeof style.fontSizeHalfPoints !== "number" ||
      !Number.isInteger(style.fontSizeHalfPoints) ||
      style.fontSizeHalfPoints < MIN_WRITER_FONT_SIZE_HALF_POINTS ||
      style.fontSizeHalfPoints > MAX_WRITER_FONT_SIZE_HALF_POINTS
    ) {
      return null;
    }
    const text = this.normalizeInlineText(value.text);
    if (text.length === 0) {
      return null;
    }
    return {
      text,
      style: {
        bold: style.bold,
        italic: style.italic,
        underline: style.underline,
        fontFamily: style.fontFamily,
        fontSizeHalfPoints: style.fontSizeHalfPoints,
      },
    };
  }

  private parseHtmlFragment(
    html: string,
    baseStyle: WriterCharacterStyleView,
  ): readonly ClipboardStyledRunModel[] | null {
    if (html.length === 0 || html.length > WRITER_CLIPBOARD_MAX_PAYLOAD_CHARS) {
      return null;
    }
    const output: ClipboardStyledRunModel[] = [];
    for (const node of this.clipboardDomTool.parse(html)) {
      this.appendHtmlNode(node, this.toClipboardStyle(baseStyle), output);
      if (output.length > WRITER_CLIPBOARD_MAX_RUNS) {
        return null;
      }
    }
    const totalLength = output.reduce((total, run) => total + Array.from(run.text).length, 0);
    if (totalLength === 0 || totalLength > WRITER_CLIPBOARD_MAX_TEXT_LENGTH) {
      return null;
    }
    return output;
  }

  private appendHtmlNode(
    node: ClipboardDomNodeModel,
    inheritedStyle: ClipboardCharacterStyleModel,
    output: ClipboardStyledRunModel[],
  ): void {
    if (node.kind === "text") {
      const text = this.normalizeInlineText(node.text);
      if (text.length > 0) {
        this.pushRun(output, { text, style: inheritedStyle });
      }
      return;
    }

    if (WRITER_CLIPBOARD_BLOCKED_TAGS.has(node.tagName)) {
      return;
    }
    const style = this.applyHtmlStyle(node.tagName, node.style, inheritedStyle);
    if (WRITER_CLIPBOARD_BREAK_TAGS.has(node.tagName)) {
      this.pushRun(output, { text: " ", style });
      return;
    }

    for (const child of node.children) {
      this.appendHtmlNode(child, style, output);
    }
    if (WRITER_CLIPBOARD_BLOCK_TAGS.has(node.tagName)) {
      this.appendBlockBreak(output, style);
    }
  }

  private applyHtmlStyle(
    tagName: string,
    css: {
      readonly fontFamily: string;
      readonly fontSize: string;
      readonly fontWeight: string;
      readonly fontStyle: string;
      readonly textDecoration: string;
    },
    inherited: ClipboardCharacterStyleModel,
  ): ClipboardCharacterStyleModel {
    const fontFamily = this.sanitizeFontFamily(css.fontFamily);
    const fontSizeHalfPoints = this.parseCssFontSize(css.fontSize);
    const numericWeight = Number.parseInt(css.fontWeight, 10);
    return {
      bold:
        inherited.bold ||
        WRITER_CLIPBOARD_BOLD_TAGS.has(tagName) ||
        css.fontWeight.toLowerCase() === "bold" ||
        (Number.isFinite(numericWeight) && numericWeight >= 600),
      italic:
        inherited.italic ||
        WRITER_CLIPBOARD_ITALIC_TAGS.has(tagName) ||
        ["italic", "oblique"].includes(css.fontStyle.toLowerCase()),
      underline:
        inherited.underline ||
        WRITER_CLIPBOARD_UNDERLINE_TAGS.has(tagName) ||
        css.textDecoration.toLowerCase().includes("underline"),
      fontFamily: fontFamily ?? inherited.fontFamily,
      fontSizeHalfPoints: fontSizeHalfPoints ?? inherited.fontSizeHalfPoints,
    };
  }

  private appendBlockBreak(
    output: ClipboardStyledRunModel[],
    style: ClipboardCharacterStyleModel,
  ): void {
    const last = output.at(-1);
    if (last !== undefined && !/\s$/u.test(last.text)) {
      this.pushRun(output, { text: " ", style });
    }
  }

  private sanitizeFontFamily(value: string): string | null {
    const first = value.split(",")[0]?.trim() ?? "";
    const unquoted = first.replace(/^['"]|['"]$/g, "").trim();
    if (
      unquoted.length === 0 ||
      unquoted.length > MAX_WRITER_FONT_FAMILY_LENGTH ||
      /[\u0000-\u001F\u007F]/u.test(unquoted)
    ) {
      return null;
    }
    return unquoted;
  }

  private parseCssFontSize(value: string): number | null {
    const match = /^([0-9]+(?:\.[0-9]+)?)(pt|px)$/u.exec(value.trim().toLowerCase());
    if (match === null) {
      return null;
    }
    const numeric = Number.parseFloat(match[1] ?? "");
    const unit = match[2];
    if (!Number.isFinite(numeric)) {
      return null;
    }
    const points = unit === "px" ? (numeric * 72) / 96 : numeric;
    const halfPoints = Math.round(points * 2);
    if (
      halfPoints < MIN_WRITER_FONT_SIZE_HALF_POINTS ||
      halfPoints > MAX_WRITER_FONT_SIZE_HALF_POINTS
    ) {
      return null;
    }
    return halfPoints;
  }

  private pushRun(
    output: ClipboardStyledRunModel[],
    run: ClipboardStyledRunModel,
  ): void {
    if (run.text.length === 0) {
      return;
    }
    const previous = output.at(-1);
    if (previous !== undefined && this.sameStyle(previous.style, run.style)) {
      output[output.length - 1] = {
        text: previous.text + run.text,
        style: previous.style,
      };
      return;
    }
    output.push(run);
  }

  private sameStyle(
    left: ClipboardCharacterStyleModel,
    right: ClipboardCharacterStyleModel,
  ): boolean {
    return (
      left.bold === right.bold &&
      left.italic === right.italic &&
      left.underline === right.underline &&
      left.fontFamily === right.fontFamily &&
      left.fontSizeHalfPoints === right.fontSizeHalfPoints
    );
  }

  private toClipboardStyle(style: WriterCharacterStyleView): ClipboardCharacterStyleModel {
    return {
      bold: style.bold,
      italic: style.italic,
      underline: style.underline,
      fontFamily: style.fontFamily,
      fontSizeHalfPoints: style.fontSizeHalfPoints,
    };
  }

  private toWriterRun(run: ClipboardStyledRunModel): WriterStyledRunInputView {
    return {
      text: run.text,
      style: {
        bold: run.style.bold,
        italic: run.style.italic,
        underline: run.style.underline,
        fontFamily: run.style.fontFamily,
        fontSizeHalfPoints: run.style.fontSizeHalfPoints,
      },
    };
  }

  private runToHtml(run: ClipboardStyledRunModel): string {
    const decorations = run.style.underline ? "text-decoration:underline;" : "";
    const weight = run.style.bold ? "font-weight:700;" : "";
    const italic = run.style.italic ? "font-style:italic;" : "";
    const family = this.escapeCssString(run.style.fontFamily);
    const sizePoints = run.style.fontSizeHalfPoints / 2;
    return `<span style="font-family:&quot;${family}&quot;;font-size:${sizePoints}pt;${weight}${italic}${decorations}">${this.escapeHtml(run.text)}</span>`;
  }

  private escapeHtml(value: string): string {
    return value
      .replace(/&/gu, "&amp;")
      .replace(/</gu, "&lt;")
      .replace(/>/gu, "&gt;")
      .replace(/"/gu, "&quot;")
      .replace(/'/gu, "&#39;");
  }

  private escapeCssString(value: string): string {
    return value
      .replace(/\\/gu, "\\\\")
      .replace(/"/gu, "\\\"")
      .replace(/[\r\n\f]/gu, " ");
  }

  private normalizeInlineText(value: string): string {
    return value
      .replace(/\u0000/gu, "")
      .replace(/\r\n?/gu, "\n")
      .replace(/\n+/gu, " ");
  }

  private isRecord(value: unknown): value is Record<string, unknown> {
    return typeof value === "object" && value !== null && !Array.isArray(value);
  }
}
