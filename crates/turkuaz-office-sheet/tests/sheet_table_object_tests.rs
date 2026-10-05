// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/tests/sheet_table_object_tests.rs
// # 📌 Amac: Canonical Sheet table object yasam dongusu ve range kurallarini regression testiyle dogrular
// # 📌 Modul - FileType: Test - Rust
// Version: 0.6.0
// Aciklama: Default ad, revision, overlap, minimum data satiri, remove ve Controller/View davranisini sabitler
// Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool -> View

use turkuaz_office_sheet::{
    InMemorySheetDocumentRepository, SequentialSheetIdTool, SheetController, SheetError,
    SheetRange, SheetService, TableId,
};

fn service() -> SheetService<InMemorySheetDocumentRepository, SequentialSheetIdTool> {
    SheetService::new(
        InMemorySheetDocumentRepository::new(),
        SequentialSheetIdTool::new(),
    )
}

#[test]
fn table_creation_assigns_deterministic_names_and_revision() {
    let mut service = service();
    let document = service.create_document("Tables");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = service
        .create_table(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 0,
                end_row: 3,
                start_column: 0,
                end_column: 2,
            },
        )
        .expect("first table");

    assert_eq!(document.revision, 1);
    assert_eq!(document.tables.len(), 1);
    let first = document.tables.values().next().expect("table");
    assert_eq!(first.name, "Table1");
    assert_eq!(first.range.start_row, 0);
    assert_eq!(first.range.end_row, 3);

    let document = service
        .create_table(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 0,
                end_row: 2,
                start_column: 4,
                end_column: 5,
            },
        )
        .expect("second table");

    assert_eq!(document.revision, 2);
    assert_eq!(document.tables.len(), 2);
    assert!(document.tables.values().any(|table| table.name == "Table2"));
}

#[test]
fn table_range_requires_header_plus_data_and_rejects_overlap() {
    let mut service = service();
    let document = service.create_document("Validation");
    let worksheet_id = document.worksheets[0].id.clone();

    assert_eq!(
        service.create_table(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 0,
                end_row: 0,
                start_column: 0,
                end_column: 2,
            },
        ),
        Err(SheetError::InvalidTableRange)
    );

    let document = service
        .create_table(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 0,
                end_row: 4,
                start_column: 0,
                end_column: 2,
            },
        )
        .expect("base table");

    assert_eq!(
        service.create_table(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 2,
                end_row: 6,
                start_column: 2,
                end_column: 4,
            },
        ),
        Err(SheetError::TableRangeOverlap)
    );
    assert_eq!(
        service
            .get_document(&document.id)
            .expect("document")
            .revision,
        1
    );
}

#[test]
fn table_remove_is_typed_and_increments_revision() {
    let mut service = service();
    let document = service.create_document("Remove");
    let worksheet_id = document.worksheets[0].id.clone();
    let document = service
        .create_table(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 0,
                end_row: 2,
                start_column: 0,
                end_column: 1,
            },
        )
        .expect("table");
    let table_id = document.tables.keys().next().expect("table id").clone();

    let document = service
        .remove_table(&document.id, &table_id)
        .expect("remove table");
    assert_eq!(document.revision, 2);
    assert!(document.tables.is_empty());

    assert_eq!(
        service.remove_table(&document.id, &TableId::new("table-missing")),
        Err(SheetError::TableNotFound)
    );
}

#[test]
fn controller_exposes_table_view_without_business_logic() {
    let service = service();
    let mut controller = SheetController::new(service);
    let document = controller.create("Controller Table");
    let worksheet_id = document.worksheets[0].id.clone();

    let document = controller
        .create_table(
            &document.id,
            &worksheet_id,
            SheetRange {
                start_row: 1,
                end_row: 5,
                start_column: 1,
                end_column: 3,
            },
        )
        .expect("controller create table");

    assert_eq!(document.tables.len(), 1);
    let table = &document.tables[0];
    assert_eq!(table.name, "Table1");
    assert_eq!(table.start_row, 1);
    assert_eq!(table.end_row, 5);
    assert_eq!(table.start_column, 1);
    assert_eq!(table.end_column, 3);
}
