# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/README.md
# 📌 Amac: Turkuaz Office detayli dokumantasyon haritasini ve okuma sirasini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.3.1
# Aciklama: Product, architecture, development, operations, roadmap, ADR, quality ve R2 Sheet Desktop dokumanlarini indeksler

Bagimli Oldugu Katman: Documentation

# Turkuaz Office Docs

## 01 Product

- `01-product/vision.md`
- `01-product/scope-v0.1.md`

## 02 Architecture

- `02-architecture/system-context.md`
- `02-architecture/layers.md`
- `02-architecture/document-model.md`
- `02-architecture/native-document-format.md`
- `02-architecture/format-adapters.md`
- `02-architecture/storage.md`
- `02-architecture/autosave-recovery.md`
- `02-architecture/font-layout-engine.md`
- `02-architecture/clipboard-contract.md`
- `02-architecture/print-preview.md`
- `02-architecture/localization.md`
- `02-architecture/platform-strategy.md`
- `02-architecture/offline-sync.md`
- `02-architecture/plugin-system.md`
- `02-architecture/plugin-api-contract.md`

## 03 Development

- `03-development/coding-standard.md`
- `03-development/local-development.md`
- `03-development/testing-strategy.md`

## 04 Operations

- `04-operations/security.md`
- `04-operations/untrusted-document-security.md`
- `04-operations/file-locking-external-change.md`
- `04-operations/release-distribution.md`
- `04-operations/licensing-third-party.md`
- `04-operations/versioning-release.md`

## 05 Roadmap

- `05-roadmap/roadmap.md`

## 06 ADR

- `06-adr/0001-monorepo.md`
- `06-adr/0002-rust-core.md`
- `06-adr/0003-web-compatible-core.md`
- `06-adr/0004-ui-framework-deferred.md`
- `06-adr/0005-canonical-native-format.md`
- `06-adr/0006-schema-versioning.md`
- `06-adr/0007-offline-first-sync-boundary.md`
- `06-adr/0008-macros-disabled-by-default.md`
- `06-adr/0009-versioned-plugin-capabilities.md`
- `06-adr/0010-writer-logical-offset.md`
- `06-adr/0011-writer-undo-redo-baseline.md`
- `06-adr/0012-solidjs-desktop-ui.md`
- `06-adr/0013-desktop-ipc-thin-bridge.md`
- `06-adr/0014-ime-rich-text-contenteditable.md`
- `06-adr/0015-caret-typing-style-ribbon-typography.md`
- `06-adr/0016-tko-v1-local-safe-save.md`
- `06-adr/0017-autosave-recovery-snapshot.md`
- `06-adr/0018-external-change-cooperative-lock.md`
- `06-adr/0019-twip-page-layout-font-fallback.md`
- `06-adr/0020-sheet-basic-formula-evaluation.md`

## 07 Quality

- `07-quality/compatibility-matrix.md`
- `07-quality/definition-of-done.md`
- `07-quality/accessibility-keyboard.md`
- `07-quality/performance-budgets.md`
- `07-quality/foundation-hardening-checklist.md`
- `07-quality/writer-domain-test-matrix.md`
- `07-quality/desktop-shell-test-matrix.md`
- `07-quality/rich-text-ime-test-matrix.md`
- `07-quality/typography-ribbon-test-matrix.md`
- `07-quality/local-open-save-test-matrix.md`
- `07-quality/autosave-recovery-test-matrix.md`
- `07-quality/external-change-test-matrix.md`
- `07-quality/font-layout-test-matrix.md`
- `07-quality/print-preview-test-matrix.md`
- `07-quality/docx-minimum-test-matrix.md`
- `07-quality/pdf-export-test-matrix.md`
- `07-quality/recent-files-test-matrix.md`
- `07-quality/file-associations-test-matrix.md`
- `07-quality/template-foundation-test-matrix.md`
- `07-quality/turkish-english-ui-test-matrix.md`
- `07-quality/keyboard-only-smoke-test.md`
- `07-quality/sheet-cell-model-test-matrix.md`
- `07-quality/sheet-csv-xlsx-test-matrix.md`
- `07-quality/sheet-formula-engine-test-matrix.md`
- `07-quality/sheet-format-filter-sort-test-matrix.md`
- `07-quality/sheet-basic-charts-test-matrix.md`
- `07-quality/sheet-100k-benchmark-profile.md`

