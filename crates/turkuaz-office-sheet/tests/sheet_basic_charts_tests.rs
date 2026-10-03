// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/tests/sheet_basic_charts_tests.rs
// # 📌 Amac: M2 Basic Charts canonical definition, lifecycle ve formula-aware data projection davranislarini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.3.0
// Aciklama: Bar/Line/Pie chart creation, validation, data projection, revision ve Controller/View akislarini kabul testi yapar
// Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View

use turkuaz_office_sheet::{
    CellValue, ChartType, ChartTypeView, InMemorySheetDocumentRepository, SequentialSheetIdTool,
    SheetController, SheetError, SheetService,
};

fn service() -> SheetService<InMemorySheetDocumentRepository, SequentialSheetIdTool> {
    SheetService::new(
        InMemorySheetDocumentRepository::new(),
        SequentialSheetIdTool::new(),
    )
}

#[test]
fn chart_creation_is_canonical_and_revision_aware() {
    let mut service = service();
    let document = service.create_document("Charts");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .create_chart(
            &document.id,
            &worksheet_id,
            ChartType::Bar,
            "Revenue",
            0,
            2,
            0,
            1,
        )
        .expect("chart");

    assert_eq!(document.revision, 1);
    assert_eq!(document.charts.len(), 1);
    let chart = document.charts.values().next().expect("chart exists");
    assert_eq!(chart.title, "Revenue");
    assert_eq!(chart.chart_type, ChartType::Bar);
    assert_eq!(chart.start_row, 0);
    assert_eq!(chart.end_row, 2);
    assert_eq!(chart.category_column, 0);
    assert_eq!(chart.value_column, 1);
}

#[test]
fn chart_data_supports_formula_values() {
    let mut service = service();
    let document = service.create_document("Formula Chart");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "A1",
            CellValue::Text("Jan".to_owned()),
        )
        .expect("A1");
    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "B1", CellValue::Number(10.0))
        .expect("B1");
    let document = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "A2",
            CellValue::Text("Feb".to_owned()),
        )
        .expect("A2");
    let document = service
        .set_formula_by_a1(&document.id, &worksheet_id, "B2", "=B1*2")
        .expect("B2 formula");
    let document = service
        .create_chart(
            &document.id,
            &worksheet_id,
            ChartType::Line,
            "Monthly",
            0,
            1,
            0,
            1,
        )
        .expect("chart");

    let chart_id = document.charts.keys().next().expect("chart id").clone();
    let points = service
        .chart_data(&document.id, &chart_id)
        .expect("chart data");

    assert_eq!(points.len(), 2);
    assert_eq!(points[0].category, "Jan");
    assert_eq!(points[0].value, 10.0);
    assert_eq!(points[1].category, "Feb");
    assert_eq!(points[1].value, 20.0);
}

#[test]
fn chart_validation_rejects_invalid_title_range_and_types() {
    let mut service = service();
    let document = service.create_document("Validation");
    let worksheet_id = document.worksheets[0].id.clone();

    assert_eq!(
        service.create_chart(&document.id, &worksheet_id, ChartType::Pie, " ", 0, 1, 0, 1,),
        Err(SheetError::InvalidChartTitle)
    );

    assert_eq!(
        service.create_chart(
            &document.id,
            &worksheet_id,
            ChartType::Pie,
            "Bad Range",
            2,
            1,
            0,
            1,
        ),
        Err(SheetError::InvalidChartRange)
    );

    let document = service
        .set_cell_by_a1(&document.id, &worksheet_id, "A1", CellValue::Number(1.0))
        .expect("category");
    let document = service
        .set_cell_by_a1(
            &document.id,
            &worksheet_id,
            "B1",
            CellValue::Text("not-number".to_owned()),
        )
        .expect("value");
    let document = service
        .create_chart(
            &document.id,
            &worksheet_id,
            ChartType::Pie,
            "Typed",
            0,
            0,
            0,
            1,
        )
        .expect("chart");
    let chart_id = document.charts.keys().next().expect("chart id").clone();

    assert_eq!(
        service.chart_data(&document.id, &chart_id),
        Err(SheetError::ChartCategoryNotText)
    );
}

#[test]
fn controller_exposes_chart_definition_and_data_view() {
    let mut controller = SheetController::new(service());
    let document = controller.create("Controller Charts");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = controller
        .set_cell_a1(
            &document.id,
            &worksheet_id,
            "A1",
            CellValue::Text("Q1".to_owned()),
        )
        .expect("category");
    let document = controller
        .set_cell_a1(&document.id, &worksheet_id, "B1", CellValue::Number(42.0))
        .expect("value");
    let document = controller
        .create_chart(
            &document.id,
            &worksheet_id,
            ChartType::Bar,
            "Quarter",
            0,
            0,
            0,
            1,
        )
        .expect("chart view");

    assert_eq!(document.charts.len(), 1);
    assert_eq!(document.charts[0].chart_type, ChartTypeView::Bar);
    let data = controller
        .chart_data(&document.id, &document.charts[0].id)
        .expect("data view");
    assert_eq!(data.points.len(), 1);
    assert_eq!(data.points[0].category, "Q1");
    assert_eq!(data.points[0].value, 42.0);
}
