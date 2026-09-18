# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/document-model.md
# 📌 Amac: Writer Sheet Slides icin ortak canonical belge modelinin gelisim prensiplerini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Schema version, revision, stabil node kimligi ve format bagimsizligini temel belge kontrati olarak sabitler

Bagimli Oldugu Katman: Documentation

# Document Model

## Amac

Document Model dosya formatindan, UI framework'ten ve render teknolojisinden bagimsiz canonical modeldir.

## Envelope

Tum editor belgeleri ortak envelope kavramini kullanir:

```text
DocumentEnvelope
|-- DocumentId
|-- DocumentKind
|-- SchemaVersion
|-- Revision
|-- Metadata
`-- ModuleContent
```

Writer, Sheet ve Slides `ModuleContent` icinde farkli domain modelleri kullanabilir. Writer block modeline diger editorler zorla baglanmaz.

## Writer icin planlanan temel agac

```text
WriterDocument
|-- Section[]
|   |-- PageSettings
|   `-- Block[]
|       |-- Paragraph
|       |   `-- TextRun[]
|       |-- Table
|       `-- Image
`-- ResourceRefs[]
```

## Stabil kimlik

Duzenlenebilir structural node stabil kimlik tasir. Kimlikler undo/redo, selection, comment, collaboration ve conflict resolution icin gereklidir.

## Schema version

`schema_version` disk formatinin document model semantigini belirtir. Application version ile ayni sey degildir. Eski schema migration service ile current modele tasinir.

## Revision

Her mutating command belge revision degerini ilerletir. Autosave, external-change ve synchronization bu revision semantigine dayanir.

## Format bagimsizligi

DOCX paragraph dogrudan Core paragraph degildir. Import adapter DOCX semantigini canonical modele map eder. Export adapter tersini yapar. Kayipsiz map edilemeyen ozellikler compatibility report ile gorunur hale getirilir.

## Serialization

Core domain type'larinin serialization sekli public API sayilmaz. Native `.tko` paket serializer ayri adapter/service sinirinda tutulur.
