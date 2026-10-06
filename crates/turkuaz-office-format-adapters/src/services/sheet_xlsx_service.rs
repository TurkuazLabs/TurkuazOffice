// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/services/sheet_xlsx_service.rs
// # 📌 Amac: XLSX ZIP/XML modelini canonical SheetDocument modeline map eder ve canonical workbook'u XLSX byte akimina cevirir
// # 📌 Modul - FileType: Service - Rust
// Version: 0.9.0
// Aciklama: SpreadsheetML value, table ve desteklenen conditional-format metadata import/export kurallarini uygular
// Bagimli Oldugu Katman: Service -> Model -> Tool -> Sheet

use std::collections::{BTreeMap, HashSet};

use turkuaz_office_core::DocumentSchemaVersion;
use turkuaz_office_core::config::constants::DEFAULT_DOCUMENT_TITLE;
use turkuaz_office_sheet::config::constants::{MAX_CELL_TEXT_LENGTH, MAX_WORKSHEET_NAME_LENGTH};
use turkuaz_office_sheet::{
    CellReferenceTool, CellValue, ConditionalFormatRuleId, SheetConditionalFormatCondition,
    SheetConditionalFormatRule, SheetConditionalFormatStyle, SheetDocument, SheetIdTool,
    SheetRange, SheetTable, TableId, Worksheet, WorksheetId,
};

