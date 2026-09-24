// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/tools/cell_reference_tool.rs
// # 📌 Amac: A1 cell reference metni ile zero-based CellAddress arasinda guvenli parse/format adaptasyonu yapar
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.3.0
// Aciklama: XLSX grid sinirlari icinde A1..XFD1048576 parse/format eder ve gecersiz referansi typed hata ile reddeder
// Bagimli Oldugu Katman: Tool -> Config -> Service

use crate::config::constants::{MAX_SHEET_COLUMNS, MAX_SHEET_ROWS};
use crate::services::sheet_types::CellAddress;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellReferenceError {
    Empty,
    InvalidFormat,
    OutOfBounds,
}

pub struct CellReferenceTool;

impl CellReferenceTool {
    pub fn parse(reference: &str) -> Result<CellAddress, CellReferenceError> {
        let reference = reference.trim();
        if reference.is_empty() {
            return Err(CellReferenceError::Empty);
        }

        let split = reference
            .bytes()
            .position(|byte| byte.is_ascii_digit())
            .ok_or(CellReferenceError::InvalidFormat)?;
        if split == 0 || split == reference.len() {
            return Err(CellReferenceError::InvalidFormat);
        }

        let (column_text, row_text) = reference.split_at(split);
        if !column_text.bytes().all(|byte| byte.is_ascii_alphabetic())
            || !row_text.bytes().all(|byte| byte.is_ascii_digit())
            || row_text.starts_with('0')
        {
            return Err(CellReferenceError::InvalidFormat);
        }

        let mut column_number = 0_u32;
        for byte in column_text.bytes() {
            let upper = byte.to_ascii_uppercase();
            let value = u32::from(upper - b'A' + 1);
            column_number = column_number
                .checked_mul(26)
                .and_then(|current| current.checked_add(value))
                .ok_or(CellReferenceError::OutOfBounds)?;
        }

        let row_number = row_text
            .parse::<u32>()
            .map_err(|_| CellReferenceError::OutOfBounds)?;
        if column_number == 0
            || column_number > MAX_SHEET_COLUMNS
            || row_number == 0
            || row_number > MAX_SHEET_ROWS
        {
            return Err(CellReferenceError::OutOfBounds);
        }

        Ok(CellAddress {
            row: row_number - 1,
            column: column_number - 1,
        })
    }

    pub fn format(address: CellAddress) -> Result<String, CellReferenceError> {
        if address.row >= MAX_SHEET_ROWS || address.column >= MAX_SHEET_COLUMNS {
            return Err(CellReferenceError::OutOfBounds);
        }

        let mut column_number = address.column + 1;
        let mut letters = Vec::new();
        while column_number > 0 {
            let remainder = (column_number - 1) % 26;
            let letter_offset =
                u8::try_from(remainder).map_err(|_| CellReferenceError::OutOfBounds)?;
            letters.push(char::from(b'A' + letter_offset));
            column_number = (column_number - 1) / 26;
        }
        letters.reverse();
        let column: String = letters.into_iter().collect();
        Ok(format!("{column}{}", address.row + 1))
    }
}
