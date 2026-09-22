// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/tools/docx_xml_tool.rs
// # 📌 Amac: DOCX WordprocessingML minimum profilini streaming olarak parse eder ve yazar
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Paragraph/run typography, alignment ve page settings XML semantigini format-specific modelde tutar
// Bagimli Oldugu Katman: Tool -> Model -> Config

use std::collections::BTreeSet;
use std::str;

use quick_xml::escape::{escape, unescape};
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::XmlVersion;

use crate::config::constants::{
    DOCX_CONTENT_TYPES_NAMESPACE, DOCX_DOCUMENT_CONTENT_TYPE,
    DOCX_OFFICE_DOCUMENT_RELATIONSHIP_TYPE, DOCX_PACKAGE_RELATIONSHIPS_NAMESPACE,
    DOCX_WORD_NAMESPACE, MAX_DOCX_DOCUMENT_XML_BYTES, MAX_DOCX_XML_DEPTH,
    MAX_DOCX_XML_NODES,
};
use crate::models::docx_model::{
    DocxAlignment, DocxCompatibilityReport, DocxDocumentModel, DocxPageSettingsModel,
    DocxParagraphModel, DocxRunModel, DocxUnsupportedFeature,
};

const TAG_PARAGRAPH: &[u8] = b"p";
const TAG_RUN: &[u8] = b"r";
const TAG_TEXT: &[u8] = b"t";
const TAG_BOLD: &[u8] = b"b";
const TAG_ITALIC: &[u8] = b"i";
const TAG_UNDERLINE: &[u8] = b"u";
const TAG_RUN_FONTS: &[u8] = b"rFonts";
const TAG_FONT_SIZE: &[u8] = b"sz";
const TAG_ALIGNMENT: &[u8] = b"jc";
const TAG_PAGE_SIZE: &[u8] = b"pgSz";
const TAG_PAGE_MARGIN: &[u8] = b"pgMar";
const TAG_TAB: &[u8] = b"tab";
const TAG_BREAK: &[u8] = b"br";
const TAG_CARRIAGE_RETURN: &[u8] = b"cr";
const TAG_TABLE: &[u8] = b"tbl";
const TAG_DRAWING: &[u8] = b"drawing";
const TAG_PICTURE: &[u8] = b"pict";
const TAG_OBJECT: &[u8] = b"object";
const TAG_NUMBERING: &[u8] = b"numPr";
const TAG_HYPERLINK: &[u8] = b"hyperlink";
const TAG_HEADER_REFERENCE: &[u8] = b"headerReference";
const TAG_FOOTER_REFERENCE: &[u8] = b"footerReference";
const TAG_COMMENT_REFERENCE: &[u8] = b"commentReference";
const TAG_INSERTED: &[u8] = b"ins";
const TAG_DELETED: &[u8] = b"del";
const TAG_FIELD_SIMPLE: &[u8] = b"fldSimple";
const TAG_FIELD_CHAR: &[u8] = b"fldChar";
const TAG_INSTRUCTION_TEXT: &[u8] = b"instrText";

const ATTR_VALUE: &[u8] = b"val";
const ATTR_ASCII_FONT: &[u8] = b"ascii";
const ATTR_HANSI_FONT: &[u8] = b"hAnsi";
const ATTR_WIDTH: &[u8] = b"w";
const ATTR_HEIGHT: &[u8] = b"h";
const ATTR_TOP: &[u8] = b"top";
const ATTR_RIGHT: &[u8] = b"right";
const ATTR_BOTTOM: &[u8] = b"bottom";
const ATTR_LEFT: &[u8] = b"left";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocxXmlError {
    DocumentTooLarge,
    TooDeep,
    TooManyNodes,
    InvalidXml,
    InvalidUtf8,
    InvalidAttribute,
    InvalidNumber,
    DocTypeUnsupported,
}

pub struct DocxXmlTool;

