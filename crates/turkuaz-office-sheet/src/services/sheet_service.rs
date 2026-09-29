// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/services/sheet_service.rs
// # 📌 Amac: Sheet document yasam dongusu, cell validation, formula evaluation ve sparse mutation business kurallarini yonetir
// # 📌 Modul - FileType: Service - Rust
// Version: 0.3.0
// Aciklama: Default worksheet, cell set/get/clear ve basic same-sheet formula evaluation kurallarini Repo/Tool uzerinden koordine eder
// Bagimli Oldugu Katman: Service -> Repo -> Tool

use std::collections::{BTreeMap, BTreeSet};

use turkuaz_office_core::config::constants::DEFAULT_DOCUMENT_TITLE;
use turkuaz_office_core::{DocumentId, DocumentSchemaVersion};

use crate::config::constants::{
    DEFAULT_WORKSHEET_NAME, MAX_CELL_TEXT_LENGTH, MAX_FORMULA_EVALUATION_DEPTH,
    MAX_SHEET_COLUMNS, MAX_SHEET_ROWS,
};
use crate::repositories::sheet_document_repository::SheetDocumentRepository;
use crate::services::sheet_types::{
    Cell, CellAddress, CellValue, FormulaCell, SheetDocument, Worksheet, WorksheetId,
};
use crate::tools::cell_reference_tool::CellReferenceTool;
use crate::tools::formula_tool::{
    FormulaBinaryOperator, FormulaExpression, FormulaTool, FormulaUnaryOperator,
};
use crate::tools::sheet_id_tool::SheetIdTool;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SheetError {
    DocumentNotFound,
    WorksheetNotFound,
    InvalidCellReference,
    CellOutOfBounds,
    CellTextTooLong,
    CellNumberNotFinite,
    InvalidFormula,
    FormulaCycle,
    FormulaDepthExceeded,
    FormulaDivisionByZero,
    FormulaNonNumericReference,
    FormulaResultNotFinite,
}

pub struct SheetService<R, I>
where
    R: SheetDocumentRepository,
    I: SheetIdTool,
{
    repository: R,
    id_tool: I,
}

