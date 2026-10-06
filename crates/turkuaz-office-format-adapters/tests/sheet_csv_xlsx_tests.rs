// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/tests/sheet_csv_xlsx_tests.rs
// # 📌 Amac: M2 Sheet CSV/XLSX parser, canonical mapping, round-trip ve strict unsupported davranislarini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.9.0
// Aciklama: CSV quote/text-only semantigi, XLSX typed round-trip ve formula strict-reject ile table/conditional-format OOXML round-trip kontratlarini kilitler
// Bagimli Oldugu Katman: Service -> Model -> Tool -> Sheet

use std::collections::BTreeMap;

use turkuaz_office_core::{DocumentId, DocumentSchemaVersion};
use turkuaz_office_format_adapters::{
    SheetCsvError, SheetCsvService, SheetCsvTool, SheetXlsxArchiveTool, SheetXlsxError,
    SheetXlsxService, SheetXlsxXmlTool,
};
use turkuaz_office_sheet::config::constants::{MAX_SHEET_COLUMNS, MAX_SHEET_ROWS};
use turkuaz_office_sheet::{
    CellAddress, CellValue, ConditionalFormatRuleId, FormulaCell, SequentialSheetIdTool,
    SheetConditionalFormatCondition, SheetConditionalFormatRule, SheetConditionalFormatStyle,
    SheetDocument, SheetRange, SheetTable, TableId, Worksheet, WorksheetId,
};

#[test]
fn csv_parser_handles_quotes_crlf_and_service_import_is_text_only() {
    let rows = SheetCsvTool::parse(
        b"Code,Description,Active\r\n001,\"Line, one\",TRUE\r\n002,\"A \"\"quoted\"\" value\",FALSE",
    )
    .expect("csv parse");

    assert_eq!(rows[1][0], "001");
    assert_eq!(rows[1][1], "Line, one");
    assert_eq!(rows[2][1], "A \"quoted\" value");

    let ids = SequentialSheetIdTool::new();
    let document = SheetCsvService::import(&ids, "CSV", b"001,42,TRUE").expect("csv import");

    let worksheet = &document.worksheets[0];
    assert_eq!(
        worksheet.cells.get(&CellAddress { row: 0, column: 0 }),
        Some(&CellValue::Text("001".to_owned()))
    );
    assert_eq!(
        worksheet.cells.get(&CellAddress { row: 0, column: 1 }),
        Some(&CellValue::Text("42".to_owned()))
    );
    assert_eq!(
        worksheet.cells.get(&CellAddress { row: 0, column: 2 }),
        Some(&CellValue::Text("TRUE".to_owned()))
    );
}

#[test]
fn csv_export_quotes_text_and_serializes_typed_values() {
    let worksheet = Worksheet {
        id: WorksheetId::new("worksheet-csv"),
        name: "Sheet1".to_owned(),
        cells: BTreeMap::from([
            (
                CellAddress { row: 0, column: 0 },
                CellValue::Text("A,B".to_owned()),
            ),
            (CellAddress { row: 0, column: 1 }, CellValue::Number(42.5)),
            (CellAddress { row: 0, column: 2 }, CellValue::Boolean(true)),
            (
                CellAddress { row: 1, column: 0 },
                CellValue::Text("A \"quote\"".to_owned()),
            ),
        ]),
    };

    let bytes = SheetCsvService::export_worksheet(&worksheet).expect("csv export");
    let text = String::from_utf8(bytes).expect("utf8");

    assert_eq!(text, "\"A,B\",42.5,TRUE\r\n\"A \"\"quote\"\"\",,");
}