impl DocxXmlTool {
    pub fn parse_document(xml: &[u8]) -> Result<DocxDocumentModel, DocxXmlError> {
        if xml.len() > MAX_DOCX_DOCUMENT_XML_BYTES {
            return Err(DocxXmlError::DocumentTooLarge);
        }
        let xml = str::from_utf8(xml).map_err(|_| DocxXmlError::InvalidUtf8)?;
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text_start = false;
        reader.config_mut().trim_text_end = false;
        reader.config_mut().check_comments = true;

        let mut model = DocxDocumentModel::default();
        let mut unsupported = BTreeSet::new();
        let mut paragraph: Option<DocxParagraphModel> = None;
        let mut run: Option<DocxRunModel> = None;
        let mut in_text = false;
        let mut depth = 0_usize;
        let mut node_count = 0_usize;

        loop {
            let event = reader
                .read_event()
                .map_err(|_| DocxXmlError::InvalidXml)?;
            match event {
                Event::Start(start) => {
                    depth = depth.saturating_add(1);
                    node_count = node_count.saturating_add(1);
                    Self::validate_limits(depth, node_count)?;
                    let name = start.name();
                    let local = Self::local_name(name.as_ref());
                    Self::mark_unsupported(local, &mut unsupported);
                    match local {
                        TAG_PARAGRAPH => paragraph = Some(DocxParagraphModel::default()),
                        TAG_RUN => run = Some(DocxRunModel::default()),
                        TAG_TEXT => in_text = true,
                        _ => Self::apply_properties(
                            &start,
                            local,
                            paragraph.as_mut(),
                            run.as_mut(),
                            &mut model.page_settings,
                        )?,
                    }
                }
                Event::Empty(empty) => {
                    node_count = node_count.saturating_add(1);
                    Self::validate_limits(depth, node_count)?;
                    let name = empty.name();
                    let local = Self::local_name(name.as_ref());
                    Self::mark_unsupported(local, &mut unsupported);
                    match local {
                        TAG_TAB => {
                            if let Some(current) = run.as_mut() {
                                current.text.push('\t');
                            }
                        }
                        TAG_BREAK | TAG_CARRIAGE_RETURN => {
                            if let Some(current) = run.as_mut() {
                                current.text.push('\n');
                            }
                        }
                        _ => Self::apply_properties(
                            &empty,
                            local,
                            paragraph.as_mut(),
                            run.as_mut(),
                            &mut model.page_settings,
                        )?,
                    }
                }
                Event::Text(text) => {
                    if in_text {
                        if let Some(current) = run.as_mut() {
                            let value = text.xml_content(XmlVersion::Implicit1_0);
                            current.text.push_str(&value);
                        }
                    }
                }
                Event::End(end) => {
                    let name = end.name();
                    let local = Self::local_name(name.as_ref());
                    match local {
                        TAG_TEXT => in_text = false,
                        TAG_RUN => {
                            if let (Some(current_paragraph), Some(current_run)) =
                                (paragraph.as_mut(), run.take())
                            {
                                if !current_run.text.is_empty() {
                                    current_paragraph.runs.push(current_run);
                                }
                            }
                        }
                        TAG_PARAGRAPH => {
                            if let Some(current) = paragraph.take() {
                                model.paragraphs.push(current);
                            }
                        }
                        _ => {}
                    }
                    depth = depth.saturating_sub(1);
                }
                Event::DocType(_) => return Err(DocxXmlError::DocTypeUnsupported),
                Event::Eof => break,
                _ => {}
            }
        }

        model.compatibility = DocxCompatibilityReport {
            unsupported_features: unsupported.into_iter().collect(),
        };
        Ok(model)
    }

    pub fn encode_document(model: &DocxDocumentModel) -> String {
        let mut xml = String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>",
        );
        xml.push_str("<w:document xmlns:w=\"");
        xml.push_str(DOCX_WORD_NAMESPACE);
        xml.push_str("\"><w:body>");

        for paragraph in &model.paragraphs {
            xml.push_str("<w:p>");
            if let Some(alignment) = paragraph.alignment {
                xml.push_str("<w:pPr><w:jc w:val=\"");
                xml.push_str(Self::alignment_value(alignment));
                xml.push_str("\"/></w:pPr>");
            }
            for run in &paragraph.runs {
                Self::write_run(&mut xml, run);
            }
            xml.push_str("</w:p>");
        }

