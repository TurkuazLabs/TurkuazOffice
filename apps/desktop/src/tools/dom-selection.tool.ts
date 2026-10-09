// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/dom-selection.tool.ts
// # 📌 Amac: Contenteditable DOM selection noktalarini Writer paragraph logical offsetlerine map eder
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.2.2
// # Aciklama: Browser Range/Selection ve UTF-16 detaylarini Service katmanindan izole eder; selection restore destegi saglar
// Bagimli Oldugu Katman: Tool

import {
  WRITER_PARAGRAPH_MARKER_VALUE,
  WRITER_PARAGRAPH_SELECTOR,
} from "../config/dom-contract";
import type { WriterSelectionView } from "../views/writer-types";
import type { TextOffsetTool } from "./text-offset.tool";

export class DomSelectionTool {
  public constructor(private readonly textOffsetTool: TextOffsetTool) {}

  public readParagraphSelection(
    root: HTMLElement,
    paragraphId: string,
  ): WriterSelectionView | null {
    const selection = window.getSelection();
    if (
      selection === null ||
      selection.rangeCount === 0 ||
      selection.anchorNode === null ||
      selection.focusNode === null ||
      !this.containsPoint(root, selection.anchorNode) ||
      !this.containsPoint(root, selection.focusNode)
    ) {
      return null;
    }

    const text = root.textContent ?? "";
    const anchorUtf16 = this.utf16OffsetFromPoint(root, selection.anchorNode, selection.anchorOffset);
    const focusUtf16 = this.utf16OffsetFromPoint(root, selection.focusNode, selection.focusOffset);
    if (anchorUtf16 === null || focusUtf16 === null) {
      return null;
    }

    const anchor = this.textOffsetTool.utf16ToLogical(text, anchorUtf16);
    const focus = this.textOffsetTool.utf16ToLogical(text, focusUtf16);
    return {
      paragraphId,
      startOffset: Math.min(anchor, focus),
      endOffset: Math.max(anchor, focus),
    };
  }

  public restoreParagraphSelection(
    root: HTMLElement,
    selectionView: WriterSelectionView,
  ): boolean {
    if (selectionView.paragraphId !== root.dataset.writerParagraphId) {
      return false;
    }

    const start = this.domPointFromLogicalOffset(root, selectionView.startOffset);
    const end = this.domPointFromLogicalOffset(root, selectionView.endOffset);
    if (start === null || end === null) {
      return false;
    }

    const range = document.createRange();
    range.setStart(start.node, start.utf16Offset);
    range.setEnd(end.node, end.utf16Offset);
    const selection = window.getSelection();
    if (selection === null) {
      return false;
    }
    selection.removeAllRanges();
    selection.addRange(range);
    return true;
  }

  public plainText(root: HTMLElement): string {
    return root.textContent ?? "";
  }

  public isCaretAtVisualBoundary(
    root: HTMLElement,
    direction: "up" | "down",
  ): boolean {
    const selection = window.getSelection();
    if (
      selection === null ||
      selection.rangeCount === 0 ||
      !selection.isCollapsed ||
      selection.anchorNode === null ||
      !this.containsPoint(root, selection.anchorNode)
    ) {
      return false;
    }

    const range = selection.getRangeAt(0).cloneRange();
    const rootRect = root.getBoundingClientRect();
    const caretRect = range.getBoundingClientRect();
    const computedLineHeight = Number.parseFloat(window.getComputedStyle(root).lineHeight);
    const lineHeight =
      Number.isFinite(computedLineHeight) && computedLineHeight > 0
        ? computedLineHeight
        : Math.max(caretRect.height, 16);

    if (rootRect.height <= 0 || caretRect.height <= 0) {
      return true;
    }

    const tolerance = Math.max(lineHeight * 0.6, 6);
    return direction === "up"
      ? caretRect.top <= rootRect.top + tolerance
      : caretRect.bottom >= rootRect.bottom - tolerance;
  }

  public activeWriterParagraph(): HTMLElement | null {
    const active = document.activeElement;
    if (
      active instanceof HTMLElement &&
      active.dataset.writerParagraph === WRITER_PARAGRAPH_MARKER_VALUE &&
      typeof active.dataset.writerParagraphId === "string"
    ) {
      return active;
    }
    return null;
  }

  public focusAndRestoreParagraphSelection(
    selectionView: WriterSelectionView,
    scrollIntoView = false,
  ): boolean {
    const candidates = document.querySelectorAll<HTMLElement>(WRITER_PARAGRAPH_SELECTOR);
    const editor = Array.from(candidates).find(
      (candidate) => candidate.dataset.writerParagraphId === selectionView.paragraphId,
    );
    if (editor === undefined) {
      return false;
    }
    editor.focus({ preventScroll: true });
    const restored = this.restoreParagraphSelection(editor, selectionView);
    if (restored && scrollIntoView) {
      editor.scrollIntoView?.({ block: "nearest" });
    }
    return restored;
  }

