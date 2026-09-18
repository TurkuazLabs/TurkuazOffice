// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/tools/tko_yaml_tool.rs
// # 📌 Amac: TKO format DTO'larini YAML byte akimina ceviren dis format adaptorunu saglar
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Noyalib serde-yaml compatibility detayini Service katmanindan gizleyen serialize/deserialize Tool siniridir
// Bagimli Oldugu Katman: Tool

use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TkoYamlError {
    SerializeFailed,
    DeserializeFailed,
}

pub struct TkoYamlTool;

impl TkoYamlTool {
    pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, TkoYamlError> {
        noyalib::compat::serde_yaml::to_string(value)
            .map(String::into_bytes)
            .map_err(|_| TkoYamlError::SerializeFailed)
    }

    pub fn deserialize<T: DeserializeOwned + 'static>(bytes: &[u8]) -> Result<T, TkoYamlError> {
        let text = std::str::from_utf8(bytes).map_err(|_| TkoYamlError::DeserializeFailed)?;
        noyalib::compat::serde_yaml::from_str(text).map_err(|_| TkoYamlError::DeserializeFailed)
    }
}
