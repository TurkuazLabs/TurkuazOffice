// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/controllers/desktop_launch_controller.rs
// # 📌 Amac: Desktop suite launch IPC requestlerini DesktopLaunchService katmanina aktarir
// # 📌 Modul - FileType: Controller - Rust
// Version: 0.4.0
// Aciklama: Baslangic modulunu okur ve Start Center'dan ayri Writer/Sheet sureci baslatir
// Bagimli Oldugu Katman: Controller -> Service -> View

use tauri::State;

use crate::services::desktop_launch_service::DesktopLaunchService;
use crate::views::desktop_launch_dto::DesktopLaunchContextDto;
use crate::views::error_dto::DesktopErrorDto;

#[tauri::command]
pub fn desktop_get_launch_context(
    state: State<'_, DesktopLaunchService>,
) -> DesktopLaunchContextDto {
    state.module().into()
}

#[tauri::command]
pub fn desktop_launch_module(
    state: State<'_, DesktopLaunchService>,
    module: String,
) -> Result<(), DesktopErrorDto> {
    state
        .launch(&module)
        .map_err(DesktopErrorDto::new)
}
