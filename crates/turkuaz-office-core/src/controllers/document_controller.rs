// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-core/src/controllers/document_controller.rs
// # 📌 Amac: Request benzeri girdileri alip sadece DocumentService cagirir ve View DTO dondurur
// # 📌 Modul - FileType: Core - Rust
// # Version: 0.2.0
// # Aciklama: Request benzeri girdileri alip sadece DocumentService cagirir ve View DTO dondurur
// Bagimli Oldugu Katman: Controller -> Service

use crate::repositories::document_repository::DocumentRepository;
use crate::services::document_service::DocumentService;
use crate::services::document_types::DocumentId;
use crate::tools::id_tool::IdTool;
use crate::views::document_view::DocumentView;

pub struct DocumentController<R, I>
where
    R: DocumentRepository,
    I: IdTool,
{
    service: DocumentService<R, I>,
}

impl<R, I> DocumentController<R, I>
where
    R: DocumentRepository,
    I: IdTool,
{
    pub fn new(service: DocumentService<R, I>) -> Self {
        Self { service }
    }

    pub fn create(&mut self, title: Option<&str>) -> DocumentView {
        self.service.create_document(title).into()
    }

    pub fn append_text(&mut self, id: &str, text: &str) -> Option<DocumentView> {
        self.service
            .append_text(&DocumentId::new(id), text)
            .map(DocumentView::from)
    }

    pub fn get(&self, id: &str) -> Option<DocumentView> {
        self.service
            .get_document(&DocumentId::new(id))
            .map(DocumentView::from)
    }
}