#[test]
fn csv_sparse_export_handles_max_grid_address_without_rectangular_allocation() {
    let worksheet = Worksheet {
        id: WorksheetId::new("worksheet-sparse-max"),
        name: "Sheet1".to_owned(),
        cells: BTreeMap::from([(
            CellAddress {
                row: MAX_SHEET_ROWS - 1,
                column: MAX_SHEET_COLUMNS - 1,
            },
            CellValue::Boolean(true),
        )]),
    };

    let bytes = SheetCsvService::export_worksheet(&worksheet).expect("sparse csv export");
    let expected_length = usize::try_from(MAX_SHEET_ROWS - 1).expect("rows") * 2
        + usize::try_from(MAX_SHEET_COLUMNS - 1).expect("columns")
        + "TRUE".len();

    assert_eq!(bytes.len(), expected_length);
    assert!(bytes.ends_with(b"TRUE"));
}

#[test]
fn xlsx_round_trip_preserves_multi_sheet_text_number_and_boolean_values() {
    let document = SheetDocument {
        id: DocumentId::new("sheet-document-source"),
        title: "Typed Workbook".to_owned(),
        schema_version: DocumentSchemaVersion::current(),
        revision: 7,
        worksheets: vec![
            Worksheet {
                id: WorksheetId::new("worksheet-source-1"),
                name: "Data".to_owned(),
                cells: BTreeMap::from([
                    (
                        CellAddress { row: 0, column: 0 },
                        CellValue::Text("Sivas & Turkuaz".to_owned()),
                    ),
                    (CellAddress { row: 1, column: 1 }, CellValue::Number(1234.5)),
                    (CellAddress { row: 2, column: 2 }, CellValue::Boolean(true)),
                ]),
            },
            Worksheet {
                id: WorksheetId::new("worksheet-source-2"),
                name: "Second".to_owned(),
                cells: BTreeMap::from([(
                    CellAddress { row: 0, column: 0 },
                    CellValue::Boolean(false),
                )]),
            },
        ],
        cell_formats: BTreeMap::new(),
        conditional_formats: BTreeMap::new(),
        tables: BTreeMap::new(),
        charts: BTreeMap::new(),
    };

    let bytes = SheetXlsxService::export(&document).expect("xlsx export");
    let ids = SequentialSheetIdTool::new();
    let restored = SheetXlsxService::import(&ids, "Restored", &bytes).expect("xlsx import");

    assert_eq!(restored.revision, 0);
    assert_eq!(restored.worksheets.len(), 2);
    assert_eq!(restored.worksheets[0].name, "Data");
    assert_eq!(restored.worksheets[1].name, "Second");
    assert_eq!(
        restored.worksheets[0]
            .cells
            .get(&CellAddress { row: 0, column: 0 }),
        Some(&CellValue::Text("Sivas & Turkuaz".to_owned()))
    );
    assert_eq!(
        restored.worksheets[0]
            .cells
            .get(&CellAddress { row: 1, column: 1 }),
        Some(&CellValue::Number(1234.5))
    );
    assert_eq!(
        restored.worksheets[0]
            .cells
            .get(&CellAddress { row: 2, column: 2 }),
        Some(&CellValue::Boolean(true))
    );
    assert_eq!(
        restored.worksheets[1]
            .cells
            .get(&CellAddress { row: 0, column: 0 }),
        Some(&CellValue::Boolean(false))
    );
}

#[test]
fn xlsx_export_rejects_duplicate_worksheet_names_case_insensitively() {
    let document = SheetDocument {
        id: DocumentId::new("sheet-document-duplicate-names"),
        title: "Duplicate Names".to_owned(),
        schema_version: DocumentSchemaVersion::current(),
        revision: 0,
        worksheets: vec![
            Worksheet {
                id: WorksheetId::new("worksheet-1"),
                name: "Data".to_owned(),
                cells: BTreeMap::new(),
            },
            Worksheet {
                id: WorksheetId::new("worksheet-2"),
                name: "data".to_owned(),
                cells: BTreeMap::new(),
            },
        ],
        cell_formats: BTreeMap::new(),
        conditional_formats: BTreeMap::new(),
        tables: BTreeMap::new(),
        charts: BTreeMap::new(),
    };

    assert_eq!(
        SheetXlsxService::export(&document),
        Err(SheetXlsxError::InvalidWorksheetName)
    );
}

