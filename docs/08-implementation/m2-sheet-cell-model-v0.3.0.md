# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m2-sheet-cell-model-v0.3.0.md
# 📌 Amac: M2 Sheet Cell Model implementation kapsam ve mimari sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.0
# Aciklama: New Sheet crate, sparse workbook domain, A1 Tool, Repo/Service/Controller/View katmanlarini tanimlar

Bagimli Oldugu Katman: Documentation

# M2 Sheet Cell Model

## Crate

crates/turkuaz-office-sheet

Katmanlar:

Controller -> Service -> Repo -> Tool -> View -> Language

Public API lib.rs uzerinden kontrollu acilir.

## Canonical document

SheetDocument:

- Core DocumentId.
- Core DocumentSchemaVersion.
- title.
- revision.
- worksheets.

Worksheet:

- stable WorksheetId.
- name.
- sparse BTreeMap<CellAddress, CellValue>.

CellAddress canonical olarak zero-based row/column tasir.

## Cell values

Initial Cell Model slice:

- text.
- finite number.
- boolean.

M2 Basic Formula Engine follow-on parcasi daha sonra canonical CellValue::Formula source varyantini ekledi. Formula parser/evaluation detaylari m2-sheet-formula-engine-v0.3.0.md icindedir.

## A1 boundary

CellReferenceTool presentation/file/formula tarafinin A1 text representation'ini canonical CellAddress'e cevirir.

Supported simple A1:

- case-insensitive column letters.
- one-based row number.
- XLSX grid max XFD1048576.

Absolute/mixed reference syntax henuz yoktur.

## Business rules

SheetService:

- default one worksheet create.
- shared Core default document title.
- address validation.
- cell value validation.
- sparse set/get/clear.
- revision mutation.
- A1 Tool coordination.
- Repo persistence.

Controller logic tasimaz.

## Determinism

BTreeMap tercih edildi.

100000 cell icin performans benchmark roadmap'te ayri M2 parcasi olarak kalir; bu faz benchmark sonucu iddia etmez.

## Version

M2 development workspace/project/Desktop package metadata 0.3.0'a tasindi.
