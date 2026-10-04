// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/controllers/desktop_launch_controller.rs
// # 📌 Amac: Native suite baslatma hedefini frontend composition rootuna IPC ile sunar
// # 📌 Modul - FileType: Controller - Rust
// Version: 0.5.0
// Aciklama: Tool katmaninda belirlenen Writer/Sheet launch hedefini ince Tauri command olarak expose eder
// Bagimli Oldugu Katman: Controller -> Tool

use crate::tools::desktop_launch_tool::DesktopLaunchTool;

#[tauri::command]
#[must_use]
pub fn desktop_get_launch_module() -> &'static str {
    DesktopLaunchTool::from_environment().as_str()
}