## 08 Implementation

- `08-implementation/m1-writer-entry.md`
- `08-implementation/m1-writer-domain-v0.2.0.md`
- `08-implementation/m1-writer-validation.md`
- `08-implementation/m1-desktop-shell-v0.2.0.md`
- `08-implementation/m1-desktop-shell-validation.md`
- `08-implementation/m1-rich-text-ime-v0.2.0.md`
- `08-implementation/m1-rich-text-ime-validation.md`
- `08-implementation/m1-typography-ribbon-v0.2.0.md`
- `08-implementation/m1-typography-ribbon-validation.md`
- `08-implementation/m1-local-open-save-v0.2.0.md`
- `08-implementation/m1-local-open-save-validation.md`
- `08-implementation/m1-autosave-recovery-v0.2.0.md`
- `08-implementation/m1-autosave-recovery-validation.md`
- `08-implementation/m1-external-change-v0.2.0.md`
- `08-implementation/m1-external-change-validation.md`
- `08-implementation/m1-font-layout-v0.2.0.md`
- `08-implementation/m1-font-layout-validation.md`
- `08-implementation/m1-print-preview-v0.2.0.md`
- `08-implementation/m1-print-preview-validation.md`
- `08-implementation/m1-docx-minimum-v0.2.0.md`
- `08-implementation/m1-docx-minimum-validation.md`
- `08-implementation/m1-pdf-export-v0.2.0.md`
- `08-implementation/m1-pdf-export-validation.md`
- `08-implementation/m1-recent-files-v0.2.0.md`
- `08-implementation/m1-recent-files-validation.md`
- `08-implementation/m1-file-associations-v0.2.0.md`
- `08-implementation/m1-file-associations-validation.md`
- `08-implementation/m1-template-foundation-v0.2.0.md`
- `08-implementation/m1-template-foundation-validation.md`
- `08-implementation/m1-turkish-english-ui-v0.2.0.md`
- `08-implementation/m1-turkish-english-ui-validation.md`
- `08-implementation/m1-keyboard-only-smoke-v0.2.0.md`
- `08-implementation/m1-keyboard-only-smoke-validation.md`
- `08-implementation/m2-sheet-cell-model-v0.3.0.md`
- `08-implementation/m2-sheet-cell-model-validation.md`
- `08-implementation/m2-sheet-csv-xlsx-v0.3.0.md`
- `08-implementation/m2-sheet-csv-xlsx-validation.md`
- `08-implementation/m2-sheet-formula-engine-v0.3.0.md`
- `08-implementation/m2-sheet-formula-engine-validation.md`
- `08-implementation/m2-sheet-format-filter-sort-v0.3.0.md`
- `08-implementation/m2-sheet-format-filter-sort-validation.md`
- `08-implementation/m2-sheet-basic-charts-v0.3.0.md`
- `08-implementation/m2-sheet-basic-charts-validation.md`
- `08-implementation/m2-sheet-100k-benchmark-v0.3.0.md`
- `08-implementation/m2-sheet-100k-benchmark-validation.md`
- `08-implementation/r2-sheet-desktop-integration.md`

## Kural

Kod ile dokuman farkli gerceklikler olamaz. Public contract, schema, security veya architecture degisirse ayni degisiklik setinde docs guncellenir.

## Aktif implementation

- `08-implementation/m1-writer-entry.md`: Writer milestone giris plani.
- `08-implementation/m1-writer-domain-v0.2.0.md`: Headless Writer Domain gerceklesen kapsam ve bilinen sinirlar.
- `06-adr/0010-writer-logical-offset.md`: Selection offset semantigi.
- `06-adr/0011-writer-undo-redo-baseline.md`: Undo/redo baseline karari.
- `07-quality/writer-domain-test-matrix.md`: Writer headless domain test ve CI bariyeri.
- `08-implementation/m1-writer-validation.md`: Artifact validation raporu.


## M1 Desktop Shell ekleri

