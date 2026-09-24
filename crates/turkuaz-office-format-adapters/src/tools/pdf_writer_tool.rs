// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/src/tools/pdf_writer_tool.rs
// # 📌 Amac: Format-specific PDF modelini embedded external fontlarla deterministik PDF byte'ina cevirir
// # 📌 Modul - FileType: Tool - Rust
// # Version: 0.2.0
// # Aciklama: Physical twip geometry, multi-page text flow, alignment ve underline operasyonlarini printpdf uzerinden yazar
// Bagimli Oldugu Katman: Tool -> Model -> Config

use std::collections::BTreeMap;

use printpdf::{
    Color, Line, LinePoint, Mm, Op, ParsedFont, PdfDocument, PdfFontHandle, PdfPage,
    PdfSaveOptions, Point, Pt, Rgb, TextItem,
};

use crate::config::pdf_constants::{
    MAX_PDF_OUTPUT_BYTES, MAX_PDF_PAGES, PDF_LINE_HEIGHT_MULTIPLIER,
    PDF_PARAGRAPH_GAP_POINTS, PDF_UNDERLINE_OFFSET_MULTIPLIER,
    PDF_UNDERLINE_THICKNESS_MULTIPLIER, TWIPS_PER_POINT,
};
use crate::models::pdf_model::{
    PdfAlignment, PdfDocumentModel, PdfFontData, PdfFontKey, PdfRunModel,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PdfWriterError {
    FontMissing,
    FontInvalid,
    GlyphMissing,
    InvalidPageGeometry,
    PageLimitExceeded,
    OutputTooLarge,
}

#[derive(Clone)]
struct LoadedFont {
    parsed: ParsedFont,
    handle: PdfFontHandle,
}

#[derive(Clone)]
struct LineFragment {
    text: String,
    font: PdfFontKey,
    size_pt: f32,
    underline: bool,
    width_pt: f32,
}

#[derive(Clone, Default)]
struct LayoutLine {
    fragments: Vec<LineFragment>,
    width_pt: f32,
    height_pt: f32,
}

pub struct PdfWriterTool;

impl PdfWriterTool {
    pub fn encode(
        model: &PdfDocumentModel,
        fonts: &[PdfFontData],
    ) -> Result<Vec<u8>, PdfWriterError> {
        let page_width_pt = Self::twips_to_points(model.page_settings.width_twips);
        let page_height_pt = Self::twips_to_points(model.page_settings.height_twips);
        let margin_top_pt = Self::twips_to_points(model.page_settings.margin_top_twips);
        let margin_right_pt = Self::twips_to_points(model.page_settings.margin_right_twips);
        let margin_bottom_pt = Self::twips_to_points(model.page_settings.margin_bottom_twips);
        let margin_left_pt = Self::twips_to_points(model.page_settings.margin_left_twips);
        let content_width_pt = page_width_pt - margin_left_pt - margin_right_pt;
        let content_height_pt = page_height_pt - margin_top_pt - margin_bottom_pt;
        if page_width_pt <= 0.0
            || page_height_pt <= 0.0
            || content_width_pt <= 0.0
            || content_height_pt <= 0.0
        {
            return Err(PdfWriterError::InvalidPageGeometry);
        }

        let mut document = PdfDocument::new(&model.title);
        let loaded = Self::load_fonts(&mut document, fonts)?;
        let mut pages = Vec::new();
        let mut page_ops = Vec::new();
        let mut y_top_pt = margin_top_pt;

        for paragraph in &model.paragraphs {
            let lines = Self::layout_paragraph(&paragraph.runs, &loaded, content_width_pt)?;
            for (line_index, line) in lines.iter().enumerate() {
                if y_top_pt + line.height_pt > page_height_pt - margin_bottom_pt {
                    pages.push(Self::page(
                        page_width_pt,
                        page_height_pt,
                        std::mem::take(&mut page_ops),
                    ));
                    if pages.len() >= MAX_PDF_PAGES {
                        return Err(PdfWriterError::PageLimitExceeded);
                    }
                    y_top_pt = margin_top_pt;
                }
                Self::render_line(
                    &mut page_ops,
                    line,
                    paragraph.alignment,
                    line_index + 1 == lines.len(),
                    margin_left_pt,
                    content_width_pt,
                    page_height_pt,
                    y_top_pt,
                    &loaded,
                );
                y_top_pt += line.height_pt;
            }
            y_top_pt += PDF_PARAGRAPH_GAP_POINTS;
        }

        if !page_ops.is_empty() || pages.is_empty() {
            pages.push(Self::page(page_width_pt, page_height_pt, page_ops));
        }
        if pages.len() > MAX_PDF_PAGES {
            return Err(PdfWriterError::PageLimitExceeded);
        }

        let mut warnings = Vec::new();
        let bytes = document
            .with_pages(pages)
            .save(
                &PdfSaveOptions {
                    subset_fonts: true,
                    ..Default::default()
                },
                &mut warnings,
            );
        if bytes.len() > MAX_PDF_OUTPUT_BYTES {
            return Err(PdfWriterError::OutputTooLarge);
        }
        Ok(bytes)
    }

    fn load_fonts(
        document: &mut PdfDocument,
        fonts: &[PdfFontData],
    ) -> Result<BTreeMap<PdfFontKey, LoadedFont>, PdfWriterError> {
        let mut loaded = BTreeMap::new();
        for font in fonts {
            let mut warnings = Vec::new();
            let parsed = ParsedFont::from_bytes(
                &font.bytes,
                font.face_index as usize,
                &mut warnings,
            )
            .ok_or(PdfWriterError::FontInvalid)?;
            let id = document.add_font(&parsed);
            loaded.insert(
                font.key.clone(),
                LoadedFont {
                    parsed,
                    handle: PdfFontHandle::External(id),
                },
            );
        }
        Ok(loaded)
    }

    fn layout_paragraph(
        runs: &[PdfRunModel],
        fonts: &BTreeMap<PdfFontKey, LoadedFont>,
        max_width_pt: f32,
    ) -> Result<Vec<LayoutLine>, PdfWriterError> {
        let mut lines = vec![LayoutLine::default()];
        for run in runs {
            let loaded = fonts.get(&run.font).ok_or(PdfWriterError::FontMissing)?;
            let size_pt = run.font_size_half_points as f32 / 2.0;
            for character in run.text.chars() {
                if character == '\r' {
                    continue;
                }
                if character == '\n' {
                    lines.push(LayoutLine::default());
                    continue;
                }
                let width_pt = Self::character_width(&loaded.parsed, character, size_pt)?;
                let current = lines.last_mut().expect("layout always has line");
                if current.width_pt > 0.0 && current.width_pt + width_pt > max_width_pt {
                    lines.push(LayoutLine::default());
                }
                let current = lines.last_mut().expect("layout always has line");
                current.height_pt = current
                    .height_pt
                    .max(size_pt * PDF_LINE_HEIGHT_MULTIPLIER);
                current.width_pt += width_pt;
                if let Some(fragment) = current.fragments.last_mut() {
                    if fragment.font == run.font
                        && fragment.size_pt == size_pt
                        && fragment.underline == run.underline
                    {
                        fragment.text.push(character);
                        fragment.width_pt += width_pt;
                        continue;
                    }
                }
                current.fragments.push(LineFragment {
                    text: character.to_string(),
                    font: run.font.clone(),
                    size_pt,
                    underline: run.underline,
                    width_pt,
                });
            }
        }
        for line in &mut lines {
            if line.height_pt <= 0.0 {
                line.height_pt = 12.0 * PDF_LINE_HEIGHT_MULTIPLIER;
            }
        }
        Ok(lines)
    }

    fn render_line(
        ops: &mut Vec<Op>,
        line: &LayoutLine,
        alignment: PdfAlignment,
        is_last_line: bool,
        margin_left_pt: f32,
        content_width_pt: f32,
        page_height_pt: f32,
        y_top_pt: f32,
        fonts: &BTreeMap<PdfFontKey, LoadedFont>,
    ) {
        let justify = alignment == PdfAlignment::Justify && !is_last_line;
        let space_count = line
            .fragments
            .iter()
            .flat_map(|fragment| fragment.text.chars())
            .filter(|character| *character == ' ')
            .count();
        let extra_space_pt = if justify && space_count > 0 {
            ((content_width_pt - line.width_pt) / space_count as f32).max(0.0)
        } else {
            0.0
        };
        let start_x_pt = match alignment {
            PdfAlignment::Left | PdfAlignment::Justify => margin_left_pt,
            PdfAlignment::Center => {
                margin_left_pt + ((content_width_pt - line.width_pt) / 2.0).max(0.0)
            }
            PdfAlignment::Right => {
                margin_left_pt + (content_width_pt - line.width_pt).max(0.0)
            }
        };
        let baseline_y_pt = page_height_pt - y_top_pt - line.height_pt * 0.78;
        let mut x_pt = start_x_pt;

        for fragment in &line.fragments {
            let loaded = fonts
                .get(&fragment.font)
                .expect("validated font must remain loaded");
            ops.push(Op::StartTextSection);
            ops.push(Op::SetTextCursor {
                pos: Point {
                    x: Pt(x_pt),
                    y: Pt(baseline_y_pt),
                },
            });
            ops.push(Op::SetFont {
                font: loaded.handle.clone(),
                size: Pt(fragment.size_pt),
            });
            if extra_space_pt > 0.0 {
                ops.push(Op::SetWordSpacing {
                    pt: Pt(extra_space_pt),
                });
            }
            ops.push(Op::ShowText {
                items: vec![TextItem::Text(fragment.text.clone())],
            });
            ops.push(Op::EndTextSection);

            let spaces = fragment.text.chars().filter(|character| *character == ' ').count();
            let rendered_width = fragment.width_pt + extra_space_pt * spaces as f32;
            if fragment.underline && rendered_width > 0.0 {
                let underline_y =
                    baseline_y_pt - fragment.size_pt * PDF_UNDERLINE_OFFSET_MULTIPLIER;
                ops.push(Op::SetOutlineColor {
                    col: Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
                });
                ops.push(Op::SetOutlineThickness {
                    pt: Pt(
                        fragment.size_pt * PDF_UNDERLINE_THICKNESS_MULTIPLIER,
                    ),
                });
                ops.push(Op::DrawLine {
                    line: Line {
                        points: vec![
                            LinePoint {
                                p: Point {
                                    x: Pt(x_pt),
                                    y: Pt(underline_y),
                                },
                                bezier: false,
                            },
                            LinePoint {
                                p: Point {
                                    x: Pt(x_pt + rendered_width),
                                    y: Pt(underline_y),
                                },
                                bezier: false,
                            },
                        ],
                        is_closed: false,
                    },
                });
            }
            x_pt += rendered_width;
        }
    }

    fn character_width(
        font: &ParsedFont,
        character: char,
        size_pt: f32,
    ) -> Result<f32, PdfWriterError> {
        let glyph = font
            .lookup_glyph_index(character as u32)
            .ok_or(PdfWriterError::GlyphMissing)?;
        let units_per_em = font.pdf_font_metrics.units_per_em as f32;
        if units_per_em <= 0.0 {
            return Err(PdfWriterError::FontInvalid);
        }
        Ok(font.get_horizontal_advance(glyph) as f32 / units_per_em * size_pt)
    }

    fn page(width_pt: f32, height_pt: f32, ops: Vec<Op>) -> PdfPage {
        PdfPage::new(
            Mm(width_pt / 72.0 * 25.4),
            Mm(height_pt / 72.0 * 25.4),
            ops,
        )
    }

    fn twips_to_points(value: u32) -> f32 {
        value as f32 / TWIPS_PER_POINT
    }
}
