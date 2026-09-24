# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-template-foundation-v0.2.0.md
# 📌 Amac: M1 Writer Template Foundation implementation kapsam ve mimari sinirlarini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Built-in YAML catalog, WriterTemplateService, Desktop IPC ve ribbon quick-create akisini tanimlar

Bagimli Oldugu Katman: Documentation

# M1 Template Foundation

## Mimari

Built-in YAML -> WriterTemplateCatalogTool -> TkoYamlTool -> WriterTemplateService -> WriterEditorService -> WriterController.

Desktop:

View -> Controller -> WriterSessionService -> TauriWriterTool -> Tauri Controller -> WriterDesktopService -> WriterController.

Template olusturulduktan sonra ayri runtime document tipi yoktur. Sonuc normal canonical WriterDocument'tir.

## Config source

Template metadata ve paragraph skeleton kod icinde inline tutulmaz.

Kaynak:

crates/turkuaz-office-writer/templates/builtin.yml

Catalog metadata:

- id
- name_key
- description_key
- quick_create
- paragraph style skeleton

## Built-in M1 set

- blank
- letter
- report

Bos template quick-create ribbon'da gosterilmez; mevcut Yeni Belge komutu blank akisidir.

Mektup ve Rapor quick-create olarak gosterilir.

## Localization boundary

Template katalogu kullaniciya gosterilecek Turkce/Ing metni document content'e gommez.

name_key ve description_key Language katmaninda cozulur.

Bu sayede sonraki Turkish + English UI fazi template document modelini degistirmeden ilerler.

## Lifecycle

WriterEditorService create_document_from_template sonrasi repository ve history state'ini normal create_document ile ayni sekilde kurar.

Desktop file-session untracked olarak reset edilir.

Template document Save/Open/TKO/DOCX/PDF pipeline'larinda normal WriterDocument olarak davranir.

## M1 siniri

- User-defined template persistence yok.
- Cloud template catalog yok.
- Marketplace yok.
- Template thumbnail asset pipeline yok.
- Dynamic field/merge token engine yok.

Bu faz yalniz extensible offline built-in foundation'dir.
