// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/config/runtime-config.ts
// # 📌 Amac: M3 Web runtime sabitlerini merkezi konfigurasyonda tutar
// # 📌 Modul - FileType: Config - TypeScript
// Version: 0.4.0
// Aciklama: Dev server, metadata-index, Web Core ABI/bridge, Writer TKO Tool, session ve browser import/export runtime kontratini tanimlar
// Bagimli Oldugu Katman: Config

export const WEB_APP_VERSION = "0.4.0";
export const WEB_DEV_HOST = "127.0.0.1";
export const WEB_DEV_PORT = 1430;
export const WEB_STORAGE_NAMESPACE = "turkuaz-office:web:index:v1";
export const WEB_CORE_BRIDGE_KIND = "browser-contract";
export const WEB_CORE_ABI_VERSION = 1;
export const WEB_CORE_SCHEMA_VERSION = 1;
export const WEB_CORE_WASM_MODULE_URL = "/wasm/turkuaz_office_web_bridge.js";
export const WEB_CORE_NATIVE_FS_CONTRACT_ERROR =
  "Web Core native filesystem capability false olmali.";
export const WEB_WASM_BINDING_FUNCTION_MISSING_ERROR =
  "WASM Web bridge binding function eksik";
export const WEB_INVALID_TKO_SUMMARY_ERROR =
  "Writer TKO WASM inspect cevabi gecersiz.";
export const WEB_TKO_CODEC_UNAVAILABLE_ERROR =
  "Writer TKO codec browser runtime'da kullanilabilir degil.";
export const WEB_WRITER_SESSION_EMPTY_ERROR =
  "Aktif Writer Web oturumu bulunamadi.";
export const WEB_APP_ROOT_ID = "app";
export const WEB_APP_ROOT_ERROR = "Turkuaz Office Web app root bulunamadi.";

export const WEB_DOCUMENT_DB_NAME = "turkuaz-office-web";
export const WEB_DOCUMENT_DB_VERSION = 1;
export const WEB_DOCUMENT_STORE_NAME = "documents";
export const WEB_DOCUMENT_STORE_KEY_PATH = "id";
export const WEB_DOCUMENT_STORAGE_KIND = "indexed-db";
export const WEB_INDEXED_DB_OPERATION_ERROR =
  "IndexedDB belge islemi tamamlanamadi.";
export const WEB_INVALID_CANONICAL_DOCUMENT_ERROR =
  "Canonical web belge kaydi gecersiz.";

export const WEB_NATIVE_DOCUMENT_EXTENSION = ".tko";
export const WEB_NATIVE_DOCUMENT_MIME_TYPE = "application/x-turkuaz-office";
export const WEB_NATIVE_DOCUMENT_ACCEPT =
  ".tko,application/x-turkuaz-office";
export const WEB_BROWSER_IMPORT_MAX_BYTES = 16 * 1024 * 1024;
export const WEB_DOWNLOAD_URL_REVOKE_DELAY_MS = 1_000;
export const WEB_IMPORT_FILE_TOO_LARGE_ERROR =
  "Secilen browser import dosyasi boyut limitini asiyor.";
export const WEB_UNSUPPORTED_IMPORT_FILE_ERROR =
  "Secilen browser import dosya uzantisi desteklenmiyor.";