#[test]
fn xlsx_export_rejects_xml_invalid_cell_text() {
    let document = SheetDocument {
        id: DocumentId::new("sheet-document-invalid-text"),
        title: "Invalid Text".to_owned(),
        schema_version: DocumentSchemaVersion::current(),
        revision: 0,
        worksheets: vec![Worksheet {
            id: WorksheetId::new("worksheet-invalid-text"),
            name: "Sheet1".to_owned(),
            cells: BTreeMap::from([(
                CellAddress { row: 0, column: 0 },
                CellValue::Text("bad\u{0000}text".to_owned()),
            )]),
        }],
        cell_formats: BTreeMap::new(),
        conditional_formats: BTreeMap::new(),
        tables: BTreeMap::new(),
        charts: BTreeMap::new(),
    };

    assert_eq!(
        SheetXlsxService::export(&document),
        Err(SheetXlsxError::InvalidCellText)
    );
}

#[test]
fn xlsx_import_reads_shared_string_cells() {
    let bytes = workbook_package(
        br#"<?xml version="1.0" encoding="UTF-8"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1"><c r="A1" t="s"><v>0</v></c></row></sheetData></worksheet>"#,
        Some(
            br#"<?xml version="1.0" encoding="UTF-8"?><sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><si><t>Shared &amp; Safe</t></si></sst>"#,
        ),
    );

    let ids = SequentialSheetIdTool::new();
    let document = SheetXlsxService::import(&ids, "Shared", &bytes).expect("shared strings import");

    assert_eq!(
        document.worksheets[0]
            .cells
            .get(&CellAddress { row: 0, column: 0 }),
        Some(&CellValue::Text("Shared & Safe".to_owned()))
    );
}

#[test]
fn xlsx_import_rejects_missing_root_workbook_relationship() {
    let entries = vec![
        (
            "[Content_Types].xml".to_owned(),
            br#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>"#.to_vec(),
        ),
        (
            "_rels/.rels".to_owned(),
            br#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"/>"#.to_vec(),
        ),
        (
            "xl/workbook.xml".to_owned(),
            br#"<?xml version="1.0" encoding="UTF-8"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="rId1"/></sheets></workbook>"#.to_vec(),
        ),
        (
            "xl/_rels/workbook.xml.rels".to_owned(),
            br#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#.to_vec(),
        ),
        (
            "xl/worksheets/sheet1.xml".to_owned(),
            br#"<?xml version="1.0" encoding="UTF-8"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData/></worksheet>"#.to_vec(),
        ),
    ];
    let bytes = SheetXlsxArchiveTool::encode(&entries).expect("invalid root package");
    let ids = SequentialSheetIdTool::new();

    assert_eq!(
        SheetXlsxService::import(&ids, "Invalid Root", &bytes),
        Err(SheetXlsxError::InvalidPackage)
    );
}

#[test]
fn csv_and_xlsx_export_reject_canonical_formula_until_adapter_formula_profile() {
    let worksheet = Worksheet {
        id: WorksheetId::new("worksheet-formula-export"),
        name: "Sheet1".to_owned(),
        cells: BTreeMap::from([(
            CellAddress { row: 0, column: 0 },
            CellValue::Formula(FormulaCell {
                expression: "=1+2".to_owned(),
            }),
        )]),
    };

    assert_eq!(
        SheetCsvService::export_worksheet(&worksheet),
        Err(SheetCsvError::UnsupportedFormula)
    );

    let document = SheetDocument {
        id: DocumentId::new("sheet-document-formula-export"),
        title: "Formula Export".to_owned(),
        schema_version: DocumentSchemaVersion::current(),
        revision: 1,
        worksheets: vec![worksheet],
        cell_formats: BTreeMap::new(),
        conditional_formats: BTreeMap::new(),
        tables: BTreeMap::new(),
        charts: BTreeMap::new(),
    };
    assert_eq!(
        SheetXlsxService::export(&document),
        Err(SheetXlsxError::UnsupportedFormula)
    );
}

