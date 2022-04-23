use std::convert::Infallible;

use crate::entity::note::Note;
use super::{Repository, NoteIterator};

pub struct InMemoryRepository {
    notes: Vec<Note>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self { notes: vec![] }
    }
}

impl Repository for InMemoryRepository {
    type Error = Infallible;

    fn insert(&mut self, note: Note) -> Result<Note, Infallible> {
        self.notes.push(note.clone());
        Ok(note)
    }

    fn list(&self) -> Result<NoteIterator<'_>, Self::Error> {
        Ok(Box::new(self.notes.iter().cloned()))
    }
}

impl Default for InMemoryRepository {
    fn default() -> Self {
        InMemoryRepository::new()
    }
}
