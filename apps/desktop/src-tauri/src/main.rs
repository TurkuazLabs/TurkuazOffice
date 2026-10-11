#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/src/main.rs
// # 📌 Amac: Turkuaz Office Desktop binary giris noktasini tanimlar
// # 📌 Modul - FileType: Desktop - Rust
// # Version: 0.5.4
// # Aciklama: Release Windows buildinde konsol penceresi acmadan library composition root run fonksiyonunu cagirir
// Bagimli Oldugu Katman: Controller

fn main() {
    // User-reported Windows pilot bug: plain cargo build --release did not
    // enable Tauri's production feature and launched WebView2 at 127.0.0.1.
    // The optional pilot feature now also activates tauri/custom-protocol.
    // This headless gate verifies the generated native binary truly embeds
    // the frontend before it can be published or installed by any CI job.
    #[cfg(all(windows, feature = "installer-velopack-pilot"))]
    if std::env::args_os().any(|argument| argument == "--office-pilot-verify-embedded-frontend") {
        let context: tauri::Context<tauri::Wry> = tauri::generate_context!();
        // Tauri release embeds Brotli-compressed assets. iter() returns
        // compressed bytes; get() resolves/decompresses the requested file.
        let index_key = tauri::utils::assets::AssetKey::from("index.html");
        let index_found = context.assets().get(&index_key).is_some_and(|html| {
            html.len() > 64 && String::from_utf8_lossy(&html).contains("<html")
        });
        if tauri::is_dev() || !index_found {
            std::process::exit(31);
        }
        return;
    }

    // Only the explicit Windows pilot handles Velopack lifecycle callbacks.
    // Default Tauri NSIS and Linux entrypoints remain unchanged.
    #[cfg(all(windows, feature = "installer-velopack-pilot"))]
    velopack::VelopackApp::build()
        .set_auto_apply_on_startup(false)
        .run();

    turkuaz_office_desktop_lib::run();
}