#[test]
fn xlsx_import_rejects_formula_cells_until_adapter_formula_profile() {
    let bytes = workbook_package(
        br#"<?xml version="1.0" encoding="UTF-8"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1"><c r="A1"><f>1+2</f><v>3</v></c></row></sheetData></worksheet>"#,
        None,
    );

    let ids = SequentialSheetIdTool::new();
    assert_eq!(
        SheetXlsxService::import(&ids, "Formula", &bytes),
        Err(SheetXlsxError::UnsupportedFormula)
    );
}

fn workbook_package(worksheet_xml: &[u8], shared_strings: Option<&[u8]>) -> Vec<u8> {
    let mut entries = vec![
        (
            "[Content_Types].xml".to_owned(),
            br#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>"#.to_vec(),
        ),
        (
            "_rels/.rels".to_owned(),
            br#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#.to_vec(),
        ),
        (
            "xl/workbook.xml".to_owned(),
            br#"<?xml version="1.0" encoding="UTF-8"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="rId1"/></sheets></workbook>"#.to_vec(),
        ),
        (
            "xl/_rels/workbook.xml.rels".to_owned(),
            br#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#.to_vec(),
        ),
        (
            "xl/worksheets/sheet1.xml".to_owned(),
            worksheet_xml.to_vec(),
        ),
    ];
    if let Some(shared) = shared_strings {
        entries.push(("xl/sharedStrings.xml".to_owned(), shared.to_vec()));
    }
    SheetXlsxArchiveTool::encode(&entries).expect("test xlsx package")
}

