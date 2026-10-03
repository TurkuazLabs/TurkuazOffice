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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SheetCsvField {
    pub row: usize,
    pub column: usize,
    pub value: String,
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
                '\n' => {
                    row.push(std::mem::take(&mut field));
                    field_started = false;
                    Self::push_row(&mut rows, &mut row)?;
                    ended_row = true;
                }
                '\r' => {
                    if chars.peek() == Some(&'\n') {
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

    pub fn encode_sparse(fields: &[SheetCsvField]) -> Result<Vec<u8>, SheetCsvToolError> {
        if fields.is_empty() {
            return Ok(Vec::new());
        }

        let mut ordered = fields.iter().collect::<Vec<_>>();
        ordered.sort_by_key(|field| (field.row, field.column));

        let mut output = String::new();
        let mut current_row = 0_usize;
        let mut last_column: Option<usize> = None;
        let mut last_address: Option<(usize, usize)> = None;

        for field in ordered {
            if field.row >= MAX_CSV_ROWS {
                return Err(SheetCsvToolError::TooManyRows);
            }
            if field.column >= MAX_CSV_COLUMNS {
                return Err(SheetCsvToolError::TooManyColumns);
            }
            if last_address == Some((field.row, field.column)) {
                return Err(SheetCsvToolError::InvalidCsv);
            }

            while current_row < field.row {
                output.push_str("\r\n");
                Self::validate_output_size(&output)?;
                current_row += 1;
                last_column = None;
            }

            let comma_count = match last_column {
                Some(previous) => field.column.saturating_sub(previous),
                None => field.column,
            };
            if last_column.is_some_and(|previous| field.column <= previous) {
                return Err(SheetCsvToolError::InvalidCsv);
            }
            for _ in 0..comma_count {
                output.push(',');
                Self::validate_output_size(&output)?;
            }

            Self::write_field(&mut output, &field.value);
            Self::validate_output_size(&output)?;
            last_column = Some(field.column);
            last_address = Some((field.row, field.column));
        }

        Ok(output.into_bytes())
    }

    pub fn encode(rows: &[Vec<String>]) -> Result<Vec<u8>, SheetCsvToolError> {
        if rows.len() > MAX_CSV_ROWS {
            return Err(SheetCsvToolError::TooManyRows);
        }

        let mut output = String::new();
        for (row_index, row) in rows.iter().enumerate() {
            Self::validate_columns(row)?;
            if row_index > 0 {
                output.push_str(
                    "
",
                );
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

    fn validate_output_size(output: &str) -> Result<(), SheetCsvToolError> {
        if output.len() > MAX_CSV_BYTES {
            return Err(SheetCsvToolError::TooLarge);
        }
        Ok(())
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
            || value.contains('\r')
            || value.contains('\n');
        if !requires_quotes {
            output.push_str(value);
            return;
        }

        output.push('"');
        for character in value.chars() {
            if character == '"' {
                output.push_str("\"\"");
            } else {
                output.push(character);
            }
        }
        output.push('"');
    }
}
