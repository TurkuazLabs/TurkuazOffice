// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/crates/turkuaz-office-writer/src/services/writer_editor_service.rs
// # 📌 Amac: Writer belge yasam dongusu, command ve undo/redo is kurallarini koordine eder
// # 📌 Modul - FileType: Writer - Rust
// # Version: 0.2.0
// # Aciklama: Repository, load, snapshot, command ve history yuzeyini ID Tool uzerinden saglar
// Bagimli Oldugu Katman: Service -> Repo -> Tool

use std::collections::HashMap;

use turkuaz_office_core::DocumentId;

use crate::config::constants::DEFAULT_HISTORY_LIMIT;
use crate::repositories::writer_document_repository::WriterDocumentRepository;
use crate::services::writer_command::{WriterCommand, WriterCommandError};
use crate::services::writer_command_service::WriterCommandService;
use crate::services::writer_document_factory_service::WriterDocumentFactoryService;
use crate::services::writer_types::WriterDocument;
use crate::tools::writer_id_tool::WriterIdTool;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriterEditorError {
    DocumentNotFound,
    Command(WriterCommandError),
    NothingToUndo,
    NothingToRedo,
}

impl From<WriterCommandError> for WriterEditorError {
    fn from(value: WriterCommandError) -> Self {
        Self::Command(value)
    }
}

#[derive(Default)]
struct CommandHistory {
    undo: Vec<WriterDocument>,
    redo: Vec<WriterDocument>,
}

pub struct WriterEditorService<R, I>
where
    R: WriterDocumentRepository,
    I: WriterIdTool,
{
    repository: R,
    id_tool: I,
    history: HashMap<DocumentId, CommandHistory>,
    history_limit: usize,
}

impl<R, I> WriterEditorService<R, I>
where
    R: WriterDocumentRepository,
    I: WriterIdTool,
{
    pub fn new(repository: R, id_tool: I) -> Self {
        Self {
            repository,
            id_tool,
            history: HashMap::new(),
            history_limit: DEFAULT_HISTORY_LIMIT,
        }
    }

    pub fn create_document(&mut self, title: impl Into<String>) -> WriterDocument {
        let document = WriterDocumentFactoryService::create(&self.id_tool, title);
        self.repository.save(document.clone());
        self.history
            .insert(document.id.clone(), CommandHistory::default());
        document
    }

    pub fn get_document(&self, id: &DocumentId) -> Option<WriterDocument> {
        self.repository.find(id)
    }

    pub fn load_document(&mut self, document: WriterDocument) -> WriterDocument {
        self.history
            .insert(document.id.clone(), CommandHistory::default());
        self.repository.save(document.clone());
        document
    }

    pub fn execute(
        &mut self,
        id: &DocumentId,
        command: WriterCommand,
    ) -> Result<WriterDocument, WriterEditorError> {
        let mut document = self
            .repository
            .find(id)
            .ok_or(WriterEditorError::DocumentNotFound)?;
        let before = document.clone();
        WriterCommandService::apply(&mut document, &command, &self.id_tool)?;

        let history = self.history.entry(id.clone()).or_default();
        history.undo.push(before);
        if history.undo.len() > self.history_limit {
            let overflow = history.undo.len() - self.history_limit;
            drop(history.undo.drain(0..overflow));
        }
        history.redo.clear();
        self.repository.save(document.clone());
        Ok(document)
    }

    pub fn execute_batch(
        &mut self,
        id: &DocumentId,
        commands: &[WriterCommand],
    ) -> Result<WriterDocument, WriterEditorError> {
        if commands.is_empty() {
            return Err(WriterEditorError::Command(WriterCommandError::EmptyCommand));
        }

        let mut document = self
            .repository
            .find(id)
            .ok_or(WriterEditorError::DocumentNotFound)?;
        let before = document.clone();

        for command in commands {
            WriterCommandService::apply(&mut document, command, &self.id_tool)?;
        }
        document.revision = before.revision.saturating_add(1);

        let history = self.history.entry(id.clone()).or_default();
        history.undo.push(before);
        if history.undo.len() > self.history_limit {
            let overflow = history.undo.len() - self.history_limit;
            drop(history.undo.drain(0..overflow));
        }
        history.redo.clear();
        self.repository.save(document.clone());
        Ok(document)
    }

    pub fn undo(&mut self, id: &DocumentId) -> Result<WriterDocument, WriterEditorError> {
        let current = self
            .repository
            .find(id)
            .ok_or(WriterEditorError::DocumentNotFound)?;
        let history = self.history.entry(id.clone()).or_default();
        let mut previous = history.undo.pop().ok_or(WriterEditorError::NothingToUndo)?;
        previous.revision = current.revision.saturating_add(1);
        history.redo.push(current);
        self.repository.save(previous.clone());
        Ok(previous)
    }

    pub fn redo(&mut self, id: &DocumentId) -> Result<WriterDocument, WriterEditorError> {
        let current = self
            .repository
            .find(id)
            .ok_or(WriterEditorError::DocumentNotFound)?;
        let history = self.history.entry(id.clone()).or_default();
        let mut next = history.redo.pop().ok_or(WriterEditorError::NothingToRedo)?;
        next.revision = current.revision.saturating_add(1);
        history.undo.push(current);
        self.repository.save(next.clone());
        Ok(next)
    }
}