impl<R, I> SheetService<R, I>
where
    R: SheetDocumentRepository,
    I: SheetIdTool,
{
    pub fn new(repository: R, id_tool: I) -> Self {
        Self {
            repository,
            id_tool,
        }
    }

    pub fn create_document(&mut self, title: impl Into<String>) -> SheetDocument {
        let raw_title = title.into();
        let normalized_title = raw_title.trim();
        let title = if normalized_title.is_empty() {
            DEFAULT_DOCUMENT_TITLE.to_owned()
        } else {
            normalized_title.to_owned()
        };

        let document = SheetDocument {
            id: self.id_tool.next_document_id(),
            title,
            schema_version: DocumentSchemaVersion::current(),
            revision: 0,
            worksheets: vec![Worksheet {
                id: self.id_tool.next_worksheet_id(),
                name: DEFAULT_WORKSHEET_NAME.to_owned(),
                cells: BTreeMap::new(),
            }],
        };
        self.repository.save(document.clone());
        document
    }

    pub fn get_document(&self, id: &DocumentId) -> Option<SheetDocument> {
        self.repository.find(id)
    }

    pub fn set_cell_by_a1(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        reference: &str,
        value: CellValue,
    ) -> Result<SheetDocument, SheetError> {
        let address =
            CellReferenceTool::parse(reference).map_err(|_| SheetError::InvalidCellReference)?;
        self.set_cell(document_id, worksheet_id, address, value)
    }

    pub fn set_formula_by_a1(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        reference: &str,
        expression: &str,
    ) -> Result<SheetDocument, SheetError> {
        let parsed = FormulaTool::parse(expression).map_err(|_| SheetError::InvalidFormula)?;
        self.set_cell_by_a1(
            document_id,
            worksheet_id,
            reference,
            CellValue::Formula(FormulaCell {
                expression: parsed.source,
            }),
        )
    }

    pub fn set_cell(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        address: CellAddress,
        value: CellValue,
    ) -> Result<SheetDocument, SheetError> {
        Self::validate_address(address)?;
        Self::validate_value(&value)?;

        let mut document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        let worksheet = document
            .worksheets
            .iter_mut()
            .find(|worksheet| &worksheet.id == worksheet_id)
            .ok_or(SheetError::WorksheetNotFound)?;

        if worksheet.cells.get(&address) == Some(&value) {
            return Ok(document);
        }

        worksheet.cells.insert(address, value);
        document.revision = document.revision.saturating_add(1);
        self.repository.save(document.clone());
        Ok(document)
    }

    pub fn clear_cell_by_a1(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        reference: &str,
    ) -> Result<SheetDocument, SheetError> {
        let address =
            CellReferenceTool::parse(reference).map_err(|_| SheetError::InvalidCellReference)?;
        self.clear_cell(document_id, worksheet_id, address)
    }

    pub fn clear_cell(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        address: CellAddress,
    ) -> Result<SheetDocument, SheetError> {
        Self::validate_address(address)?;
        let mut document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        let worksheet = document
            .worksheets
            .iter_mut()
            .find(|worksheet| &worksheet.id == worksheet_id)
            .ok_or(SheetError::WorksheetNotFound)?;

        if worksheet.cells.remove(&address).is_some() {
            document.revision = document.revision.saturating_add(1);
            self.repository.save(document.clone());
        }
        Ok(document)
    }

    pub fn cell_by_a1(
        &self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        reference: &str,
    ) -> Result<Option<Cell>, SheetError> {
        let address =
            CellReferenceTool::parse(reference).map_err(|_| SheetError::InvalidCellReference)?;
        self.cell(document_id, worksheet_id, address)
    }

    pub fn cell(
        &self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        address: CellAddress,
    ) -> Result<Option<Cell>, SheetError> {
        Self::validate_address(address)?;
        let document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        let worksheet = document
            .worksheets
            .iter()
            .find(|worksheet| &worksheet.id == worksheet_id)
            .ok_or(SheetError::WorksheetNotFound)?;
        Ok(worksheet.cells.get(&address).cloned().map(|value| Cell {
            address,
            value,
        }))
    }

    pub fn evaluated_cell_by_a1(
        &self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        reference: &str,
    ) -> Result<Option<Cell>, SheetError> {
        let address =
            CellReferenceTool::parse(reference).map_err(|_| SheetError::InvalidCellReference)?;
        self.evaluated_cell(document_id, worksheet_id, address)
    }

    pub fn evaluated_cell(
        &self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        address: CellAddress,
    ) -> Result<Option<Cell>, SheetError> {
        Self::validate_address(address)?;
        let document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        let worksheet = document
            .worksheets
            .iter()
            .find(|worksheet| &worksheet.id == worksheet_id)
            .ok_or(SheetError::WorksheetNotFound)?;

        let Some(value) = worksheet.cells.get(&address) else {
            return Ok(None);
        };

        let value =
            Self::evaluate_value(worksheet, address, value, &mut BTreeSet::new(), 0)?;
        Ok(Some(Cell { address, value }))
    }

    fn evaluate_value(
        worksheet: &Worksheet,
        address: CellAddress,
        value: &CellValue,
        stack: &mut BTreeSet<CellAddress>,
        depth: usize,
    ) -> Result<CellValue, SheetError> {
        match value {
            CellValue::Formula(formula) => {
                Self::evaluate_formula_numeric(worksheet, address, formula, stack, depth)
                    .map(CellValue::Number)
            }
            CellValue::Text(_) | CellValue::Number(_) | CellValue::Boolean(_) => Ok(value.clone()),
        }
    }

    fn evaluate_formula_numeric(
        worksheet: &Worksheet,
        address: CellAddress,
        formula: &FormulaCell,
        stack: &mut BTreeSet<CellAddress>,
        depth: usize,
    ) -> Result<f64, SheetError> {
        if depth >= MAX_FORMULA_EVALUATION_DEPTH {
            return Err(SheetError::FormulaDepthExceeded);
        }
        if !stack.insert(address) {
            return Err(SheetError::FormulaCycle);
        }

        let result = FormulaTool::parse(&formula.expression)
            .map_err(|_| SheetError::InvalidFormula)
            .and_then(|parsed| {
                Self::evaluate_expression(worksheet, &parsed.expression, stack, depth)
            });
        stack.remove(&address);
        result
    }

    fn evaluate_expression(
        worksheet: &Worksheet,
        expression: &FormulaExpression,
        stack: &mut BTreeSet<CellAddress>,
        depth: usize,
    ) -> Result<f64, SheetError> {
        let value = match expression {
            FormulaExpression::Number(value) => *value,
            FormulaExpression::Reference(address) => match worksheet.cells.get(address) {
                None => 0.0,
                Some(CellValue::Number(value)) => *value,
                Some(CellValue::Formula(formula)) => Self::evaluate_formula_numeric(
                    worksheet,
                    *address,
                    formula,
                    stack,
                    depth.saturating_add(1),
                )?,
                Some(CellValue::Text(_)) | Some(CellValue::Boolean(_)) => {
                    return Err(SheetError::FormulaNonNumericReference);
                }
            },
            FormulaExpression::Unary { operator, operand } => {
                let operand = Self::evaluate_expression(worksheet, operand, stack, depth)?;
                match operator {
                    FormulaUnaryOperator::Plus => operand,
                    FormulaUnaryOperator::Minus => -operand,
                }
            }
            FormulaExpression::Binary {
                operator,
                left,
                right,
            } => {
                let left = Self::evaluate_expression(worksheet, left, stack, depth)?;
                let right = Self::evaluate_expression(worksheet, right, stack, depth)?;
                match operator {
                    FormulaBinaryOperator::Add => left + right,
                    FormulaBinaryOperator::Subtract => left - right,
                    FormulaBinaryOperator::Multiply => left * right,
                    FormulaBinaryOperator::Divide => {
                        if right == 0.0 {
                            return Err(SheetError::FormulaDivisionByZero);
                        }
                        left / right
                    }
                }
            }
        };

        if !value.is_finite() {
            return Err(SheetError::FormulaResultNotFinite);
        }
        Ok(value)
    }

    fn validate_address(address: CellAddress) -> Result<(), SheetError> {
        if address.row >= MAX_SHEET_ROWS || address.column >= MAX_SHEET_COLUMNS {
            return Err(SheetError::CellOutOfBounds);
        }
        Ok(())
    }

    fn validate_value(value: &CellValue) -> Result<(), SheetError> {
        match value {
            CellValue::Text(text) if text.chars().count() > MAX_CELL_TEXT_LENGTH => {
                Err(SheetError::CellTextTooLong)
            }
            CellValue::Number(number) if !number.is_finite() => Err(SheetError::CellNumberNotFinite),
            CellValue::Formula(formula) => FormulaTool::parse(&formula.expression)
                .map(|_| ())
                .map_err(|_| SheetError::InvalidFormula),
            CellValue::Text(_) | CellValue::Number(_) | CellValue::Boolean(_) => Ok(()),
        }
    }
}
