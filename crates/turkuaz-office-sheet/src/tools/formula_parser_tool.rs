// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/tools/formula_parser_tool.rs
// # 📌 Amac: Basic Sheet formula source metnini arithmetic, A1 reference, range ve SUM AST modeline parse eder
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.3.0
// Aciklama: Formula syntax/token/precedence teknik detayini Service business evaluation mantigindan ayirir
// Bagimli Oldugu Katman: Tool -> Config

use crate::config::constants::{MAX_FORMULA_LENGTH, MAX_FORMULA_PARSE_DEPTH};
use crate::services::sheet_types::CellAddress;
use crate::tools::cell_reference_tool::CellReferenceTool;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaParseError {
    MissingEquals,
    TooLong,
    TooDeep,
    UnexpectedEnd,
    UnexpectedToken,
    InvalidNumber,
    InvalidReference,
    UnsupportedFunction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaUnaryOperator {
    Positive,
    Negative,
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
    Range {
        start: CellAddress,
        end: CellAddress,
    },
    Unary {
        operator: FormulaUnaryOperator,
        operand: Box<FormulaExpression>,
    },
    Binary {
        operator: FormulaBinaryOperator,
        left: Box<FormulaExpression>,
        right: Box<FormulaExpression>,
    },
    Sum(Vec<FormulaExpression>),
}

pub struct FormulaParserTool;

impl FormulaParserTool {
    pub fn parse(source: &str) -> Result<FormulaExpression, FormulaParseError> {
        if source.len() > MAX_FORMULA_LENGTH {
            return Err(FormulaParseError::TooLong);
        }
        let body = source
            .strip_prefix('=')
            .ok_or(FormulaParseError::MissingEquals)?;
        let mut parser = Parser::new(body);
        parser.skip_whitespace();
        if parser.is_eof() {
            return Err(FormulaParseError::UnexpectedEnd);
        }
        let expression = parser.parse_expression(0)?;
        parser.skip_whitespace();
        if !parser.is_eof() {
            return Err(FormulaParseError::UnexpectedToken);
        }
        Ok(expression)
    }
}

struct Parser<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    fn parse_expression(&mut self, depth: usize) -> Result<FormulaExpression, FormulaParseError> {
        Self::validate_depth(depth)?;
        let mut expression = self.parse_term(depth)?;
        loop {
            self.skip_whitespace();
            let operator = match self.peek_byte() {
                Some(b'+') => FormulaBinaryOperator::Add,
                Some(b'-') => FormulaBinaryOperator::Subtract,
                _ => break,
            };
            self.position += 1;
            let right = self.parse_term(depth)?;
            expression = FormulaExpression::Binary {
                operator,
                left: Box::new(expression),
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    fn parse_term(&mut self, depth: usize) -> Result<FormulaExpression, FormulaParseError> {
        Self::validate_depth(depth)?;
        let mut expression = self.parse_unary(depth)?;
        loop {
            self.skip_whitespace();
            let operator = match self.peek_byte() {
                Some(b'*') => FormulaBinaryOperator::Multiply,
                Some(b'/') => FormulaBinaryOperator::Divide,
                _ => break,
            };
            self.position += 1;
            let right = self.parse_unary(depth)?;
            expression = FormulaExpression::Binary {
                operator,
                left: Box::new(expression),
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    fn parse_unary(&mut self, depth: usize) -> Result<FormulaExpression, FormulaParseError> {
        Self::validate_depth(depth)?;
        self.skip_whitespace();
        match self.peek_byte() {
            Some(b'+') => {
                self.position += 1;
                Ok(FormulaExpression::Unary {
                    operator: FormulaUnaryOperator::Positive,
                    operand: Box::new(self.parse_unary(depth + 1)?),
                })
            }
            Some(b'-') => {
                self.position += 1;
                Ok(FormulaExpression::Unary {
                    operator: FormulaUnaryOperator::Negative,
                    operand: Box::new(self.parse_unary(depth + 1)?),
                })
            }
            _ => self.parse_primary(depth),
        }
    }

    fn parse_primary(&mut self, depth: usize) -> Result<FormulaExpression, FormulaParseError> {
        Self::validate_depth(depth)?;
        self.skip_whitespace();
        match self.peek_byte() {
            Some(b'(') => {
                self.position += 1;
                let expression = self.parse_expression(depth + 1)?;
                self.skip_whitespace();
                self.expect_byte(b')')?;
                Ok(expression)
            }
            Some(byte) if byte.is_ascii_digit() || byte == b'.' => self.parse_number(),
            Some(byte) if byte.is_ascii_alphabetic() => self.parse_identifier_or_reference(depth),
            Some(_) => Err(FormulaParseError::UnexpectedToken),
            None => Err(FormulaParseError::UnexpectedEnd),
        }
    }

    fn parse_identifier_or_reference(
        &mut self,
        depth: usize,
    ) -> Result<FormulaExpression, FormulaParseError> {
        let token = self.take_ascii_alphanumeric();
        if token.is_empty() {
            return Err(FormulaParseError::UnexpectedToken);
        }

        self.skip_whitespace();
        if self.peek_byte() == Some(b'(') {
            if !token.eq_ignore_ascii_case("SUM") {
                return Err(FormulaParseError::UnsupportedFunction);
            }
            return self.parse_sum(depth + 1);
        }

        let start =
            CellReferenceTool::parse(token).map_err(|_| FormulaParseError::InvalidReference)?;
        self.skip_whitespace();
        if self.peek_byte() != Some(b':') {
            return Ok(FormulaExpression::Reference(start));
        }

        self.position += 1;
        self.skip_whitespace();
        let end_token = self.take_ascii_alphanumeric();
        if end_token.is_empty() {
            return Err(FormulaParseError::InvalidReference);
        }
        let end = CellReferenceTool::parse(end_token)
            .map_err(|_| FormulaParseError::InvalidReference)?;
        Ok(FormulaExpression::Range { start, end })
    }

    fn parse_sum(&mut self, depth: usize) -> Result<FormulaExpression, FormulaParseError> {
        Self::validate_depth(depth)?;
        self.expect_byte(b'(')?;
        self.skip_whitespace();
        if self.peek_byte() == Some(b')') {
            return Err(FormulaParseError::UnexpectedToken);
        }

        let mut arguments = Vec::new();
        loop {
            arguments.push(self.parse_expression(depth)?);
            self.skip_whitespace();
            match self.peek_byte() {
                Some(b',') => {
                    self.position += 1;
                    self.skip_whitespace();
                }
                Some(b')') => {
                    self.position += 1;
                    break;
                }
                Some(_) => return Err(FormulaParseError::UnexpectedToken),
                None => return Err(FormulaParseError::UnexpectedEnd),
            }
        }
        Ok(FormulaExpression::Sum(arguments))
    }

    fn parse_number(&mut self) -> Result<FormulaExpression, FormulaParseError> {
        let start = self.position;
        let mut has_digit = false;

        while self.peek_byte().is_some_and(|byte| byte.is_ascii_digit()) {
            self.position += 1;
            has_digit = true;
        }
        if self.peek_byte() == Some(b'.') {
            self.position += 1;
            while self.peek_byte().is_some_and(|byte| byte.is_ascii_digit()) {
                self.position += 1;
                has_digit = true;
            }
        }
        if !has_digit {
            return Err(FormulaParseError::InvalidNumber);
        }

        if matches!(self.peek_byte(), Some(b'e' | b'E')) {
            self.position += 1;
            if matches!(self.peek_byte(), Some(b'+' | b'-')) {
                self.position += 1;
            }
            let exponent_start = self.position;
            while self.peek_byte().is_some_and(|byte| byte.is_ascii_digit()) {
                self.position += 1;
            }
            if self.position == exponent_start {
                return Err(FormulaParseError::InvalidNumber);
            }
        }

        let value = self.source[start..self.position]
            .parse::<f64>()
            .map_err(|_| FormulaParseError::InvalidNumber)?;
        if !value.is_finite() {
            return Err(FormulaParseError::InvalidNumber);
        }
        Ok(FormulaExpression::Number(value))
    }

    fn take_ascii_alphanumeric(&mut self) -> &'a str {
        let start = self.position;
        while self
            .peek_byte()
            .is_some_and(|byte| byte.is_ascii_alphanumeric())
        {
            self.position += 1;
        }
        &self.source[start..self.position]
    }

    fn skip_whitespace(&mut self) {
        while self
            .peek_byte()
            .is_some_and(|byte| byte.is_ascii_whitespace())
        {
            self.position += 1;
        }
    }

    fn expect_byte(&mut self, expected: u8) -> Result<(), FormulaParseError> {
        if self.peek_byte() != Some(expected) {
            return Err(if self.is_eof() {
                FormulaParseError::UnexpectedEnd
            } else {
                FormulaParseError::UnexpectedToken
            });
        }
        self.position += 1;
        Ok(())
    }

    fn peek_byte(&self) -> Option<u8> {
        self.source.as_bytes().get(self.position).copied()
    }

    fn is_eof(&self) -> bool {
        self.position >= self.source.len()
    }

    fn validate_depth(depth: usize) -> Result<(), FormulaParseError> {
        if depth > MAX_FORMULA_PARSE_DEPTH {
            return Err(FormulaParseError::TooDeep);
        }
        Ok(())
    }
}
