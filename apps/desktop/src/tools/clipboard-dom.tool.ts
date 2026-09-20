// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/clipboard-dom.tool.ts
// # 📌 Amac: HTML clipboard verisini inert DOM agacina parse edip browser API detaylarini izole eder
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.2.0
// # Aciklama: DOMParser sonucunu limitli ve inert tag/style/text Model agacina cevirir
// Bagimli Oldugu Katman: Tool -> Config -> Model

import {
  WRITER_CLIPBOARD_MAX_DOM_DEPTH,
  WRITER_CLIPBOARD_MAX_DOM_NODES,
} from "../config/clipboard";
import type {
  ClipboardDomElementNodeModel,
  ClipboardDomNodeModel,
  ClipboardDomStyleModel,
} from "../models/clipboard-model";

const EMPTY_STYLE: ClipboardDomStyleModel = {
  fontFamily: "",
  fontSize: "",
  fontWeight: "",
  fontStyle: "",
  textDecoration: "",
};

export class ClipboardDomTool {
  private nodeCount = 0;
  private limitExceeded = false;

  public parse(html: string): readonly ClipboardDomNodeModel[] | null {
    this.nodeCount = 0;
    this.limitExceeded = false;

    const documentView = new DOMParser().parseFromString(html, "text/html");
    const nodes = Array.from(documentView.body.childNodes)
      .map((node) => this.mapNode(node, 0))
      .filter((node): node is ClipboardDomNodeModel => node !== null);

    return this.limitExceeded ? null : nodes;
  }

  private mapNode(node: Node, depth: number): ClipboardDomNodeModel | null {
    this.nodeCount += 1;
    if (
      this.nodeCount > WRITER_CLIPBOARD_MAX_DOM_NODES ||
      depth > WRITER_CLIPBOARD_MAX_DOM_DEPTH
    ) {
      this.limitExceeded = true;
      return null;
    }

    if (node.nodeType === Node.TEXT_NODE) {
      return {
        kind: "text",
        text: node.textContent ?? "",
      };
    }
    if (!(node instanceof Element)) {
      return null;
    }

    const style = node instanceof HTMLElement
      ? {
          fontFamily: node.style.fontFamily,
          fontSize: node.style.fontSize,
          fontWeight: node.style.fontWeight,
          fontStyle: node.style.fontStyle,
          textDecoration: node.style.textDecoration,
        }
      : EMPTY_STYLE;

    const mapped: ClipboardDomElementNodeModel = {
      kind: "element",
      tagName: node.tagName.toLowerCase(),
      style,
      children: Array.from(node.childNodes)
        .map((child) => this.mapNode(child, depth + 1))
        .filter((child): child is ClipboardDomNodeModel => child !== null),
    };
    return mapped;
  }
}
