// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/tools/sheet_xlsx_xml_tool.rs
// # 📌 Amac: XLSX SpreadsheetML workbook/relationship/worksheet/table/style/shared-string XML parcalarini streaming parse eder ve yazar
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.9.0
// Aciklama: Value hucrelerine ek olarak OOXML table ve desteklenen conditional-format metadata semantigini format modelinde tutar
// Bagimli Oldugu Katman: Tool -> Model -> Config

use std::collections::HashMap;

use quick_xml::XmlVersion;
use quick_xml::escape::{escape, resolve_predefined_entity, unescape};
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

use crate::config::sheet_constants::{
    MAX_XLSX_CONDITIONAL_FORMATS, MAX_XLSX_TABLES, MAX_XLSX_XML_BYTES, MAX_XLSX_XML_DEPTH,
    MAX_XLSX_XML_NODES, XLSX_CONDITIONAL_ACCENT_RGB, XLSX_CONDITIONAL_SUCCESS_RGB,
    XLSX_CONDITIONAL_WARNING_RGB, XLSX_CONTENT_TYPES_NAMESPACE,
    XLSX_OFFICE_DOCUMENT_RELATIONSHIP_TYPE, XLSX_OFFICE_RELATIONSHIPS_NAMESPACE,
    XLSX_PACKAGE_RELATIONSHIPS_NAMESPACE, XLSX_SPREADSHEET_NAMESPACE, XLSX_STYLES_CONTENT_TYPE,
    XLSX_STYLES_RELATIONSHIP_TYPE, XLSX_TABLE_CONTENT_TYPE, XLSX_TABLE_RELATIONSHIP_TYPE,
    XLSX_WORKBOOK_CONTENT_TYPE, XLSX_WORKSHEET_CONTENT_TYPE, XLSX_WORKSHEET_RELATIONSHIP_TYPE,
};
use crate::models::sheet_xlsx_model::{
    XlsxCellModel, XlsxCellValue, XlsxConditionalFormatCondition, XlsxConditionalFormatModel,
    XlsxDifferentialStyleModel, XlsxSheetDescriptor, XlsxTableModel, XlsxWorksheetModel,
};

const TAG_SHEET: &str = "sheet";
const TAG_RELATIONSHIP: &str = "Relationship";
const TAG_SHARED_ITEM: &str = "si";
const TAG_CELL: &str = "c";
const TAG_VALUE: &str = "v";
const TAG_TEXT: &str = "t";
const TAG_FORMULA: &str = "f";
const TAG_CONDITIONAL_FORMATTING: &str = "conditionalFormatting";
const TAG_CONDITIONAL_RULE: &str = "cfRule";
const TAG_TABLE_PART: &str = "tablePart";
const TAG_TABLE: &str = "table";
const TAG_TABLE_COLUMN: &str = "tableColumn";
const TAG_DXF: &str = "dxf";
const TAG_FG_COLOR: &str = "fgColor";

const ATTR_NAME: &str = "name";
const ATTR_SHEET_RELATIONSHIP_ID: &str = "id";
const ATTR_PACKAGE_RELATIONSHIP_ID: &str = "Id";
const ATTR_TARGET: &str = "Target";
const ATTR_TYPE: &str = "Type";
const ATTR_RULE_TYPE: &str = "type";
const ATTR_REFERENCE: &str = "r";
const ATTR_TABLE_REFERENCE: &str = "ref";
const ATTR_CELL_TYPE: &str = "t";
const ATTR_RANGE_REFERENCE: &str = "sqref";
const ATTR_DXF_ID: &str = "dxfId";
const ATTR_PRIORITY: &str = "priority";
const ATTR_OPERATOR: &str = "operator";
const ATTR_TEXT_VALUE: &str = "text";
const ATTR_RGB: &str = "rgb";
const ATTR_DISPLAY_NAME: &str = "displayName";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SheetXlsxXmlError {
    DocumentTooLarge,
    TooDeep,
    TooManyNodes,
    InvalidXml,
    InvalidUtf8,
    InvalidAttribute,
    InvalidNumber,
    MissingAttribute,
    SharedStringOutOfBounds,
    UnsupportedCellType,
    UnsupportedFormula,
    UnsupportedConditionalFormat,
    UnsupportedTable,
    DocTypeUnsupported,
}

#[derive(Clone, Debug)]
struct CellState {
    reference: String,
    cell_type: Option<String>,
    value: String,
    inline_text: String,
}

#[derive(Clone, Debug)]
struct ConditionalRuleState {
    range_reference: String,
    rule_type: String,
    operator: Option<String>,
    differential_style_id: usize,
    priority: u32,
    text_value: Option<String>,
    formula: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TextTarget {
    None,
    Value,
    Inline,
    Shared,
}

pub struct SheetXlsxXmlTool;

impl SheetXlsxXmlTool {
    pub fn parse_workbook(xml: &[u8]) -> Result<Vec<XlsxSheetDescriptor>, SheetXlsxXmlError> {
        let mut reader = Self::reader(xml)?;
        let mut sheets = Vec::new();
        let mut depth = 0_usize;
        let mut nodes = 0_usize;

        loop {
            match reader
                .read_event()
                .map_err(|_| SheetXlsxXmlError::InvalidXml)?
            {
                Event::Start(start) => {
                    depth = depth.saturating_add(1);
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    if Self::local_name(start.name().as_ref()) == TAG_SHEET {
                        sheets.push(Self::sheet_descriptor(&start)?);
                    }
                }
                Event::Empty(empty) => {
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    if Self::local_name(empty.name().as_ref()) == TAG_SHEET {
                        sheets.push(Self::sheet_descriptor(&empty)?);
                    }
                }
                Event::End(_) => depth = depth.saturating_sub(1),
                Event::DocType(_) => return Err(SheetXlsxXmlError::DocTypeUnsupported),
                Event::Eof => break,
                _ => {}
            }
        }
        Ok(sheets)
    }

