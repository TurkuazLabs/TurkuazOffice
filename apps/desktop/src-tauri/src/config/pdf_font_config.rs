// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/config/pdf_font_config.rs
// # 📌 Amac: PDF export requested font family icin Windows/Linux fallback profillerini merkezi tanimlar
// # 📌 Modul - FileType: Config - Rust
// # Version: 0.2.0
// # Aciklama: Serif, sans-serif ve monospace ailelerini mevcut Writer render fallback prensibiyle ayri tutar
// Bagimli Oldugu Katman: Config

pub const PDF_FALLBACK_ARIAL: &[&str] = &["Liberation Sans", "Nimbus Sans", "Noto Sans"];
pub const PDF_FALLBACK_CALIBRI: &[&str] = &["Carlito", "Arial", "Liberation Sans", "Noto Sans"];
pub const PDF_FALLBACK_TIMES_NEW_ROMAN: &[&str] =
    &["Liberation Serif", "Nimbus Roman", "Noto Serif"];
pub const PDF_FALLBACK_GEORGIA: &[&str] = &["Liberation Serif", "Nimbus Roman", "Noto Serif"];
pub const PDF_FALLBACK_VERDANA: &[&str] = &["DejaVu Sans", "Liberation Sans", "Noto Sans"];
pub const PDF_FALLBACK_COURIER_NEW: &[&str] =
    &["Liberation Mono", "Nimbus Mono PS", "Noto Sans Mono"];
pub const PDF_FALLBACK_DEFAULT: &[&str] =
    &["Noto Sans", "DejaVu Sans", "Liberation Sans", "Nimbus Sans"];

pub fn pdf_font_fallbacks(requested_family: &str) -> &'static [&'static str] {
    match requested_family {
        "Arial" => PDF_FALLBACK_ARIAL,
        "Calibri" => PDF_FALLBACK_CALIBRI,
        "Times New Roman" => PDF_FALLBACK_TIMES_NEW_ROMAN,
        "Georgia" => PDF_FALLBACK_GEORGIA,
        "Verdana" => PDF_FALLBACK_VERDANA,
        "Courier New" => PDF_FALLBACK_COURIER_NEW,
        _ => PDF_FALLBACK_DEFAULT,
    }
}
