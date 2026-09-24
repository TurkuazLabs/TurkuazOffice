// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/tools/writer_template_catalog_tool.rs
// # 📌 Amac: Built-in Writer template YAML kaynagini Service katmanina byte olarak saglar
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Compile-time embedded template katalog dosya konumunu Service business logic'inden gizler
// Bagimli Oldugu Katman: Tool -> Config

pub struct WriterTemplateCatalogTool;

impl WriterTemplateCatalogTool {
    pub fn built_in_bytes() -> &'static [u8] {
        include_bytes!("../../templates/builtin.yml")
    }
}
