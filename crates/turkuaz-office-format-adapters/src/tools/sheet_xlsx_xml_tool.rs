// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/tools/sheet_xlsx_xml_tool.rs
// # 📌 Amac: XLSX SpreadsheetML workbook/relationship/worksheet/shared-string XML parcalarini streaming parse eder ve yazar
// # 📌 Modul - FileType: Tool - Rust
// Version: 0.3.0
// Aciklama: Value-only Sheet minimumu icin inline/shared text, finite number ve boolean XML semantigini format modelinde tutar
// Bagimli Oldugu Katman: Tool -> Model -> Config

use std::collections::HashMap;

use quick_xml::escape::{escape, resolve_predefined_entity, unescape};
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::XmlVersion;

use crate::config::sheet_constants::{
    MAX_XLSX_XML_BYTES, MAX_XLSX_XML_DEPTH, MAX_XLSX_XML_NODES,
    XLSX_CONTENT_TYPES_NAMESPACE, XLSX_OFFICE_DOCUMENT_RELATIONSHIP_TYPE,
    XLSX_OFFICE_RELATIONSHIPS_NAMESPACE, XLSX_PACKAGE_RELATIONSHIPS_NAMESPACE,
    XLSX_SPREADSHEET_NAMESPACE, XLSX_WORKBOOK_CONTENT_TYPE,
    XLSX_WORKSHEET_CONTENT_TYPE, XLSX_WORKSHEET_RELATIONSHIP_TYPE,
};
use crate::models::sheet_xlsx_model::{
    XlsxCellModel, XlsxCellValue, XlsxSheetDescriptor, XlsxWorksheetModel,
};

const TAG_SHEET: &[u8] = b"sheet";
const TAG_RELATIONSHIP: &[u8] = b"Relationship";
const TAG_SHARED_ITEM: &[u8] = b"si";
const TAG_CELL: &[u8] = b"c";
const TAG_VALUE: &[u8] = b"v";
const TAG_TEXT: &[u8] = b"t";
const TAG_FORMULA: &[u8] = b"f";

const ATTR_NAME: &[u8] = b"name";
const ATTR_ID: &[u8] = b"id";
const ATTR_TARGET: &[u8] = b"Target";
const ATTR_TYPE: &[u8] = b"Type";
const ATTR_REFERENCE: &[u8] = b"r";
const ATTR_CELL_TYPE: &[u8] = b"t";

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
    DocTypeUnsupported,
}

