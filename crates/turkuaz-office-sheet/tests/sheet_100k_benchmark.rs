// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-sheet/tests/sheet_100k_benchmark.rs
// # 📌 Amac: M2 Sheet 100000-cell benchmark profilini deterministic workload ile olcer
// # 📌 Modul - FileType: Benchmark - Rust
// Version: 0.3.0
// Aciklama: Sparse model build, deterministic View projection ve 100000-row query surelerini normal regression'dan ayri raporlar
// Bagimli Oldugu Katman: Service -> Repo -> View

use std::collections::BTreeMap;
use std::time::Instant;

use turkuaz_office_core::{DocumentId, DocumentSchemaVersion};
use turkuaz_office_sheet::{
    CellAddress, CellValue, InMemorySheetDocumentRepository, SequentialSheetIdTool, SheetDocument,
    SheetDocumentRepository, SheetDocumentView, SheetRange, SheetService, SheetSort,
    SheetSortDirection, Worksheet, WorksheetId,
};

const BENCHMARK_CELL_COUNT: u32 = 100_000;
const BENCHMARK_COLUMN: u32 = 0;

fn benchmark_document() -> SheetDocument {
    let mut cells = BTreeMap::new();
    for row in 0..BENCHMARK_CELL_COUNT {
        cells.insert(
            CellAddress {
                row,
                column: BENCHMARK_COLUMN,
            },
            CellValue::Number(f64::from(row)),
        );
    }

    SheetDocument {
        id: DocumentId::new("sheet-document-benchmark-100k"),
        title: "100K Benchmark".to_owned(),
        schema_version: DocumentSchemaVersion::current(),
        revision: 0,
        worksheets: vec![Worksheet {
            id: WorksheetId::new("worksheet-benchmark-100k"),
            name: "Benchmark".to_owned(),
            cells,
        }],
        cell_formats: BTreeMap::new(),
        conditional_formats: BTreeMap::new(),
        tables: BTreeMap::new(),
        charts: BTreeMap::new(),
    }
}

#[test]
#[ignore = "performance profile; run explicitly with --ignored --nocapture"]
fn benchmark_100k_sparse_build_and_view_projection() {
    let build_started = Instant::now();
    let document = benchmark_document();
    let build_elapsed = build_started.elapsed();

    assert_eq!(
        document.worksheets[0].cells.len(),
        BENCHMARK_CELL_COUNT as usize
    );

    let view_started = Instant::now();
    let view = SheetDocumentView::from(document);
    let view_elapsed = view_started.elapsed();

    assert_eq!(view.worksheets[0].cell_count, BENCHMARK_CELL_COUNT as usize);
    assert_eq!(view.worksheets[0].cells.first().expect("first").row, 0);
    assert_eq!(
        view.worksheets[0].cells.last().expect("last").row,
        BENCHMARK_CELL_COUNT - 1
    );

    println!(
        "sheet_100k sparse_build_ms={} view_projection_ms={}",
        build_elapsed.as_millis(),
        view_elapsed.as_millis()
    );
}

#[test]
#[ignore = "performance profile; run explicitly with --ignored --nocapture"]
fn benchmark_100k_sorted_row_query() {
    let document = benchmark_document();
    let document_id = document.id.clone();
    let worksheet_id = document.worksheets[0].id.clone();

    let mut repository = InMemorySheetDocumentRepository::new();
    repository.save(document);
    let service = SheetService::new(repository, SequentialSheetIdTool::new());

    let query_started = Instant::now();
    let rows = service
        .query_rows(
            &document_id,
            &worksheet_id,
            SheetRange {
                start_row: 0,
                end_row: BENCHMARK_CELL_COUNT - 1,
                start_column: BENCHMARK_COLUMN,
                end_column: BENCHMARK_COLUMN,
            },
            None,
            Some(SheetSort {
                column: BENCHMARK_COLUMN,
                direction: SheetSortDirection::Descending,
            }),
        )
        .expect("100k query");
    let query_elapsed = query_started.elapsed();

    assert_eq!(rows.len(), BENCHMARK_CELL_COUNT as usize);
    assert_eq!(rows.first(), Some(&(BENCHMARK_CELL_COUNT - 1)));
    assert_eq!(rows.last(), Some(&0));

    println!(
        "sheet_100k sorted_row_query_ms={}",
        query_elapsed.as_millis()
    );
}
