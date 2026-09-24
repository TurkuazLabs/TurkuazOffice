// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/tools/recovery_path_tool.rs
// # 📌 Amac: Turkuaz Office recovery storage dizinini ortak Desktop state kokunden cozer
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Recovery path mantigini AppStatePathTool uzerinden kurar ve platform path tekrarini engeller
// Bagimli Oldugu Katman: Tool -> Config

use std::path::PathBuf;

use crate::config::constants::RECOVERY_DIRECTORY_NAME;
use crate::tools::app_state_path_tool::AppStatePathTool;

pub struct RecoveryPathTool;

impl RecoveryPathTool {
    pub fn recovery_directory() -> PathBuf {
        AppStatePathTool::state_root().join(RECOVERY_DIRECTORY_NAME)
    }
}