#[derive(Clone, Debug)]
struct CellState {
    reference: String,
    cell_type: Option<String>,
    value: String,
    inline_text: String,
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
            match reader.read_event().map_err(|_| SheetXlsxXmlError::InvalidXml)? {
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

    pub fn parse_workbook_relationships(
        xml: &[u8],
    ) -> Result<HashMap<String, String>, SheetXlsxXmlError> {
        let mut reader = Self::reader(xml)?;
        let mut relationships = HashMap::new();
        let mut depth = 0_usize;
        let mut nodes = 0_usize;

        loop {
            match reader.read_event().map_err(|_| SheetXlsxXmlError::InvalidXml)? {
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
            match reader.read_event().map_err(|_| SheetXlsxXmlError::InvalidXml)? {
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
            match reader.read_event().map_err(|_| SheetXlsxXmlError::InvalidXml)? {
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
                            if let Some(cell) = current.take() {
                                if let Some(value) = Self::cell_value(&cell, shared_strings)? {
                                    cells.push(XlsxCellModel {
                                        reference: cell.reference,
                                        value,
                                    });
                                }
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

        Ok(XlsxWorksheetModel { name, cells })
    }

    pub fn content_types_xml(sheet_count: usize) -> String {
        let mut xml = format!(
            "<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Types xmlns="{XLSX_CONTENT_TYPES_NAMESPACE}"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="{XLSX_WORKBOOK_CONTENT_TYPE}"/>"
        );
        for index in 1..=sheet_count {
            xml.push_str(&format!(
                "<Override PartName="/xl/worksheets/sheet{index}.xml" ContentType="{XLSX_WORKSHEET_CONTENT_TYPE}"/>"
            ));
        }
        xml.push_str("</Types>");
        xml
    }

    pub fn root_relationships_xml() -> String {
        format!(
            "<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="{XLSX_PACKAGE_RELATIONSHIPS_NAMESPACE}"><Relationship Id="rId1" Type="{XLSX_OFFICE_DOCUMENT_RELATIONSHIP_TYPE}" Target="xl/workbook.xml"/></Relationships>"
        )
    }

    pub fn workbook_xml(worksheets: &[XlsxWorksheetModel]) -> String {
        let mut xml = format!(
            "<?xml version="1.0" encoding="UTF-8" standalone="yes"?><workbook xmlns="{XLSX_SPREADSHEET_NAMESPACE}" xmlns:r="{XLSX_OFFICE_RELATIONSHIPS_NAMESPACE}"><sheets>"
        );
        for (index, worksheet) in worksheets.iter().enumerate() {
            let sheet_id = index + 1;
            xml.push_str("<sheet name="");
            xml.push_str(&escape(worksheet.name.as_str()));
            xml.push_str(&format!("" sheetId="{sheet_id}" r:id="rId{sheet_id}"/>"));
        }
        xml.push_str("</sheets></workbook>");
        xml
    }

    pub fn workbook_relationships_xml(sheet_count: usize) -> String {
        let mut xml = format!(
            "<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="{XLSX_PACKAGE_RELATIONSHIPS_NAMESPACE}">"
        );
        for index in 1..=sheet_count {
            xml.push_str(&format!(
                "<Relationship Id="rId{index}" Type="{XLSX_WORKSHEET_RELATIONSHIP_TYPE}" Target="worksheets/sheet{index}.xml"/>"
            ));
        }
        xml.push_str("</Relationships>");
        xml
    }

    pub fn worksheet_xml(worksheet: &XlsxWorksheetModel) -> String {
        let mut xml = format!(
            "<?xml version="1.0" encoding="UTF-8" standalone="yes"?><worksheet xmlns="{XLSX_SPREADSHEET_NAMESPACE}"><sheetData>"
        );
        let mut current_row: Option<String> = None;

        for cell in &worksheet.cells {
            let row = Self::reference_row(&cell.reference);
            if current_row.as_deref() != Some(row) {
                if current_row.is_some() {
                    xml.push_str("</row>");
                }
                xml.push_str("<row r="");
                xml.push_str(row);
                xml.push_str("">");
                current_row = Some(row.to_owned());
            }

            match &cell.value {
                XlsxCellValue::Text(value) => {
                    xml.push_str("<c r="");
                    xml.push_str(&cell.reference);
                    xml.push_str("" t="inlineStr"><is><t xml:space="preserve">");
                    xml.push_str(&escape(value.as_str()));
                    xml.push_str("</t></is></c>");
                }
                XlsxCellValue::Number(value) => {
                    xml.push_str("<c r="");
                    xml.push_str(&cell.reference);
                    xml.push_str(""><v>");
                    xml.push_str(&value.to_string());
                    xml.push_str("</v></c>");
                }
                XlsxCellValue::Boolean(value) => {
                    xml.push_str("<c r="");
                    xml.push_str(&cell.reference);
                    xml.push_str("" t="b"><v>");
                    xml.push(if *value { '1' } else { '0' });
                    xml.push_str("</v></c>");
                }
            }
        }

        if current_row.is_some() {
            xml.push_str("</row>");
        }
        xml.push_str("</sheetData></worksheet>");
        xml
    }

    fn reader(xml: &[u8]) -> Result<Reader<&str>, SheetXlsxXmlError> {
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

    fn sheet_descriptor(element: &BytesStart<'_>) -> Result<XlsxSheetDescriptor, SheetXlsxXmlError> {
        Ok(XlsxSheetDescriptor {
            name: Self::attribute(element, ATTR_NAME)?
                .ok_or(SheetXlsxXmlError::MissingAttribute)?,
            relationship_id: Self::attribute(element, ATTR_ID)?
                .ok_or(SheetXlsxXmlError::MissingAttribute)?,
        })
    }

    fn read_relationship(
        element: &BytesStart<'_>,
        relationships: &mut HashMap<String, String>,
    ) -> Result<(), SheetXlsxXmlError> {
        if Self::local_name(element.name().as_ref()) != TAG_RELATIONSHIP {
            return Ok(());
        }
        let relation_type = Self::attribute(element, ATTR_TYPE)?;
        if relation_type.as_deref() != Some(XLSX_WORKSHEET_RELATIONSHIP_TYPE) {
            return Ok(());
        }
        let id = Self::attribute(element, ATTR_ID)?
            .ok_or(SheetXlsxXmlError::MissingAttribute)?;
        let target = Self::attribute(element, ATTR_TARGET)?
            .ok_or(SheetXlsxXmlError::MissingAttribute)?;
        relationships.insert(id, target);
        Ok(())
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

    fn attribute(
        element: &BytesStart<'_>,
        key: &[u8],
    ) -> Result<Option<String>, SheetXlsxXmlError> {
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

    fn local_name(name: &[u8]) -> &[u8] {
        name.rsplit(|byte| *byte == b':').next().unwrap_or(name)
    }
}
