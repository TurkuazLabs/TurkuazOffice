// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/tests/sheet_formula_engine_tests.rs
// # 📌 Amac: M2 Formula Engine parse, function library, evaluation, dependency ve error davranislarini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.8.0
// Aciklama: Same-sheet arithmetic, range/function, comparison, lazy IF, absolute A1, cycle ve Controller/View akislarini kabul testi yapar
// Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View

use turkuaz_office_sheet::config::constants::{
    MAX_FORMULA_OPERATIONS, MAX_FORMULA_PARSE_DEPTH, MAX_FORMULA_RANGE_CELLS,
};
use turkuaz_office_sheet::{
    CellValue, CellValueView, FormulaCell, FormulaComparisonOperator, FormulaExpression,
    FormulaFunction, FormulaTool, FormulaToolError,
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
fn formula_tool_rejects_excessive_nesting_and_operator_count() {
    let nesting = MAX_FORMULA_PARSE_DEPTH + 1;
    let nested = format!("={}1{}", "(".repeat(nesting), ")".repeat(nesting));
    assert_eq!(
        FormulaTool::parse(&nested),
        Err(FormulaToolError::TooComplex)
    );

    let mut operation_heavy = "=1".to_owned();
    for _ in 0..=MAX_FORMULA_OPERATIONS {
        operation_heavy.push_str("+1");
    }
    assert_eq!(
        FormulaTool::parse(&operation_heavy),
        Err(FormulaToolError::TooComplex)
    );
}


#[test]
fn formula_tool_parses_ranges_functions_comparisons_and_both_argument_separators() {
    let parsed = FormulaTool::parse("=SUM(A1:B3, 5, MAX(C1;C2))").expect("function formula");
    let FormulaExpression::Function {
        function: FormulaFunction::Sum,
        arguments,
    } = parsed.expression
    else {
        panic!("SUM function expected");
    };
    assert_eq!(arguments.len(), 3);
    assert!(matches!(arguments[0], FormulaExpression::Range(_)));
    assert!(matches!(
        arguments[2],
        FormulaExpression::Function {
            function: FormulaFunction::Max,
            ..
        }
    ));

    let comparison = FormulaTool::parse("=A1>=B1").expect("comparison");
    assert!(matches!(
        comparison.expression,
        FormulaExpression::Comparison {
            operator: FormulaComparisonOperator::GreaterThanOrEqual,
            ..
        }
    ));

    assert_eq!(
        FormulaTool::parse("=UNKNOWN(A1)"),
        Err(FormulaToolError::UnknownFunction)
    );
    assert_eq!(
        FormulaTool::parse("=IF(1,2)"),
        Err(FormulaToolError::InvalidArgumentCount)
    );
}

#[test]
fn aggregate_functions_support_ranges_scalars_formulas_and_ignore_non_numeric_cells() {
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

    for (reference, expression, expected) in [
        ("B1", "=SUM(A1:A5,5)", 65.0),
        ("B2", "=AVERAGE(A1:A5)", 20.0),
        ("B3", "=MIN(A1:A5)", 10.0),
        ("B4", "=MAX(A1:A5)", 30.0),
    ] {
        let document = service
            .set_formula_by_a1(&document.id, &worksheet_id, reference, expression)
            .expect("function formula");
        let evaluated = service
            .evaluated_cell_by_a1(&document.id, &worksheet_id, reference)
            .expect("evaluate function")
            .expect("formula cell");
        assert_eq!(evaluated.value, CellValue::Number(expected));
    }
}

#[test]
fn comparisons_return_numeric_booleans_and_if_evaluates_only_selected_branch() {
    let mut service = service();
    let document = service.create_document("If");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A1", CellValue::Number(10.0))
        .expect("A1");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B1", "=A1>5")
        .expect("comparison");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B2", "=A1<>10")
        .expect("comparison false");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B3", "=IF(A1>=10,42,1/0)")
        .expect("lazy true");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B4", "=IF(A1<10,1/0,7)")
        .expect("lazy false");

    for (reference, expected) in [("B1", 1.0), ("B2", 0.0), ("B3", 42.0), ("B4", 7.0)] {
        let evaluated = service
            .evaluated_cell_by_a1(&document.id, &worksheet_id, reference)
            .expect("evaluate")
            .expect("formula cell");
        assert_eq!(evaluated.value, CellValue::Number(expected));
    }
}

#[test]
fn aggregate_functions_preserve_cycle_detection_and_enforce_range_expansion_limit() {
    let mut service = service();
    let document = service.create_document("Function Safety");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A1", "=SUM(A1:A2)")
        .expect("cycle formula");
    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "A1"),
        Err(SheetError::FormulaCycle)
    );

    let end_row = MAX_FORMULA_RANGE_CELLS as u32 + 1;
    let expression = format!("=SUM(B1:B{end_row})");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "C1", &expression)
        .expect("large range formula");
    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "C1"),
        Err(SheetError::FormulaRangeTooLarge)
    );
}

#[test]
fn standalone_range_expression_is_rejected_as_non_scalar() {
    let mut service = service();
    let document = service.create_document("Range Scalar");
    let worksheet_id = document.worksheets[0].id.clone();
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A1", "=B1:B2")
        .expect("range formula");

    assert_eq!(
        service.evaluated_cell_by_a1(&document.id, &worksheet_id, "A1"),
        Err(SheetError::FormulaRangeNotAllowed)
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
