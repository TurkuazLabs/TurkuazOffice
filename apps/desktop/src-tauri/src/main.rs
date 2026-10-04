// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/main.rs
// # 📌 Amac: Turkuaz Office Desktop binary giris noktasini tanimlar
// # 📌 Modul - FileType: Desktop - Rust
// # Version: 0.2.0
// # Aciklama: Binary yalnizca library composition root run fonksiyonunu cagirir
// Bagimli Oldugu Katman: Controller

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    turkuaz_office_desktop_lib::run();
}
