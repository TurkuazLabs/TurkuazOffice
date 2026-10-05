// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/tools/formula_tool.rs
// # 📌 Amac: Sheet formula metnini typed expression agacina parse eder
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.8.0
// Aciklama: Same-sheet A1/range, arithmetic, comparison ve temel function library syntax'ini guvenli parse eder
// Bagimli Oldugu Katman: Tool -> Config -> Service

use crate::config::constants::{
    FORMULA_FUNCTION_AVERAGE, FORMULA_FUNCTION_IF, FORMULA_FUNCTION_MAX, FORMULA_FUNCTION_MIN,
    FORMULA_FUNCTION_SUM, FORMULA_PREFIX, MAX_FORMULA_FUNCTION_ARGUMENTS, MAX_FORMULA_LENGTH,
    MAX_FORMULA_OPERATIONS, MAX_FORMULA_PARSE_DEPTH,
};
use crate::services::sheet_types::{CellAddress, SheetRange};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaFunction {
    Sum,
    Average,
    Min,
    Max,
    If,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaExpression {
    Number(f64),
    Reference(CellAddress),
    Range(SheetRange),
    Unary {
        operator: FormulaUnaryOperator,
        operand: Box<FormulaExpression>,
    },
    Binary {
        operator: FormulaBinaryOperator,
        left: Box<FormulaExpression>,
        right: Box<FormulaExpression>,
    },
    Comparison {
        operator: FormulaComparisonOperator,
        left: Box<FormulaExpression>,
        right: Box<FormulaExpression>,
    },
    Function {
        function: FormulaFunction,
        arguments: Vec<FormulaExpression>,
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
    UnknownFunction,
    InvalidArgumentCount,
    TooComplex,
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
    nested_depth: usize,
    operation_count: usize,
}

impl<'a> FormulaParser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            position: 0,
            nested_depth: 0,
            operation_count: 0,
        }
    }

    fn parse_expression(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        let left = self.parse_additive()?;
        self.skip_whitespace();

        let Some((operator, token_length)) = self.comparison_operator() else {
            return Ok(left);
        };

        self.record_operation()?;
        self.position = self.position.saturating_add(token_length);
        let right = self.parse_additive()?;
        Ok(FormulaExpression::Comparison {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
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
            self.record_operation()?;
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
            self.record_operation()?;
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
            self.enter_nested()?;
            self.position += 1;
            let operand = self.parse_unary();
            self.exit_nested();
            return Ok(FormulaExpression::Unary {
                operator,
                operand: Box::new(operand?),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        self.skip_whitespace();
        match self.current() {
            Some(b'(') => {
                self.enter_nested()?;
                self.position += 1;
                let expression = self.parse_expression();
                self.exit_nested();
                let expression = expression?;
                self.skip_whitespace();
                if self.current() != Some(b')') {
                    return Err(FormulaToolError::UnexpectedToken);
                }
                self.position += 1;
                Ok(expression)
            }
            Some(byte) if byte.is_ascii_digit() || byte == b'.' => self.parse_number(),
            Some(b'$') => self.parse_reference_or_range(),
            Some(byte) if byte.is_ascii_alphabetic() => self.parse_identifier_or_reference(),
            _ => Err(FormulaToolError::UnexpectedToken),
        }
    }

    fn parse_identifier_or_reference(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        let start = self.position;
        while self
            .current()
            .is_some_and(|byte| byte.is_ascii_alphabetic())
        {
            self.position += 1;
        }
        let identifier_end = self.position;
        self.skip_whitespace();

        if self.current() == Some(b'(') {
            let name = self.source[start..identifier_end].to_owned();
            return self.parse_function(&name);
        }

        self.position = start;
        self.parse_reference_or_range()
    }

    fn parse_function(&mut self, name: &str) -> Result<FormulaExpression, FormulaToolError> {
        let normalized = name.to_ascii_uppercase();
        let function = match normalized.as_str() {
            FORMULA_FUNCTION_SUM => FormulaFunction::Sum,
            FORMULA_FUNCTION_AVERAGE => FormulaFunction::Average,
            FORMULA_FUNCTION_MIN => FormulaFunction::Min,
            FORMULA_FUNCTION_MAX => FormulaFunction::Max,
            FORMULA_FUNCTION_IF => FormulaFunction::If,
            _ => return Err(FormulaToolError::UnknownFunction),
        };

        self.record_operation()?;
        self.enter_nested()?;
        self.position += 1;

        let arguments_result = (|| -> Result<Vec<FormulaExpression>, FormulaToolError> {
            let mut arguments = Vec::new();
            self.skip_whitespace();

            if self.current() == Some(b')') {
                self.position += 1;
                return Ok(arguments);
            }

            loop {
                if arguments.len() >= MAX_FORMULA_FUNCTION_ARGUMENTS {
                    return Err(FormulaToolError::TooComplex);
                }

                arguments.push(self.parse_expression()?);
                self.skip_whitespace();

                match self.current() {
                    Some(b',') | Some(b';') => {
                        self.position += 1;
                        self.skip_whitespace();
                        if self.current() == Some(b')') {
                            return Err(FormulaToolError::UnexpectedToken);
                        }
                    }
                    Some(b')') => {
                        self.position += 1;
                        break;
                    }
                    _ => return Err(FormulaToolError::UnexpectedToken),
                }
            }

            Ok(arguments)
        })();
        self.exit_nested();

        let arguments = arguments_result?;
        let valid_count = match function {
            FormulaFunction::Sum
            | FormulaFunction::Average
            | FormulaFunction::Min
            | FormulaFunction::Max => !arguments.is_empty(),
            FormulaFunction::If => arguments.len() == 3,
        };
        if !valid_count {
            return Err(FormulaToolError::InvalidArgumentCount);
        }

        Ok(FormulaExpression::Function {
            function,
            arguments,
        })
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

    fn parse_reference_or_range(&mut self) -> Result<FormulaExpression, FormulaToolError> {
        let first = self.parse_reference_address()?;
        self.skip_whitespace();

        if self.current() != Some(b':') {
            return Ok(FormulaExpression::Reference(first));
        }

        self.position += 1;
        self.skip_whitespace();
        let second = self.parse_reference_address()?;

        Ok(FormulaExpression::Range(SheetRange {
            start_row: first.row.min(second.row),
            end_row: first.row.max(second.row),
            start_column: first.column.min(second.column),
            end_column: first.column.max(second.column),
        }))
    }

    fn parse_reference_address(&mut self) -> Result<CellAddress, FormulaToolError> {
        let token_start = self.position;
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

        let token = &self.source[token_start..self.position];
        let normalized = token.replace('$', "").to_ascii_uppercase();
        CellReferenceTool::parse(&normalized).map_err(|_| FormulaToolError::InvalidReference)
    }

    fn comparison_operator(&self) -> Option<(FormulaComparisonOperator, usize)> {
        let remaining = self.bytes.get(self.position..)?;
        if remaining.starts_with(b"<>") {
            return Some((FormulaComparisonOperator::NotEqual, 2));
        }
        if remaining.starts_with(b">=") {
            return Some((FormulaComparisonOperator::GreaterThanOrEqual, 2));
        }
        if remaining.starts_with(b"<=") {
            return Some((FormulaComparisonOperator::LessThanOrEqual, 2));
        }
        match remaining.first().copied() {
            Some(b'=') => Some((FormulaComparisonOperator::Equal, 1)),
            Some(b'>') => Some((FormulaComparisonOperator::GreaterThan, 1)),
            Some(b'<') => Some((FormulaComparisonOperator::LessThan, 1)),
            _ => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while self
            .current()
            .is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
        {
            self.position += 1;
        }
    }

    fn record_operation(&mut self) -> Result<(), FormulaToolError> {
        self.operation_count = self.operation_count.saturating_add(1);
        if self.operation_count > MAX_FORMULA_OPERATIONS {
            return Err(FormulaToolError::TooComplex);
        }
        Ok(())
    }

    fn enter_nested(&mut self) -> Result<(), FormulaToolError> {
        self.nested_depth = self.nested_depth.saturating_add(1);
        if self.nested_depth > MAX_FORMULA_PARSE_DEPTH {
            self.nested_depth = self.nested_depth.saturating_sub(1);
            return Err(FormulaToolError::TooComplex);
        }
        Ok(())
    }

    fn exit_nested(&mut self) {
        self.nested_depth = self.nested_depth.saturating_sub(1);
    }

    fn current(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    fn is_finished(&self) -> bool {
        self.position >= self.bytes.len()
    }
}