    pub fn validate_root_relationships(xml: &[u8]) -> Result<(), SheetXlsxXmlError> {
        let mut reader = Self::reader(xml)?;
        let mut depth = 0_usize;
        let mut nodes = 0_usize;
        let mut valid = false;

        loop {
            match reader
                .read_event()
                .map_err(|_| SheetXlsxXmlError::InvalidXml)?
            {
                Event::Start(start) => {
                    depth = depth.saturating_add(1);
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    valid |= Self::is_office_document_relationship(&start)?;
                }
                Event::Empty(empty) => {
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    valid |= Self::is_office_document_relationship(&empty)?;
                }
                Event::End(_) => depth = depth.saturating_sub(1),
                Event::DocType(_) => return Err(SheetXlsxXmlError::DocTypeUnsupported),
                Event::Eof => break,
                _ => {}
            }
        }

        if valid {
            Ok(())
        } else {
            Err(SheetXlsxXmlError::MissingAttribute)
        }
    }

    pub fn parse_workbook_relationships(
        xml: &[u8],
    ) -> Result<HashMap<String, String>, SheetXlsxXmlError> {
        let mut reader = Self::reader(xml)?;
        let mut relationships = HashMap::new();
        let mut depth = 0_usize;
        let mut nodes = 0_usize;

        loop {
            match reader
                .read_event()
                .map_err(|_| SheetXlsxXmlError::InvalidXml)?
            {
                Event::Start(start) => {
                    depth = depth.saturating_add(1);
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    Self::read_relationship(&start, &mut relationships)?;
                }
                Event::Empty(empty) => {
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    Self::read_relationship(&empty, &mut relationships)?;
                }
                Event::End(_) => depth = depth.saturating_sub(1),
                Event::DocType(_) => return Err(SheetXlsxXmlError::DocTypeUnsupported),
                Event::Eof => break,
                _ => {}
            }
        }
        Ok(relationships)
    }

    pub fn parse_shared_strings(xml: &[u8]) -> Result<Vec<String>, SheetXlsxXmlError> {
        let mut reader = Self::reader(xml)?;
        let mut strings = Vec::new();
        let mut current: Option<String> = None;
        let mut target = TextTarget::None;
        let mut depth = 0_usize;
        let mut nodes = 0_usize;

        loop {
            match reader
                .read_event()
                .map_err(|_| SheetXlsxXmlError::InvalidXml)?
            {
                Event::Start(start) => {
                    depth = depth.saturating_add(1);
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    match Self::local_name(start.name().as_ref()) {
                        TAG_SHARED_ITEM => current = Some(String::new()),
                        TAG_TEXT if current.is_some() => target = TextTarget::Shared,
                        _ => {}
                    }
                }
                Event::Text(text) if target == TextTarget::Shared => {
                    if let Some(value) = current.as_mut() {
                        value.push_str(&text.xml_content(XmlVersion::Implicit1_0));
                    }
                }
                Event::CData(text) if target == TextTarget::Shared => {
                    if let Some(value) = current.as_mut() {
                        value.push_str(&text.xml_content(XmlVersion::Implicit1_0));
                    }
                }
                Event::GeneralRef(reference) if target == TextTarget::Shared => {
                    if let Some(value) = current.as_mut() {
                        Self::append_reference(value, reference.as_ref(), &reference)?;
                    }
                }
                Event::End(end) => {
                    match Self::local_name(end.name().as_ref()) {
                        TAG_TEXT => target = TextTarget::None,
                        TAG_SHARED_ITEM => {
                            if let Some(value) = current.take() {
                                strings.push(value);
                            }
                        }
                        _ => {}
                    }
                    depth = depth.saturating_sub(1);
                }
                Event::DocType(_) => return Err(SheetXlsxXmlError::DocTypeUnsupported),
                Event::Eof => break,
                _ => {}
            }
        }
        Ok(strings)
    }

