// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/tests/sheet_format_filter_sort_tests.rs
// # 📌 Amac: M2 Sheet format/filter/sort canonical metadata ve non-mutating row-query davranislarini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.3.0
// Aciklama: Format revision no-op, typed validation, formula-aware filter ve deterministic sort akislarini kabul testi yapar
// Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View

use turkuaz_office_sheet::{
    CellFormat, CellValue, HorizontalAlignment, InMemorySheetDocumentRepository,
    SequentialSheetIdTool, SheetController, SheetError, SheetFilter, SheetFilterCondition,
    SheetRange, SheetService, SheetSort, SheetSortDirection,
};

fn service() -> SheetService<InMemorySheetDocumentRepository, SequentialSheetIdTool> {
    SheetService::new(
        InMemorySheetDocumentRepository::new(),
        SequentialSheetIdTool::new(),
    )
}

#[test]
fn cell_format_is_canonical_sparse_and_revision_aware() {
    let mut service = service();
    let document = service.create_document("Format");
    let worksheet_id = document.worksheets[0].id.clone();
    let format = CellFormat {
        bold: true,
        italic: false,
        underline: true,
        horizontal_alignment: HorizontalAlignment::Right,
        decimal_places: Some(2),
    };

    let document = service
        .set_cell_format_by_a1(&document.id, &worksheet_id, "B2", format)
        .expect("format");
    assert_eq!(document.revision, 1);
    assert_eq!(
        service
            .cell_format_by_a1(&document.id, &worksheet_id, "B2")
            .expect("format read"),
        format
    );

    let same = service
        .set_cell_format_by_a1(&document.id, &worksheet_id, "B2", format)
        .expect("same format");
    assert_eq!(same.revision, 1);

    let cleared = service
        .set_cell_format_by_a1(&document.id, &worksheet_id, "B2", CellFormat::default())
        .expect("clear format");
    assert_eq!(cleared.revision, 2);
    assert!(cleared.cell_formats.is_empty());
}

#[test]
fn cell_format_rejects_excessive_decimal_places() {
    let mut service = service();
    let document = service.create_document("Format Validation");
    let worksheet_id = document.worksheets[0].id.clone();

    assert_eq!(
        service.set_cell_format_by_a1(
            &document.id,
            &worksheet_id,
            "A1",
            CellFormat {
                decimal_places: Some(13),
                ..CellFormat::default()
            },
        ),
        Err(SheetError::InvalidCellFormat)
    );
}

#[test]
fn filter_and_sort_query_is_formula_aware_and_does_not_mutate_document() {
    let mut service = service();
    let document = service.create_document("Query");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A1", CellValue::Text("Alpha".to_owned()))
        .expect("A1");
    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "B1", CellValue::Number(30.0))
        .expect("B1");
    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A2", CellValue::Text("Beta".to_owned()))
        .expect("A2");
    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "B2", CellValue::Number(10.0))
        .expect("B2");
    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A3", CellValue::Text("Gamma".to_owned()))
        .expect("A3");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B3", "=B2+10")
        .expect("B3 formula");

    let revision_before_query = document.revision;
    let rows = service
        .query_rows(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 0,
                end_row: 2,
                start_column: 0,
                end_column: 1,
            },
            Some(&SheetFilter {
                column: 1,
                condition: SheetFilterCondition::NumberGreaterThan(15.0),
            }),
            Some(SheetSort {
                column: 1,
                direction: SheetSortDirection::Ascending,
            }),
        )
        .expect("query");

    assert_eq!(rows, vec![2, 0]);
    assert_eq!(
        service.get_document(&document.id).expect("document").revision,
        revision_before_query
    );
}

#[test]
fn controller_maps_format_and_row_query_to_views() {
    let mut controller = SheetController::new(service());
    let document = controller.create("Controller Query");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = controller
        .set_cell_a1(
            &document.id,
            &worksheet_id,
            "A1",
            CellValue::Text("Turkuaz".to_owned()),
        )
        .expect("cell");
    controller
        .set_cell_format_a1(
            &document.id,
            &worksheet_id,
            "A1",
            CellFormat {
                bold: true,
                ..CellFormat::default()
            },
        )
        .expect("format");

    assert!(
        controller
            .cell_format_a1(&document.id, &worksheet_id, "A1")
            .expect("format view")
            .bold
    );

    let rows = controller
        .query_rows(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 0,
                end_row: 5,
                start_column: 0,
                end_column: 1,
            },
            Some(&SheetFilter {
                column: 0,
                condition: SheetFilterCondition::TextContains("kuaz".to_owned()),
            }),
            None,
        )
        .expect("query view");
    assert_eq!(rows.rows, vec![0]);
}
