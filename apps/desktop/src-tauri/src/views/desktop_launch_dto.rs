// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/views/desktop_launch_dto.rs
// # 📌 Amac: Desktop suite baslangic modulunu frontend'e typed DTO olarak tasir
// # 📌 Modul - FileType: View - Rust
// Version: 0.4.0
// Aciklama: Start Center, Writer ve Sheet launch sonucunu JSON uyumlu sekilde aktarir
// Bagimli Oldugu Katman: View

use serde::Serialize;

use crate::services::desktop_launch_service::DesktopLaunchModule;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DesktopLaunchContextDto {
    pub module: String,
}

impl From<DesktopLaunchModule> for DesktopLaunchContextDto {
    fn from(value: DesktopLaunchModule) -> Self {
        Self {
            module: value.as_str().to_owned(),
        }
    }
}
