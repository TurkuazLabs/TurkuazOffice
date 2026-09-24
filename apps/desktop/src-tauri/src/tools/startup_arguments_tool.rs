// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/tools/startup_arguments_tool.rs
// # 📌 Amac: Windows/Linux process startup argumentlarini Desktop Service katmanina platformdan bagimsiz saglar
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Executable argv[0] degerini atlayip kalan process argumentlarini dis dunya girdisi olarak toplar
// Bagimli Oldugu Katman: Tool

use std::env;

pub struct StartupArgumentsTool;

impl StartupArgumentsTool {
    pub fn arguments() -> Vec<String> {
        env::args().skip(1).collect()
    }
}