    pub fn parse_worksheet(
        name: String,
        xml: &[u8],
        shared_strings: &[String],
    ) -> Result<XlsxWorksheetModel, SheetXlsxXmlError> {
        let mut reader = Self::reader(xml)?;
        let mut cells = Vec::new();
        let mut current: Option<CellState> = None;
        let mut target = TextTarget::None;
        let mut depth = 0_usize;
        let mut nodes = 0_usize;

        loop {
            match reader
                .read_event()
                .map_err(|_| SheetXlsxXmlError::InvalidXml)?
            {
                Event::Start(start) => {
                    depth = depth.saturating_add(1);
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    match Self::local_name(start.name().as_ref()) {
                        TAG_CELL => {
                            current = Some(CellState {
                                reference: Self::attribute(&start, ATTR_REFERENCE)?
                                    .ok_or(SheetXlsxXmlError::MissingAttribute)?,
                                cell_type: Self::attribute(&start, ATTR_CELL_TYPE)?,
                                value: String::new(),
                                inline_text: String::new(),
                            });
                        }
                        TAG_FORMULA if current.is_some() => {
                            return Err(SheetXlsxXmlError::UnsupportedFormula);
                        }
                        TAG_VALUE if current.is_some() => target = TextTarget::Value,
                        TAG_TEXT if current.is_some() => target = TextTarget::Inline,
                        _ => {}
                    }
                }
                Event::Text(text) if current.is_some() => {
                    let decoded = text.xml_content(XmlVersion::Implicit1_0);
                    Self::append_target(current.as_mut().expect("checked"), target, &decoded);
                }
                Event::CData(text) if current.is_some() => {
                    let decoded = text.xml_content(XmlVersion::Implicit1_0);
                    Self::append_target(current.as_mut().expect("checked"), target, &decoded);
                }
                Event::GeneralRef(reference) if current.is_some() => {
                    let mut decoded = String::new();
                    Self::append_reference(&mut decoded, reference.as_ref(), &reference)?;
                    Self::append_target(current.as_mut().expect("checked"), target, &decoded);
                }
                Event::End(end) => {
                    match Self::local_name(end.name().as_ref()) {
                        TAG_VALUE | TAG_TEXT => target = TextTarget::None,
                        TAG_CELL => {
                            if let Some(cell) = current.take()
                                && let Some(value) = Self::cell_value(&cell, shared_strings)?
                            {
                                cells.push(XlsxCellModel {
                                    reference: cell.reference,
                                    value,
                                });
                            }
                            target = TextTarget::None;
                        }
                        _ => {}
                    }
                    depth = depth.saturating_sub(1);
                }
                Event::DocType(_) => return Err(SheetXlsxXmlError::DocTypeUnsupported),
                Event::Eof => break,
                _ => {}
            }
        }

        Ok(XlsxWorksheetModel {
            name,
            cells,
            conditional_formats: Vec::new(),
            table_relationship_ids: Vec::new(),
        })
    }

    pub fn parse_worksheet_metadata(
        xml: &[u8],
    ) -> Result<(Vec<XlsxConditionalFormatModel>, Vec<String>), SheetXlsxXmlError> {
        let mut reader = Self::reader(xml)?;
        let mut conditional_formats = Vec::new();
        let mut table_relationship_ids = Vec::new();
        let mut current_range: Option<String> = None;
        let mut current_rule: Option<ConditionalRuleState> = None;
        let mut in_rule_formula = false;
        let mut depth = 0_usize;
        let mut nodes = 0_usize;

        loop {
            match reader
                .read_event()
                .map_err(|_| SheetXlsxXmlError::InvalidXml)?
            {
                Event::Start(start) => {
                    depth = depth.saturating_add(1);
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    match Self::local_name(start.name().as_ref()) {
                        TAG_CONDITIONAL_FORMATTING => {
                            let range = Self::attribute(&start, ATTR_RANGE_REFERENCE)?
                                .ok_or(SheetXlsxXmlError::MissingAttribute)?;
                            if range.split_whitespace().count() != 1 {
                                return Err(SheetXlsxXmlError::UnsupportedConditionalFormat);
                            }
                            current_range = Some(range);
                        }
                        TAG_CONDITIONAL_RULE => {
                            let range =
                                current_range.clone().ok_or(SheetXlsxXmlError::InvalidXml)?;
                            current_rule = Some(Self::conditional_rule_state(&start, range)?);
                        }
                        TAG_FORMULA if current_rule.is_some() => in_rule_formula = true,
                        TAG_TABLE_PART => {
                            Self::push_table_relationship_id(&start, &mut table_relationship_ids)?;
                        }
                        _ => {}
                    }
                }
                Event::Empty(empty) => {
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    match Self::local_name(empty.name().as_ref()) {
                        TAG_TABLE_PART => {
                            Self::push_table_relationship_id(&empty, &mut table_relationship_ids)?;
                        }
                        TAG_CONDITIONAL_RULE => {
                            let range =
                                current_range.clone().ok_or(SheetXlsxXmlError::InvalidXml)?;
                            let state = Self::conditional_rule_state(&empty, range)?;
                            conditional_formats.push(Self::finish_conditional_rule(state)?);
                            if conditional_formats.len() > MAX_XLSX_CONDITIONAL_FORMATS {
                                return Err(SheetXlsxXmlError::TooManyNodes);
                            }
                        }
                        _ => {}
                    }
                }
                Event::Text(text) if in_rule_formula => {
                    if let Some(rule) = current_rule.as_mut() {
                        rule.formula
                            .push_str(&text.xml_content(XmlVersion::Implicit1_0));
                    }
                }
                Event::CData(text) if in_rule_formula => {
                    if let Some(rule) = current_rule.as_mut() {
                        rule.formula
                            .push_str(&text.xml_content(XmlVersion::Implicit1_0));
                    }
                }
                Event::GeneralRef(reference) if in_rule_formula => {
                    if let Some(rule) = current_rule.as_mut() {
                        Self::append_reference(&mut rule.formula, reference.as_ref(), &reference)?;
                    }
                }
                Event::End(end) => {
                    match Self::local_name(end.name().as_ref()) {
                        TAG_FORMULA if current_rule.is_some() => in_rule_formula = false,
                        TAG_CONDITIONAL_RULE => {
                            let state = current_rule.take().ok_or(SheetXlsxXmlError::InvalidXml)?;
                            conditional_formats.push(Self::finish_conditional_rule(state)?);
                            if conditional_formats.len() > MAX_XLSX_CONDITIONAL_FORMATS {
                                return Err(SheetXlsxXmlError::TooManyNodes);
                            }
                            in_rule_formula = false;
                        }
                        TAG_CONDITIONAL_FORMATTING => current_range = None,
                        _ => {}
                    }
                    depth = depth.saturating_sub(1);
                }
                Event::DocType(_) => return Err(SheetXlsxXmlError::DocTypeUnsupported),
                Event::Eof => break,
                _ => {}
            }
        }

        Ok((conditional_formats, table_relationship_ids))
    }

