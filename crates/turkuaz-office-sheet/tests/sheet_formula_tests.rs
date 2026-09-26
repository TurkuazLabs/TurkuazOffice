// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/tests/sheet_formula_tests.rs
// # 📌 Amac: M2 Basic Formula Engine parser, evaluator, dependency ve SheetService lifecycle davranislarini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.3.0
// Aciklama: Arithmetic precedence, A1, SUM, missing-zero, cycle, div0, type/range hatasi ve formula revision no-op kontratlarini kilitler
// Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View

use turkuaz_office_sheet::{
    CellValue, EvaluatedCellValue, FormulaError, FormulaExpression, FormulaParseError,
    FormulaParserTool, InMemorySheetDocumentRepository, SequentialSheetIdTool, SheetController,
    SheetError, SheetService,
};

fn service() -> SheetService<InMemorySheetDocumentRepository, SequentialSheetIdTool> {
    SheetService::new(
        InMemorySheetDocumentRepository::new(),
        SequentialSheetIdTool::new(),
    )
}

#[test]
fn formula_parser_preserves_arithmetic_precedence_and_parentheses() {
    let parsed = FormulaParserTool::parse("=1+2*3").expect("formula parse");
    let FormulaExpression::Binary { .. } = parsed else {
        panic!("binary expression");
    };

    assert!(FormulaParserTool::parse("=(1+2)*3").is_ok());
    assert_eq!(
        FormulaParserTool::parse("1+2"),
        Err(FormulaParseError::MissingEquals)
    );
    assert_eq!(
        FormulaParserTool::parse("=UNKNOWN(1)"),
        Err(FormulaParseError::UnsupportedFunction)
    );
}

#[test]
fn formula_engine_evaluates_references_precedence_unary_and_missing_as_zero() {
    let mut service = service();
    let document = service.create_document("Formula");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "A1",
            CellValue::Number(10.0),
        )
        .expect("A1");
    let document = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "B1",
            CellValue::Number(5.0),
        )
        .expect("B1");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "C1", "=A1+B1*2")
        .expect("C1 formula");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "D1", "=-(A1-2)")
        .expect("D1 formula");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "E1", "=Z99+1")
        .expect("E1 formula");

    assert_eq!(
        service
            .evaluated_cell_by_a1(&document.id, &worksheet_id, "C1")
            .expect("C1 evaluation"),
        Some(EvaluatedCellValue::Number(20.0))
    );
    assert_eq!(
        service
            .evaluated_cell_by_a1(&document.id, &worksheet_id, "D1")
            .expect("D1 evaluation"),
        Some(EvaluatedCellValue::Number(-8.0))
    );
    assert_eq!(
        service
            .evaluated_cell_by_a1(&document.id, &worksheet_id, "E1")
            .expect("E1 evaluation"),
        Some(EvaluatedCellValue::Number(1.0))
    );
}

#[test]
fn formula_sum_accepts_ranges_and_expression_arguments() {
    let mut service = service();
    let document = service.create_document("SUM");
    let worksheet_id = document.worksheets[0].id.clone();

    let mut current = document;
    for (reference, value) in [("A1", 1.0), ("A2", 2.0), ("B1", 3.0), ("B2", 4.0)] {
        current = service
            .set_cell_by_a1(
                &current.id,
                &worksheet_id,
                reference,
                CellValue::Number(value),
            )
            .expect("seed number");
    }
    current = service
        .set_formula_by_a1(
            &current.id,
            &worksheet_id,
            "C1",
            "=SUM(A1:B2, 5, A1*2)",
        )
        .expect("SUM formula");

    assert_eq!(
        service
            .evaluated_cell_by_a1(&current.id, &worksheet_id, "C1")
            .expect("SUM evaluation"),
        Some(EvaluatedCellValue::Number(17.0))
    );
}

