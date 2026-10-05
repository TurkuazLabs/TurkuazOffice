// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/src/services/sheet_service.rs
// # 📌 Amac: Sheet document yasam dongusu, cell validation, table object, conditional formatting, formula evaluation, range summary ve sparse mutation business kurallarini yonetir
// # 📌 Modul - FileType: Service - Rust
// Version: 0.7.0
// Aciklama: Default worksheet, cell set/get/clear ve basic same-sheet formula evaluation kurallarini Repo/Tool uzerinden koordine eder
// Bagimli Oldugu Katman: Service -> Repo -> Tool

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use turkuaz_office_core::config::constants::DEFAULT_DOCUMENT_TITLE;
use turkuaz_office_core::{DocumentId, DocumentSchemaVersion};

use crate::config::constants::{
    DEFAULT_TABLE_NAME_PREFIX, DEFAULT_WORKSHEET_NAME, MAX_CELL_DECIMAL_PLACES,
    MAX_CELL_TEXT_LENGTH, MAX_CHART_POINTS, MAX_CHART_TITLE_LENGTH, MAX_CONDITIONAL_FORMAT_RULES,
    MAX_FORMULA_EVALUATION_DEPTH, MAX_SHEET_COLUMNS, MAX_SHEET_QUERY_ROWS, MAX_SHEET_ROWS,
};
use crate::repositories::sheet_document_repository::SheetDocumentRepository;
use crate::services::sheet_types::{
    Cell, CellAddress, CellFormat, CellValue, ChartDataPoint, ChartId, ChartType,
    ConditionalFormatRuleId, FormulaCell, SheetChart, SheetConditionalFormatCondition,
    SheetConditionalFormatMatch, SheetConditionalFormatRule, SheetConditionalFormatStyle,
    SheetDocument, SheetFilter, SheetFilterCondition, SheetRange, SheetRangeSummary, SheetSort,
    SheetSortDirection, SheetTable, TableId, Worksheet, WorksheetId,
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
    InvalidCellFormat,
    InvalidRange,
    QueryTooLarge,
    InvalidFilter,
    ConditionalFormatRuleNotFound,
    InvalidConditionalFormat,
    ConditionalFormatLimitExceeded,
    TableNotFound,
    InvalidTableRange,
    TableRangeOverlap,
    ChartNotFound,
    InvalidChartTitle,
    InvalidChartRange,
    ChartTooManyPoints,
    ChartCategoryNotText,
    ChartValueNotNumeric,
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
            cell_formats: BTreeMap::new(),
            conditional_formats: BTreeMap::new(),
            tables: BTreeMap::new(),
            charts: BTreeMap::new(),
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
        Ok(worksheet
            .cells
            .get(&address)
            .cloned()
            .map(|value| Cell { address, value }))
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

        let value = Self::evaluate_value(worksheet, address, value, &mut BTreeSet::new(), 0)?;
        Ok(Some(Cell { address, value }))
    }

    pub fn set_cell_format_by_a1(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        reference: &str,
        format: CellFormat,
    ) -> Result<SheetDocument, SheetError> {
        let address =
            CellReferenceTool::parse(reference).map_err(|_| SheetError::InvalidCellReference)?;
        self.set_cell_format(document_id, worksheet_id, address, format)
    }

    pub fn set_cell_format(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        address: CellAddress,
        format: CellFormat,
    ) -> Result<SheetDocument, SheetError> {
        Self::validate_address(address)?;
        Self::validate_format(format)?;

        let mut document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        if !document
            .worksheets
            .iter()
            .any(|worksheet| &worksheet.id == worksheet_id)
        {
            return Err(SheetError::WorksheetNotFound);
        }

        let current = document
            .cell_formats
            .get(worksheet_id)
            .and_then(|formats| formats.get(&address))
            .copied()
            .unwrap_or_default();
        if current == format {
            return Ok(document);
        }

        if format == CellFormat::default() {
            let remove_worksheet_entry =
                if let Some(formats) = document.cell_formats.get_mut(worksheet_id) {
                    formats.remove(&address);
                    formats.is_empty()
                } else {
                    false
                };
            if remove_worksheet_entry {
                document.cell_formats.remove(worksheet_id);
            }
        } else {
            document
                .cell_formats
                .entry(worksheet_id.clone())
                .or_default()
                .insert(address, format);
        }

        document.revision = document.revision.saturating_add(1);
        self.repository.save(document.clone());
        Ok(document)
    }

    pub fn cell_format_by_a1(
        &self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        reference: &str,
    ) -> Result<CellFormat, SheetError> {
        let address =
            CellReferenceTool::parse(reference).map_err(|_| SheetError::InvalidCellReference)?;
        self.cell_format(document_id, worksheet_id, address)
    }

    pub fn cell_format(
        &self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        address: CellAddress,
    ) -> Result<CellFormat, SheetError> {
        Self::validate_address(address)?;
        let document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        if !document
            .worksheets
            .iter()
            .any(|worksheet| &worksheet.id == worksheet_id)
        {
            return Err(SheetError::WorksheetNotFound);
        }
        Ok(document
            .cell_formats
            .get(worksheet_id)
            .and_then(|formats| formats.get(&address))
            .copied()
            .unwrap_or_default())
    }

    pub fn create_table(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        range: SheetRange,
    ) -> Result<SheetDocument, SheetError> {
        Self::validate_table_range(range)?;

        let mut document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        if !document
            .worksheets
            .iter()
            .any(|worksheet| &worksheet.id == worksheet_id)
        {
            return Err(SheetError::WorksheetNotFound);
        }
        if document.tables.values().any(|table| {
            &table.worksheet_id == worksheet_id && Self::ranges_overlap(table.range, range)
        }) {
            return Err(SheetError::TableRangeOverlap);
        }

        let table = SheetTable {
            id: self.id_tool.next_table_id(),
            worksheet_id: worksheet_id.clone(),
            name: Self::next_table_name(&document),
            range,
        };
        document.tables.insert(table.id.clone(), table);
        document.revision = document.revision.saturating_add(1);
        self.repository.save(document.clone());
        Ok(document)
    }

    pub fn remove_table(
        &mut self,
        document_id: &DocumentId,
        table_id: &TableId,
    ) -> Result<SheetDocument, SheetError> {
        let mut document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        if document.tables.remove(table_id).is_none() {
            return Err(SheetError::TableNotFound);
        }

        document.revision = document.revision.saturating_add(1);
        self.repository.save(document.clone());
        Ok(document)
    }

    pub fn create_conditional_format(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        range: SheetRange,
        condition: SheetConditionalFormatCondition,
        style: SheetConditionalFormatStyle,
    ) -> Result<SheetDocument, SheetError> {
        Self::validate_range(range).map_err(|_| SheetError::InvalidConditionalFormat)?;
        Self::validate_conditional_format_condition(&condition)?;

        let mut document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        if !document
            .worksheets
            .iter()
            .any(|worksheet| &worksheet.id == worksheet_id)
        {
            return Err(SheetError::WorksheetNotFound);
        }
        if document.conditional_formats.len() >= MAX_CONDITIONAL_FORMAT_RULES {
            return Err(SheetError::ConditionalFormatLimitExceeded);
        }

        let priority = document
            .conditional_formats
            .values()
            .map(|rule| rule.priority)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        let rule = SheetConditionalFormatRule {
            id: self.id_tool.next_conditional_format_rule_id(),
            worksheet_id: worksheet_id.clone(),
            range,
            condition,
            style,
            priority,
        };
        document.conditional_formats.insert(rule.id.clone(), rule);
        document.revision = document.revision.saturating_add(1);
        self.repository.save(document.clone());
        Ok(document)
    }

    pub fn remove_conditional_format(
        &mut self,
        document_id: &DocumentId,
        rule_id: &ConditionalFormatRuleId,
    ) -> Result<SheetDocument, SheetError> {
        let mut document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        if document.conditional_formats.remove(rule_id).is_none() {
            return Err(SheetError::ConditionalFormatRuleNotFound);
        }

        document.revision = document.revision.saturating_add(1);
        self.repository.save(document.clone());
        Ok(document)
    }

    pub fn conditional_format_matches(
        &self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        range: SheetRange,
    ) -> Result<Vec<SheetConditionalFormatMatch>, SheetError> {
        Self::validate_range(range)?;

        let document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        let worksheet = document
            .worksheets
            .iter()
            .find(|worksheet| &worksheet.id == worksheet_id)
            .ok_or(SheetError::WorksheetNotFound)?;

        let mut rules = document
            .conditional_formats
            .values()
            .filter(|rule| {
                &rule.worksheet_id == worksheet_id && Self::ranges_overlap(rule.range, range)
            })
            .collect::<Vec<_>>();
        rules.sort_by_key(|rule| rule.priority);

        let mut matched = BTreeMap::<CellAddress, SheetConditionalFormatStyle>::new();
        for rule in rules {
            for (address, value) in &worksheet.cells {
                if !Self::address_in_range(*address, rule.range)
                    || !Self::address_in_range(*address, range)
                    || matched.contains_key(address)
                {
                    continue;
                }

                let Ok(evaluated) =
                    Self::evaluate_value(worksheet, *address, value, &mut BTreeSet::new(), 0)
                else {
                    continue;
                };
                if Self::matches_conditional_format(&evaluated, &rule.condition) {
                    matched.insert(*address, rule.style);
                }
            }
        }

        Ok(matched
            .into_iter()
            .map(|(address, style)| SheetConditionalFormatMatch { address, style })
            .collect())
    }

    pub fn range_summary(
        &self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        range: SheetRange,
    ) -> Result<SheetRangeSummary, SheetError> {
        Self::validate_range(range)?;

        let document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        let worksheet = document
            .worksheets
            .iter()
            .find(|worksheet| &worksheet.id == worksheet_id)
            .ok_or(SheetError::WorksheetNotFound)?;

        let mut count = 0_usize;
        let mut numeric_count = 0_usize;
        let mut sum = 0.0_f64;

        for (address, value) in &worksheet.cells {
            if address.row < range.start_row
                || address.row > range.end_row
                || address.column < range.start_column
                || address.column > range.end_column
            {
                continue;
            }

            count = count.saturating_add(1);
            let evaluated =
                Self::evaluate_value(worksheet, *address, value, &mut BTreeSet::new(), 0)?;
            if let CellValue::Number(number) = evaluated {
                numeric_count = numeric_count.saturating_add(1);
                sum += number;
            }
        }

        let average = (numeric_count > 0).then(|| sum / numeric_count as f64);
        Ok(SheetRangeSummary {
            count,
            numeric_count,
            sum,
            average,
        })
    }

    pub fn query_rows(
        &self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        range: SheetRange,
        filter: Option<&SheetFilter>,
        sort: Option<SheetSort>,
    ) -> Result<Vec<u32>, SheetError> {
        Self::validate_range(range)?;
        if let Some(filter) = filter {
            Self::validate_filter(range, filter)?;
        }
        if let Some(sort) = sort
            && (sort.column < range.start_column || sort.column > range.end_column)
        {
            return Err(SheetError::InvalidRange);
        }

        let document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        let worksheet = document
            .worksheets
            .iter()
            .find(|worksheet| &worksheet.id == worksheet_id)
            .ok_or(SheetError::WorksheetNotFound)?;

        let mut rows = worksheet
            .cells
            .keys()
            .filter(|address| {
                address.row >= range.start_row
                    && address.row <= range.end_row
                    && address.column >= range.start_column
                    && address.column <= range.end_column
            })
            .map(|address| address.row)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        if let Some(filter) = filter {
            let mut filtered = Vec::with_capacity(rows.len());
            for row in rows {
                let address = CellAddress {
                    row,
                    column: filter.column,
                };
                let value = Self::query_value(worksheet, address)?;
                if Self::matches_filter(value.as_ref(), &filter.condition) {
                    filtered.push(row);
                }
            }
            rows = filtered;
        }

        if let Some(sort) = sort {
            let mut keyed = Vec::with_capacity(rows.len());
            for row in rows {
                let address = CellAddress {
                    row,
                    column: sort.column,
                };
                keyed.push((row, Self::query_value(worksheet, address)?));
            }
            keyed.sort_by(|left, right| {
                let order = Self::compare_query_values(left.1.as_ref(), right.1.as_ref());
                let order = match sort.direction {
                    SheetSortDirection::Ascending => order,
                    SheetSortDirection::Descending => order.reverse(),
                };
                order.then_with(|| left.0.cmp(&right.0))
            });
            rows = keyed.into_iter().map(|(row, _)| row).collect();
        }

        Ok(rows)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_chart(
        &mut self,
        document_id: &DocumentId,
        worksheet_id: &WorksheetId,
        chart_type: ChartType,
        title: &str,
        start_row: u32,
        end_row: u32,
        category_column: u32,
        value_column: u32,
    ) -> Result<SheetDocument, SheetError> {
        Self::validate_chart_definition(title, start_row, end_row, category_column, value_column)?;

        let mut document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        if !document
            .worksheets
            .iter()
            .any(|worksheet| &worksheet.id == worksheet_id)
        {
            return Err(SheetError::WorksheetNotFound);
        }

        let chart = SheetChart {
            id: self.id_tool.next_chart_id(),
            worksheet_id: worksheet_id.clone(),
            chart_type,
            title: title.trim().to_owned(),
            start_row,
            end_row,
            category_column,
            value_column,
        };
        document.charts.insert(chart.id.clone(), chart);
        document.revision = document.revision.saturating_add(1);
        self.repository.save(document.clone());
        Ok(document)
    }

    pub fn remove_chart(
        &mut self,
        document_id: &DocumentId,
        chart_id: &ChartId,
    ) -> Result<SheetDocument, SheetError> {
        let mut document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        if document.charts.remove(chart_id).is_none() {
            return Err(SheetError::ChartNotFound);
        }
        document.revision = document.revision.saturating_add(1);
        self.repository.save(document.clone());
        Ok(document)
    }

    pub fn chart_data(
        &self,
        document_id: &DocumentId,
        chart_id: &ChartId,
    ) -> Result<Vec<ChartDataPoint>, SheetError> {
        let document = self
            .repository
            .find(document_id)
            .ok_or(SheetError::DocumentNotFound)?;
        let chart = document
            .charts
            .get(chart_id)
            .ok_or(SheetError::ChartNotFound)?;
        let worksheet = document
            .worksheets
            .iter()
            .find(|worksheet| worksheet.id == chart.worksheet_id)
            .ok_or(SheetError::WorksheetNotFound)?;

        let mut points = Vec::new();
        for row in chart.start_row..=chart.end_row {
            let category_address = CellAddress {
                row,
                column: chart.category_column,
            };
            let value_address = CellAddress {
                row,
                column: chart.value_column,
            };
            let category = worksheet.cells.get(&category_address);
            let value = worksheet.cells.get(&value_address);

            if category.is_none() && value.is_none() {
                continue;
            }

            let category = match category {
                Some(CellValue::Text(text)) => text.clone(),
                _ => return Err(SheetError::ChartCategoryNotText),
            };
            let value = match value {
                Some(raw) => {
                    Self::evaluate_value(worksheet, value_address, raw, &mut BTreeSet::new(), 0)?
                }
                None => return Err(SheetError::ChartValueNotNumeric),
            };
            let CellValue::Number(value) = value else {
                return Err(SheetError::ChartValueNotNumeric);
            };
            points.push(ChartDataPoint { category, value });
        }
        Ok(points)
    }

    fn validate_chart_definition(
        title: &str,
        start_row: u32,
        end_row: u32,
        category_column: u32,
        value_column: u32,
    ) -> Result<(), SheetError> {
        let normalized_title = title.trim();
        if normalized_title.is_empty() || normalized_title.chars().count() > MAX_CHART_TITLE_LENGTH
        {
            return Err(SheetError::InvalidChartTitle);
        }
        if start_row > end_row
            || end_row >= MAX_SHEET_ROWS
            || category_column >= MAX_SHEET_COLUMNS
            || value_column >= MAX_SHEET_COLUMNS
            || category_column == value_column
        {
            return Err(SheetError::InvalidChartRange);
        }
        let point_count = u64::from(end_row)
            .saturating_sub(u64::from(start_row))
            .saturating_add(1);
        if point_count > MAX_CHART_POINTS as u64 {
            return Err(SheetError::ChartTooManyPoints);
        }
        Ok(())
    }

    fn query_value(
        worksheet: &Worksheet,
        address: CellAddress,
    ) -> Result<Option<CellValue>, SheetError> {
        let Some(value) = worksheet.cells.get(&address) else {
            return Ok(None);
        };
        Self::evaluate_value(worksheet, address, value, &mut BTreeSet::new(), 0).map(Some)
    }

    fn matches_filter(value: Option<&CellValue>, condition: &SheetFilterCondition) -> bool {
        match condition {
            SheetFilterCondition::NonEmpty => value.is_some(),
            SheetFilterCondition::TextContains(needle) => {
                matches!(value, Some(CellValue::Text(text)) if text.contains(needle))
            }
            SheetFilterCondition::NumberGreaterThan(limit) => {
                matches!(value, Some(CellValue::Number(number)) if number > limit)
            }
            SheetFilterCondition::NumberLessThan(limit) => {
                matches!(value, Some(CellValue::Number(number)) if number < limit)
            }
            SheetFilterCondition::BooleanEquals(expected) => {
                matches!(value, Some(CellValue::Boolean(value)) if value == expected)
            }
        }
    }

    fn compare_query_values(left: Option<&CellValue>, right: Option<&CellValue>) -> Ordering {
        let rank = |value: Option<&CellValue>| match value {
            None => 0_u8,
            Some(CellValue::Number(_)) => 1,
            Some(CellValue::Text(_)) => 2,
            Some(CellValue::Boolean(_)) => 3,
            Some(CellValue::Formula(_)) => 4,
        };

        match rank(left).cmp(&rank(right)) {
            Ordering::Equal => match (left, right) {
                (None, None) => Ordering::Equal,
                (Some(CellValue::Number(left)), Some(CellValue::Number(right))) => {
                    left.partial_cmp(right).unwrap_or(Ordering::Equal)
                }
                (Some(CellValue::Text(left)), Some(CellValue::Text(right))) => left.cmp(right),
                (Some(CellValue::Boolean(left)), Some(CellValue::Boolean(right))) => {
                    left.cmp(right)
                }
                (Some(CellValue::Formula(left)), Some(CellValue::Formula(right))) => {
                    left.expression.cmp(&right.expression)
                }
                _ => Ordering::Equal,
            },
            order => order,
        }
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

    fn next_table_name(document: &SheetDocument) -> String {
        let mut index = document.tables.len().saturating_add(1);
        loop {
            let candidate = format!("{DEFAULT_TABLE_NAME_PREFIX}{index}");
            if document
                .tables
                .values()
                .all(|table| !table.name.eq_ignore_ascii_case(&candidate))
            {
                return candidate;
            }
            index = index.saturating_add(1);
        }
    }

    const fn ranges_overlap(left: SheetRange, right: SheetRange) -> bool {
        left.start_row <= right.end_row
            && right.start_row <= left.end_row
            && left.start_column <= right.end_column
            && right.start_column <= left.end_column
    }

    fn address_in_range(address: CellAddress, range: SheetRange) -> bool {
        address.row >= range.start_row
            && address.row <= range.end_row
            && address.column >= range.start_column
            && address.column <= range.end_column
    }

    fn validate_conditional_format_condition(
        condition: &SheetConditionalFormatCondition,
    ) -> Result<(), SheetError> {
        match condition {
            SheetConditionalFormatCondition::NumberGreaterThan(value)
            | SheetConditionalFormatCondition::NumberLessThan(value)
            | SheetConditionalFormatCondition::NumberEquals(value)
                if !value.is_finite() =>
            {
                Err(SheetError::InvalidConditionalFormat)
            }
            SheetConditionalFormatCondition::TextContains(text)
                if text.trim().is_empty() || text.chars().count() > MAX_CELL_TEXT_LENGTH =>
            {
                Err(SheetError::InvalidConditionalFormat)
            }
            _ => Ok(()),
        }
    }

    fn matches_conditional_format(
        value: &CellValue,
        condition: &SheetConditionalFormatCondition,
    ) -> bool {
        match (value, condition) {
            (
                CellValue::Number(value),
                SheetConditionalFormatCondition::NumberGreaterThan(limit),
            ) => value > limit,
            (CellValue::Number(value), SheetConditionalFormatCondition::NumberLessThan(limit)) => {
                value < limit
            }
            (CellValue::Number(value), SheetConditionalFormatCondition::NumberEquals(limit)) => {
                value == limit
            }
            (CellValue::Text(value), SheetConditionalFormatCondition::TextContains(needle)) => {
                value.to_lowercase().contains(&needle.to_lowercase())
            }
            _ => false,
        }
    }

    fn validate_table_range(range: SheetRange) -> Result<(), SheetError> {
        if range.start_row >= range.end_row {
            return Err(SheetError::InvalidTableRange);
        }
        Self::validate_range(range).map_err(|_| SheetError::InvalidTableRange)
    }

    fn validate_format(format: CellFormat) -> Result<(), SheetError> {
        if format
            .decimal_places
            .is_some_and(|places| places > MAX_CELL_DECIMAL_PLACES)
        {
            return Err(SheetError::InvalidCellFormat);
        }
        Ok(())
    }

    fn validate_range(range: SheetRange) -> Result<(), SheetError> {
        if range.start_row > range.end_row
            || range.start_column > range.end_column
            || range.end_row >= MAX_SHEET_ROWS
            || range.end_column >= MAX_SHEET_COLUMNS
        {
            return Err(SheetError::InvalidRange);
        }

        let row_count = u64::from(range.end_row)
            .saturating_sub(u64::from(range.start_row))
            .saturating_add(1);
        if row_count > MAX_SHEET_QUERY_ROWS as u64 {
            return Err(SheetError::QueryTooLarge);
        }
        Ok(())
    }

    fn validate_filter(range: SheetRange, filter: &SheetFilter) -> Result<(), SheetError> {
        if filter.column < range.start_column || filter.column > range.end_column {
            return Err(SheetError::InvalidRange);
        }
        match &filter.condition {
            SheetFilterCondition::TextContains(text)
                if text.chars().count() > MAX_CELL_TEXT_LENGTH =>
            {
                Err(SheetError::InvalidFilter)
            }
            SheetFilterCondition::NumberGreaterThan(value)
            | SheetFilterCondition::NumberLessThan(value)
                if !value.is_finite() =>
            {
                Err(SheetError::InvalidFilter)
            }
            _ => Ok(()),
        }
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
            CellValue::Number(number) if !number.is_finite() => {
                Err(SheetError::CellNumberNotFinite)
            }
            CellValue::Formula(formula) => FormulaTool::parse(&formula.expression)
                .map(|_| ())
                .map_err(|_| SheetError::InvalidFormula),
            CellValue::Text(_) | CellValue::Number(_) | CellValue::Boolean(_) => Ok(()),
        }
    }
}
