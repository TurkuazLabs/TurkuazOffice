// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/web/src/config/runtime-config.ts
// # 📌 Amac: M3 Web runtime sabitlerini merkezi konfigurasyonda tutar
// # 📌 Modul - FileType: Config - TypeScript
// # Version: 0.4.0
// # Aciklama: Dev server, metadata-index, Web Core ABI/bridge ve WASM runtime binding yolunu tanimlar
// Bagimli Oldugu Katman: Config

export const WEB_APP_VERSION = "0.4.0";
export const WEB_DEV_HOST = "127.0.0.1";
export const WEB_DEV_PORT = 1430;
export const WEB_STORAGE_NAMESPACE = "turkuaz-office:web:index:v1";
export const WEB_CORE_BRIDGE_KIND = "browser-contract";
export const WEB_CORE_ABI_VERSION = 1;
export const WEB_CORE_SCHEMA_VERSION = 1;
export const WEB_CORE_WASM_MODULE_URL = "/wasm/turkuaz_office_core.js";
export const WEB_CORE_NATIVE_FS_CONTRACT_ERROR =
  "Web Core native filesystem capability false olmali.";
export const WEB_APP_ROOT_ID = "app";
export const WEB_APP_ROOT_ERROR = "Turkuaz Office Web app root bulunamadi.";
