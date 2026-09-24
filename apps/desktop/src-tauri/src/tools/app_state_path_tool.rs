// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/tools/app_state_path_tool.rs
// # 📌 Amac: Windows/Linux/macOS Turkuaz Office local state kok dizinini tek adaptor arkasinda cozer
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Recovery ve recent-files gibi Desktop state tuketicilerinin platform path mantigini tekrar etmesini engeller
// Bagimli Oldugu Katman: Tool -> Config

use std::env;
use std::path::PathBuf;

use crate::config::constants::APP_DIRECTORY_NAME;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use crate::config::constants::HOME_DIRECTORY_ENV;
#[cfg(target_os = "linux")]
use crate::config::constants::{
    LINUX_APP_DIRECTORY_NAME, LINUX_LOCAL_STATE_SEGMENTS, XDG_STATE_HOME_ENV,
};
#[cfg(target_os = "macos")]
use crate::config::constants::MACOS_APP_SUPPORT_SEGMENTS;
#[cfg(target_os = "windows")]
use crate::config::constants::{
    TURKUAZLABS_DIRECTORY_NAME, WINDOWS_LOCAL_APP_DATA_ENV,
};

pub struct AppStatePathTool;

impl AppStatePathTool {
    #[cfg(target_os = "windows")]
    pub fn state_root() -> PathBuf {
        env::var_os(WINDOWS_LOCAL_APP_DATA_ENV)
            .map(PathBuf::from)
            .unwrap_or_else(env::temp_dir)
            .join(TURKUAZLABS_DIRECTORY_NAME)
            .join(APP_DIRECTORY_NAME)
    }

    #[cfg(target_os = "linux")]
    pub fn state_root() -> PathBuf {
        if let Some(path) = env::var_os(XDG_STATE_HOME_ENV) {
            return PathBuf::from(path).join(LINUX_APP_DIRECTORY_NAME);
        }
        if let Some(home) = env::var_os(HOME_DIRECTORY_ENV) {
            let mut path = PathBuf::from(home);
            for segment in LINUX_LOCAL_STATE_SEGMENTS {
                path.push(segment);
            }
            return path.join(LINUX_APP_DIRECTORY_NAME);
        }
        env::temp_dir().join(APP_DIRECTORY_NAME)
    }

    #[cfg(target_os = "macos")]
    pub fn state_root() -> PathBuf {
        let mut path = env::var_os(HOME_DIRECTORY_ENV)
            .map(PathBuf::from)
            .unwrap_or_else(env::temp_dir);
        for segment in MACOS_APP_SUPPORT_SEGMENTS {
            path.push(segment);
        }
        path.join(APP_DIRECTORY_NAME)
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    pub fn state_root() -> PathBuf {
        env::temp_dir().join(APP_DIRECTORY_NAME)
    }
}
