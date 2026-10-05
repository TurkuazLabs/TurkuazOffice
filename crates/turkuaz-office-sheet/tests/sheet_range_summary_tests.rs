// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/tests/sheet_range_summary_tests.rs
// # 📌 Amac: Sheet range status aggregate hesaplarini canonical Service ve Controller seviyesinde dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.5.0
// Aciklama: Number, text, boolean ve formula hucrelerinde Count/NumericCount/Sum/Average davranisini regression testiyle sabitler
// Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View

use turkuaz_office_sheet::{
    CellValue, InMemorySheetDocumentRepository, SequentialSheetIdTool, SheetController, SheetRange,
    SheetService,
};

fn service() -> SheetService<InMemorySheetDocumentRepository, SequentialSheetIdTool> {
    SheetService::new(
        InMemorySheetDocumentRepository::new(),
        SequentialSheetIdTool::new(),
    )
}

#[test]
fn range_summary_counts_non_empty_cells_and_aggregates_numeric_values() {
    let mut service = service();
    let document = service.create_document("Summary");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A1", CellValue::Number(10.0))
        .expect("A1 number");
    let document = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "A2",
            CellValue::Text("text".to_owned()),
        )
        .expect("A2 text");
    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A3", CellValue::Boolean(true))
        .expect("A3 boolean");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A4", "=A1+5")
        .expect("A4 formula");

    let summary = service
        .range_summary(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 0,
                end_row: 3,
                start_column: 0,
                end_column: 0,
            },
        )
        .expect("range summary");

    assert_eq!(summary.count, 4);
    assert_eq!(summary.numeric_count, 2);
    assert_eq!(summary.sum, 25.0);
    assert_eq!(summary.average, Some(12.5));
}

#[test]
fn controller_exposes_range_summary_without_business_logic() {
    let service = service();
    let mut controller = SheetController::new(service);
    let document = controller.create("Controller Summary");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = controller
        .set_cell_a1(&document.id, &worksheet_id, "B2", CellValue::Number(3.0))
        .expect("B2");
    let document = controller
        .set_cell_a1(&document.id, &worksheet_id, "C2", CellValue::Number(7.0))
        .expect("C2");

    let summary = controller
        .range_summary(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 1,
                end_row: 1,
                start_column: 1,
                end_column: 2,
            },
        )
        .expect("controller summary");

    assert_eq!(summary.count, 2);
    assert_eq!(summary.numeric_count, 2);
    assert_eq!(summary.sum, 10.0);
    assert_eq!(summary.average, Some(5.0));
}
