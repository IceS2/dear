use crate::entity::note::Note;
use crate::repository::Repository;

type Error = &'static str;

pub struct InMemoryRepository {
    notes: Vec<Note>
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let notes: Vec<Note> = vec![];
        Self { notes }
    }
}

impl Repository for InMemoryRepository {
    fn insert(&mut self, note: Note) -> Result<Note, Error> {
        self.notes.push(note.clone());
        Ok(note)
    }

    fn list(&self) -> &Vec<Note> {
        &self.notes
    }
}
