// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/tests/sheet_formula_engine_tests.rs
// # 📌 Amac: M2 Basic Formula Engine parse, evaluation, dependency ve error davranislarini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.3.0
// Aciklama: Same-sheet arithmetic, absolute A1, empty reference, cycle, division ve Controller/View akislarini kabul testi yapar
// Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View

use turkuaz_office_sheet::{
    CellValue, CellValueView, FormulaCell, FormulaTool, FormulaToolError,
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
fn formula_tool_accepts_precedence_parentheses_and_absolute_references() {
    assert!(FormulaTool::parse("=1 + A1 * ($B$2 - -3)").is_ok());
    assert!(FormulaTool::parse("=$A1+A$1+$A$1").is_ok());

    assert_eq!(
        FormulaTool::parse("A1+1"),
        Err(FormulaToolError::MissingPrefix)
    );
    assert_eq!(
        FormulaTool::parse("=A0+1"),
        Err(FormulaToolError::InvalidReference)
    );
    assert_eq!(
        FormulaTool::parse("=1+"),
        Err(FormulaToolError::UnexpectedToken)
    );
}

#[test]
fn formula_evaluation_respects_operator_precedence_and_revision_no_op() {
    let mut service = service();
    let document = service.create_document("Formula");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A1", CellValue::Number(10.0))
        .expect("A1");
    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "B1", CellValue::Number(2.0))
        .expect("B1");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "C1", "=A1+B1*3")
        .expect("C1 formula");

    assert_eq!(document.revision, 3);
    let same = service
        .set_formula_by_a1(&document.id, &worksheet_id, "C1", "=A1+B1*3")
        .expect("same formula");
    assert_eq!(same.revision, 3);

    let raw = service
        .cell_by_a1(&document.id, &worksheet_id, "C1")
        .expect("raw")
        .expect("raw exists");
    assert_eq!(
        raw.value,
        CellValue::Formula(FormulaCell {
            expression: "=A1+B1*3".to_owned(),
        })
    );

    let evaluated = service
        .evaluated_cell_by_a1(&document.id, &worksheet_id, "C1")
        .expect("evaluated")
        .expect("evaluated exists");
    assert_eq!(evaluated.value, CellValue::Number(16.0));
}

#[test]
fn formula_dependencies_support_absolute_references_and_empty_as_zero() {
    let mut service = service();
    let document = service.create_document("Dependencies");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A1", CellValue::Number(4.0))
        .expect("A1");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B1", "=$A$1*2")
        .expect("B1");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "C1", "=B1+Z99")
        .expect("C1");

    let evaluated = service
        .evaluated_cell_by_a1(&document.id, &worksheet_id, "C1")
        .expect("evaluate C1")
        .expect("C1 exists");
    assert_eq!(evaluated.value, CellValue::Number(8.0));
}

#[test]
fn formula_cycle_division_and_non_numeric_reference_are_typed_errors() {
    let mut service = service();
    let document = service.create_document("Errors");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A1", "=B1+1")
        .expect("A1 formula");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B1", "=A1+1")
        .expect("B1 formula");

    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "A1"),
        Err(SheetError::FormulaCycle)
    );

    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "C1", "=1/0")
        .expect("C1 formula");
    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "C1"),
        Err(SheetError::FormulaDivisionByZero)
    );

    let document = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "D1",
            CellValue::Text("text".to_owned()),
        )
        .expect("D1");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "E1", "=D1+1")
        .expect("E1 formula");
    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "E1"),
        Err(SheetError::FormulaNonNumericReference)
    );
}

#[test]
fn invalid_formula_is_rejected_before_document_mutation() {
    let mut service = service();
    let document = service.create_document("Invalid");
    let worksheet_id = document.worksheets[0].id.clone();

    assert_eq!(
        service.set_formula_by_a1(&document.id, &worksheet_id, "A1", "=1+"),
        Err(SheetError::InvalidFormula)
    );
    assert_eq!(
        service
            .get_document(&document.id)
            .expect("document still exists")
            .revision,
        0
    );
}

#[test]
fn controller_exposes_raw_formula_and_evaluated_value_without_business_logic() {
    let service = service();
    let mut controller = SheetController::new(service);
    let document = controller.create("Controller Formula");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = controller
        .set_formula_a1(&document.id, &worksheet_id, "A1", "=(2+3)*4")
        .expect("formula");

    assert_eq!(
        controller
            .cell_a1(&document.id, &worksheet_id, "A1")
            .expect("raw request")
            .expect("raw value")
            .value,
        CellValueView::Formula("=(2+3)*4".to_owned())
    );
    assert_eq!(
        controller
            .evaluated_cell_a1(&document.id, &worksheet_id, "A1")
            .expect("evaluate request")
            .expect("evaluated value")
            .value,
        CellValueView::Number(20.0)
    );
}