        let page = &model.page_settings;
        xml.push_str("<w:sectPr>");
        if let (Some(width), Some(height)) = (page.width_twips, page.height_twips) {
            xml.push_str(&format!("<w:pgSz w:w=\"{width}\" w:h=\"{height}\"/>"));
        }
        if let (Some(top), Some(right), Some(bottom), Some(left)) = (
            page.margin_top_twips,
            page.margin_right_twips,
            page.margin_bottom_twips,
            page.margin_left_twips,
        ) {
            xml.push_str(&format!(
                "<w:pgMar w:top=\"{top}\" w:right=\"{right}\" w:bottom=\"{bottom}\" w:left=\"{left}\"/>"
            ));
        }
        xml.push_str("</w:sectPr></w:body></w:document>");
        xml
    }

    pub fn content_types_xml() -> String {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><Types xmlns=\"{DOCX_CONTENT_TYPES_NAMESPACE}\"><Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/><Default Extension=\"xml\" ContentType=\"application/xml\"/><Override PartName=\"/word/document.xml\" ContentType=\"{DOCX_DOCUMENT_CONTENT_TYPE}\"/></Types>"
        )
    }

    pub fn root_relationships_xml() -> String {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><Relationships xmlns=\"{DOCX_PACKAGE_RELATIONSHIPS_NAMESPACE}\"><Relationship Id=\"rId1\" Type=\"{DOCX_OFFICE_DOCUMENT_RELATIONSHIP_TYPE}\" Target=\"word/document.xml\"/></Relationships>"
        )
    }

    fn apply_properties(
        element: &BytesStart<'_>,
        local: &[u8],
        paragraph: Option<&mut DocxParagraphModel>,
        run: Option<&mut DocxRunModel>,
        page: &mut DocxPageSettingsModel,
    ) -> Result<(), DocxXmlError> {
        match local {
            TAG_BOLD => {
                if let Some(current) = run {
                    current.bold = Self::on_off(Self::attribute(element, ATTR_VALUE)?);
                }
            }
            TAG_ITALIC => {
                if let Some(current) = run {
                    current.italic = Self::on_off(Self::attribute(element, ATTR_VALUE)?);
                }
            }
            TAG_UNDERLINE => {
                if let Some(current) = run {
                    current.underline = Self::underline_enabled(Self::attribute(
                        element,
                        ATTR_VALUE,
                    )?);
                }
            }
            TAG_RUN_FONTS => {
                if let Some(current) = run {
                    current.font_family = Self::attribute(element, ATTR_ASCII_FONT)?
                        .or(Self::attribute(element, ATTR_HANSI_FONT)?);
                }
            }
            TAG_FONT_SIZE => {
                if let Some(current) = run {
                    current.font_size_half_points =
                        Self::parse_u16(Self::attribute(element, ATTR_VALUE)?)?;
                }
            }
            TAG_ALIGNMENT => {
                if let Some(current) = paragraph {
                    current.alignment =
                        Self::parse_alignment(Self::attribute(element, ATTR_VALUE)?);
                }
            }
            TAG_PAGE_SIZE => {
                page.width_twips =
                    Self::parse_u32(Self::attribute(element, ATTR_WIDTH)?)?;
                page.height_twips =
                    Self::parse_u32(Self::attribute(element, ATTR_HEIGHT)?)?;
            }
            TAG_PAGE_MARGIN => {
                page.margin_top_twips =
                    Self::parse_u32(Self::attribute(element, ATTR_TOP)?)?;
                page.margin_right_twips =
                    Self::parse_u32(Self::attribute(element, ATTR_RIGHT)?)?;
                page.margin_bottom_twips =
                    Self::parse_u32(Self::attribute(element, ATTR_BOTTOM)?)?;
                page.margin_left_twips =
                    Self::parse_u32(Self::attribute(element, ATTR_LEFT)?)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn write_run(xml: &mut String, run: &DocxRunModel) {
        xml.push_str("<w:r>");
        if run.bold
            || run.italic
            || run.underline
            || run.font_family.is_some()
            || run.font_size_half_points.is_some()
        {
            xml.push_str("<w:rPr>");
            if run.bold {
                xml.push_str("<w:b/>");
            }
            if run.italic {
                xml.push_str("<w:i/>");
            }
            if run.underline {
                xml.push_str("<w:u w:val=\"single\"/>");
            }
            if let Some(font_family) = &run.font_family {
                let family = escape(font_family);
                xml.push_str("<w:rFonts w:ascii=\"");
                xml.push_str(&family);
                xml.push_str("\" w:hAnsi=\"");
                xml.push_str(&family);
                xml.push_str("\"/>");
            }
            if let Some(size) = run.font_size_half_points {
                xml.push_str(&format!(
                    "<w:sz w:val=\"{size}\"/><w:szCs w:val=\"{size}\"/>"
                ));
            }
            xml.push_str("</w:rPr>");
        }

        Self::write_text_content(xml, &run.text);
        xml.push_str("</w:r>");
    }

    fn write_text_content(xml: &mut String, text: &str) {
        let mut buffer = String::new();
        let mut chars = text.chars().peekable();
        while let Some(character) = chars.next() {
            match character {
                '\t' => {
                    Self::flush_text(xml, &mut buffer);
                    xml.push_str("<w:tab/>");
                }
                '\r' => {
                    Self::flush_text(xml, &mut buffer);
                    if matches!(chars.peek(), Some('\n')) {
                        chars.next();
                    }
                    xml.push_str("<w:br/>");
                }
                '\n' => {
                    Self::flush_text(xml, &mut buffer);
                    xml.push_str("<w:br/>");
                }
                value => buffer.push(value),
            }
        }
        Self::flush_text(xml, &mut buffer);
        if text.is_empty() {
            xml.push_str("<w:t xml:space=\"preserve\"></w:t>");
        }
    }

    fn flush_text(xml: &mut String, buffer: &mut String) {
        if buffer.is_empty() {
            return;
        }
        let value = escape(buffer.as_str());
        xml.push_str("<w:t xml:space=\"preserve\">");
        xml.push_str(&value);
        xml.push_str("</w:t>");
        buffer.clear();
    }

    fn attribute(
        element: &BytesStart<'_>,
        key: &[u8],
    ) -> Result<Option<String>, DocxXmlError> {
        for attribute in element.attributes() {
            let attribute = attribute.map_err(|_| DocxXmlError::InvalidAttribute)?;
            let name = attribute.key;
            if Self::local_name(name.as_ref()) != key {
                continue;
            }
            let value = unescape(attribute.value.as_ref())
                .map_err(|_| DocxXmlError::InvalidAttribute)?;
            return Ok(Some(value.into_owned()));
        }
        Ok(None)
    }

    fn parse_u16(value: Option<String>) -> Result<Option<u16>, DocxXmlError> {
        value
            .map(|raw| raw.parse::<u16>().map_err(|_| DocxXmlError::InvalidNumber))
            .transpose()
    }

    fn parse_u32(value: Option<String>) -> Result<Option<u32>, DocxXmlError> {
        value
            .map(|raw| raw.parse::<u32>().map_err(|_| DocxXmlError::InvalidNumber))
            .transpose()
    }

    fn on_off(value: Option<String>) -> bool {
        !matches!(
            value.as_deref(),
            Some("0") | Some("false") | Some("off") | Some("none")
        )
    }

    fn underline_enabled(value: Option<String>) -> bool {
        !matches!(value.as_deref(), Some("none") | Some("0") | Some("false"))
    }

    fn parse_alignment(value: Option<String>) -> Option<DocxAlignment> {
        match value.as_deref() {
            Some("left" | "start") => Some(DocxAlignment::Left),
            Some("center") => Some(DocxAlignment::Center),
            Some("right" | "end") => Some(DocxAlignment::Right),
            Some("both" | "distribute") => Some(DocxAlignment::Justify),
            _ => None,
        }
    }

    fn alignment_value(value: DocxAlignment) -> &'static str {
        match value {
            DocxAlignment::Left => "left",
            DocxAlignment::Center => "center",
            DocxAlignment::Right => "right",
            DocxAlignment::Justify => "both",
        }
    }

    fn mark_unsupported(local: &[u8], features: &mut BTreeSet<DocxUnsupportedFeature>) {
        let feature = match local {
            TAG_TABLE => Some(DocxUnsupportedFeature::Table),
            TAG_DRAWING | TAG_PICTURE | TAG_OBJECT => Some(DocxUnsupportedFeature::Image),
            TAG_NUMBERING => Some(DocxUnsupportedFeature::Numbering),
            TAG_HYPERLINK => Some(DocxUnsupportedFeature::Hyperlink),
            TAG_HEADER_REFERENCE | TAG_FOOTER_REFERENCE => {
                Some(DocxUnsupportedFeature::HeaderFooter)
            }
            TAG_COMMENT_REFERENCE => Some(DocxUnsupportedFeature::Comments),
            TAG_INSERTED | TAG_DELETED => Some(DocxUnsupportedFeature::TrackedChanges),
            TAG_FIELD_SIMPLE | TAG_FIELD_CHAR | TAG_INSTRUCTION_TEXT => {
                Some(DocxUnsupportedFeature::Fields)
            }
            _ => None,
        };
        if let Some(feature) = feature {
            features.insert(feature);
        }
    }

    fn validate_limits(depth: usize, node_count: usize) -> Result<(), DocxXmlError> {
        if depth > MAX_DOCX_XML_DEPTH {
            return Err(DocxXmlError::TooDeep);
        }
        if node_count > MAX_DOCX_XML_NODES {
            return Err(DocxXmlError::TooManyNodes);
        }
        Ok(())
    }

    fn local_name(name: &[u8]) -> &[u8] {
        name.rsplit(|byte| *byte == b':').next().unwrap_or(name)
    }
}