use crate::config::sheet_constants::{
    MAX_XLSX_CONDITIONAL_FORMATS, MAX_XLSX_TABLES, MAX_XLSX_WORKSHEETS,
    XLSX_CONDITIONAL_ACCENT_RGB, XLSX_CONDITIONAL_SUCCESS_RGB, XLSX_CONDITIONAL_WARNING_RGB,
    XLSX_CONTENT_TYPES_ENTRY, XLSX_RELATIONSHIP_SUFFIX, XLSX_ROOT_RELATIONSHIPS_ENTRY,
    XLSX_SHARED_STRINGS_ENTRY, XLSX_STYLES_ENTRY, XLSX_TABLE_PREFIX, XLSX_TABLE_SUFFIX,
    XLSX_WORKBOOK_ENTRY, XLSX_WORKBOOK_RELATIONSHIPS_ENTRY, XLSX_WORKSHEET_PREFIX,
    XLSX_WORKSHEET_RELS_PREFIX, XLSX_WORKSHEET_SUFFIX,
};
use crate::models::sheet_xlsx_model::{
    XlsxCellModel, XlsxCellValue, XlsxConditionalFormatCondition, XlsxConditionalFormatModel,
    XlsxDifferentialStyleModel, XlsxTableModel, XlsxWorkbookModel, XlsxWorksheetModel,
};
use crate::tools::sheet_xlsx_archive_tool::{SheetXlsxArchiveError, SheetXlsxArchiveTool};
use crate::tools::sheet_xlsx_xml_tool::{SheetXlsxXmlError, SheetXlsxXmlTool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SheetXlsxError {
    InvalidPackage,
    ResourceLimit,
    InvalidWorksheetName,
    InvalidCellReference,
    CellTextTooLong,
    InvalidCellText,
    CellNumberNotFinite,
    UnsupportedFormula,
    UnsupportedTable,
    UnsupportedConditionalFormat,
    UnsupportedCellType,
}

pub struct SheetXlsxService;

impl SheetXlsxService {
    pub fn import<I>(
        id_tool: &I,
        title: impl Into<String>,
        bytes: &[u8],
    ) -> Result<SheetDocument, SheetXlsxError>
    where
        I: SheetIdTool,
    {
        let entries = SheetXlsxArchiveTool::decode(bytes).map_err(Self::map_archive_error)?;
        for required in [
            XLSX_CONTENT_TYPES_ENTRY,
            XLSX_ROOT_RELATIONSHIPS_ENTRY,
            XLSX_WORKBOOK_ENTRY,
            XLSX_WORKBOOK_RELATIONSHIPS_ENTRY,
        ] {
            if !entries.contains_key(required) {
                return Err(SheetXlsxError::InvalidPackage);
            }
        }

        let root_relationships = entries
            .get(XLSX_ROOT_RELATIONSHIPS_ENTRY)
            .ok_or(SheetXlsxError::InvalidPackage)?;
        SheetXlsxXmlTool::validate_root_relationships(root_relationships)
            .map_err(Self::map_xml_error)?;

        let workbook_xml = entries
            .get(XLSX_WORKBOOK_ENTRY)
            .ok_or(SheetXlsxError::InvalidPackage)?;
        let relationship_xml = entries
            .get(XLSX_WORKBOOK_RELATIONSHIPS_ENTRY)
            .ok_or(SheetXlsxError::InvalidPackage)?;

        let descriptors =
            SheetXlsxXmlTool::parse_workbook(workbook_xml).map_err(Self::map_xml_error)?;
        if descriptors.is_empty() || descriptors.len() > MAX_XLSX_WORKSHEETS {
            return Err(SheetXlsxError::ResourceLimit);
        }

        let relationships = SheetXlsxXmlTool::parse_workbook_relationships(relationship_xml)
            .map_err(Self::map_xml_error)?;
        let shared_strings = entries
            .get(XLSX_SHARED_STRINGS_ENTRY)
            .map(|xml| SheetXlsxXmlTool::parse_shared_strings(xml))
            .transpose()
            .map_err(Self::map_xml_error)?
            .unwrap_or_default();

        let differential_styles = entries
            .get(XLSX_STYLES_ENTRY)
            .map(|xml| SheetXlsxXmlTool::parse_differential_styles(xml))
            .transpose()
            .map_err(Self::map_xml_error)?
            .unwrap_or_default();

        let mut worksheets = Vec::with_capacity(descriptors.len());
        let mut imported_tables = BTreeMap::new();
        let mut imported_conditional_formats = BTreeMap::new();
        let mut worksheet_names = HashSet::new();
        for descriptor in descriptors {
            let normalized_name = descriptor.name.to_lowercase();
            if !worksheet_names.insert(normalized_name) {
                return Err(SheetXlsxError::InvalidWorksheetName);
            }
            Self::validate_worksheet_name(&descriptor.name)?;
            let target = relationships
                .get(&descriptor.relationship_id)
                .ok_or(SheetXlsxError::InvalidPackage)?;
            let entry_name = Self::worksheet_entry_name(target)?;
            let worksheet_xml = entries
                .get(&entry_name)
                .ok_or(SheetXlsxError::InvalidPackage)?;
            let mut model =
                SheetXlsxXmlTool::parse_worksheet(descriptor.name, worksheet_xml, &shared_strings)
                    .map_err(Self::map_xml_error)?;
            let (conditional_formats, table_relationship_ids) =
                SheetXlsxXmlTool::parse_worksheet_metadata(worksheet_xml)
                    .map_err(Self::map_xml_error)?;
            model.conditional_formats = conditional_formats;
            model.table_relationship_ids = table_relationship_ids;

            let worksheet = Self::to_canonical_worksheet(id_tool, model.clone())?;
            Self::import_conditional_formats(
                id_tool,
                &worksheet.id,
                &model.conditional_formats,
                &differential_styles,
                &mut imported_conditional_formats,
            )?;

            if !model.table_relationship_ids.is_empty() {
                let rel_entry = Self::worksheet_relationship_entry_name(&entry_name)?;
                let rel_xml = entries
                    .get(&rel_entry)
                    .ok_or(SheetXlsxError::InvalidPackage)?;
                let table_relationships = SheetXlsxXmlTool::parse_table_relationships(rel_xml)
                    .map_err(Self::map_xml_error)?;
                for relationship_id in &model.table_relationship_ids {
                    let target = table_relationships
                        .get(relationship_id)
                        .ok_or(SheetXlsxError::InvalidPackage)?;
                    let table_entry = Self::table_entry_name(target)?;
                    let table_xml = entries
                        .get(&table_entry)
                        .ok_or(SheetXlsxError::InvalidPackage)?;
                    let table_model =
                        SheetXlsxXmlTool::parse_table(table_xml).map_err(Self::map_xml_error)?;
                    let table = Self::to_canonical_table(id_tool, &worksheet.id, table_model)?;
                    if imported_tables.insert(table.id.clone(), table).is_some() {
                        return Err(SheetXlsxError::InvalidPackage);
                    }
                    if imported_tables.len() > MAX_XLSX_TABLES {
                        return Err(SheetXlsxError::ResourceLimit);
                    }
                }
            }

            worksheets.push(worksheet);
        }

        let raw_title = title.into();
        let normalized = raw_title.trim();
        let title = if normalized.is_empty() {
            DEFAULT_DOCUMENT_TITLE.to_owned()
        } else {
            normalized.to_owned()
        };

        Ok(SheetDocument {
            id: id_tool.next_document_id(),
            title,
            schema_version: DocumentSchemaVersion::current(),
            revision: 0,
            worksheets,
            cell_formats: BTreeMap::new(),
            conditional_formats: imported_conditional_formats,
            tables: imported_tables,
            charts: BTreeMap::new(),
        })
    }

    pub fn export(document: &SheetDocument) -> Result<Vec<u8>, SheetXlsxError> {
        if document.worksheets.is_empty() || document.worksheets.len() > MAX_XLSX_WORKSHEETS {
            return Err(SheetXlsxError::ResourceLimit);
        }
        if document.tables.len() > MAX_XLSX_TABLES
            || document.conditional_formats.len() > MAX_XLSX_CONDITIONAL_FORMATS
        {
            return Err(SheetXlsxError::ResourceLimit);
        }

        let include_styles = !document.conditional_formats.is_empty();
        let mut worksheet_names = HashSet::new();
        let mut table_index = 1_usize;
        let mut table_parts = Vec::<(usize, XlsxTableModel)>::new();
        let mut worksheet_table_indices = Vec::<Vec<usize>>::new();

        let worksheets = document
            .worksheets
            .iter()
            .map(|worksheet| {
                let normalized_name = worksheet.name.to_lowercase();
                if !worksheet_names.insert(normalized_name) {
                    return Err(SheetXlsxError::InvalidWorksheetName);
                }

                let mut model = Self::to_xlsx_worksheet(worksheet)?;
                model.conditional_formats =
                    Self::to_xlsx_conditional_formats(document, &worksheet.id)?;

                let mut table_indices = Vec::new();
                let mut worksheet_tables = document
                    .tables
                    .values()
                    .filter(|table| table.worksheet_id == worksheet.id)
                    .collect::<Vec<_>>();
                worksheet_tables.sort_by(|left, right| left.id.as_str().cmp(right.id.as_str()));

                for table in worksheet_tables {
                    let model_table = Self::to_xlsx_table(worksheet, table)?;
                    table_parts.push((table_index, model_table));
                    table_indices.push(table_index);
                    model
                        .table_relationship_ids
                        .push(format!("rId{}", table_indices.len()));
                    table_index = table_index.saturating_add(1);
                }

                worksheet_table_indices.push(table_indices);
                Ok(model)
            })
            .collect::<Result<Vec<_>, SheetXlsxError>>()?;
        let workbook = XlsxWorkbookModel { worksheets };

        let mut entries = Vec::with_capacity(workbook.worksheets.len() + table_parts.len() * 2 + 6);
        entries.push((
            XLSX_CONTENT_TYPES_ENTRY.to_owned(),
            SheetXlsxXmlTool::content_types_xml_with_metadata(
                workbook.worksheets.len(),
                table_parts.len(),
                include_styles,
            )
            .into_bytes(),
        ));
        entries.push((
            XLSX_ROOT_RELATIONSHIPS_ENTRY.to_owned(),
            SheetXlsxXmlTool::root_relationships_xml().into_bytes(),
        ));
        entries.push((
            XLSX_WORKBOOK_ENTRY.to_owned(),
            SheetXlsxXmlTool::workbook_xml(&workbook.worksheets).into_bytes(),
        ));
        entries.push((
            XLSX_WORKBOOK_RELATIONSHIPS_ENTRY.to_owned(),
            SheetXlsxXmlTool::workbook_relationships_xml_with_styles(
                workbook.worksheets.len(),
                include_styles,
            )
            .into_bytes(),
        ));

        if include_styles {
            entries.push((
                XLSX_STYLES_ENTRY.to_owned(),
                SheetXlsxXmlTool::styles_xml().into_bytes(),
            ));
        }

        for (index, worksheet) in workbook.worksheets.iter().enumerate() {
            let sheet_number = index + 1;
            entries.push((
                format!("{XLSX_WORKSHEET_PREFIX}{sheet_number}{XLSX_WORKSHEET_SUFFIX}"),
                SheetXlsxXmlTool::worksheet_xml(worksheet).into_bytes(),
            ));

            let table_indices = &worksheet_table_indices[index];
            if !table_indices.is_empty() {
                entries.push((
                    format!("{XLSX_WORKSHEET_RELS_PREFIX}{sheet_number}{XLSX_RELATIONSHIP_SUFFIX}"),
                    SheetXlsxXmlTool::worksheet_relationships_xml(table_indices).into_bytes(),
                ));
            }
        }

        for (index, table) in table_parts {
            entries.push((
                format!("{XLSX_TABLE_PREFIX}{index}{XLSX_TABLE_SUFFIX}"),
                SheetXlsxXmlTool::table_xml(index, &table).into_bytes(),
            ));
        }

        SheetXlsxArchiveTool::encode(&entries).map_err(Self::map_archive_error)
    }

    fn to_canonical_worksheet<I>(
        id_tool: &I,
        model: XlsxWorksheetModel,
    ) -> Result<Worksheet, SheetXlsxError>
    where
        I: SheetIdTool,
    {
        let mut cells = BTreeMap::new();
        for cell in model.cells {
            let address = CellReferenceTool::parse(&cell.reference)
                .map_err(|_| SheetXlsxError::InvalidCellReference)?;
            let value = match cell.value {
                XlsxCellValue::Text(value) => {
                    Self::validate_cell_text(&value)?;
                    CellValue::Text(value)
                }
                XlsxCellValue::Number(value) => {
                    if !value.is_finite() {
                        return Err(SheetXlsxError::CellNumberNotFinite);
                    }
                    CellValue::Number(value)
                }
                XlsxCellValue::Boolean(value) => CellValue::Boolean(value),
            };
            if cells.insert(address, value).is_some() {
                return Err(SheetXlsxError::InvalidPackage);
            }
        }

        Ok(Worksheet {
            id: id_tool.next_worksheet_id(),
            name: model.name,
            cells,
        })
    }

    fn to_xlsx_worksheet(worksheet: &Worksheet) -> Result<XlsxWorksheetModel, SheetXlsxError> {
        Self::validate_worksheet_name(&worksheet.name)?;
        let mut cells = Vec::with_capacity(worksheet.cells.len());

        for (address, value) in &worksheet.cells {
            let reference = CellReferenceTool::format(*address)
                .map_err(|_| SheetXlsxError::InvalidCellReference)?;
            let value = match value {
                CellValue::Text(value) => {
                    Self::validate_cell_text(value)?;
                    XlsxCellValue::Text(value.clone())
                }
                CellValue::Number(value) => {
                    if !value.is_finite() {
                        return Err(SheetXlsxError::CellNumberNotFinite);
                    }
                    XlsxCellValue::Number(*value)
                }
                CellValue::Boolean(value) => XlsxCellValue::Boolean(*value),
                CellValue::Formula(_) => return Err(SheetXlsxError::UnsupportedFormula),
            };
            cells.push(XlsxCellModel { reference, value });
        }

        Ok(XlsxWorksheetModel {
            name: worksheet.name.clone(),
            cells,
            conditional_formats: Vec::new(),
            table_relationship_ids: Vec::new(),
        })
    }

    fn import_conditional_formats<I>(
        id_tool: &I,
        worksheet_id: &WorksheetId,
        rules: &[XlsxConditionalFormatModel],
        styles: &[XlsxDifferentialStyleModel],
        output: &mut BTreeMap<ConditionalFormatRuleId, SheetConditionalFormatRule>,
    ) -> Result<(), SheetXlsxError>
    where
        I: SheetIdTool,
    {
        for rule in rules {
            if output.len() >= MAX_XLSX_CONDITIONAL_FORMATS {
                return Err(SheetXlsxError::ResourceLimit);
            }
            let range = Self::parse_range_reference(&rule.range_reference)?;
            let condition = match &rule.condition {
                XlsxConditionalFormatCondition::NumberGreaterThan(value) => {
                    SheetConditionalFormatCondition::NumberGreaterThan(*value)
                }
                XlsxConditionalFormatCondition::NumberLessThan(value) => {
                    SheetConditionalFormatCondition::NumberLessThan(*value)
                }
                XlsxConditionalFormatCondition::NumberEquals(value) => {
                    SheetConditionalFormatCondition::NumberEquals(*value)
                }
                XlsxConditionalFormatCondition::TextContains(value) => {
                    SheetConditionalFormatCondition::TextContains(value.clone())
                }
            };
            let style = Self::style_from_dxf(rule.differential_style_id, styles)?;
            let id = id_tool.next_conditional_format_rule_id();
            output.insert(
                id.clone(),
                SheetConditionalFormatRule {
                    id,
                    worksheet_id: worksheet_id.clone(),
                    range,
                    condition,
                    style,
                    priority: rule.priority,
                },
            );
        }
        Ok(())
    }

    fn style_from_dxf(
        dxf_id: usize,
        styles: &[XlsxDifferentialStyleModel],
    ) -> Result<SheetConditionalFormatStyle, SheetXlsxError> {
        let style = styles
            .get(dxf_id)
            .ok_or(SheetXlsxError::UnsupportedConditionalFormat)?;
        match style.fill_rgb.as_deref() {
            Some(XLSX_CONDITIONAL_WARNING_RGB) => Ok(SheetConditionalFormatStyle::Warning),
            Some(XLSX_CONDITIONAL_SUCCESS_RGB) => Ok(SheetConditionalFormatStyle::Success),
            Some(XLSX_CONDITIONAL_ACCENT_RGB) => Ok(SheetConditionalFormatStyle::Accent),
            Some(_) | None => Ok(SheetConditionalFormatStyle::Accent),
        }
    }

    fn to_canonical_table<I>(
        id_tool: &I,
        worksheet_id: &WorksheetId,
        table: XlsxTableModel,
    ) -> Result<SheetTable, SheetXlsxError>
    where
        I: SheetIdTool,
    {
        let range = Self::parse_range_reference(&table.range_reference)?;
        if range.start_row >= range.end_row {
            return Err(SheetXlsxError::UnsupportedTable);
        }
        let id = id_tool.next_table_id();
        Ok(SheetTable {
            id,
            worksheet_id: worksheet_id.clone(),
            name: table.name,
            range,
        })
    }

    fn to_xlsx_table(
        worksheet: &Worksheet,
        table: &SheetTable,
    ) -> Result<XlsxTableModel, SheetXlsxError> {
        let range_reference = Self::format_range_reference(table.range)?;
        let mut column_names = Vec::new();
        for column in table.range.start_column..=table.range.end_column {
            let address = turkuaz_office_sheet::CellAddress {
                row: table.range.start_row,
                column,
            };
            let fallback = format!("Column{}", column_names.len() + 1);
            let name = match worksheet.cells.get(&address) {
                Some(CellValue::Text(value)) if !value.trim().is_empty() => value.clone(),
                Some(CellValue::Number(value)) => value.to_string(),
                Some(CellValue::Boolean(value)) => value.to_string(),
                Some(CellValue::Formula(_)) | Some(CellValue::Text(_)) | None => fallback,
            };
            let unique = Self::unique_table_column_name(name, &column_names);
            column_names.push(unique);
        }
        Ok(XlsxTableModel {
            name: table.name.clone(),
            range_reference,
            column_names,
        })
    }

    fn unique_table_column_name(candidate: String, existing: &[String]) -> String {
        let base = if candidate.trim().is_empty() {
            "Column".to_owned()
        } else {
            candidate
        };
        if existing
            .iter()
            .all(|name| !name.eq_ignore_ascii_case(&base))
        {
            return base;
        }
        let mut suffix = 2_usize;
        loop {
            let value = format!("{base}_{suffix}");
            if existing
                .iter()
                .all(|name| !name.eq_ignore_ascii_case(&value))
            {
                return value;
            }
            suffix = suffix.saturating_add(1);
        }
    }

    fn to_xlsx_conditional_formats(
        document: &SheetDocument,
        worksheet_id: &WorksheetId,
    ) -> Result<Vec<XlsxConditionalFormatModel>, SheetXlsxError> {
        let mut rules = document
            .conditional_formats
            .values()
            .filter(|rule| &rule.worksheet_id == worksheet_id)
            .collect::<Vec<_>>();
        rules.sort_by_key(|rule| rule.priority);

        rules
            .into_iter()
            .map(|rule| {
                let condition = match &rule.condition {
                    SheetConditionalFormatCondition::NumberGreaterThan(value) => {
                        XlsxConditionalFormatCondition::NumberGreaterThan(*value)
                    }
                    SheetConditionalFormatCondition::NumberLessThan(value) => {
                        XlsxConditionalFormatCondition::NumberLessThan(*value)
                    }
                    SheetConditionalFormatCondition::NumberEquals(value) => {
                        XlsxConditionalFormatCondition::NumberEquals(*value)
                    }
                    SheetConditionalFormatCondition::TextContains(value) => {
                        XlsxConditionalFormatCondition::TextContains(value.clone())
                    }
                };
                let differential_style_id = match rule.style {
                    SheetConditionalFormatStyle::Warning => 0,
                    SheetConditionalFormatStyle::Success => 1,
                    SheetConditionalFormatStyle::Accent => 2,
                };
                Ok(XlsxConditionalFormatModel {
                    range_reference: Self::format_range_reference(rule.range)?,
                    condition,
                    differential_style_id,
                    priority: rule.priority,
                })
            })
            .collect()
    }

    fn parse_range_reference(reference: &str) -> Result<SheetRange, SheetXlsxError> {
        let mut parts = reference.split(':');
        let first = parts.next().ok_or(SheetXlsxError::InvalidCellReference)?;
        let second = parts.next().unwrap_or(first);
        if parts.next().is_some() {
            return Err(SheetXlsxError::InvalidCellReference);
        }
        let start =
            CellReferenceTool::parse(first).map_err(|_| SheetXlsxError::InvalidCellReference)?;
        let end =
            CellReferenceTool::parse(second).map_err(|_| SheetXlsxError::InvalidCellReference)?;
        Ok(SheetRange {
            start_row: start.row.min(end.row),
            end_row: start.row.max(end.row),
            start_column: start.column.min(end.column),
            end_column: start.column.max(end.column),
        })
    }

    fn format_range_reference(range: SheetRange) -> Result<String, SheetXlsxError> {
        let start = CellReferenceTool::format(turkuaz_office_sheet::CellAddress {
            row: range.start_row,
            column: range.start_column,
        })
        .map_err(|_| SheetXlsxError::InvalidCellReference)?;
        let end = CellReferenceTool::format(turkuaz_office_sheet::CellAddress {
            row: range.end_row,
            column: range.end_column,
        })
        .map_err(|_| SheetXlsxError::InvalidCellReference)?;
        Ok(if start == end {
            start
        } else {
            format!("{start}:{end}")
        })
    }

    fn table_entry_name(target: &str) -> Result<String, SheetXlsxError> {
        let trimmed = target.trim();
        if trimmed.is_empty() || trimmed.contains('\\') || trimmed.contains(':') {
            return Err(SheetXlsxError::InvalidPackage);
        }

        let relative = trimmed.trim_start_matches('/');
        if relative.starts_with("xl/tables/") {
            return Self::validate_resolved_entry(relative);
        }
        if let Some(stripped) = relative.strip_prefix("../tables/") {
            return Self::validate_resolved_entry(&format!("xl/tables/{stripped}"));
        }
        Err(SheetXlsxError::InvalidPackage)
    }

    fn validate_resolved_entry(entry: &str) -> Result<String, SheetXlsxError> {
        if entry
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err(SheetXlsxError::InvalidPackage);
        }
        Ok(entry.to_owned())
    }

    fn validate_worksheet_name(name: &str) -> Result<(), SheetXlsxError> {
        let trimmed = name.trim();
        if trimmed.is_empty()
            || trimmed.chars().count() > MAX_WORKSHEET_NAME_LENGTH
            || trimmed.starts_with('\'')
            || trimmed.ends_with('\'')
            || trimmed.chars().any(|character| {
                !Self::is_xml_1_0_character(character)
                    || matches!(character, '[' | ']' | ':' | '*' | '?' | '/' | '\\')
            })
        {
            return Err(SheetXlsxError::InvalidWorksheetName);
        }
        Ok(())
    }

    fn validate_cell_text(value: &str) -> Result<(), SheetXlsxError> {
        if value.chars().count() > MAX_CELL_TEXT_LENGTH {
            return Err(SheetXlsxError::CellTextTooLong);
        }
        if value
            .chars()
            .any(|character| !Self::is_xml_1_0_character(character))
        {
            return Err(SheetXlsxError::InvalidCellText);
        }
        Ok(())
    }

    fn is_xml_1_0_character(character: char) -> bool {
        matches!(character, '\u{0009}' | '\u{000A}' | '\u{000D}')
            || ('\u{0020}'..='\u{D7FF}').contains(&character)
            || ('\u{E000}'..='\u{FFFD}').contains(&character)
            || ('\u{10000}'..='\u{10FFFF}').contains(&character)
    }

    fn worksheet_relationship_entry_name(entry_name: &str) -> Result<String, SheetXlsxError> {
        let file_name = entry_name
            .strip_prefix("xl/worksheets/")
            .ok_or(SheetXlsxError::InvalidPackage)?;
        if file_name.is_empty()
            || file_name.contains('/')
            || file_name.contains('\\')
            || !file_name.ends_with(XLSX_WORKSHEET_SUFFIX)
        {
            return Err(SheetXlsxError::InvalidPackage);
        }
        Ok(format!("xl/worksheets/_rels/{file_name}.rels"))
    }

    fn worksheet_entry_name(target: &str) -> Result<String, SheetXlsxError> {
        let trimmed = target.trim();
        if trimmed.is_empty()
            || trimmed.contains('\\')
            || trimmed.contains(':')
            || trimmed
                .trim_start_matches('/')
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err(SheetXlsxError::InvalidPackage);
        }

        let relative = trimmed.trim_start_matches('/');
        if relative.starts_with("xl/") {
            Ok(relative.to_owned())
        } else {
            Ok(format!("xl/{relative}"))
        }
    }

    fn map_archive_error(error: SheetXlsxArchiveError) -> SheetXlsxError {
        match error {
            SheetXlsxArchiveError::PackageTooLarge
            | SheetXlsxArchiveError::TooManyEntries
            | SheetXlsxArchiveError::EntryTooLarge => SheetXlsxError::ResourceLimit,
            SheetXlsxArchiveError::UnsafeEntryName
            | SheetXlsxArchiveError::DuplicateEntry
            | SheetXlsxArchiveError::UnsupportedCompression
            | SheetXlsxArchiveError::ReadFailed
            | SheetXlsxArchiveError::WriteFailed => SheetXlsxError::InvalidPackage,
        }
    }

    fn map_xml_error(error: SheetXlsxXmlError) -> SheetXlsxError {
        match error {
            SheetXlsxXmlError::DocumentTooLarge
            | SheetXlsxXmlError::TooDeep
            | SheetXlsxXmlError::TooManyNodes => SheetXlsxError::ResourceLimit,
            SheetXlsxXmlError::UnsupportedFormula => SheetXlsxError::UnsupportedFormula,
            SheetXlsxXmlError::UnsupportedTable => SheetXlsxError::UnsupportedTable,
            SheetXlsxXmlError::UnsupportedConditionalFormat => {
                SheetXlsxError::UnsupportedConditionalFormat
            }
            SheetXlsxXmlError::UnsupportedCellType => SheetXlsxError::UnsupportedCellType,
            SheetXlsxXmlError::InvalidNumber
            | SheetXlsxXmlError::InvalidXml
            | SheetXlsxXmlError::InvalidUtf8
            | SheetXlsxXmlError::InvalidAttribute
            | SheetXlsxXmlError::MissingAttribute
            | SheetXlsxXmlError::SharedStringOutOfBounds
            | SheetXlsxXmlError::DocTypeUnsupported => SheetXlsxError::InvalidPackage,
        }
    }
}