    pub fn parse_table_relationships(
        xml: &[u8],
    ) -> Result<HashMap<String, String>, SheetXlsxXmlError> {
        Self::parse_relationships_of_type(xml, XLSX_TABLE_RELATIONSHIP_TYPE)
    }

    pub fn parse_table(xml: &[u8]) -> Result<XlsxTableModel, SheetXlsxXmlError> {
        let mut reader = Self::reader(xml)?;
        let mut name: Option<String> = None;
        let mut range_reference: Option<String> = None;
        let mut column_names = Vec::new();
        let mut depth = 0_usize;
        let mut nodes = 0_usize;

        loop {
            match reader
                .read_event()
                .map_err(|_| SheetXlsxXmlError::InvalidXml)?
            {
                Event::Start(start) => {
                    depth = depth.saturating_add(1);
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    match Self::local_name(start.name().as_ref()) {
                        TAG_TABLE => {
                            name = Self::attribute(&start, ATTR_DISPLAY_NAME)?
                                .or(Self::attribute(&start, ATTR_NAME)?);
                            range_reference = Self::attribute(&start, ATTR_TABLE_REFERENCE)?;
                        }
                        TAG_TABLE_COLUMN => {
                            if let Some(value) = Self::attribute(&start, ATTR_NAME)? {
                                column_names.push(value);
                            }
                        }
                        _ => {}
                    }
                }
                Event::Empty(empty) => {
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    match Self::local_name(empty.name().as_ref()) {
                        TAG_TABLE => {
                            name = Self::attribute(&empty, ATTR_DISPLAY_NAME)?
                                .or(Self::attribute(&empty, ATTR_NAME)?);
                            range_reference = Self::attribute(&empty, ATTR_TABLE_REFERENCE)?;
                        }
                        TAG_TABLE_COLUMN => {
                            if let Some(value) = Self::attribute(&empty, ATTR_NAME)? {
                                column_names.push(value);
                            }
                        }
                        _ => {}
                    }
                }
                Event::End(_) => depth = depth.saturating_sub(1),
                Event::DocType(_) => return Err(SheetXlsxXmlError::DocTypeUnsupported),
                Event::Eof => break,
                _ => {}
            }
        }

        Ok(XlsxTableModel {
            name: name.ok_or(SheetXlsxXmlError::MissingAttribute)?,
            range_reference: range_reference.ok_or(SheetXlsxXmlError::MissingAttribute)?,
            column_names,
        })
    }

    pub fn parse_differential_styles(
        xml: &[u8],
    ) -> Result<Vec<XlsxDifferentialStyleModel>, SheetXlsxXmlError> {
        let mut reader = Self::reader(xml)?;
        let mut styles = Vec::new();
        let mut current: Option<Option<String>> = None;
        let mut depth = 0_usize;
        let mut nodes = 0_usize;

        loop {
            match reader
                .read_event()
                .map_err(|_| SheetXlsxXmlError::InvalidXml)?
            {
                Event::Start(start) => {
                    depth = depth.saturating_add(1);
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    match Self::local_name(start.name().as_ref()) {
                        TAG_DXF => current = Some(None),
                        TAG_FG_COLOR if current.is_some() => {
                            if let Some(slot) = current.as_mut() {
                                *slot = Self::attribute(&start, ATTR_RGB)?
                                    .map(|value| value.to_ascii_uppercase());
                            }
                        }
                        _ => {}
                    }
                }
                Event::Empty(empty) => {
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    if Self::local_name(empty.name().as_ref()) == TAG_FG_COLOR
                        && let Some(slot) = current.as_mut()
                    {
                        *slot = Self::attribute(&empty, ATTR_RGB)?
                            .map(|value| value.to_ascii_uppercase());
                    }
                }
                Event::End(end) => {
                    if Self::local_name(end.name().as_ref()) == TAG_DXF {
                        let fill_rgb = current.take().ok_or(SheetXlsxXmlError::InvalidXml)?;
                        styles.push(XlsxDifferentialStyleModel { fill_rgb });
                    }
                    depth = depth.saturating_sub(1);
                }
                Event::DocType(_) => return Err(SheetXlsxXmlError::DocTypeUnsupported),
                Event::Eof => break,
                _ => {}
            }
        }

        Ok(styles)
    }

    pub fn content_types_xml(sheet_count: usize) -> String {
        Self::content_types_xml_with_metadata(sheet_count, 0, false)
    }

