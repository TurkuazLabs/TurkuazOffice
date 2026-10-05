// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/tests/sheet_conditional_formatting_tests.rs
// # 📌 Amac: Canonical Sheet conditional formatting yasam dongusu ve match semantigini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.7.0
// Aciklama: Formula-aware match, priority, invalid condition, remove ve Controller/View davranisini sabitler
// Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View

use turkuaz_office_sheet::{
    CellValue, ConditionalFormatRuleId, InMemorySheetDocumentRepository, SequentialSheetIdTool,
    SheetConditionalFormatCondition, SheetConditionalFormatStyle, SheetController, SheetError,
    SheetRange, SheetService,
};

fn service() -> SheetService<InMemorySheetDocumentRepository, SequentialSheetIdTool> {
    SheetService::new(
        InMemorySheetDocumentRepository::new(),
        SequentialSheetIdTool::new(),
    )
}

#[test]
fn conditional_format_matches_evaluated_formula_values_and_ignores_formula_errors() {
    let mut service = service();
    let document = service.create_document("Conditional");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A1", CellValue::Number(10.0))
        .expect("A1");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A2", "=A1+5")
        .expect("A2");
    let document = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "A3",
            CellValue::Text("Turkuaz Office".to_owned()),
        )
        .expect("A3");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "A4", "=1/0")
        .expect("A4");

    let range = SheetRange {
        start_row: 0,
        end_row: 3,
        start_column: 0,
        end_column: 0,
    };
    let document = service
        .create_conditional_format(
            &document.id,
            &worksheet_id,
            range,
            SheetConditionalFormatCondition::NumberGreaterThan(12.0),
            SheetConditionalFormatStyle::Warning,
        )
        .expect("numeric rule");
    let document = service
        .create_conditional_format(
            &document.id,
            &worksheet_id,
            range,
            SheetConditionalFormatCondition::TextContains("turkuaz".to_owned()),
            SheetConditionalFormatStyle::Accent,
        )
        .expect("text rule");

    let matches = service
        .conditional_format_matches(&document.id, &worksheet_id, range)
        .expect("matches");

    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0].address.row, 1);
    assert_eq!(matches[0].style, SheetConditionalFormatStyle::Warning);
    assert_eq!(matches[1].address.row, 2);
    assert_eq!(matches[1].style, SheetConditionalFormatStyle::Accent);
}

#[test]
fn conditional_format_priority_is_deterministic_and_remove_reveals_next_rule() {
    let mut service = service();
    let document = service.create_document("Priority");
    let worksheet_id = document.worksheets[0].id.clone();
    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A1", CellValue::Number(20.0))
        .expect("A1");
    let range = SheetRange {
        start_row: 0,
        end_row: 0,
        start_column: 0,
        end_column: 0,
    };

    let document = service
        .create_conditional_format(
            &document.id,
            &worksheet_id,
            range,
            SheetConditionalFormatCondition::NumberGreaterThan(0.0),
            SheetConditionalFormatStyle::Warning,
        )
        .expect("first rule");
    let first_id = document
        .conditional_formats
        .values()
        .find(|rule| rule.priority == 1)
        .expect("priority 1")
        .id
        .clone();
    let document = service
        .create_conditional_format(
            &document.id,
            &worksheet_id,
            range,
            SheetConditionalFormatCondition::NumberGreaterThan(0.0),
            SheetConditionalFormatStyle::Success,
        )
        .expect("second rule");

    let matches = service
        .conditional_format_matches(&document.id, &worksheet_id, range)
        .expect("priority matches");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].style, SheetConditionalFormatStyle::Warning);

    let document = service
        .remove_conditional_format(&document.id, &first_id)
        .expect("remove first");
    let matches = service
        .conditional_format_matches(&document.id, &worksheet_id, range)
        .expect("second matches");
    assert_eq!(matches[0].style, SheetConditionalFormatStyle::Success);

    assert_eq!(
        service.remove_conditional_format(
            &document.id,
            &ConditionalFormatRuleId::new("conditional-format-missing"),
        ),
        Err(SheetError::ConditionalFormatRuleNotFound)
    );
}

#[test]
fn conditional_format_rejects_invalid_conditions_without_revision_change() {
    let mut service = service();
    let document = service.create_document("Validation");
    let worksheet_id = document.worksheets[0].id.clone();
    let range = SheetRange {
        start_row: 0,
        end_row: 0,
        start_column: 0,
        end_column: 0,
    };

    assert_eq!(
        service.create_conditional_format(
            &document.id,
            &worksheet_id,
            range,
            SheetConditionalFormatCondition::TextContains("   ".to_owned()),
            SheetConditionalFormatStyle::Accent,
        ),
        Err(SheetError::InvalidConditionalFormat)
    );
    assert_eq!(
        service.create_conditional_format(
            &document.id,
            &worksheet_id,
            range,
            SheetConditionalFormatCondition::NumberEquals(f64::NAN),
            SheetConditionalFormatStyle::Accent,
        ),
        Err(SheetError::InvalidConditionalFormat)
    );
    assert_eq!(
        service
            .get_document(&document.id)
            .expect("document")
            .revision,
        0
    );
}

#[test]
fn controller_exposes_conditional_format_metadata_without_business_logic() {
    let service = service();
    let mut controller = SheetController::new(service);
    let document = controller.create("Controller Conditional");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = controller
        .create_conditional_format(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 1,
                end_row: 4,
                start_column: 2,
                end_column: 3,
            },
            SheetConditionalFormatCondition::NumberLessThan(0.0),
            SheetConditionalFormatStyle::Warning,
        )
        .expect("controller create");

    assert_eq!(document.conditional_formats.len(), 1);
    let rule = &document.conditional_formats[0];
    assert_eq!(rule.start_row, 1);
    assert_eq!(rule.end_row, 4);
    assert_eq!(rule.start_column, 2);
    assert_eq!(rule.end_column, 3);
    assert_eq!(rule.priority, 1);
}
