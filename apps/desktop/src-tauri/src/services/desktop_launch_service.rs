// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/services/desktop_launch_service.rs
// # 📌 Amac: Turkuaz Office suite baslangic modulunu process argumentlarindan cozer ve ayri modul surecleri baslatir
// # 📌 Modul - FileType: Service - Rust
// Version: 0.4.0
// Aciklama: Start Center, Writer ve Sheet launch kontratini tek Service sinirinda tutar
// Bagimli Oldugu Katman: Service -> Tool -> Config

use std::env;
use std::path::Path;
use std::process::Command;

use crate::config::constants::{
    DESKTOP_MODULE_ARGUMENT_PREFIX, DESKTOP_MODULE_SHEET, DESKTOP_MODULE_START,
    DESKTOP_MODULE_WRITER, DESKTOP_TITLE_SHEET, DESKTOP_TITLE_START, DESKTOP_TITLE_WRITER,
    ERROR_DESKTOP_LAUNCH_FAILED, ERROR_DESKTOP_MODULE_INVALID,
};
use turkuaz_office_writer::config::constants::TKO_FILE_EXTENSION;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DesktopLaunchModule {
    Start,
    Writer,
    Sheet,
}

impl DesktopLaunchModule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => DESKTOP_MODULE_START,
            Self::Writer => DESKTOP_MODULE_WRITER,
            Self::Sheet => DESKTOP_MODULE_SHEET,
        }
    }

    pub fn window_title(self) -> &'static str {
        match self {
            Self::Start => DESKTOP_TITLE_START,
            Self::Writer => DESKTOP_TITLE_WRITER,
            Self::Sheet => DESKTOP_TITLE_SHEET,
        }
    }

    fn from_value(value: &str) -> Option<Self> {
        match value {
            DESKTOP_MODULE_START => Some(Self::Start),
            DESKTOP_MODULE_WRITER => Some(Self::Writer),
            DESKTOP_MODULE_SHEET => Some(Self::Sheet),
            _ => None,
        }
    }
}

pub struct DesktopLaunchService {
    module: DesktopLaunchModule,
}

impl DesktopLaunchService {
    pub fn from_arguments<I>(arguments: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        let arguments: Vec<String> = arguments.into_iter().collect();
        let explicit = arguments.iter().find_map(|argument| {
            argument
                .strip_prefix(DESKTOP_MODULE_ARGUMENT_PREFIX)
                .and_then(DesktopLaunchModule::from_value)
        });
        let module = explicit.unwrap_or_else(|| {
            if arguments.iter().any(|argument| Self::is_native_document(argument)) {
                DesktopLaunchModule::Writer
            } else {
                DesktopLaunchModule::Start
            }
        });
        Self { module }
    }

    pub fn module(&self) -> DesktopLaunchModule {
        self.module
    }

    pub fn launch(&self, module: &str) -> Result<(), &'static str> {
        let module = DesktopLaunchModule::from_value(module)
            .filter(|value| *value != DesktopLaunchModule::Start)
            .ok_or(ERROR_DESKTOP_MODULE_INVALID)?;
        let executable = env::current_exe().map_err(|_| ERROR_DESKTOP_LAUNCH_FAILED)?;
        Command::new(executable)
            .arg(format!("{DESKTOP_MODULE_ARGUMENT_PREFIX}{}", module.as_str()))
            .spawn()
            .map_err(|_| ERROR_DESKTOP_LAUNCH_FAILED)?;
        Ok(())
    }

    fn is_native_document(argument: &str) -> bool {
        Path::new(argument)
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(TKO_FILE_EXTENSION))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_start_center() {
        let service = DesktopLaunchService::from_arguments(Vec::<String>::new());
        assert_eq!(service.module(), DesktopLaunchModule::Start);
    }

    #[test]
    fn explicit_sheet_argument_selects_sheet() {
        let service = DesktopLaunchService::from_arguments(vec!["--module=sheet".to_owned()]);
        assert_eq!(service.module(), DesktopLaunchModule::Sheet);
    }

    #[test]
    fn native_document_defaults_to_writer() {
        let service = DesktopLaunchService::from_arguments(vec!["C:/Temp/test.tko".to_owned()]);
        assert_eq!(service.module(), DesktopLaunchModule::Writer);
    }
}