    pub fn content_types_xml_with_metadata(
        sheet_count: usize,
        table_count: usize,
        include_styles: bool,
    ) -> String {
        let mut xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Types xmlns="{XLSX_CONTENT_TYPES_NAMESPACE}"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="{XLSX_WORKBOOK_CONTENT_TYPE}"/>"#
        );
        for index in 1..=sheet_count {
            xml.push_str(&format!(
                r#"<Override PartName="/xl/worksheets/sheet{index}.xml" ContentType="{XLSX_WORKSHEET_CONTENT_TYPE}"/>"#
            ));
        }
        for index in 1..=table_count {
            xml.push_str(&format!(
                r#"<Override PartName="/xl/tables/table{index}.xml" ContentType="{XLSX_TABLE_CONTENT_TYPE}"/>"#
            ));
        }
        if include_styles {
            xml.push_str(&format!(
                r#"<Override PartName="/xl/styles.xml" ContentType="{XLSX_STYLES_CONTENT_TYPE}"/>"#
            ));
        }
        xml.push_str("</Types>");
        xml
    }

    pub fn root_relationships_xml() -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="{XLSX_PACKAGE_RELATIONSHIPS_NAMESPACE}"><Relationship Id="rId1" Type="{XLSX_OFFICE_DOCUMENT_RELATIONSHIP_TYPE}" Target="xl/workbook.xml"/></Relationships>"#
        )
    }

    pub fn workbook_xml(worksheets: &[XlsxWorksheetModel]) -> String {
        let mut xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><workbook xmlns="{XLSX_SPREADSHEET_NAMESPACE}" xmlns:r="{XLSX_OFFICE_RELATIONSHIPS_NAMESPACE}"><sheets>"#
        );
        for (index, worksheet) in worksheets.iter().enumerate() {
            let sheet_id = index + 1;
            xml.push_str(r#"<sheet name=""#);
            xml.push_str(&escape(worksheet.name.as_str()));
            xml.push_str(&format!(r#"" sheetId="{sheet_id}" r:id="rId{sheet_id}"/>"#));
        }
        xml.push_str("</sheets></workbook>");
        xml
    }

    pub fn workbook_relationships_xml(sheet_count: usize) -> String {
        Self::workbook_relationships_xml_with_styles(sheet_count, false)
    }

    pub fn workbook_relationships_xml_with_styles(
        sheet_count: usize,
        include_styles: bool,
    ) -> String {
        let mut xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="{XLSX_PACKAGE_RELATIONSHIPS_NAMESPACE}">"#
        );
        for index in 1..=sheet_count {
            xml.push_str(&format!(
                r#"<Relationship Id="rId{index}" Type="{XLSX_WORKSHEET_RELATIONSHIP_TYPE}" Target="worksheets/sheet{index}.xml"/>"#
            ));
        }
        if include_styles {
            let relationship_id = sheet_count + 1;
            xml.push_str(&format!(
                r#"<Relationship Id="rId{relationship_id}" Type="{XLSX_STYLES_RELATIONSHIP_TYPE}" Target="styles.xml"/>"#
            ));
        }
        xml.push_str("</Relationships>");
        xml
    }

    pub fn worksheet_xml(worksheet: &XlsxWorksheetModel) -> String {
        let mut xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><worksheet xmlns="{XLSX_SPREADSHEET_NAMESPACE}" xmlns:r="{XLSX_OFFICE_RELATIONSHIPS_NAMESPACE}"><sheetData>"#
        );
        let mut current_row: Option<String> = None;

        for cell in &worksheet.cells {
            let row = Self::reference_row(&cell.reference);
            if current_row.as_deref() != Some(row) {
                if current_row.is_some() {
                    xml.push_str("</row>");
                }
                xml.push_str(r#"<row r=""#);
                xml.push_str(row);
                xml.push_str(r#"">"#);
                current_row = Some(row.to_owned());
            }

            match &cell.value {
                XlsxCellValue::Text(value) => {
                    xml.push_str(r#"<c r=""#);
                    xml.push_str(&cell.reference);
                    xml.push_str(r#"" t="inlineStr"><is><t xml:space="preserve">"#);
                    xml.push_str(&escape(value.as_str()));
                    xml.push_str("</t></is></c>");
                }
                XlsxCellValue::Number(value) => {
                    xml.push_str(r#"<c r=""#);
                    xml.push_str(&cell.reference);
                    xml.push_str(r#""><v>"#);
                    xml.push_str(&value.to_string());
                    xml.push_str("</v></c>");
                }
                XlsxCellValue::Boolean(value) => {
                    xml.push_str(r#"<c r=""#);
                    xml.push_str(&cell.reference);
                    xml.push_str(r#"" t="b"><v>"#);
                    xml.push(if *value { '1' } else { '0' });
                    xml.push_str("</v></c>");
                }
            }
        }

        if current_row.is_some() {
            xml.push_str("</row>");
        }
        xml.push_str("</sheetData>");

        for rule in &worksheet.conditional_formats {
            xml.push_str(r#"<conditionalFormatting sqref=""#);
            xml.push_str(&escape(rule.range_reference.as_str()));
            xml.push_str(r#""><cfRule "#);
            match &rule.condition {
                XlsxConditionalFormatCondition::NumberGreaterThan(value) => {
                    xml.push_str(&format!(
                        r#"type="cellIs" dxfId="{}" priority="{}" operator="greaterThan"><formula>{value}</formula>"#,
                        rule.differential_style_id, rule.priority
                    ));
                }
                XlsxConditionalFormatCondition::NumberLessThan(value) => {
                    xml.push_str(&format!(
                        r#"type="cellIs" dxfId="{}" priority="{}" operator="lessThan"><formula>{value}</formula>"#,
                        rule.differential_style_id, rule.priority
                    ));
                }
                XlsxConditionalFormatCondition::NumberEquals(value) => {
                    xml.push_str(&format!(
                        r#"type="cellIs" dxfId="{}" priority="{}" operator="equal"><formula>{value}</formula>"#,
                        rule.differential_style_id, rule.priority
                    ));
                }
                XlsxConditionalFormatCondition::TextContains(text) => {
                    let top_left = rule
                        .range_reference
                        .split(':')
                        .next()
                        .unwrap_or(rule.range_reference.as_str());
                    let excel_text = text.replace('"', "\"\"");
                    xml.push_str(&format!(
                        r#"type="containsText" dxfId="{}" priority="{}" operator="containsText" text=""#,
                        rule.differential_style_id, rule.priority
                    ));
                    xml.push_str(&escape(text.as_str()));
                    xml.push_str("\"><formula>NOT(ISERROR(SEARCH(\"");
                    xml.push_str(&escape(excel_text.as_str()));
                    xml.push_str("\",");
                    xml.push_str(&escape(top_left));
                    xml.push_str(")))</formula>");
                }
            }
            xml.push_str("</cfRule></conditionalFormatting>");
        }

        if !worksheet.table_relationship_ids.is_empty() {
            xml.push_str(&format!(
                r#"<tableParts count="{}">"#,
                worksheet.table_relationship_ids.len()
            ));
            for relationship_id in &worksheet.table_relationship_ids {
                xml.push_str(r#"<tablePart r:id=""#);
                xml.push_str(&escape(relationship_id.as_str()));
                xml.push_str(r#""/>"#);
            }
            xml.push_str("</tableParts>");
        }

        xml.push_str("</worksheet>");
        xml
    }

    pub fn worksheet_relationships_xml(table_indices: &[usize]) -> String {
        let mut xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="{XLSX_PACKAGE_RELATIONSHIPS_NAMESPACE}">"#
        );
        for (offset, table_index) in table_indices.iter().enumerate() {
            let relationship_id = offset + 1;
            xml.push_str(&format!(
                r#"<Relationship Id="rId{relationship_id}" Type="{XLSX_TABLE_RELATIONSHIP_TYPE}" Target="../tables/table{table_index}.xml"/>"#
            ));
        }
        xml.push_str("</Relationships>");
        xml
    }

    pub fn table_xml(table_index: usize, table: &XlsxTableModel) -> String {
        let mut xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><table xmlns="{XLSX_SPREADSHEET_NAMESPACE}" id="{table_index}" name=""#
        );
        xml.push_str(&escape(table.name.as_str()));
        xml.push_str(r#"" displayName=""#);
        xml.push_str(&escape(table.name.as_str()));
        xml.push_str(r#"" ref=""#);
        xml.push_str(&escape(table.range_reference.as_str()));
        xml.push_str(r#"" totalsRowShown="0"><autoFilter ref=""#);
        xml.push_str(&escape(table.range_reference.as_str()));
        xml.push_str(r#""/><tableColumns count=""#);
        xml.push_str(&table.column_names.len().to_string());
        xml.push_str(r#"">"#);
        for (index, name) in table.column_names.iter().enumerate() {
            let column_id = index + 1;
            xml.push_str(&format!(r#"<tableColumn id="{column_id}" name=""#));
            xml.push_str(&escape(name.as_str()));
            xml.push_str(r#""/>"#);
        }
        xml.push_str(
            r#"</tableColumns><tableStyleInfo name="TableStyleMedium2" showFirstColumn="0" showLastColumn="0" showRowStripes="1" showColumnStripes="0"/></table>"#,
        );
        xml
    }

    pub fn styles_xml() -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><styleSheet xmlns="{XLSX_SPREADSHEET_NAMESPACE}"><fonts count="1"><font><sz val="11"/><name val="Calibri"/></font></fonts><fills count="2"><fill><patternFill patternType="none"/></fill><fill><patternFill patternType="gray125"/></fill></fills><borders count="1"><border><left/><right/><top/><bottom/><diagonal/></border></borders><cellStyleXfs count="1"><xf numFmtId="0" fontId="0" fillId="0" borderId="0"/></cellStyleXfs><cellXfs count="1"><xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"/></cellXfs><cellStyles count="1"><cellStyle name="Normal" xfId="0" builtinId="0"/></cellStyles><dxfs count="3"><dxf><fill><patternFill patternType="solid"><fgColor rgb="{XLSX_CONDITIONAL_WARNING_RGB}"/><bgColor indexed="64"/></patternFill></fill></dxf><dxf><fill><patternFill patternType="solid"><fgColor rgb="{XLSX_CONDITIONAL_SUCCESS_RGB}"/><bgColor indexed="64"/></patternFill></fill></dxf><dxf><fill><patternFill patternType="solid"><fgColor rgb="{XLSX_CONDITIONAL_ACCENT_RGB}"/><bgColor indexed="64"/></patternFill></fill></dxf></dxfs></styleSheet>"#
        )
    }

    fn reader(xml: &[u8]) -> Result<Reader<&[u8]>, SheetXlsxXmlError> {
        if xml.len() > MAX_XLSX_XML_BYTES {
            return Err(SheetXlsxXmlError::DocumentTooLarge);
        }
        let text = std::str::from_utf8(xml).map_err(|_| SheetXlsxXmlError::InvalidUtf8)?;
        let mut reader = Reader::from_str(text);
        reader.config_mut().trim_text_start = false;
        reader.config_mut().trim_text_end = false;
        reader.config_mut().check_comments = true;
        Ok(reader)
    }

    fn sheet_descriptor(
        element: &BytesStart<'_>,
    ) -> Result<XlsxSheetDescriptor, SheetXlsxXmlError> {
        Ok(XlsxSheetDescriptor {
            name: Self::attribute(element, ATTR_NAME)?
                .ok_or(SheetXlsxXmlError::MissingAttribute)?,
            relationship_id: Self::attribute(element, ATTR_SHEET_RELATIONSHIP_ID)?
                .ok_or(SheetXlsxXmlError::MissingAttribute)?,
        })
    }

    fn is_office_document_relationship(
        element: &BytesStart<'_>,
    ) -> Result<bool, SheetXlsxXmlError> {
        if Self::local_name(element.name().as_ref()) != TAG_RELATIONSHIP {
            return Ok(false);
        }
        let relation_type = Self::attribute(element, ATTR_TYPE)?;
        let target = Self::attribute(element, ATTR_TARGET)?;
        Ok(
            relation_type.as_deref() == Some(XLSX_OFFICE_DOCUMENT_RELATIONSHIP_TYPE)
                && target
                    .as_deref()
                    .is_some_and(|value| value.trim_start_matches('/') == "xl/workbook.xml"),
        )
    }

    fn conditional_rule_state(
        element: &BytesStart<'_>,
        range_reference: String,
    ) -> Result<ConditionalRuleState, SheetXlsxXmlError> {
        let rule_type =
            Self::attribute(element, ATTR_RULE_TYPE)?.ok_or(SheetXlsxXmlError::MissingAttribute)?;
        let differential_style_id = Self::attribute(element, ATTR_DXF_ID)?
            .ok_or(SheetXlsxXmlError::MissingAttribute)?
            .parse::<usize>()
            .map_err(|_| SheetXlsxXmlError::InvalidNumber)?;
        let priority = Self::attribute(element, ATTR_PRIORITY)?
            .ok_or(SheetXlsxXmlError::MissingAttribute)?
            .parse::<u32>()
            .map_err(|_| SheetXlsxXmlError::InvalidNumber)?;
        if priority == 0 {
            return Err(SheetXlsxXmlError::UnsupportedConditionalFormat);
        }

        Ok(ConditionalRuleState {
            range_reference,
            rule_type,
            operator: Self::attribute(element, ATTR_OPERATOR)?,
            differential_style_id,
            priority,
            text_value: Self::attribute(element, ATTR_TEXT_VALUE)?,
            formula: String::new(),
        })
    }

    fn finish_conditional_rule(
        state: ConditionalRuleState,
    ) -> Result<XlsxConditionalFormatModel, SheetXlsxXmlError> {
        let condition = if state.rule_type.eq_ignore_ascii_case("cellIs") {
            let operator = state
                .operator
                .as_deref()
                .ok_or(SheetXlsxXmlError::MissingAttribute)?;
            let value = state
                .formula
                .trim()
                .parse::<f64>()
                .map_err(|_| SheetXlsxXmlError::UnsupportedConditionalFormat)?;
            if !value.is_finite() {
                return Err(SheetXlsxXmlError::UnsupportedConditionalFormat);
            }
            match operator {
                "greaterThan" => XlsxConditionalFormatCondition::NumberGreaterThan(value),
                "lessThan" => XlsxConditionalFormatCondition::NumberLessThan(value),
                "equal" => XlsxConditionalFormatCondition::NumberEquals(value),
                _ => return Err(SheetXlsxXmlError::UnsupportedConditionalFormat),
            }
        } else if state.rule_type.eq_ignore_ascii_case("containsText") {
            let text = state
                .text_value
                .filter(|value| !value.is_empty())
                .ok_or(SheetXlsxXmlError::UnsupportedConditionalFormat)?;
            XlsxConditionalFormatCondition::TextContains(text)
        } else {
            return Err(SheetXlsxXmlError::UnsupportedConditionalFormat);
        };

        Ok(XlsxConditionalFormatModel {
            range_reference: state.range_reference,
            condition,
            differential_style_id: state.differential_style_id,
            priority: state.priority,
        })
    }

    fn push_table_relationship_id(
        element: &BytesStart<'_>,
        output: &mut Vec<String>,
    ) -> Result<(), SheetXlsxXmlError> {
        let id = Self::attribute(element, ATTR_SHEET_RELATIONSHIP_ID)?
            .ok_or(SheetXlsxXmlError::MissingAttribute)?;
        if output.len() >= MAX_XLSX_TABLES {
            return Err(SheetXlsxXmlError::TooManyNodes);
        }
        output.push(id);
        Ok(())
    }

    fn parse_relationships_of_type(
        xml: &[u8],
        relationship_type: &str,
    ) -> Result<HashMap<String, String>, SheetXlsxXmlError> {
        let mut reader = Self::reader(xml)?;
        let mut relationships = HashMap::new();
        let mut depth = 0_usize;
        let mut nodes = 0_usize;

        loop {
            match reader
                .read_event()
                .map_err(|_| SheetXlsxXmlError::InvalidXml)?
            {
                Event::Start(start) => {
                    depth = depth.saturating_add(1);
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    Self::read_relationship_of_type(&start, relationship_type, &mut relationships)?;
                }
                Event::Empty(empty) => {
                    nodes = nodes.saturating_add(1);
                    Self::validate_limits(depth, nodes)?;
                    Self::read_relationship_of_type(&empty, relationship_type, &mut relationships)?;
                }
                Event::End(_) => depth = depth.saturating_sub(1),
                Event::DocType(_) => return Err(SheetXlsxXmlError::DocTypeUnsupported),
                Event::Eof => break,
                _ => {}
            }
        }

        Ok(relationships)
    }

    fn read_relationship_of_type(
        element: &BytesStart<'_>,
        relationship_type: &str,
        relationships: &mut HashMap<String, String>,
    ) -> Result<(), SheetXlsxXmlError> {
        if Self::local_name(element.name().as_ref()) != TAG_RELATIONSHIP {
            return Ok(());
        }
        let relation_type = Self::attribute(element, ATTR_TYPE)?;
        if relation_type.as_deref() != Some(relationship_type) {
            return Ok(());
        }
        let id = Self::attribute(element, ATTR_PACKAGE_RELATIONSHIP_ID)?
            .ok_or(SheetXlsxXmlError::MissingAttribute)?;
        let target =
            Self::attribute(element, ATTR_TARGET)?.ok_or(SheetXlsxXmlError::MissingAttribute)?;
        if relationships.insert(id, target).is_some() {
            return Err(SheetXlsxXmlError::InvalidXml);
        }
        Ok(())
    }

    fn read_relationship(
        element: &BytesStart<'_>,
        relationships: &mut HashMap<String, String>,
    ) -> Result<(), SheetXlsxXmlError> {
        Self::read_relationship_of_type(element, XLSX_WORKSHEET_RELATIONSHIP_TYPE, relationships)
    }

    fn cell_value(
        cell: &CellState,
        shared_strings: &[String],
    ) -> Result<Option<XlsxCellValue>, SheetXlsxXmlError> {
        match cell.cell_type.as_deref() {
            Some("inlineStr") => Ok(Some(XlsxCellValue::Text(cell.inline_text.clone()))),
            Some("s") => {
                if cell.value.is_empty() {
                    return Ok(None);
                }
                let index = cell
                    .value
                    .parse::<usize>()
                    .map_err(|_| SheetXlsxXmlError::InvalidNumber)?;
                let value = shared_strings
                    .get(index)
                    .ok_or(SheetXlsxXmlError::SharedStringOutOfBounds)?;
                Ok(Some(XlsxCellValue::Text(value.clone())))
            }
            Some("b") => match cell.value.as_str() {
                "" => Ok(None),
                "0" | "false" => Ok(Some(XlsxCellValue::Boolean(false))),
                "1" | "true" => Ok(Some(XlsxCellValue::Boolean(true))),
                _ => Err(SheetXlsxXmlError::InvalidNumber),
            },
            Some("str") => Ok(Some(XlsxCellValue::Text(cell.value.clone()))),
            Some("n") | None => {
                if cell.value.is_empty() {
                    return Ok(None);
                }
                let value = cell
                    .value
                    .parse::<f64>()
                    .map_err(|_| SheetXlsxXmlError::InvalidNumber)?;
                if !value.is_finite() {
                    return Err(SheetXlsxXmlError::InvalidNumber);
                }
                Ok(Some(XlsxCellValue::Number(value)))
            }
            Some(_) => Err(SheetXlsxXmlError::UnsupportedCellType),
        }
    }

    fn append_target(cell: &mut CellState, target: TextTarget, value: &str) {
        match target {
            TextTarget::Value => cell.value.push_str(value),
            TextTarget::Inline => cell.inline_text.push_str(value),
            TextTarget::None | TextTarget::Shared => {}
        }
    }

    fn append_reference(
        output: &mut String,
        raw: &str,
        reference: &quick_xml::events::BytesRef<'_>,
    ) -> Result<(), SheetXlsxXmlError> {
        if let Some(character) = reference
            .resolve_char_ref()
            .map_err(|_| SheetXlsxXmlError::InvalidXml)?
        {
            output.push(character);
            return Ok(());
        }
        if let Some(value) = resolve_predefined_entity(raw) {
            output.push_str(value);
            return Ok(());
        }
        Err(SheetXlsxXmlError::InvalidXml)
    }

    fn attribute(element: &BytesStart<'_>, key: &str) -> Result<Option<String>, SheetXlsxXmlError> {
        for attribute in element.attributes() {
            let attribute = attribute.map_err(|_| SheetXlsxXmlError::InvalidAttribute)?;
            if Self::local_name(attribute.key.as_ref()) != key {
                continue;
            }
            let value = unescape(attribute.value.as_ref())
                .map_err(|_| SheetXlsxXmlError::InvalidAttribute)?;
            return Ok(Some(value.into_owned()));
        }
        Ok(None)
    }

    fn reference_row(reference: &str) -> &str {
        reference
            .find(|character: char| character.is_ascii_digit())
            .map(|index| &reference[index..])
            .unwrap_or("1")
    }

    fn validate_limits(depth: usize, nodes: usize) -> Result<(), SheetXlsxXmlError> {
        if depth > MAX_XLSX_XML_DEPTH {
            return Err(SheetXlsxXmlError::TooDeep);
        }
        if nodes > MAX_XLSX_XML_NODES {
            return Err(SheetXlsxXmlError::TooManyNodes);
        }
        Ok(())
    }

    fn local_name(name: &str) -> &str {
        name.rsplit(':').next().unwrap_or(name)
    }
}