  public adjacentParagraphSelection(
    sourceRoot: HTMLElement,
    targetParagraphId: string,
    direction: "up" | "down",
  ): WriterSelectionView | null {
    const selection = window.getSelection();
    if (
      selection === null ||
      selection.rangeCount === 0 ||
      !selection.isCollapsed ||
      selection.anchorNode === null ||
      !this.containsPoint(sourceRoot, selection.anchorNode)
    ) {
      return null;
    }

    const targetRoot = this.writerParagraphById(targetParagraphId);
    if (targetRoot === null) {
      return null;
    }

    const sourceCaretRect = selection.getRangeAt(0).getBoundingClientRect();
    const targetRect = targetRoot.getBoundingClientRect();
    const computedLineHeight = Number.parseFloat(window.getComputedStyle(targetRoot).lineHeight);
    const lineHeight =
      Number.isFinite(computedLineHeight) && computedLineHeight > 0
        ? computedLineHeight
        : Math.max(sourceCaretRect.height, 16);
    const targetX = Math.min(
      Math.max(sourceCaretRect.left, targetRect.left + 1),
      Math.max(targetRect.left + 1, targetRect.right - 1),
    );
    const halfLine = Math.max(lineHeight / 2, 1);
    const targetY =
      direction === "down"
        ? Math.min(targetRect.bottom - 1, targetRect.top + halfLine)
        : Math.max(targetRect.top + 1, targetRect.bottom - halfLine);

    const point = this.caretPointFromViewport(targetX, targetY);
    if (point === null || !this.containsPoint(targetRoot, point.node)) {
      return null;
    }

    const utf16Offset = this.utf16OffsetFromPoint(targetRoot, point.node, point.offset);
    if (utf16Offset === null) {
      return null;
    }
    const text = targetRoot.textContent ?? "";
    const logicalOffset = this.textOffsetTool.utf16ToLogical(text, utf16Offset);
    return {
      paragraphId: targetParagraphId,
      startOffset: logicalOffset,
      endOffset: logicalOffset,
    };
  }

  private containsPoint(root: HTMLElement, node: Node): boolean {
    return node === root || root.contains(node);
  }

  private writerParagraphById(paragraphId: string): HTMLElement | null {
    const candidates = document.querySelectorAll<HTMLElement>(WRITER_PARAGRAPH_SELECTOR);
    return (
      Array.from(candidates).find(
        (candidate) => candidate.dataset.writerParagraphId === paragraphId,
      ) ?? null
    );
  }

  private caretPointFromViewport(
    x: number,
    y: number,
  ): { readonly node: Node; readonly offset: number } | null {
    const browserDocument = document as Document & {
      caretPositionFromPoint?: (
        x: number,
        y: number,
      ) => { readonly offsetNode: Node; readonly offset: number } | null;
      caretRangeFromPoint?: (x: number, y: number) => Range | null;
    };

    const caretPosition = browserDocument.caretPositionFromPoint?.(x, y);
    if (caretPosition !== null && caretPosition !== undefined) {
      return {
        node: caretPosition.offsetNode,
        offset: caretPosition.offset,
      };
    }

    const caretRange = browserDocument.caretRangeFromPoint?.(x, y);
    if (caretRange === null || caretRange === undefined) {
      return null;
    }
    return {
      node: caretRange.startContainer,
      offset: caretRange.startOffset,
    };
  }

  private utf16OffsetFromPoint(root: HTMLElement, node: Node, offset: number): number | null {
    try {
      const range = document.createRange();
      range.selectNodeContents(root);
      range.setEnd(node, offset);
      return range.toString().length;
    } catch {
      return null;
    }
  }

  private domPointFromLogicalOffset(
    root: HTMLElement,
    logicalOffset: number,
  ): { readonly node: Node; readonly utf16Offset: number } | null {
    const totalText = root.textContent ?? "";
    const totalLogical = this.textOffsetTool.logicalLength(totalText);
    if (logicalOffset < 0 || logicalOffset > totalLogical) {
      return null;
    }

    const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT);
    let remaining = logicalOffset;
    let current = walker.nextNode();
    while (current !== null) {
      const text = current.textContent ?? "";
      const length = this.textOffsetTool.logicalLength(text);
      if (remaining <= length) {
        return {
          node: current,
          utf16Offset: this.textOffsetTool.logicalToUtf16(text, remaining),
        };
      }
      remaining -= length;
      current = walker.nextNode();
    }

    if (logicalOffset === 0 || logicalOffset === totalLogical) {
      return { node: root, utf16Offset: root.childNodes.length };
    }
    return null;
  }
}
