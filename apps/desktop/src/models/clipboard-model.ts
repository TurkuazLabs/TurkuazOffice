// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/models/clipboard-model.ts
// # 📌 Amac: Clipboard transfer, internal fragment ve inert HTML parse modellerini tanimlar
// # 📌 Modul - FileType: Model - TypeScript
// # Version: 0.2.1
// # Aciklama: Service ve Tool katmanlari arasinda browser nesnesi tasimayan typed clipboard kontratini saglar
// Bagimli Oldugu Katman: Model

export interface ClipboardCharacterStyleModel {
  readonly bold: boolean;
  readonly italic: boolean;
  readonly underline: boolean;
  readonly fontFamily: string;
  readonly fontSizeHalfPoints: number;
}

export interface ClipboardStyledRunModel {
  readonly text: string;
  readonly style: ClipboardCharacterStyleModel;
}

export interface WriterClipboardFragmentModel {
  readonly kind: string;
  readonly schemaVersion: number;
  readonly runs: readonly ClipboardStyledRunModel[];
}

export interface ClipboardImageModel {
  readonly mediaType: string;
  readonly data: readonly number[];
}

export interface ClipboardTransferModel {
  readonly internalFragment: string;
  readonly html: string;
  readonly plainText: string;
}

export interface ClipboardWriteModel {
  readonly internalFragment: string;
  readonly html: string;
  readonly plainText: string;
}

export interface ClipboardDomStyleModel {
  readonly fontFamily: string;
  readonly fontSize: string;
  readonly fontWeight: string;
  readonly fontStyle: string;
  readonly textDecoration: string;
}

interface ClipboardDomTextNodeModel {
  readonly kind: "text";
  readonly text: string;
}

export interface ClipboardDomElementNodeModel {
  readonly kind: "element";
  readonly tagName: string;
  readonly style: ClipboardDomStyleModel;
  readonly children: readonly ClipboardDomNodeModel[];
}

export type ClipboardDomNodeModel =
  | ClipboardDomTextNodeModel
  | ClipboardDomElementNodeModel;