- `06-adr/0012-solidjs-desktop-ui.md`
- `06-adr/0013-desktop-ipc-thin-bridge.md`
- `06-adr/0014-ime-rich-text-contenteditable.md`
- `06-adr/0015-caret-typing-style-ribbon-typography.md`
- `06-adr/0016-tko-v1-local-safe-save.md`
- `06-adr/0017-autosave-recovery-snapshot.md`
- `06-adr/0018-external-change-cooperative-lock.md`
- `06-adr/0019-twip-page-layout-font-fallback.md`
- `07-quality/desktop-shell-test-matrix.md`
- `07-quality/rich-text-ime-test-matrix.md`
- `07-quality/typography-ribbon-test-matrix.md`
- `07-quality/local-open-save-test-matrix.md`
- `07-quality/autosave-recovery-test-matrix.md`
- `07-quality/external-change-test-matrix.md`
- `07-quality/font-layout-test-matrix.md`
- `07-quality/print-preview-test-matrix.md`
- `07-quality/docx-minimum-test-matrix.md`
- `07-quality/pdf-export-test-matrix.md`
- `07-quality/recent-files-test-matrix.md`
- `07-quality/file-associations-test-matrix.md`
- `07-quality/template-foundation-test-matrix.md`
- `07-quality/turkish-english-ui-test-matrix.md`
- `07-quality/keyboard-only-smoke-test.md`
- `08-implementation/m1-desktop-shell-v0.2.0.md`
- `08-implementation/m1-desktop-shell-validation.md`
- `08-implementation/m1-rich-text-ime-v0.2.0.md`
- `08-implementation/m1-rich-text-ime-validation.md`
- `08-implementation/m1-typography-ribbon-v0.2.0.md`
- `08-implementation/m1-typography-ribbon-validation.md`
- `08-implementation/m1-local-open-save-v0.2.0.md`
- `08-implementation/m1-local-open-save-validation.md`
- `08-implementation/m1-autosave-recovery-v0.2.0.md`
- `08-implementation/m1-autosave-recovery-validation.md`
- `08-implementation/m1-external-change-v0.2.0.md`
- `08-implementation/m1-external-change-validation.md`
- `08-implementation/m1-font-layout-v0.2.0.md`
- `08-implementation/m1-font-layout-validation.md`
- `08-implementation/m1-print-preview-v0.2.0.md`
- `08-implementation/m1-print-preview-validation.md`
- `08-implementation/m1-docx-minimum-v0.2.0.md`
- `08-implementation/m1-docx-minimum-validation.md`
- `08-implementation/m1-pdf-export-v0.2.0.md`
- `08-implementation/m1-pdf-export-validation.md`
- `08-implementation/m1-recent-files-v0.2.0.md`
- `08-implementation/m1-recent-files-validation.md`
- `08-implementation/m1-file-associations-v0.2.0.md`
- `08-implementation/m1-file-associations-validation.md`
- `08-implementation/m1-template-foundation-v0.2.0.md`
- `08-implementation/m1-template-foundation-validation.md`
- `08-implementation/m1-turkish-english-ui-v0.2.0.md`
- `08-implementation/m1-turkish-english-ui-validation.md`
- `08-implementation/m1-keyboard-only-smoke-v0.2.0.md`
- `08-implementation/m1-keyboard-only-smoke-validation.md`


## M2 Sheet ekleri

- `07-quality/sheet-cell-model-test-matrix.md`
- `08-implementation/m2-sheet-cell-model-v0.3.0.md`
- `08-implementation/m2-sheet-cell-model-validation.md`
- `07-quality/sheet-csv-xlsx-test-matrix.md`
- `08-implementation/m2-sheet-csv-xlsx-v0.3.0.md`
- `08-implementation/m2-sheet-csv-xlsx-validation.md`
- `06-adr/0020-sheet-basic-formula-evaluation.md`
- `07-quality/sheet-formula-engine-test-matrix.md`
- `08-implementation/m2-sheet-formula-engine-v0.3.0.md`
- `08-implementation/m2-sheet-formula-engine-validation.md`


## R2 Sheet Desktop ekleri

- `05-roadmap/roadmap.md`
- `08-implementation/r2-sheet-desktop-integration.md`
- `07-quality/sheet-cell-model-test-matrix.md`
- `07-quality/sheet-formula-engine-test-matrix.md`
- `07-quality/sheet-format-filter-sort-test-matrix.md`
