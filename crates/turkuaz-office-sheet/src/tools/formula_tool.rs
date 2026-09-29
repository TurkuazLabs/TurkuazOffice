// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/tools/formula_tool.rs
// # 📌 Amac: Basic Sheet formula metnini typed expression agacina parse eder
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.3.0
// Aciklama: Same-sheet A1/absolute references, numeric literal, parentheses ve + - * / operatorlerini guvenli parse eder
// Bagimli Oldugu Katman: Tool -> Config -> Service

use crate::config::constants::{FORMULA_PREFIX, MAX_FORMULA_LENGTH};
use crate::services::sheet_types::CellAddress;
use crate::tools::cell_reference_tool::CellReferenceTool;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaUnaryOperator {
    Plus,
    Minus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaBinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaExpression {
    Number(f64),
    Reference(CellAddress),
    Unary {
        operator: FormulaUnaryOperator,
        operand: Box<FormulaExpression>,
    },
    Binary {
        operator: FormulaBinaryOperator,
        left: Box<FormulaExpression>,
        right: Box<FormulaExpression>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParsedFormula {
    pub source: String,
    pub expression: FormulaExpression,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaToolError {
    Empty,
    TooLong,
    MissingPrefix,
    UnexpectedToken,
    InvalidNumber,
    InvalidReference,
    NonFiniteNumber,
}

pub struct FormulaTool;

impl FormulaTool {
    pub fn parse(source: &str) -> Result<ParsedFormula, FormulaToolError> {
        let source = source.trim();
        if source.is_empty() {
            return Err(FormulaToolError::Empty);
        }
        if source.chars().count() > MAX_FORMULA_LENGTH {
            return Err(FormulaToolError::TooLong);
        }

        let body = source
            .strip_prefix(FORMULA_PREFIX)
            .ok_or(FormulaToolError::MissingPrefix)?;
        if body.trim().is_empty() {
            return Err(FormulaToolError::Empty);
        }

        let mut parser = FormulaParser::new(body);
        let expression = parser.parse_expression()?;
        parser.skip_whitespace();
        if !parser.is_finished() {
            return Err(FormulaToolError::UnexpectedToken);
        }

        Ok(ParsedFormula {
            source: source.to_owned(),
            expression,
        })
    }
}

struct FormulaParser<'a> {
    source: &'a str,
    bytes: &'a [u8],
    position: usize,
}

impl<'a> FormulaParser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            position: 0,
        }
    }

    fn parse_expression(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        let mut left = self.parse_multiplicative()?;
        loop {
            self.skip_whitespace();
            let operator = match self.current() {
                Some(b'+') => FormulaBinaryOperator::Add,
                Some(b'-') => FormulaBinaryOperator::Subtract,
                _ => break,
            };
            self.position += 1;
            let right = self.parse_multiplicative()?;
            left = FormulaExpression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        let mut left = self.parse_unary()?;
        loop {
            self.skip_whitespace();
            let operator = match self.current() {
                Some(b'*') => FormulaBinaryOperator::Multiply,
                Some(b'/') => FormulaBinaryOperator::Divide,
                _ => break,
            };
            self.position += 1;
            let right = self.parse_unary()?;
            left = FormulaExpression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        self.skip_whitespace();
        let operator = match self.current() {
            Some(b'+') => Some(FormulaUnaryOperator::Plus),
            Some(b'-') => Some(FormulaUnaryOperator::Minus),
            _ => None,
        };

        if let Some(operator) = operator {
            self.position += 1;
            return Ok(FormulaExpression::Unary {
                operator,
                operand: Box::new(self.parse_unary()?),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        self.skip_whitespace();
        match self.current() {
            Some(b'(') => {
                self.position += 1;
                let expression = self.parse_expression()?;
                self.skip_whitespace();
                if self.current() != Some(b')') {
                    return Err(FormulaToolError::UnexpectedToken);
                }
                self.position += 1;
                Ok(expression)
            }
            Some(byte) if byte.is_ascii_digit() || byte == b'.' => self.parse_number(),
            Some(byte) if byte.is_ascii_alphabetic() || byte == b'$' => self.parse_reference(),
            _ => Err(FormulaToolError::UnexpectedToken),
        }
    }

    fn parse_number(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        let start = self.position;
        let mut has_digit = false;

        while self.current().is_some_and(|byte| byte.is_ascii_digit()) {
            has_digit = true;
            self.position += 1;
        }

        if self.current() == Some(b'.') {
            self.position += 1;
            while self.current().is_some_and(|byte| byte.is_ascii_digit()) {
                has_digit = true;
                self.position += 1;
            }
        }

        if !has_digit {
            return Err(FormulaToolError::InvalidNumber);
        }

        let token = &self.source[start..self.position];
        let value = token
            .parse::<f64>()
            .map_err(|_| FormulaToolError::InvalidNumber)?;
        if !value.is_finite() {
            return Err(FormulaToolError::NonFiniteNumber);
        }
        Ok(FormulaExpression::Number(value))
    }

    fn parse_reference(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        if self.current() == Some(b'$') {
            self.position += 1;
        }

        let column_start = self.position;
        while self
            .current()
            .is_some_and(|byte| byte.is_ascii_alphabetic())
        {
            self.position += 1;
        }
        if column_start == self.position {
            return Err(FormulaToolError::InvalidReference);
        }

        if self.current() == Some(b'$') {
            self.position += 1;
        }

        let row_start = self.position;
        while self.current().is_some_and(|byte| byte.is_ascii_digit()) {
            self.position += 1;
        }
        if row_start == self.position {
            return Err(FormulaToolError::InvalidReference);
        }

        let mut normalized = String::with_capacity(self.position - column_start);
        for byte in self.source.as_bytes()[column_start..row_start].iter().copied() {
            if byte != b'$' {
                normalized.push(char::from(byte.to_ascii_uppercase()));
            }
        }
        for byte in self.source.as_bytes()[row_start..self.position].iter().copied() {
            normalized.push(char::from(byte));
        }

        let address =
            CellReferenceTool::parse(&normalized).map_err(|_| FormulaToolError::InvalidReference)?;
        Ok(FormulaExpression::Reference(address))
    }

    fn skip_whitespace(&mut self) {
        while self
            .current()
            .is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
        {
            self.position += 1;
        }
    }

    fn current(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    fn is_finished(&self) -> bool {
        self.position >= self.bytes.len()
    }
}
