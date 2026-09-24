// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/services/startup_file_service.rs
// # 📌 Amac: Windows/Linux file association startup argumentlarindan ilk gecerli native TKO belgesini secer
// # 📌 Modul - FileType: Service - Rust
// # Version: 0.2.0
// # Aciklama: Flag argumentlarini eler, existing TKO path'i canonicalize eder ve startup file'i tek kullanimlik state olarak tutar
// Bagimli Oldugu Katman: Service -> Tool

use std::path::Path;

use turkuaz_office_writer::config::constants::TKO_FILE_EXTENSION;

use crate::config::constants::STARTUP_ARGUMENT_FLAG_PREFIX;
use crate::tools::local_file_tool::LocalFileTool;

#[derive(Default)]
pub struct StartupFileService {
    pending_path: Option<String>,
}

impl StartupFileService {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_arguments<I>(arguments: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        let pending_path = arguments.into_iter().find_map(Self::native_file_path);
        Self { pending_path }
    }

    pub fn take(&mut self) -> Option<String> {
        self.pending_path.take()
    }

    fn native_file_path(argument: String) -> Option<String> {
        if argument.starts_with(STARTUP_ARGUMENT_FLAG_PREFIX) {
            return None;
        }
        let path = LocalFileTool::canonicalize_file(Path::new(&argument)).ok()?;
        let is_tko = path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(TKO_FILE_EXTENSION));
        is_tko.then(|| path.to_string_lossy().into_owned())
    }
}
