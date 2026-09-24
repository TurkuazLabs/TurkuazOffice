# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/template-foundation-test-matrix.md
# 📌 Amac: M1 Writer template katalog, canonical document creation ve Desktop quick-create test kapsamlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Built-in YAML katalog, typed metadata, style skeleton ve normal Writer lifecycle bariyerlerini sabitler

Bagimli Oldugu Katman: Documentation

# Template Foundation Test Matrix

## Catalog

- Built-in source: crates/turkuaz-office-writer/templates/builtin.yml.
- Catalog version: 1.
- Stable template id.
- Language name/description key.
- quick_create presentation metadata.
- Duplicate id reddi.
- Empty catalog/template paragraph reddi.
- Font family/size limit validation.

## Built-in minimum

- blank: standard empty Writer document.
- letter: four paragraph alignment/spacing skeleton.
- report: title, subtitle and body style skeleton.

Template content user language text'i gommez. UI label localization Language katmaninda kalir.

## Document lifecycle

Template create sonucunda normal WriterDocument uretilir.

- New document id.
- New node ids.
- revision = 0.
- Empty assets.
- Normal repository/history kaydi.
- Native file session untracked.
- Save/Open/TKO pipeline template-aware ozel dal kullanmaz.

## Frontend

- Startup template catalog load.
- Catalog failure document startup'i engellemez.
- Ribbon yalniz quick_create=true template'leri gosterir.
- Template ID View icinde magic string ile filtrelenmez.
- Template create normal unsaved-change guard kullanir.

## Regression

writer_template_tests.rs:

- stable catalog order.
- quick-create metadata.
- report canonical style skeleton.
- unknown template typed error.
- editor repository lifecycle.

writer_template_desktop_tests.rs:

- Desktop catalog.
- report create.
- native session isolation.
