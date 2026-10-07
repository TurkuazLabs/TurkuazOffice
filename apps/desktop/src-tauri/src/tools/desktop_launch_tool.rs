// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/tools/desktop_launch_tool.rs
// # 📌 Amac: Desktop suite baslatma argumanindan Baslangic Merkezi, Writer veya Sheet hedefini belirler
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.12.0
// Aciklama: Varsayilan suite girisini Baslangic Merkezi yapar; --module writer/sheet/home kontratini parse eder
// Bagimli Oldugu Katman: Tool

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DesktopLaunchModule {
    Home,
    Writer,
    Sheet,
}

impl DesktopLaunchModule {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Writer => "writer",
            Self::Sheet => "sheet",
        }
    }

    #[must_use]
    pub const fn window_title(self) -> &'static str {
        match self {
            Self::Home => "Turkuaz Office",
            Self::Writer => "Turkuaz Office Writer",
            Self::Sheet => "Turkuaz Office Sheet",
        }
    }
}

pub struct DesktopLaunchTool;

impl DesktopLaunchTool {
    #[must_use]
    pub fn from_environment() -> DesktopLaunchModule {
        Self::from_args(std::env::args())
    }

    #[must_use]
    pub fn from_args<I, S>(args: I) -> DesktopLaunchModule
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let args = args
            .into_iter()
            .map(|value| value.as_ref().to_owned())
            .collect::<Vec<_>>();

        for (index, value) in args.iter().enumerate() {
            if value == "--module"
                && let Some(module) = args.get(index + 1)
            {
                return Self::parse_module(module);
            }
            if let Some(module) = value.strip_prefix("--module=") {
                return Self::parse_module(module);
            }
        }

        DesktopLaunchModule::Home
    }

    fn parse_module(value: &str) -> DesktopLaunchModule {
        if value.eq_ignore_ascii_case("writer") {
            DesktopLaunchModule::Writer
        } else if value.eq_ignore_ascii_case("sheet") {
            DesktopLaunchModule::Sheet
        } else {
            DesktopLaunchModule::Home
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DesktopLaunchModule, DesktopLaunchTool};

    #[test]
    fn defaults_to_home() {
        assert_eq!(
            DesktopLaunchTool::from_args(["turkuaz-office.exe"]),
            DesktopLaunchModule::Home
        );
    }

    #[test]
    fn parses_sheet_separate_argument() {
        assert_eq!(
            DesktopLaunchTool::from_args(["turkuaz-office.exe", "--module", "sheet"]),
            DesktopLaunchModule::Sheet
        );
    }

    #[test]
    fn parses_writer_equals_argument() {
        assert_eq!(
            DesktopLaunchTool::from_args(["turkuaz-office.exe", "--module=writer"]),
            DesktopLaunchModule::Writer
        );
    }

    #[test]
    fn parses_home_and_unknown_as_safe_start_center() {
        assert_eq!(
            DesktopLaunchTool::from_args(["turkuaz-office.exe", "--module=home"]),
            DesktopLaunchModule::Home
        );
        assert_eq!(
            DesktopLaunchTool::from_args(["turkuaz-office.exe", "--module=unknown"]),
            DesktopLaunchModule::Home
        );
    }
}
