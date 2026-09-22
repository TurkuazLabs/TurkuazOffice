// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/controllers/writer_controller.rs
// # 📌 Amac: Writer request girdilerini alip yalnizca WriterEditorService cagirir
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Controller logic tasimadan create/load/snapshot/execute/undo/redo yuzeyi saglar
// Bagimli Oldugu Katman: Controller -> Service

use turkuaz_office_core::DocumentId;

use crate::repositories::writer_document_repository::WriterDocumentRepository;
use crate::services::writer_command::WriterCommand;
use crate::services::writer_editor_service::{WriterEditorError, WriterEditorService};
use crate::services::writer_types::WriterDocument;
use crate::tools::writer_id_tool::WriterIdTool;
use crate::views::writer_view::WriterDocumentView;

pub struct WriterController<R, I>
where
    R: WriterDocumentRepository,
    I: WriterIdTool,
{
    service: WriterEditorService<R, I>,
}

impl<R, I> WriterController<R, I>
where
    R: WriterDocumentRepository,
    I: WriterIdTool,
{
    pub fn new(service: WriterEditorService<R, I>) -> Self {
        Self { service }
    }

    pub fn create(&mut self, title: &str) -> WriterDocumentView {
        self.service.create_document(title).into()
    }

    pub fn get(&self, id: &str) -> Option<WriterDocumentView> {
        self.service
            .get_document(&DocumentId::new(id))
            .map(WriterDocumentView::from)
    }

    pub fn load(&mut self, document: WriterDocument) -> WriterDocumentView {
        self.service.load_document(document).into()
    }

    pub fn load_external(&mut self, document: WriterDocument) -> WriterDocumentView {
        self.service.load_external_document(document).into()
    }

    pub fn snapshot(&self, id: &str) -> Option<WriterDocument> {
        self.service.get_document(&DocumentId::new(id))
    }

    pub fn execute(
        &mut self,
        id: &str,
        command: WriterCommand,
    ) -> Result<WriterDocumentView, WriterEditorError> {
        self.service
            .execute(&DocumentId::new(id), command)
            .map(WriterDocumentView::from)
    }

    pub fn execute_batch(
        &mut self,
        id: &str,
        commands: &[WriterCommand],
    ) -> Result<WriterDocumentView, WriterEditorError> {
        self.service
            .execute_batch(&DocumentId::new(id), commands)
            .map(WriterDocumentView::from)
    }

    pub fn undo(&mut self, id: &str) -> Result<WriterDocumentView, WriterEditorError> {
        self.service
            .undo(&DocumentId::new(id))
            .map(WriterDocumentView::from)
    }

    pub fn redo(&mut self, id: &str) -> Result<WriterDocumentView, WriterEditorError> {
        self.service
            .redo(&DocumentId::new(id))
            .map(WriterDocumentView::from)
    }
}