#[test]
fn formula_engine_reports_cycle_division_type_and_range_errors() {
    let mut service = service();
    let document = service.create_document("Errors");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A1", "=B1")
        .expect("A1 formula");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B1", "=A1")
        .expect("B1 formula");
    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "A1"),
        Err(SheetError::Formula(FormulaError::CircularReference))
    );

    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "C1", "=1/0")
        .expect("C1 formula");
    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "C1"),
        Err(SheetError::Formula(FormulaError::DivisionByZero))
    );

    let document = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "D1",
            CellValue::Text("text".to_owned()),
        )
        .expect("D1 text");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "E1", "=D1+1")
        .expect("E1 formula");
    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "E1"),
        Err(SheetError::Formula(FormulaError::TypeMismatch))
    );

    let document = service
        .set_formula_by_a1(
            &document.id,
            &worksheet_id,
            "F1",
            "=SUM(A1:XFD1048576)",
        )
        .expect("range formula");
    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "F1"),
        Err(SheetError::Formula(FormulaError::RangeTooLarge))
    );
}

#[test]
fn formula_parser_rejects_overlong_source() {
    let source = format!("={}", "1".repeat(8_192));

    assert_eq!(
        FormulaParserTool::parse(&source),
        Err(FormulaParseError::TooLong)
    );
}

#[test]
fn formula_engine_bounds_dependency_chain_depth() {
    let mut service = service();
    let document = service.create_document("Dependency Depth");
    let worksheet_id = document.worksheets[0].id.clone();
    let mut current = document;

    for row in 1..=130 {
        let reference = format!("A{row}");
        let next = format!("=A{}", row + 1);
        current = service
            .set_formula_by_a1(&current.id, &worksheet_id, &reference, &next)
            .expect("dependency formula");
    }
    current = service
        .set_cell_by_a1(
            &current.id,
            &worksheet_id,
            "A131",
            CellValue::Number(1.0),
        )
        .expect("dependency terminal");

    assert_eq!(
        service.evaluated_cell_by_a1(&current.id, &worksheet_id, "A1"),
        Err(SheetError::Formula(FormulaError::DependencyDepthExceeded))
    );
}

#[test]
fn formula_engine_bounds_expression_recursion_depth() {
    let mut service = service();
    let document = service.create_document("Depth");
    let worksheet_id = document.worksheets[0].id.clone();
    let source = format!(
        "={}",
        std::iter::repeat_n("1", 140).collect::<Vec<_>>().join("+")
    );
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A1", &source)
        .expect("deep expression formula");

    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "A1"),
        Err(SheetError::Formula(FormulaError::ExpressionDepthExceeded))
    );
}

#[test]
fn formula_set_validates_source_and_same_formula_is_revision_noop() {
    let mut service = service();
    let document = service.create_document("Revision");
    let worksheet_id = document.worksheets[0].id.clone();

    assert_eq!(
        service.set_formula_by_a1(&document.id, &worksheet_id, "A1", "1+2"),
        Err(SheetError::Formula(FormulaError::Parse(
            FormulaParseError::MissingEquals
        )))
    );

    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A1", "=1+2")
        .expect("formula");
    assert_eq!(document.revision, 1);

    let same = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A1", "=1+2")
        .expect("same formula");
    assert_eq!(same.revision, 1);
}

#[test]
fn controller_exposes_raw_formula_and_evaluated_number_separately() {
    let service = service();
    let mut controller = SheetController::new(service);
    let document = controller.create("Controller Formula");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = controller
        .set_formula_a1(&document.id, &worksheet_id, "A1", "=2+3*4")
        .expect("controller formula");
    assert_eq!(document.revision, 1);

    let evaluated = controller
        .evaluated_cell_a1(&document.id, &worksheet_id, "A1")
        .expect("controller evaluation")
        .expect("value");
    assert_eq!(
        evaluated,
        turkuaz_office_sheet::EvaluatedCellValueView::Number(14.0)
    );

    let raw = controller
        .cell_a1(&document.id, &worksheet_id, "A1")
        .expect("raw query")
        .expect("raw cell");
    assert_eq!(
        raw.value,
        turkuaz_office_sheet::CellValueView::Formula("=2+3*4".to_owned())
    );
}
