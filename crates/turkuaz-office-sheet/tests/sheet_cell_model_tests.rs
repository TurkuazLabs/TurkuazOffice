// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/tests/sheet_cell_model_tests.rs
// # 📌 Amac: M2 Sheet cell model, A1 reference, sparse mutation ve validation davranislarini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.3.0
// Aciklama: Grid sinirlari, text/number validation, revision no-op ve Controller/View akislarini cell-model kabul testi yapar
// Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View

use turkuaz_office_core::config::constants::DEFAULT_DOCUMENT_TITLE;
use turkuaz_office_sheet::config::constants::{
    DEFAULT_WORKSHEET_NAME, MAX_CELL_TEXT_LENGTH, MAX_SHEET_COLUMNS, MAX_SHEET_ROWS,
};
use turkuaz_office_sheet::{
    CellAddress, CellReferenceError, CellReferenceTool, CellValue, CellValueView,
    InMemorySheetDocumentRepository, SequentialSheetIdTool, SheetController, SheetError,
    SheetService,
};

fn service() -> SheetService<InMemorySheetDocumentRepository, SequentialSheetIdTool> {
    SheetService::new(
        InMemorySheetDocumentRepository::new(),
        SequentialSheetIdTool::new(),
    )
}

#[test]
fn new_sheet_document_is_sparse_with_one_default_worksheet() {
    let mut service = service();
    let document = service.create_document("   ");

    assert_eq!(document.title, DEFAULT_DOCUMENT_TITLE);
    assert_eq!(document.revision, 0);
    assert_eq!(document.worksheets.len(), 1);
    assert_eq!(document.worksheets[0].name, DEFAULT_WORKSHEET_NAME);
    assert!(document.worksheets[0].cells.is_empty());
}

#[test]
fn a1_reference_tool_parses_and_formats_grid_boundaries() {
    assert_eq!(
        CellReferenceTool::parse("A1"),
        Ok(CellAddress { row: 0, column: 0 })
    );
    assert_eq!(
        CellReferenceTool::parse("aa10"),
        Ok(CellAddress { row: 9, column: 26 })
    );
    assert_eq!(
        CellReferenceTool::parse("XFD1048576"),
        Ok(CellAddress {
            row: MAX_SHEET_ROWS - 1,
            column: MAX_SHEET_COLUMNS - 1,
        })
    );
    assert_eq!(
        CellReferenceTool::format(CellAddress {
            row: MAX_SHEET_ROWS - 1,
            column: MAX_SHEET_COLUMNS - 1,
        }),
        Ok("XFD1048576".to_owned())
    );
}

#[test]
fn a1_reference_tool_rejects_invalid_or_out_of_bounds_values() {
    assert_eq!(
        CellReferenceTool::parse(""),
        Err(CellReferenceError::Empty)
    );
    assert_eq!(
        CellReferenceTool::parse("A0"),
        Err(CellReferenceError::InvalidFormat)
    );
    assert_eq!(
        CellReferenceTool::parse("1A"),
        Err(CellReferenceError::InvalidFormat)
    );
    assert_eq!(
        CellReferenceTool::parse("$A$1"),
        Err(CellReferenceError::InvalidFormat)
    );
    assert_eq!(
        CellReferenceTool::parse("XFE1"),
        Err(CellReferenceError::OutOfBounds)
    );
    assert_eq!(
        CellReferenceTool::parse("A1048577"),
        Err(CellReferenceError::OutOfBounds)
    );
}

#[test]
fn sparse_set_get_and_clear_mutate_revision_only_when_data_changes() {
    let mut service = service();
    let document = service.create_document("Budget");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "B2",
            CellValue::Text("Revenue".to_owned()),
        )
        .expect("set cell");
    assert_eq!(document.revision, 1);
    assert_eq!(document.worksheets[0].cells.len(), 1);

    let same = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "B2",
            CellValue::Text("Revenue".to_owned()),
        )
        .expect("same cell");
    assert_eq!(same.revision, 1);

    let cell = service
        .cell_by_a1(&document.id, &worksheet_id, "B2")
        .expect("cell query")
        .expect("cell exists");
    assert_eq!(cell.address, CellAddress { row: 1, column: 1 });
    assert_eq!(cell.value, CellValue::Text("Revenue".to_owned()));

    let cleared = service
        .clear_cell_by_a1(&document.id, &worksheet_id, "B2")
        .expect("clear cell");
    assert_eq!(cleared.revision, 2);
    assert!(cleared.worksheets[0].cells.is_empty());

    let clear_again = service
        .clear_cell_by_a1(&document.id, &worksheet_id, "B2")
        .expect("clear missing cell");
    assert_eq!(clear_again.revision, 2);
}

#[test]
fn cell_validation_rejects_oversized_text_non_finite_number_and_grid_overflow() {
    let mut service = service();
    let document = service.create_document("Validation");
    let worksheet_id = document.worksheets[0].id.clone();

    assert_eq!(
        service.set_cell(
            &document.id,
            &worksheet_id,
            CellAddress { row: MAX_SHEET_ROWS, column: 0 },
            CellValue::Boolean(true),
        ),
        Err(SheetError::CellOutOfBounds)
    );

    assert_eq!(
        service.set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "A1",
            CellValue::Text("x".repeat(MAX_CELL_TEXT_LENGTH + 1)),
        ),
        Err(SheetError::CellTextTooLong)
    );

    assert_eq!(
        service.set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "A1",
            CellValue::Number(f64::NAN),
        ),
        Err(SheetError::CellNumberNotFinite)
    );
}

#[test]
fn controller_returns_deterministic_sparse_cell_view() {
    let service = service();
    let mut controller = SheetController::new(service);
    let document = controller.create("Controller");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = controller
        .set_cell_a1(
            &document.id,
            &worksheet_id,
            "C3",
            CellValue::Number(42.5),
        )
        .expect("controller set");
    assert_eq!(document.revision, 1);
    assert_eq!(document.worksheets[0].cell_count, 1);
    assert_eq!(document.worksheets[0].cells[0].row, 2);
    assert_eq!(document.worksheets[0].cells[0].column, 2);
    assert_eq!(
        document.worksheets[0].cells[0].value,
        CellValueView::Number(42.5)
    );
}
