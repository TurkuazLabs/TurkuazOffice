// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/models/web-models.ts
// # 📌 Amac: M3 Web istemcisinin typed read-model ve browser document index tiplerini tanimlar
// # 📌 Modul - FileType: Model - TypeScript
// # Version: 0.4.0
// # Aciklama: View ile Service arasindaki platformdan bagimsiz web read-model ve metadata kontratini tasir
// Bagimli Oldugu Katman: Service

export interface WebDocumentIndexEntry {
  readonly id: string;
  readonly title: string;
  readonly revision: number;
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
  readonly storedDocumentCount: number;
}
