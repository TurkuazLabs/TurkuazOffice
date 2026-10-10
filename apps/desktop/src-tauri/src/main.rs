#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/main.rs
// # 📌 Amac: Turkuaz Office Desktop binary giris noktasini tanimlar
// # 📌 Modul - FileType: Desktop - Rust
// # Version: 0.5.1
// # Aciklama: Release Windows buildinde konsol penceresi acmadan library composition root run fonksiyonunu cagirir
// Bagimli Oldugu Katman: Controller

fn main() {
    // Only the explicit Windows pilot handles Velopack lifecycle callbacks.
    // Default Tauri NSIS and Linux entrypoints remain unchanged.
    #[cfg(all(windows, feature = "installer-velopack-pilot"))]
    velopack::VelopackApp::build()
        .set_auto_apply_on_startup(false)
        .run();

    turkuaz_office_desktop_lib::run();
}
