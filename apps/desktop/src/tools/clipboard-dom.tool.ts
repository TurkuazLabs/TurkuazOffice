// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/clipboard-dom.tool.ts
// # 📌 Amac: HTML clipboard verisini inert DOM agacina parse edip browser API detaylarini izole eder
// # 📌 Modul - FileType: Tool - TypeScript
// # Version: 0.2.0
// # Aciklama: DOMParser sonucunu yalniz tag, desteklenen inline style ve text alanlarindan olusan Model agacina cevirir
// Bagimli Oldugu Katman: Tool -> Model

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
  public parse(html: string): readonly ClipboardDomNodeModel[] {
    const documentView = new DOMParser().parseFromString(html, "text/html");
    return Array.from(documentView.body.childNodes)
      .map((node) => this.mapNode(node))
      .filter((node): node is ClipboardDomNodeModel => node !== null);
  }

  private mapNode(node: Node): ClipboardDomNodeModel | null {
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
        .map((child) => this.mapNode(child))
        .filter((child): child is ClipboardDomNodeModel => child !== null),
    };
    return mapped;
  }
}
