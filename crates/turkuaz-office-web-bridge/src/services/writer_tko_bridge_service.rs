// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-web-bridge/src/services/writer_tko_bridge_service.rs
// # 📌 Amac: Writer TKO byte payloadini mevcut Rust domain codec ile Web bridge yuzeyine map eder
// # 📌 Modul - FileType: Service - Rust
// Version: 0.4.0
// Aciklama: TKO deserialize/inspect/re-encode akisini koordine eder ve domain hatalarini stable Web hata kodlarina map eder
// Bagimli Oldugu Katman: Service -> Writer Service -> View -> Config

use turkuaz_office_writer::tools::tko_archive_tool::TkoArchiveError;
use turkuaz_office_writer::{TkoPackageError, TkoPackageService, TkoProfileError};

use crate::config::constants::{
    WEB_TKO_ERROR_FUTURE_SCHEMA, WEB_TKO_ERROR_INVALID_PACKAGE, WEB_TKO_ERROR_MIGRATION_REQUIRED,
    WEB_TKO_ERROR_PACKAGE_TOO_LARGE,
};
use crate::views::writer_tko_bridge_view::WebWriterTkoSummaryView;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WebWriterTkoBridgeError {
    PackageTooLarge,
    MigrationRequired,
    FutureSchema,
    InvalidPackage,
}

impl WebWriterTkoBridgeError {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::PackageTooLarge => WEB_TKO_ERROR_PACKAGE_TOO_LARGE,
            Self::MigrationRequired => WEB_TKO_ERROR_MIGRATION_REQUIRED,
            Self::FutureSchema => WEB_TKO_ERROR_FUTURE_SCHEMA,
            Self::InvalidPackage => WEB_TKO_ERROR_INVALID_PACKAGE,
        }
    }
}

pub struct WebWriterTkoBridgeService;

impl WebWriterTkoBridgeService {
    pub fn inspect(bytes: &[u8]) -> Result<WebWriterTkoSummaryView, WebWriterTkoBridgeError> {
        let document = TkoPackageService::deserialize(bytes).map_err(map_tko_error)?;
        Ok(WebWriterTkoSummaryView::from(&document))
    }

    pub fn reencode(bytes: &[u8], app_version: &str) -> Result<Vec<u8>, WebWriterTkoBridgeError> {
        let document = TkoPackageService::deserialize(bytes).map_err(map_tko_error)?;
        TkoPackageService::serialize(&document, app_version).map_err(map_tko_error)
    }
}

fn map_tko_error(error: TkoPackageError) -> WebWriterTkoBridgeError {
    match error {
        TkoPackageError::Archive(
            TkoArchiveError::PackageTooLarge | TkoArchiveError::EntryTooLarge,
        ) => WebWriterTkoBridgeError::PackageTooLarge,
        TkoPackageError::Profile(TkoProfileError::MigrationRequired) => {
            WebWriterTkoBridgeError::MigrationRequired
        }
        TkoPackageError::Profile(TkoProfileError::FutureSchema) => {
            WebWriterTkoBridgeError::FutureSchema
        }
        _ => WebWriterTkoBridgeError::InvalidPackage,
    }
}
