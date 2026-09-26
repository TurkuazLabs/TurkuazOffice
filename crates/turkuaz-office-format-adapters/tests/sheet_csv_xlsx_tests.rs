// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-format-adapters/tests/sheet_csv_xlsx_tests.rs
// # 📌 Amac: M2 Sheet CSV/XLSX parser, canonical mapping, round-trip ve strict unsupported davranislarini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.3.0
// Aciklama: CSV quote/text-only semantigi, XLSX typed multi-sheet round-trip, sharedStrings import ve formula reject kontratlarini kilitler
// Bagimli Oldugu Katman: Service -> Model -> Tool -> Sheet

use std::collections::BTreeMap;

use turkuaz_office_core::{DocumentId, DocumentSchemaVersion};
use turkuaz_office_format_adapters::{
    SheetCsvService, SheetCsvTool, SheetXlsxArchiveTool, SheetXlsxError, SheetXlsxService,
};
use turkuaz_office_sheet::{
    CellAddress, CellValue, SequentialSheetIdTool, SheetDocument, Worksheet, WorksheetId,
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
    let document = SheetCsvService::import(
        &ids,
        "CSV",
        b"001,42,TRUE",
    )
    .expect("csv import");

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
            (
                CellAddress { row: 0, column: 1 },
                CellValue::Number(42.5),
            ),
            (
                CellAddress { row: 0, column: 2 },
                CellValue::Boolean(true),
            ),
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
                    (
                        CellAddress { row: 1, column: 1 },
                        CellValue::Number(1234.5),
                    ),
                    (
                        CellAddress { row: 2, column: 2 },
                        CellValue::Boolean(true),
                    ),
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
    };

    let bytes = SheetXlsxService::export(&document).expect("xlsx export");
    let ids = SequentialSheetIdTool::new();
    let restored =
        SheetXlsxService::import(&ids, "Restored", &bytes).expect("xlsx import");

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
fn xlsx_import_reads_shared_string_cells() {
    let bytes = workbook_package(
        br#"<?xml version="1.0" encoding="UTF-8"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1"><c r="A1" t="s"><v>0</v></c></row></sheetData></worksheet>"#,
        Some(
            br#"<?xml version="1.0" encoding="UTF-8"?><sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><si><t>Shared &amp; Safe</t></si></sst>"#,
        ),
    );

    let ids = SequentialSheetIdTool::new();
    let document =
        SheetXlsxService::import(&ids, "Shared", &bytes).expect("shared strings import");

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
fn xlsx_import_rejects_formula_cells_until_formula_engine_phase() {
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
