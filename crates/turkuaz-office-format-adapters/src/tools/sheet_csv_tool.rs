// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/tools/sheet_csv_tool.rs
// # 📌 Amac: CSV byte akimini quote/CRLF kurallari ve resource limitleriyle satir-hucre matrisine parse eder/yazar
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.3.0
// Aciklama: CSV teknik syntax detayini canonical Sheet model ve Service business logic'inden ayirir
// Bagimli Oldugu Katman: Tool -> Config

use crate::config::sheet_constants::{MAX_CSV_BYTES, MAX_CSV_COLUMNS, MAX_CSV_ROWS};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SheetCsvToolError {
    TooLarge,
    TooManyRows,
    TooManyColumns,
    InvalidUtf8,
    InvalidCsv,
}

pub struct SheetCsvTool;

impl SheetCsvTool {
    pub fn parse(bytes: &[u8]) -> Result<Vec<Vec<String>>, SheetCsvToolError> {
        if bytes.len() > MAX_CSV_BYTES {
            return Err(SheetCsvToolError::TooLarge);
        }
        let raw = std::str::from_utf8(bytes).map_err(|_| SheetCsvToolError::InvalidUtf8)?;
        let text = raw.strip_prefix('﻿').unwrap_or(raw);
        if text.is_empty() {
            return Ok(Vec::new());
        }

        let mut rows = Vec::new();
        let mut row = Vec::new();
        let mut field = String::new();
        let mut chars = text.chars().peekable();
        let mut in_quotes = false;
        let mut field_started = false;
        let mut ended_row = false;

        while let Some(character) = chars.next() {
            if in_quotes {
                if character == '"' {
                    if chars.peek() == Some(&'"') {
                        chars.next();
                        field.push('"');
                    } else {
                        in_quotes = false;
                    }
                } else {
                    field.push(character);
                }
                ended_row = false;
                continue;
            }

            match character {
                '"' if !field_started && field.is_empty() => {
                    in_quotes = true;
                    field_started = true;
                    ended_row = false;
                }
                '"' => return Err(SheetCsvToolError::InvalidCsv),
                ',' => {
                    row.push(std::mem::take(&mut field));
                    field_started = false;
                    ended_row = false;
                    Self::validate_columns(&row)?;
                }
                '
' => {
                    row.push(std::mem::take(&mut field));
                    field_started = false;
                    Self::push_row(&mut rows, &mut row)?;
                    ended_row = true;
                }
                '' => {
                    if chars.peek() == Some(&'
') {
                        chars.next();
                    }
                    row.push(std::mem::take(&mut field));
                    field_started = false;
                    Self::push_row(&mut rows, &mut row)?;
                    ended_row = true;
                }
                _ => {
                    field.push(character);
                    field_started = true;
                    ended_row = false;
                }
            }
        }

        if in_quotes {
            return Err(SheetCsvToolError::InvalidCsv);
        }

        if !ended_row || !field.is_empty() || !row.is_empty() {
            row.push(field);
            Self::push_row(&mut rows, &mut row)?;
        }

        Ok(rows)
    }

    pub fn encode(rows: &[Vec<String>]) -> Result<Vec<u8>, SheetCsvToolError> {
        if rows.len() > MAX_CSV_ROWS {
            return Err(SheetCsvToolError::TooManyRows);
        }

        let mut output = String::new();
        for (row_index, row) in rows.iter().enumerate() {
            Self::validate_columns(row)?;
            if row_index > 0 {
                output.push_str("
");
            }
            for (column_index, value) in row.iter().enumerate() {
                if column_index > 0 {
                    output.push(',');
                }
                Self::write_field(&mut output, value);
            }
            if output.len() > MAX_CSV_BYTES {
                return Err(SheetCsvToolError::TooLarge);
            }
        }

        if output.len() > MAX_CSV_BYTES {
            return Err(SheetCsvToolError::TooLarge);
        }
        Ok(output.into_bytes())
    }

    fn push_row(
        rows: &mut Vec<Vec<String>>,
        row: &mut Vec<String>,
    ) -> Result<(), SheetCsvToolError> {
        Self::validate_columns(row)?;
        if rows.len() >= MAX_CSV_ROWS {
            return Err(SheetCsvToolError::TooManyRows);
        }
        rows.push(std::mem::take(row));
        Ok(())
    }

    fn validate_columns<T>(row: &[T]) -> Result<(), SheetCsvToolError> {
        if row.len() > MAX_CSV_COLUMNS {
            return Err(SheetCsvToolError::TooManyColumns);
        }
        Ok(())
    }

    fn write_field(output: &mut String, value: &str) {
        let requires_quotes = value.contains(',')
            || value.contains('"')
            || value.contains('')
            || value.contains('
');
        if !requires_quotes {
            output.push_str(value);
            return;
        }

        output.push('"');
        for character in value.chars() {
            if character == '"' {
                output.push_str("""");
            } else {
                output.push(character);
            }
        }
        output.push('"');
    }
}
