// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/tests/sheet_function_library_tests.rs
// # 📌 Amac: Sheet function library range, aggregate, comparison ve IF davranislarini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.8.0
// Aciklama: SUM/AVERAGE/MIN/MAX/IF, range limiti, lazy IF ve cycle semantigini sabitler
// Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View

use turkuaz_office_sheet::{
    CellValue, FormulaTool, FormulaToolError, InMemorySheetDocumentRepository,
    SequentialSheetIdTool, SheetError, SheetService,
};

fn service() -> SheetService<InMemorySheetDocumentRepository, SequentialSheetIdTool> {
    SheetService::new(
        InMemorySheetDocumentRepository::new(),
        SequentialSheetIdTool::new(),
    )
}

fn evaluated_number(
    service: &SheetService<InMemorySheetDocumentRepository, SequentialSheetIdTool>,
    document_id: &turkuaz_office_core::DocumentId,
    worksheet_id: &turkuaz_office_sheet::WorksheetId,
    reference: &str,
) -> f64 {
    match service
        .evaluated_cell_by_a1(document_id, worksheet_id, reference)
        .expect("evaluate")
        .expect("cell exists")
        .value
    {
        CellValue::Number(value) => value,
        other => panic!("expected numeric value, got {other:?}"),
    }
}

#[test]
fn formula_tool_accepts_functions_ranges_comparisons_and_locale_separators() {
    for formula in [
        "=SUM(A1:A10)",
        "=average(A1;A2;5)",
        "=MIN(B3:A1)",
        "=MAX(A1*2,B2)",
        "=IF(A1>=10,100,0)",
        "=IF(A1<>B1;1;0)",
    ] {
        assert!(FormulaTool::parse(formula).is_ok(), "{formula}");
    }

    assert_eq!(
        FormulaTool::parse("=UNKNOWN(A1)"),
        Err(FormulaToolError::UnknownFunction)
    );
    assert_eq!(
        FormulaTool::parse("=IF(A1,1)"),
        Err(FormulaToolError::InvalidArgumentCount)
    );
    assert_eq!(
        FormulaTool::parse("=SUM()"),
        Err(FormulaToolError::InvalidArgumentCount)
    );
}

#[test]
fn aggregate_functions_use_numeric_cells_formulas_and_ignore_text_boolean_empty_cells() {
    let mut service = service();
    let document = service.create_document("Functions");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A1", CellValue::Number(10.0))
        .expect("A1");
    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A2", CellValue::Number(20.0))
        .expect("A2");
    let document = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "A3",
            CellValue::Text("ignored".to_owned()),
        )
        .expect("A3");
    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A4", CellValue::Boolean(true))
        .expect("A4");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A5", "=A1+A2")
        .expect("A5");

    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B1", "=SUM(A1:A6)")
        .expect("B1");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B2", "=AVERAGE(A1:A6)")
        .expect("B2");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B3", "=MIN(A1:A6)")
        .expect("B3");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B4", "=MAX(A1:A6)")
        .expect("B4");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B5", "=SUM(A1,5,A2*2)")
        .expect("B5");

    assert_eq!(
        evaluated_number(&service, &document.id, &worksheet_id, "B1"),
        60.0
    );
    assert_eq!(
        evaluated_number(&service, &document.id, &worksheet_id, "B2"),
        20.0
    );
    assert_eq!(
        evaluated_number(&service, &document.id, &worksheet_id, "B3"),
        10.0
    );
    assert_eq!(
        evaluated_number(&service, &document.id, &worksheet_id, "B4"),
        30.0
    );
    assert_eq!(
        evaluated_number(&service, &document.id, &worksheet_id, "B5"),
        55.0
    );
}

#[test]
fn comparisons_return_numeric_boolean_and_if_evaluates_only_selected_branch() {
    let mut service = service();
    let document = service.create_document("If");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A1", CellValue::Number(10.0))
        .expect("A1");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B1", "=A1>=10")
        .expect("B1");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B2", "=A1<10")
        .expect("B2");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B3", "=IF(A1>0,42,1/0)")
        .expect("B3");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B4", "=IF(A1<0,1/0,7)")
        .expect("B4");

    assert_eq!(
        evaluated_number(&service, &document.id, &worksheet_id, "B1"),
        1.0
    );
    assert_eq!(
        evaluated_number(&service, &document.id, &worksheet_id, "B2"),
        0.0
    );
    assert_eq!(
        evaluated_number(&service, &document.id, &worksheet_id, "B3"),
        42.0
    );
    assert_eq!(
        evaluated_number(&service, &document.id, &worksheet_id, "B4"),
        7.0
    );
}

#[test]
fn aggregate_ranges_preserve_cycle_detection_and_reject_oversized_expansion() {
    let mut service = service();
    let document = service.create_document("Limits");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A1", "=SUM(A1:A2)")
        .expect("cycle formula");
    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "A1"),
        Err(SheetError::FormulaCycle)
    );

    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B1", "=SUM(A1:A100001)")
        .expect("large range parses");
    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "B1"),
        Err(SheetError::FormulaRangeTooLarge)
    );
}

#[test]
fn raw_range_expression_is_rejected_outside_supported_function_context() {
    let mut service = service();
    let document = service.create_document("Raw Range");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A1", "=B1:B3")
        .expect("range syntax parses");

    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "A1"),
        Err(SheetError::FormulaRangeNotAllowed)
    );
}
