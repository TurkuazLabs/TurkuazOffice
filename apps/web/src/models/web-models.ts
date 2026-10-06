// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/models/web-models.ts
// # 📌 Amac: M3 Web istemcisinin typed read-model ve browser document index tiplerini tanimlar
// # 📌 Modul - FileType: Model - TypeScript
// Version: 0.4.0
// Aciklama: View ile Service arasindaki platformdan bagimsiz web read-model, TKO import/session summary ve metadata kontratini tasir
// Bagimli Oldugu Katman: Service

export interface WebDocumentIndexEntry {
  readonly id: string;
  readonly title: string;
  readonly revision: number;
}

export interface WebCanonicalDocumentRecord {
  readonly id: string;
  readonly title: string;
  readonly text: string;
  readonly schemaVersion: number;
  readonly revision: number;
}

export type WebDocumentStorageKind = "indexed-db";

export interface WebFilePickRequest {
  readonly accept: string;
  readonly maxBytes: number;
}

export interface WebPickedFile {
  readonly name: string;
  readonly mediaType: string;
  readonly bytes: Uint8Array;
}

export interface WebDownloadFile {
  readonly fileName: string;
  readonly mediaType: string;
  readonly bytes: Uint8Array;
}

export interface WebWriterTkoSummary {
  readonly id: string;
  readonly title: string;
  readonly schemaVersion: number;
  readonly revision: number;
  readonly sectionCount: number;
  readonly assetCount: number;
}

export interface WebNativeDocumentImport {
  readonly fileName: string;
  readonly bytes: Uint8Array;
  readonly summary: WebWriterTkoSummary;
}

export interface WebWriterSessionDocument extends WebNativeDocumentImport {}

export interface WebImportExportCapabilities {
  readonly browserFileTransferAvailable: true;
  readonly nativeTkoCodecAvailable: boolean;
}

export interface WebCoreCapabilities {
  readonly bridgeKind: string;
  readonly abiVersion: number;
  readonly schemaVersion: number;
  readonly browserMetadataStorage: true;
  readonly nativeFileSystemAccess: false;
}

export interface WebBootstrapViewModel {
  readonly version: string;
  readonly capabilities: WebCoreCapabilities;
  readonly documentStorageKind: WebDocumentStorageKind;
  readonly documentStorageAvailable: boolean;
  readonly storedDocumentCount: number | null;
}
