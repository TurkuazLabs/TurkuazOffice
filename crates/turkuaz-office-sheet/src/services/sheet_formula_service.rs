// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/services/sheet_formula_service.rs
// # 📌 Amac: Basic Sheet formula AST'ini worksheet cell degerleri uzerinden cycle-safe numeric olarak hesaplar
// # 📌 Modul - FileType: Service - Rust
// Version: 0.3.0
// Aciklama: Arithmetic, A1 reference, range, SUM, dependency depth, type/division/cycle ve finite-result business kurallarini uygular
// Bagimli Oldugu Katman: Service -> Tool

use std::collections::HashSet;

use crate::config::constants::{
    MAX_FORMULA_DEPENDENCY_DEPTH, MAX_FORMULA_RANGE_CELLS,
};
use crate::services::sheet_types::{CellAddress, CellValue, EvaluatedCellValue, Worksheet};
use crate::tools::formula_parser_tool::{
    FormulaBinaryOperator, FormulaExpression, FormulaParseError, FormulaParserTool,
    FormulaUnaryOperator,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaError {
    Parse(FormulaParseError),
    DependencyDepthExceeded,
    CircularReference,
    RangeOutsideFunction,
    RangeTooLarge,
    TypeMismatch,
    DivisionByZero,
    NonFiniteResult,
}

impl From<FormulaParseError> for FormulaError {
    fn from(value: FormulaParseError) -> Self {
        Self::Parse(value)
    }
}

pub struct SheetFormulaService;

impl SheetFormulaService {
    pub fn validate(source: &str) -> Result<(), FormulaError> {
        FormulaParserTool::parse(source)?;
        Ok(())
    }

    pub fn evaluate_cell(
        worksheet: &Worksheet,
        address: CellAddress,
    ) -> Result<Option<EvaluatedCellValue>, FormulaError> {
        let Some(value) = worksheet.cells.get(&address) else {
            return Ok(None);
        };
        let mut visiting = HashSet::new();
        Self::evaluate_cell_value(worksheet, address, value, &mut visiting, 0).map(Some)
    }

    fn evaluate_cell_value(
        worksheet: &Worksheet,
        address: CellAddress,
        value: &CellValue,
        visiting: &mut HashSet<CellAddress>,
        depth: usize,
    ) -> Result<EvaluatedCellValue, FormulaError> {
        match value {
            CellValue::Text(text) => Ok(EvaluatedCellValue::Text(text.clone())),
            CellValue::Number(number) => Ok(EvaluatedCellValue::Number(*number)),
            CellValue::Boolean(value) => Ok(EvaluatedCellValue::Boolean(*value)),
            CellValue::Formula(source) => {
                let number =
                    Self::evaluate_formula(worksheet, address, source, visiting, depth)?;
                Ok(EvaluatedCellValue::Number(number))
            }
        }
    }

    fn evaluate_formula(
        worksheet: &Worksheet,
        address: CellAddress,
        source: &str,
        visiting: &mut HashSet<CellAddress>,
        depth: usize,
    ) -> Result<f64, FormulaError> {
        if depth > MAX_FORMULA_DEPENDENCY_DEPTH {
            return Err(FormulaError::DependencyDepthExceeded);
        }
        if !visiting.insert(address) {
            return Err(FormulaError::CircularReference);
        }

        let expression = FormulaParserTool::parse(source)?;
        let result = Self::evaluate_expression(worksheet, &expression, visiting, depth);
        visiting.remove(&address);
        let value = result?;
        Self::finite(value)
    }

    fn evaluate_expression(
        worksheet: &Worksheet,
        expression: &FormulaExpression,
        visiting: &mut HashSet<CellAddress>,
        depth: usize,
    ) -> Result<f64, FormulaError> {
        let value = match expression {
            FormulaExpression::Number(value) => *value,
            FormulaExpression::Reference(address) => {
                Self::numeric_reference(worksheet, *address, visiting, depth + 1)?
            }
            FormulaExpression::Range { .. } => return Err(FormulaError::RangeOutsideFunction),
            FormulaExpression::Unary { operator, operand } => {
                let value = Self::evaluate_expression(worksheet, operand, visiting, depth)?;
                match operator {
                    FormulaUnaryOperator::Positive => value,
                    FormulaUnaryOperator::Negative => -value,
                }
            }
            FormulaExpression::Binary {
                operator,
                left,
                right,
            } => {
                let left = Self::evaluate_expression(worksheet, left, visiting, depth)?;
                let right = Self::evaluate_expression(worksheet, right, visiting, depth)?;
                match operator {
                    FormulaBinaryOperator::Add => left + right,
                    FormulaBinaryOperator::Subtract => left - right,
                    FormulaBinaryOperator::Multiply => left * right,
                    FormulaBinaryOperator::Divide => {
                        if right == 0.0 {
                            return Err(FormulaError::DivisionByZero);
                        }
                        left / right
                    }
                }
            }
            FormulaExpression::Sum(arguments) => {
                let mut total = 0.0_f64;
                for argument in arguments {
                    total += match argument {
                        FormulaExpression::Range { start, end } => {
                            Self::sum_range(worksheet, *start, *end, visiting, depth)?
                        }
                        _ => Self::evaluate_expression(worksheet, argument, visiting, depth)?,
                    };
                    total = Self::finite(total)?;
                }
                total
            }
        };
        Self::finite(value)
    }

    fn numeric_reference(
        worksheet: &Worksheet,
        address: CellAddress,
        visiting: &mut HashSet<CellAddress>,
        depth: usize,
    ) -> Result<f64, FormulaError> {
        let Some(value) = worksheet.cells.get(&address) else {
            return Ok(0.0);
        };
        match value {
            CellValue::Number(number) => Self::finite(*number),
            CellValue::Formula(source) => {
                Self::evaluate_formula(worksheet, address, source, visiting, depth)
            }
            CellValue::Text(_) | CellValue::Boolean(_) => Err(FormulaError::TypeMismatch),
        }
    }

    fn sum_range(
        worksheet: &Worksheet,
        start: CellAddress,
        end: CellAddress,
        visiting: &mut HashSet<CellAddress>,
        depth: usize,
    ) -> Result<f64, FormulaError> {
        let row_start = start.row.min(end.row);
        let row_end = start.row.max(end.row);
        let column_start = start.column.min(end.column);
        let column_end = start.column.max(end.column);

        let rows = u64::from(row_end - row_start) + 1;
        let columns = u64::from(column_end - column_start) + 1;
        if rows.saturating_mul(columns) > MAX_FORMULA_RANGE_CELLS {
            return Err(FormulaError::RangeTooLarge);
        }

        let mut total = 0.0_f64;
        for row in row_start..=row_end {
            for column in column_start..=column_end {
                total += Self::numeric_reference(
                    worksheet,
                    CellAddress { row, column },
                    visiting,
                    depth + 1,
                )?;
                total = Self::finite(total)?;
            }
        }
        Ok(total)
    }

    fn finite(value: f64) -> Result<f64, FormulaError> {
        if value.is_finite() {
            Ok(value)
        } else {
            Err(FormulaError::NonFiniteResult)
        }
    }
}