#[test]
fn xlsx_round_trip_preserves_canonical_table_and_conditional_format_metadata() {
    let worksheet_id = WorksheetId::new("worksheet-metadata-export");
    let table_id = TableId::new("table-1");
    let warning_rule_id = ConditionalFormatRuleId::new("conditional-format-warning");
    let text_rule_id = ConditionalFormatRuleId::new("conditional-format-text");

    let document = SheetDocument {
        id: DocumentId::new("sheet-document-metadata-export"),
        title: "Metadata Export".to_owned(),
        schema_version: DocumentSchemaVersion::current(),
        revision: 4,
        worksheets: vec![Worksheet {
            id: worksheet_id.clone(),
            name: "Data".to_owned(),
            cells: BTreeMap::from([
                (
                    CellAddress { row: 0, column: 0 },
                    CellValue::Text("Name".to_owned()),
                ),
                (
                    CellAddress { row: 0, column: 1 },
                    CellValue::Text("Score".to_owned()),
                ),
                (
                    CellAddress { row: 1, column: 0 },
                    CellValue::Text("Turkuaz".to_owned()),
                ),
                (CellAddress { row: 1, column: 1 }, CellValue::Number(20.0)),
                (
                    CellAddress { row: 2, column: 0 },
                    CellValue::Text("Office".to_owned()),
                ),
                (CellAddress { row: 2, column: 1 }, CellValue::Number(5.0)),
            ]),
        }],
        cell_formats: BTreeMap::new(),
        conditional_formats: BTreeMap::from([
            (
                warning_rule_id.clone(),
                SheetConditionalFormatRule {
                    id: warning_rule_id,
                    worksheet_id: worksheet_id.clone(),
                    range: SheetRange {
                        start_row: 1,
                        end_row: 2,
                        start_column: 1,
                        end_column: 1,
                    },
                    condition: SheetConditionalFormatCondition::NumberGreaterThan(10.0),
                    style: SheetConditionalFormatStyle::Warning,
                    priority: 1,
                },
            ),
            (
                text_rule_id.clone(),
                SheetConditionalFormatRule {
                    id: text_rule_id,
                    worksheet_id: worksheet_id.clone(),
                    range: SheetRange {
                        start_row: 1,
                        end_row: 2,
                        start_column: 0,
                        end_column: 0,
                    },
                    condition: SheetConditionalFormatCondition::TextContains("Tur*?~kuaz".to_owned()),
                    style: SheetConditionalFormatStyle::Accent,
                    priority: 2,
                },
            ),
        ]),
        tables: BTreeMap::from([(
            table_id.clone(),
            SheetTable {
                id: table_id,
                worksheet_id: worksheet_id.clone(),
                name: "Scores".to_owned(),
                range: SheetRange {
                    start_row: 0,
                    end_row: 2,
                    start_column: 0,
                    end_column: 1,
                },
            },
        )]),
        charts: BTreeMap::new(),
    };

    let bytes = SheetXlsxService::export(&document).expect("metadata xlsx export");
    let entries = SheetXlsxArchiveTool::decode(&bytes).expect("decode exported package");

    assert!(entries.contains_key("xl/tables/table1.xml"));
    assert!(entries.contains_key("xl/styles.xml"));
    assert!(entries.contains_key("xl/worksheets/_rels/sheet1.xml.rels"));

    let worksheet_xml = String::from_utf8(
        entries
            .get("xl/worksheets/sheet1.xml")
            .expect("worksheet")
            .clone(),
    )
    .expect("worksheet utf8");
    assert!(worksheet_xml.contains("<conditionalFormatting"));
    assert!(worksheet_xml.contains("<tableParts count=\"1\">"));
    assert!(worksheet_xml.contains(r#"SEARCH("Tur~*~?~~kuaz",A2)"#));
    assert!(!worksheet_xml.contains(r#"SEARCH(\"Turkuaz",A2)"#));

    let parsed_styles = SheetXlsxXmlTool::parse_differential_styles(
        entries.get("xl/styles.xml").expect("styles"),
    )
    .expect("parse differential styles");
    assert_eq!(parsed_styles.len(), 3);

    let (parsed_rules, parsed_table_relationships) =
        SheetXlsxXmlTool::parse_worksheet_metadata(
            entries
                .get("xl/worksheets/sheet1.xml")
                .expect("worksheet metadata"),
        )
        .expect("parse worksheet metadata");
    assert_eq!(parsed_rules.len(), 2);
    assert_eq!(parsed_rules[0].differential_style_id, 0);
    assert_eq!(parsed_rules[1].differential_style_id, 2);
    assert_eq!(parsed_table_relationships, vec!["rId1".to_owned()]);

    let ids = SequentialSheetIdTool::new();
    let restored =
        SheetXlsxService::import(&ids, "Restored Metadata", &bytes).expect("metadata xlsx import");

    assert_eq!(restored.tables.len(), 1);
    let table = restored.tables.values().next().expect("restored table");
    assert_eq!(table.name, "Scores");
    assert_eq!(
        table.range,
        SheetRange {
            start_row: 0,
            end_row: 2,
            start_column: 0,
            end_column: 1,
        }
    );

    assert_eq!(restored.conditional_formats.len(), 2);
    let mut rules = restored.conditional_formats.values().collect::<Vec<_>>();
    rules.sort_by_key(|rule| rule.priority);

    assert_eq!(rules[0].style, SheetConditionalFormatStyle::Warning);
    assert_eq!(
        rules[0].condition,
        SheetConditionalFormatCondition::NumberGreaterThan(10.0)
    );
    assert_eq!(
        rules[1].condition,
        SheetConditionalFormatCondition::TextContains("Turkuaz".to_owned())
    );
    assert_eq!(rules[1].style, SheetConditionalFormatStyle::Accent);
}

#[test]
fn xlsx_import_resolves_table_relationships_from_physical_worksheet_target() {
    let worksheet_id = WorksheetId::new("worksheet-reordered-target");
    let table_id = TableId::new("table-reordered-target");
    let document = SheetDocument {
        id: DocumentId::new("sheet-document-reordered-target"),
        title: "Reordered Target".to_owned(),
        schema_version: DocumentSchemaVersion::current(),
        revision: 0,
        worksheets: vec![Worksheet {
            id: worksheet_id.clone(),
            name: "Data".to_owned(),
            cells: BTreeMap::from([
                (
                    CellAddress { row: 0, column: 0 },
                    CellValue::Text("Name".to_owned()),
                ),
                (
                    CellAddress { row: 1, column: 0 },
                    CellValue::Text("Turkuaz".to_owned()),
                ),
            ]),
        }],
        cell_formats: BTreeMap::new(),
        conditional_formats: BTreeMap::new(),
        tables: BTreeMap::from([(
            table_id.clone(),
            SheetTable {
                id: table_id,
                worksheet_id,
                name: "Names".to_owned(),
                range: SheetRange {
                    start_row: 0,
                    end_row: 1,
                    start_column: 0,
                    end_column: 0,
                },
            },
        )]),
        charts: BTreeMap::new(),
    };

    let bytes = SheetXlsxService::export(&document).expect("export reordered fixture");
    let mut entries = SheetXlsxArchiveTool::decode(&bytes).expect("decode reordered fixture");
    let worksheet = entries
        .remove("xl/worksheets/sheet1.xml")
        .expect("sheet1 xml");
    let relationships = entries
        .remove("xl/worksheets/_rels/sheet1.xml.rels")
        .expect("sheet1 rels");
    entries.insert("xl/custom/sheet.xml".to_owned(), worksheet);
    entries.insert(
        "xl/custom/_rels/sheet.xml.rels".to_owned(),
        relationships,
    );

    let workbook_rels = String::from_utf8(
        entries
            .get("xl/_rels/workbook.xml.rels")
            .expect("workbook rels")
            .clone(),
    )
    .expect("workbook rels utf8")
    .replace("worksheets/sheet1.xml", "custom/sheet.xml");
    entries.insert(
        "xl/_rels/workbook.xml.rels".to_owned(),
        workbook_rels.into_bytes(),
    );

    let repacked = SheetXlsxArchiveTool::encode(&entries.into_iter().collect::<Vec<_>>())
        .expect("repack reordered fixture");
    let ids = SequentialSheetIdTool::new();
    let restored =
        SheetXlsxService::import(&ids, "Reordered Target", &repacked).expect("import reordered");

    assert_eq!(restored.tables.len(), 1);
    assert_eq!(
        restored.tables.values().next().expect("table").name,
        "Names"
    );
}

#[test]
fn xlsx_export_rejects_conditional_format_count_above_global_limit() {
    let worksheet_id = WorksheetId::new("worksheet-conditional-limit");
    let mut conditional_formats = BTreeMap::new();
    for index in 0..513_u32 {
        let id = ConditionalFormatRuleId::new(format!("conditional-format-{index}"));
        conditional_formats.insert(
            id.clone(),
            SheetConditionalFormatRule {
                id,
                worksheet_id: worksheet_id.clone(),
                range: SheetRange {
                    start_row: 0,
                    end_row: 0,
                    start_column: 0,
                    end_column: 0,
                },
                condition: SheetConditionalFormatCondition::NumberGreaterThan(f64::from(index)),
                style: SheetConditionalFormatStyle::Warning,
                priority: index + 1,
            },
        );
    }

    let document = SheetDocument {
        id: DocumentId::new("sheet-document-conditional-limit"),
        title: "Conditional Limit".to_owned(),
        schema_version: DocumentSchemaVersion::current(),
        revision: 0,
        worksheets: vec![Worksheet {
            id: worksheet_id,
            name: "Data".to_owned(),
            cells: BTreeMap::new(),
        }],
        cell_formats: BTreeMap::new(),
        conditional_formats,
        tables: BTreeMap::new(),
        charts: BTreeMap::new(),
    };

    assert_eq!(
        SheetXlsxService::export(&document),
        Err(SheetXlsxError::ResourceLimit)
    );
}
